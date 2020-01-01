use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub card_id: u64,
    pub foil_card_id: u64,
    pub card_set_id: CardSetId,
    pub card_name: Option<String>,
    pub is_foil: u8,
    pub char_type: CharType,
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
    // 通常は card_id と同じ値。
    // 別絵バージョンの場合、オリジナルのcard_idが入る。
    // 例: ジャンヌダルク 701741010
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
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

// カードパック
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u32)]
pub enum CardSetId {
    // ベーシック, ベーシックカードパック
    Basic = 10000,
    // CLC, クラシックカードパック
    Classic = 10001,
    // DRK, ダークネス・エボルヴ
    Darkness = 10002,
    // ROB, バハムート降臨
    Bahamut = 10003,
    // TOG, 神々の騒嵐
    Tempest = 10004,
    // WLD, ワンダーランド・ドリームズ
    Wonderland = 10005,
    // SFL, 星神の伝説
    Starforged = 10006,
    // CGS, 時空転生
    Chronogenesis = 10007,
    // DBN, 起源の光、終焉の闇
    Dawnbreak = 10008,
    // BOS, 蒼空の騎士
    Brigade = 10009,
    // OOT, 十禍絶傑
    Omen = 10010,
    // ALT, 次元歪曲
    Altersphere = 10011,
    // STR, 鋼鉄の反逆者
    Rebellion = 10012,
    // ROG, リバース・オブ・グローリー
    Glory = 10013,
    // VEC, 森羅咆哮
    Verdant = 10014,
    // UCL, アルティメットコロシアム
    Colosseum = 10015,
    // プライズ, 構築済みデッキ第1弾 純白の戦場
    Promo01 = 70001,
    // プライズ, 構築済みデッキ第1弾 純白の盾
    Promo02 = 70002,
    // プライズ, アニゲラコラボ
    Promo03 = 70003,
    // プライズ, 劇場版Fae[HF]コラボ
    Promo04 = 70004,
    // プライズ, 構築済みデッキ第4弾 (グラブルコラボ？)
    Promo05 = 70005,
    // プライズ, 構築済みデッキ第5弾
    Promo06 = 70006,
    // プライズ, プリンセスコネクト！Re:Diveコラボ
    Promo08 = 70008,
    // プライズ, ワンパンマンコラボ
    Promo09 = 70009,
    // プライズ, Re:ゼロから始める異世界生活コラボ
    Promo10 = 70010,
    // プライズ, 構築済みデッキ第6弾
    Promo11 = 70011,
    // プライズ, ラブライブ！スクールアイドルフェスティバルコラボ
    Promo12 = 70012,
    // プライズ, 涼宮ハルヒの憂鬱コラボ
    Promo13 = 70013,
    // 名称なし
    NotSpecified = 90000,
}

// カードの種類
// shadowverse-portal.com の表記は間違っているっぽい
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum CharType {
    Follower = 1,
    Amulet = 2,
    CountdownAmulet = 3,
    Spell = 4,
}
