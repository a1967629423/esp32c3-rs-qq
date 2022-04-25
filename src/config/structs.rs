use serde::{Serialize,Deserialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct WifiConfig<'a> {
    pub ssid:&'a str,
    pub pass:&'a str,
}
impl<'a> Default for WifiConfig<'a> {
    fn default() -> Self {
        Self {
            ssid:"SSID",
            pass:"PASS",
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct TimeConfig<'a> {
    pub time_host:&'a str,
}
impl<'a> Default for TimeConfig<'a> {
    fn default() -> Self {
        Self {
            time_host:"192.168.1.1:7000",
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct QQConfig<'a> {
    pub uin:i64,
    pub password:&'a str,
    pub group_code:i64,
    pub master_uin:i64
}
impl<'a> Default for QQConfig<'a> {
    fn default() -> Self {
        Self {
            uin:10000,
            password:"PASSWORD",
            group_code:10000,
            master_uin:10000
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct MQTTConfig<'a> {
    pub url:&'a str,
    pub receive_topic:&'a str,
    pub send_topic:&'a str
}
impl<'a> Default for MQTTConfig<'a> {
    fn default() -> Self {
        Self {
            url:"mqtt://192.168.1.1:1883",
            receive_topic:"qq/received",
            send_topic:"qq/send",
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AppConfig<'a> {
    #[serde(borrow)]
    pub wifi:WifiConfig<'a>,
    pub time:TimeConfig<'a>,
    pub qq:QQConfig<'a>,
    pub mqtt:Option<MQTTConfig<'a>>,

}
impl<'a> Default for AppConfig<'a> {
    fn default() -> Self {
        Self {
            wifi:WifiConfig::default(),
            time:TimeConfig::default(),
            qq:QQConfig::default(),
            mqtt:Some(MQTTConfig::default()),
        }
    }
}