use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    /// カードの識別子
    pub card_id: u64,
    /// おそらく対応するプレミアムカード(Animated;キラカード)の識別子だろう
    pub foil_card_id: u64,
    /// カードパック識別子
    pub card_set_id: CardSetId,
    /// カード名称
    pub card_name: Option<String>,
    /// プレミアムカードかどうかだろう。ただし現状常に0。
    pub is_foil: u8,
    /// カードの種類
    pub char_type: CharType,
    /// クラス
    pub clan: Clan,
    /// タイプ
    pub tribe_name: String,
    pub skill: String,
    pub skill_condition: String,
    pub skill_target: String,
    pub skill_option: String,
    pub skill_preprocess: String,
    /// スキル説明。(discというのはdesc(ription)の誤字だと思うがどうだろう)
    pub skill_disc: String,
    pub org_skill_disc: String,
    /// 進化後のスキル説明。
    pub evo_skill_disc: String,
    pub org_evo_skill_disc: String,
    /// PPコスト。
    pub cost: u8,
    /// 攻撃力。
    pub atk: u8,
    /// 体力。
    pub life: u8,
    /// 進化後の攻撃力。
    pub evo_atk: u8,
    /// 進化後の体力
    pub evo_life: u8,
    /// レアリティ
    pub rarity: Rarity,
    /// 分解時取得できるレッドエーテル量
    pub get_red_ether: u32,
    /// 生成に必要なレッドエーテル量
    pub use_red_ether: u32,
    /// フレーバーテキスト
    pub description: String,
    /// 進化状態のフレーバーテキスト
    pub evo_description: String,
    /// 声優
    pub cv: String,
    /// 著作者だと思うが現状は常に空。
    pub copyright: Option<String>,
    // 通常は card_id と同じ値。
    // 別絵バージョンの場合、オリジナルのcard_idが入る。
    // 例: ジャンヌダルク 701741010
    pub base_card_id: u64,
    /// 常に空。ちなみにトークン(トークンカード)というのは、召喚や錬成でのみ場に出るカードのことのようだ。
    /// フェアリー(900111010)やクレイゴーレム(900311020)が該当。
    pub tokens: Option<String>,
    /// 現状は常にcard_idと同じ。
    pub normal_card_id: u64,
    /// ローテーションなら1, それ以外は0。
    pub format_type: u8,
    /// デッキに含めることができる枚数上限。
    /// 通常3。マナリアの知識(107324010), ブラッドウルフ(101611050)が1。
    pub restricted_count: u32,
}

// クラス
// 参考:
// https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html
// https://serde.rs/enum-number.html
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Clan {
    /// ニュートラル
    Neutral,
    /// エルフ
    Forestcraft,
    /// ロイヤル
    Swordcraft,
    /// ウィッチ
    Runecraft,
    /// ドラゴン
    Dragoncraft,
    /// ネクロマンサー
    Shadowcraft,
    /// ヴァンパイア
    Bloodcraft,
    /// ビショップ
    Havencraft,
    /// ネメシス
    Portalcraft,
}

// カードパック
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u32)]
pub enum CardSetId {
    /// ベーシック, ベーシックカードパック
    Basic = 10000,
    /// CLC, クラシックカードパック
    Classic = 10001,
    /// DRK, ダークネス・エボルヴ
    Darkness = 10002,
    /// ROB, バハムート降臨
    Bahamut = 10003,
    /// TOG, 神々の騒嵐
    Tempest = 10004,
    /// WLD, ワンダーランド・ドリームズ
    Wonderland = 10005,
    /// SFL, 星神の伝説
    Starforged = 10006,
    /// CGS, 時空転生
    Chronogenesis = 10007,
    /// DBN, 起源の光、終焉の闇
    Dawnbreak = 10008,
    /// BOS, 蒼空の騎士
    Brigade = 10009,
    /// OOT, 十禍絶傑
    Omen = 10010,
    /// ALT, 次元歪曲
    Altersphere = 10011,
    /// STR, 鋼鉄の反逆者
    Rebellion = 10012,
    /// ROG, リバース・オブ・グローリー
    Glory = 10013,
    /// VEC, 森羅咆哮
    Verdant = 10014,
    /// UCL, アルティメットコロシアム
    Colosseum = 10015,
    /// プライズ, 構築済みデッキ第1弾
    PromoPrebuilt01 = 70001,
    /// プライズ, 構築済みデッキ第2弾
    PromoPrebuilt02 = 70002,
    /// プライズ, アニゲラコラボ
    PromoAnigera = 70003,
    /// プライズ, 劇場版Fae[HF]コラボ
    PromoFate = 70004,
    /// プライズ, 構築済みデッキ第4弾 (グラブルコラボ？)
    PromoPrebuilt04 = 70005,
    /// プライズ, 構築済みデッキ第5弾
    PromoPrebuilt05 = 70006,
    /// プライズ, プリンセスコネクト！Re:Diveコラボ
    PromoPrincess = 70008,
    /// プライズ, ワンパンマンコラボ
    PromoOnePunch = 70009,
    /// プライズ, Re:ゼロから始める異世界生活コラボ
    PromoReZero = 70010,
    /// プライズ, 構築済みデッキ第6弾
    PromoPrebuilt06 = 70011,
    /// プライズ, ラブライブ！スクールアイドルフェスティバルコラボ
    PromoLoveLive = 70012,
    /// プライズ, 涼宮ハルヒの憂鬱コラボ
    PromoHaruhi = 70013,
    /// 名称なし
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

/// レアリティ
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Rarity {
    Bronze = 1,
    Silver = 2,
    Gold = 3,
    Legendary = 4,
}
