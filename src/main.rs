#![allow(unused_imports)]
#![allow(clippy::single_component_path_imports)]
//#![feature(backtrace)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

pub mod provider;
pub mod handler;
pub mod config;

#[cfg(all(feature = "qemu", not(esp32)))]
compile_error!("The `qemu` feature can only be built for the `xtensa-esp32-espidf` target.");

#[cfg(all(feature = "ip101", not(esp32)))]
compile_error!("The `ip101` feature can only be built for the `xtensa-esp32-espidf` target.");

#[cfg(all(feature = "kaluga", not(esp32s2)))]
compile_error!("The `kaluga` feature can only be built for the `xtensa-esp32s2-espidf` target.");

#[cfg(all(feature = "ttgo", not(esp32)))]
compile_error!("The `ttgo` feature can only be built for the `xtensa-esp32-espidf` target.");

#[cfg(all(feature = "heltec", not(esp32)))]
compile_error!("The `heltec` feature can only be built for the `xtensa-esp32-espidf` target.");

#[cfg(all(feature = "esp32s3_usb_otg", not(esp32s3)))]
compile_error!(
    "The `esp32s3_usb_otg` feature can only be built for the `xtensa-esp32s3-espidf` target."
);
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Condvar, Mutex};
use std::task::Context;
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};

use anyhow::bail;

use log::*;

use provider::raw::RawTask;
use smol::future::FutureExt;
use url;

use smol;

use embedded_hal::adc::OneShot;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

use embedded_svc::eth;
use embedded_svc::eth::{Eth, TransitionalState};
use embedded_svc::httpd::registry::*;
use embedded_svc::httpd::*;
use embedded_svc::io;
use embedded_svc::ipv4;
use embedded_svc::mqtt::client::{Publish, QoS};
use embedded_svc::ping::Ping;
use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;
use embedded_svc::wifi::*;

use esp_idf_svc::eth::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::httpd as idf;
use esp_idf_svc::httpd::ServerRegistry;
use esp_idf_svc::mqtt::client::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sntp;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::*;
use esp_idf_svc::wifi::*;

use esp_idf_hal::adc;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;

use esp_idf_sys::esp;
use esp_idf_sys::{self, c_types};

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;

use ili9341;
use ssd1306;
use ssd1306::mode::DisplayConfig;
use st7789;

use epd_waveshare::{epd4in2::*, graphics::VarDisplay, prelude::*};
use log::info;
use nrs_qq::{msg::MessageChain, TJoinHandle};
use nrs_qq::TaskProvider;
use nrs_qq::{
    device::{Device,OSVersion},
    engine::{
        command::wtlogin::{LoginDeviceLocked, LoginResponse, LoginSuccess},
        get_timer_provider, init_random_provider, init_test_random_provider,
        init_timer_provider,
    },
    ext::common::after_login,
    handler::DefaultHandler,
    version::{get_version, Protocol},
    Client, TRwLock, TcpStreamProvider,
};

use std::time::Duration;
use handler::MyHandler;
use smol::Timer;
use provider::{task::GLOBAL_EXECUTOR,MyClientProvider,channel::MyChannelProvider,task::MyTaskProvider,oneshot::MyOneShotProvider,mutex::MyMutexProvider,rwlock::MyRwLockProvider,tcp::MyTcpStreamSyncProvider,engine::{MyRandomProvider,MyTimeProvider}};
use futures::StreamExt;
pub type MyClient = Client<MyClientProvider>;
use config::APP_CONFIG;

static UIN: i64 = APP_CONFIG.qq.uin;
static PASSWORD: &str = APP_CONFIG.qq.password;
static TIME_HOST:&str = APP_CONFIG.time.time_host;
pub static GROUP_CODE: i64 = APP_CONFIG.qq.group_code;
pub static MASTER_UIN:i64 = APP_CONFIG.qq.master_uin;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    test_print();

    test_atomics();

    esp_idf_svc::log::EspLogger::initialize_default();

    #[allow(unused)]
    let netif_stack = Arc::new(EspNetifStack::new()?);
    #[allow(unused)]
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    #[allow(unused)]
    let default_nvs = Arc::new(EspDefaultNvs::new()?);

    #[allow(clippy::redundant_clone)]
    #[cfg(not(feature = "qemu"))]
    #[allow(unused_mut)]
    let mut wifi = match wifi(
            netif_stack.clone(),
            sys_loop_stack.clone(),
            default_nvs.clone(),
        ) {
        Ok(o) => {o},
        Err(e) => {
            log::error!("{:?}",e);
            unsafe {
                esp_idf_sys::esp_wifi_disconnect();
                esp_idf_sys::esp_wifi_stop();
                esp_idf_sys::esp_wifi_deinit();
                esp_idf_sys::esp_restart();
                
            }
            panic!("unreachable");
        },
    };

    main_app_async().ok();

    #[cfg(not(feature = "qemu"))]
    {
        drop(wifi);
        info!("Wifi stopped");
    }

    Ok(())
}

#[allow(clippy::vec_init_then_push)]
fn test_print() {
    // Start simple
    println!("Hello from Rust!");

    // Check collections
    let mut children = vec![];

    children.push("foo");
    children.push("bar");
    println!("More complex print {:?}", children);
}

#[allow(deprecated)]
fn test_atomics() {
    let a = AtomicUsize::new(0);
    let v1 = a.compare_and_swap(0, 1, Ordering::SeqCst);
    let v2 = a.swap(2, Ordering::SeqCst);

    let (r1, r2) = unsafe {
        // don't optimize our atomics out
        let r1 = core::ptr::read_volatile(&v1);
        let r2 = core::ptr::read_volatile(&v2);

        (r1, r2)
    };

    println!("Result: {}, {}", r1, r2);
}


pub fn make_message(msg: String) -> MessageChain {
    use nrs_qq::engine::pb::msg::{elem::Elem as EElem, Text};
    MessageChain::new(vec![EElem::Text(Text {
        str: Some(msg),
        ..Default::default()
    })])
}

async fn app_async() {
    // 这里进行的是拉取时间
    log::info!("init timer");
    let my_timer = MyTimeProvider::new_form_net_sync(TIME_HOST);
    // 理论上没有拉取时间也行，可以考虑使用下面的代码
    // let my_timer = MyTimeProvider::new();
    // unsafe {
    //     info!("random test {}",esp_idf_sys::esp_random());
    // }
    init_timer_provider(Box::new(my_timer));
    log::info!(
        "init timer success now:{}",
        get_timer_provider().now_timestamp_nanos()
    );
    unsafe {
        init_random_provider(Box::new(MyRandomProvider::new(
            esp_idf_sys::esp_random() as u64
        )));
    }
    info!("init engine success");
    let my_device: Device = include!("../out/device.out");
    let device = my_device;
    info!("gen device success");
    let client = Arc::new(MyClient::new(
        device,
        get_version(Protocol::IPad),
        MyHandler::new(),
    ));
    info!("new client success");
    let stream = MyTcpStreamSyncProvider::connect(client.get_address())
        .await
        .unwrap();
    info!("connect success");
    let c = client.clone();
    let handle = MyTaskProvider::spawn(async move { c.start(stream).await });

    // smol::spawn(async move {
    //     c.start(stream).await;
    // })
    // .detach();
    info!("start success");
    MyTaskProvider::yield_now().await;
    // tracing::info!("准备登录");
    info!("to login");
    let mut resp = match client.password_login(UIN, PASSWORD).await {
        Ok(resp) => resp,
        Err(e) => {
            info!("failed to login with password {:?}",e);
            return;
        }
    };
    loop {
        match resp {
            LoginResponse::Success(LoginSuccess {
                ref account_info, ..
            }) => {
                // tracing::info!("login success: {:?}", account_info);
                info!("login success: {:?}", account_info);
                break;
            }
            LoginResponse::DeviceLocked(LoginDeviceLocked {
                ref sms_phone,
                ref verify_url,
                ref message,
                ..
            }) => {
                info!("device locked: {:?}", message);
                info!("sms_phone: {:?}", sms_phone);
                info!("verify_url: {:?}", verify_url);
                info!("手机打开url，处理完成后重启程序");
                std::process::exit(0);
                //也可以走短信验证
                // resp = client.request_sms().await.expect("failed to request sms");
            }
            LoginResponse::DeviceLockLogin { .. } => {
                resp = client
                    .device_lock_login()
                    .await
                    .expect("failed to login with device lock");
            }
            LoginResponse::AccountFrozen => {
                log::info!("account frozen");
            }
            LoginResponse::TooManySMSRequest => {
                log::info!("too many sms request");
            }
            _ => {
                log::info!("unknown login status: ");
            }
        }
    }
    info!("{:?}", resp);

    after_login(&client).await;
    {
        // client
        //     .reload_friends()
        //     .await
        //     .expect("failed to reload friend list");
        // info!("{:?}", client.friends.read().await);

        // client
        //     .reload_groups()
        //     .await
        //     .expect("failed to reload group list");
        // let group_list = client.groups.read().await;
        // info!("{:?}", group_list);
    }
    {
        // let d = client.get_allowed_clients().await;
        // info!("{:?}", d);
    }
    // match client
    //     .send_group_message(GROUP_CODE, make_message("测试发送 by esp32c3".to_string()))
    //     .await
    // {
    //     Ok(_) => {
    //         log::info!("send success");
    //     }
    //     Err(e) => {
    //         log::info!("send error {:?}", e);
    //     }
    // };
    //timer(client, GROUP_CODE).await;
    match handle.await {
        Ok(_) => {}
        Err(e) => {
            log::info!("handle error {:?}", e);
        }
    }
}

fn main_app_async() -> anyhow::Result<()> {
    use std::{task::Context, time::Duration};

    use crate::provider::{self, raw::RawTask, task::GLOBAL_EXECUTOR};

    // esp_idf_sys::esp!(unsafe {
    //     esp_idf_sys::esp_vfs_eventfd_register(&esp_idf_sys::esp_vfs_eventfd_config_t {
    //         max_fds: 5,
    //         ..Default::default()
    //     })
    // })?;

    match thread::Builder::new()
        .stack_size(64 * 1024)
        .name("Main".into())
        .spawn(move || {
            let ex = &GLOBAL_EXECUTOR;
            ex.spawn(async move {
                log::info!("app call");
                app_async().await;
            })
            .detach();
            let mut fu = Box::pin(ex.run(futures::future::pending::<()>()));
            let task = RawTask::new();
            let mut poll_count = 0;
            loop {
                match fu.poll(&mut Context::from_waker(&task.to_waker())) {
                    std::task::Poll::Ready(..) => {
                        log::info!("ready");
                        break;
                    }
                    std::task::Poll::Pending => {}
                }
                poll_count+=1;
                if poll_count >= 8 {
                    poll_count = 0;
                    thread::sleep(Duration::from_millis(500));
                } else {
                    thread::sleep(Duration::from_millis(50));
                }
                
            }
        })?
        .join()
    {
        Ok(_) => {}
        Err(e) => {
            log::info!("main thread error {:?}",e);
        }
    }

    Ok(())
}
#[cfg(feature = "experimental")]
mod experimental {

}

#[cfg(not(feature = "qemu"))]
#[allow(dead_code)]
fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    info!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == APP_CONFIG.wifi.ssid);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            APP_CONFIG.wifi.ssid, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            APP_CONFIG.wifi.ssid
        );
        None
    };


    // 不要开启AP，当前版本AP设置定时器时可能会导致崩溃
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: APP_CONFIG.wifi.ssid.to_string(),
        password: APP_CONFIG.wifi.pass.to_string(),
        channel,
        ..Default::default()
    }))?;

    info!("Wifi configuration set, about to get status");
    // 解决一下esp重启后下方的with_timeout可能不工作的问题
    thread::sleep(Duration::from_secs(5));
    wifi.wait_status_with_timeout(Duration::from_secs(60), |status| matches!(status,Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(_))),
        ApStatus::Stopped,
    )))
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(_))),
        ApStatus::Stopped,
    ) = status
    {
        info!("Wifi connected");
        // ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}
