use std::pin::Pin;
use std::sync::{RwLock, Weak, Arc};

use super::provider::{
    channel::MyChannelProvider, mutex::MyMutexProvider, oneshot::MyOneShotProvider,
    rwlock::MyRwLockProvider, task::MyTaskProvider, tcp::MyTcpStreamSyncProvider,
    MyClientProvider
};
use super::{GROUP_CODE, MASTER_UIN};
use crate::make_message;
use crate::provider::raw::RawTask;
use embedded_svc::mqtt::client::{Client, Connection, Message, Publish,Event};
use esp_idf_svc::mqtt::client::{
    EspMqttClient, EspMqttConnection, EspMqttMessage, MqttClientConfiguration,
};
use futures::Future;
use nrs_qq::{TaskProvider, TJoinHandle};
use nrs_qq::engine::{get_random_provider, get_timer_provider};
use nrs_qq::msg::MessageChain;
use nrs_qq::{
    engine::pb::msg::{elem::Elem, Text},
    msg::elem::RQElem,
    Handler, QEvent,
};
use super::MyClient;
type MyQEvent = QEvent<MyClientProvider>;
// static MQTT_URL: &str = "mqtt://192.168.123.166:1883";
// static MQTT_RECEIVED_TOPIC: &str = "qq/received";
// static MQTT_SEND_TOPIC:&str = "qq/send_group";
use super::config::APP_CONFIG;
struct MyHandlerInner {
    start: std::time::Instant,
    mqtt_client: RwLock<Option<RwLock<EspMqttClient>>>,
    client:RwLock<Option<Weak<MyClient>>>
}
unsafe impl Sync for MyHandlerInner {}
impl MyHandlerInner {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
            mqtt_client: RwLock::new(None),
            client:RwLock::new(None),
        }
    }
    pub fn init_mtqq(self:Arc<Self>)  {
        if let (Some(mqtt_url),Some(mqtt_send_topic)) = (APP_CONFIG.mqtt.as_ref().map(|v|v.url),APP_CONFIG.mqtt.as_ref().map(|v|v.send_topic))  {
            let (client, mut conn_thread) = match EspMqttClient::new(mqtt_url, &MqttClientConfiguration::default())
            {
                Ok(o) => o,
                Err(e) => {
                    log::info!("connect mqtt error {:?}", e);
                    panic!("{:?}", e);
                }
            };
            log::info!("connect mqtt success");
    
            self.mqtt_client.write().unwrap().replace(RwLock::new(client));
            let set_mqtt_self = self.clone();
            let (sender,receiver) = smol::channel::bounded::<(i64,MessageChain)>(2);
            let channel_self = self.clone();
            MyTaskProvider::spawn(async move {
                while let Ok((group_code,message_chain)) = receiver.recv().await {
                    let upgrade_res = match channel_self.client.read().unwrap().as_ref() {
                        Some(s) => {s.upgrade()},
                        None => {continue},
                    };
                    if let Some(client) = upgrade_res {
                        match client.send_group_message(group_code, message_chain).await {
                            Ok(_) => {},
                            Err(e) => {
                                log::info!("send group message error {:?}",e);
                            },
                        };
                    }
                }
            }).detach();
            std::thread::Builder::new().stack_size(4*1024).spawn(move || {
                while let Some(msg) = conn_thread.next() {
                    match msg {
                        Ok(e) => {
                            match e {
                                Event::Received(r) => {
                                    if let Some(topic_cow) = r.retrieve_topic() {
                                        if &topic_cow == mqtt_send_topic {
                                            if let Ok((group_id,msg_chain)) = decode_group_send_message_cmd(r.data()) {
                                                sender.try_send((group_id,msg_chain)).ok();
                                            } else {
                                                log::info!("decode error");
                                            }
                                        } 
                                    }
                                },
                                _ => {}
                            }
                            
                        },
                        Err(e) => log::info!("MQTT Error {:?}", e),
                    }
    
                }
                log::info!("exit mqtt thread");
            }).ok();
            std::thread::sleep(std::time::Duration::from_millis(500));
            let guard = set_mqtt_self.mqtt_client.read().unwrap();
            let guard_ref = guard.as_ref();
            let mut write_guard = guard_ref.unwrap().write().unwrap();
            match write_guard.subscribe(mqtt_send_topic,embedded_svc::mqtt::client::QoS::AtMostOnce) {
                Ok(o) => {
                    log::info!("subscribe success {}",o);
                },
                Err(e) => {
                    log::info!("subscribe error {:?}", e);
                },
            }
        }



    }
}
pub struct MyHandler (Arc<MyHandlerInner>);
impl MyHandler {
    pub fn new() -> Self {
        let inner = Arc::new(MyHandlerInner::new());
        inner.clone().init_mtqq();
        Self (inner)
    }


}

fn encode_group_message(group_code: i64, from_uin: i64, content: &str) -> Vec<u8> {
    let content_bytes = content.as_bytes();
    let mut bytes = Vec::<u8>::with_capacity(16 + content_bytes.len());
    for i in 0..8 {
        bytes.push(((group_code >> (i * 8)) & 0xff) as u8);
    }
    for i in 0..8 {
        bytes.push(((from_uin >> (i * 8)) & 0xff) as u8);
    }

    bytes.extend(content_bytes);
    bytes
}
#[allow(dead_code)]
fn decode_group_message(bytes: impl AsRef<[u8]>) -> (i64, i64, String) {
    let bytes = bytes.as_ref();
    let mut group_code = 0;
    let mut from_uin = 0;
    for i in 0..8 {
        group_code |= ((bytes[i] as i64) << (i * 8)) as i64;
    }
    for i in 0..8 {
        from_uin |= ((bytes[i + 8] as i64) << (i * 8)) as i64;
    }
    let content = String::from_utf8_lossy(&bytes[16..]).to_string();
    (group_code, from_uin, content)
}

fn decode_group_send_message_cmd(
    bytes: impl AsRef<[u8]>,
) -> Result<(i64, MessageChain), &'static str> {
    let bytes = bytes.as_ref();
    let mut group_code = 0;

    for i in 0..8 {
        group_code |= ((bytes[i] as i64) << (i * 8)) as i64;
    }

    let cmd_msg = &bytes[8..];
    let content = match String::from_utf8(cmd_msg.to_vec()) {
        Ok(o) => o,
        Err(_) => {
            return Err("decode group send message cmd error");
        }
    };
    let msg_chain = MessageChain::new(
        [Elem::Text(Text {
            str: Some(content),
            ..Default::default()
        })]
        .to_vec(),
    );

    //(group_code,from_uin,content)
    Ok((group_code, msg_chain))
}

impl
    Handler<
        MyClientProvider
    > for MyHandlerInner
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    fn handle(&self, event: MyQEvent) -> Self::Future {
        let start = self.start;
        match event {
            QEvent::GroupMessage(e) => {
                // if e.message.group_code != GROUP_CODE {
                //     log::info!("group code {}", e.message.group_code);
                //     return Box::pin(futures::future::ready(()));
                // }
                // if e.message.from_uin != MASTER_UIN {
                //     log::info!("from uin {}", e.message.from_uin);
                //     return Box::pin(futures::future::ready(()));
                // }
                if self.client.read().unwrap().is_none() {
                    self.client.write().unwrap().replace(Arc::downgrade(&e.client));
                }
                let mut content: Option<String> = None;
                for el in e.message.elements {
                    match el {
                        RQElem::Text(t) => {
                            content.replace(t.content);
                            break;
                        }
                        _ => {}
                    }
                }
                if content.is_none() {
                    return Box::pin(futures::future::ready(()));
                }
                let content = content.unwrap_or_default();
                if let Some(mqtt_client) = self.mqtt_client.read().unwrap().as_ref() {
                    if let Some(receive_topic) = APP_CONFIG.mqtt.as_ref().map(|v|v.receive_topic) {
                        match mqtt_client.write().unwrap().publish(
                            receive_topic,
                            embedded_svc::mqtt::client::QoS::AtMostOnce,
                            true,
                            encode_group_message(e.message.group_code, e.message.from_uin, &content),
                        ) {
                            Ok(o) => {
                                log::info!("send mqtt success {}", o);
                            }
                            Err(e) => {
                                log::info!("send mqtt error {:?}", e);
                            }
                        };
                    }

                }

                // log::info!("content {}",content);
                Box::pin(async move {
                    if content == "?" || content == "？" {
                        e.client
                            .send_group_message(e.message.group_code, make_message("?".to_string()))
                            .await
                            .ok();
                        return;
                    }
                    if !content.starts_with("esp") {
                        return;
                    }
                    let master_content = content
                        .split("esp")
                        .skip(1)
                        .next()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    match master_content.as_str() {
                        "时间" => {
                            e.client
                                .send_group_message(
                                    e.message.group_code,
                                    make_message(format!(
                                        "当前时间:{}s",
                                        get_timer_provider().now_timestamp()
                                    )),
                                )
                                .await
                                .ok();
                        }
                        "随机" => {
                            e.client
                                .send_group_message(
                                    e.message.group_code,
                                    make_message(format!(
                                        "随机结果 {}",
                                        get_random_provider().next_u32()
                                    )),
                                )
                                .await
                                .ok();
                        }
                        "Info" => {
                            // e.client.send_group_message(e.message.group_code, make_message("?".to_string())).await.ok();
                            let version = unsafe { *esp_idf_sys::esp_get_idf_version() };
                            let free_size = unsafe { esp_idf_sys::esp_get_free_heap_size() };
                            let free_size = free_size as f64 / 1024.0;
                            let running_time = start.elapsed();
                            let message = make_message(format!(
                                "IDF版本:{} 可用内存:{:.2}KB 运行时间 {:?}",
                                version, free_size, running_time
                            ));
                            e.client
                                .send_group_message(e.message.group_code, message)
                                .await
                                .ok();
                        }
                        v @ _ => {
                            e.client
                                .send_group_message(
                                    e.message.group_code,
                                    make_message(format!("未知指令 {}", v)),
                                )
                                .await
                                .ok();
                        } // _ => {
                          //     self.client.publish("/qq/message", embedded_svc::mqtt::client::QoS::AtMostOnce, false, payload)
                          // }
                    }
                })
            }
            _ => Box::pin(futures::future::ready(())),
        }
    }
}
impl
    Handler<
        MyClientProvider
    > for MyHandler
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    fn handle(&self, event: MyQEvent) -> Self::Future {
        self.0.handle(event)
    }
}