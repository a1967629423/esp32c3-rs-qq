#[cfg(not(feature = "std"))]
use crate::simulate_std::prelude::*;
use derivative;

use crate::pb::msg;

// 不需要实现 Display，因为后面一定会跟 Text
#[derive(Default, Debug, Clone)]
pub struct MarketFace {
    pub name: String,
    pub face_id: Vec<u8>,
    pub tab_id: i32,
    pub item_type: i32,
    pub sub_type: i32,
    pub media_type: i32,
    pub encrypt_key: Vec<u8>,
    pub magic_value: String,
}

impl From<MarketFace> for Vec<msg::elem::Elem> {
    fn from(e: MarketFace) -> Self {
        vec![
            msg::elem::Elem::MarketFace(msg::MarketFace {
                face_name: Some(e.name.as_bytes().to_vec()),
                item_type: Some(e.item_type as u32),
                face_info: Some(1),
                face_id: Some(e.face_id),
                tab_id: Some(e.tab_id as u32),
                sub_type: Some(e.sub_type as u32),
                key: Some(e.encrypt_key),
                media_type: Some(e.media_type as u32),
                image_width: Some(200),
                image_height: Some(200),
                mobileparam: Some(e.magic_value.as_bytes().to_vec()),
                ..Default::default()
            }),
            msg::elem::Elem::Text(msg::Text {
                str: Some(e.name),
                ..Default::default()
            }),
        ]
    }
}

impl From<msg::MarketFace> for MarketFace {
    fn from(e: msg::MarketFace) -> Self {
        Self {
            name: String::from_utf8(e.face_name.unwrap_or_default()).unwrap_or_default(),
            face_id: e.face_id.unwrap_or_default(),
            tab_id: e.tab_id.unwrap_or_default() as i32,
            item_type: e.item_type.unwrap_or_default() as i32,
            sub_type: e.sub_type.unwrap_or_default() as i32,
            media_type: e.media_type.unwrap_or_default() as i32,
            encrypt_key: e.key.unwrap_or_default(),
            magic_value: String::from_utf8(e.mobileparam.unwrap_or_default()).unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Dice {
    pub value: i32,
}

impl Dice {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
impl From<Dice> for MarketFace {
    fn from(e: Dice) -> Self {
        Self {
            name: "[骰子]".into(),
            face_id: vec![
                72, 35, 211, 173, 177, 93, 240, 128, 20, 206, 93, 103, 150, 183, 110, 225,
            ],
            tab_id: 11464,
            item_type: 6,
            sub_type: 3,
            media_type: 0,
            encrypt_key: vec![
                52, 48, 57, 101, 50, 97, 54, 57, 98, 49, 54, 57, 49, 56, 102, 57,
            ],
            magic_value: format!("rscType?1;value={}", e.value - 1),
        }
    }
}

impl From<MarketFace> for Dice {
    fn from(e: MarketFace) -> Self {
        Self {
            value: e.magic_value.split('=').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap_or_default()
                + 1,
        }
    }
}

impl From<Dice> for Vec<msg::elem::Elem> {
    fn from(e: Dice) -> Self {
        MarketFace::from(e).into()
    }
}

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(Default)]
pub enum FingerGuessing {
    #[derivative(Default)]
    Rock,
    Scissors,
    Paper,
}
impl From<FingerGuessing> for MarketFace {
    fn from(e: FingerGuessing) -> Self {
        let value = match e {
            FingerGuessing::Rock => 0,
            FingerGuessing::Scissors => 1,
            FingerGuessing::Paper => 2,
        };
        MarketFace {
            name: "[猜拳]".into(),
            face_id: vec![
                131, 200, 162, 147, 174, 101, 202, 20, 15, 52, 129, 32, 167, 116, 72, 238,
            ],
            tab_id: 11415,
            item_type: 6,
            sub_type: 3,
            media_type: 0,
            encrypt_key: vec![
                55, 100, 101, 51, 57, 102, 101, 98, 99, 102, 52, 53, 101, 54, 100, 98,
            ],
            magic_value: format!("rscType?1;value={}", value),
        }
    }
}

impl From<FingerGuessing> for Vec<msg::elem::Elem> {
    fn from(e: FingerGuessing) -> Self {
        MarketFace::from(e).into()
    }
}

impl From<MarketFace> for FingerGuessing {
    fn from(e: MarketFace) -> Self {
        let value = e.magic_value.split('=').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap_or_default();
        match value {
            0 => Self::Rock,
            1 => Self::Scissors,
            2 => Self::Paper,
            _ => Self::Rock,
        }
    }
}
