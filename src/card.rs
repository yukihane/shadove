use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub card_id: u64,
    pub foil_card_id: u64,
    pub card_set_id: u64,
    pub card_name: Option<String>,
    pub is_foil: u8,
    pub char_type: u8,
    pub clan: Clan,
    pub tribe_name: String,
    pub skill: String,
    pub skill_condition: String,
    pub skill_target: String,
    pub skill_option: String,
    pub skill_preprocess: String,
    pub skill_disc: String,
    pub org_skill_disc: String,
    pub evo_skill_disc: String,
    pub org_evo_skill_disc: String,
    pub cost: u8,
    pub atk: u8,
    pub life: u8,
    pub evo_atk: u8,
    pub evo_life: u8,
    pub rarity: u8,
    pub get_red_ether: u32,
    pub use_red_ether: u32,
    pub description: String,
    pub evo_description: String,
    pub cv: String,
    pub copyright: Option<String>,
    pub base_card_id: u64,
    pub tokens: Option<String>,
    pub normal_card_id: u64,
    pub format_type: u8,
    pub restricted_count: u32,
}

// クラス
// 参考:
// https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html
// https://serde.rs/enum-number.html
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Clan {
    // ニュートラル
    Neutral,
    // エルフ
    Forestcraft,
    // ロイヤル
    Swordcraft,
    // ウィッチ
    Runecraft,
    // ドラゴン
    Dragoncraft,
    // ネクロマンサー
    Shadowcraft,
    // ヴァンパイア
    Bloodcraft,
    // ビショップ
    Havencraft,
    // ネメシス
    Portalcraft,
}
