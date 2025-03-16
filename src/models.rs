/**
 * Cetus Aggregator APIのデータモデル定義
 *
 * このモジュールはAPIとの通信に使用するデータ構造を定義します。
 */
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ルート検索のためのパラメータ
#[derive(Debug, Serialize, Clone)]
pub struct FindRouterParams {
    /// 交換元コインのアドレス
    pub from: String,
    /// 交換先コインのアドレス
    pub target: String,
    /// 交換する金額
    #[serde(serialize_with = "serialize_biguint")]
    pub amount: BigUint,
    /// 入力量ベースで計算するかどうか
    #[serde(rename = "by_amount_in")]
    pub by_amount_in: bool,
    /// 検索の深さ（オプション）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    /// 分割アルゴリズム（オプション）
    #[serde(rename = "split_algorithm", skip_serializing_if = "Option::is_none")]
    pub split_algorithm: Option<String>,
    /// 分割係数（オプション）
    #[serde(rename = "split_factor", skip_serializing_if = "Option::is_none")]
    pub split_factor: Option<f64>,
    /// 分割数（オプション）
    #[serde(rename = "split_count", skip_serializing_if = "Option::is_none")]
    pub split_count: Option<u32>,
    /// 使用するプロバイダーのリスト（オプション）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<String>>,
    /// 流動性変更のシミュレーション（オプション）
    #[serde(rename = "liquidity_changes", skip_serializing_if = "Option::is_none")]
    pub liquidity_changes: Option<Vec<PreSwapLpChangeParams>>,
}

/// BigUintをシリアライズするためのヘルパー関数
fn serialize_biguint<S>(value: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

impl Default for FindRouterParams {
    fn default() -> Self {
        Self {
            from: String::new(),
            target: String::new(),
            amount: BigUint::from(0u32),
            by_amount_in: true,
            depth: None,
            split_algorithm: None,
            split_factor: None,
            split_count: None,
            providers: None,
            liquidity_changes: None,
        }
    }
}

/// 流動性変更のパラメータ
#[derive(Debug, Serialize, Clone)]
pub struct PreSwapLpChangeParams {
    /// プールのID
    #[serde(rename = "pool")]
    pub pool_id: String,
    /// 下限ティック
    #[serde(rename = "tick_lower")]
    pub tick_lower: i32,
    /// 上限ティック
    #[serde(rename = "tick_upper")]
    pub tick_upper: i32,
    /// 流動性の変更量
    #[serde(rename = "delta_liquidity")]
    pub delta_liquidity: i64,
}

/// パス情報
#[derive(Debug, Deserialize, Clone)]
pub struct Path {
    /// パスのID
    pub id: String,
    /// 方向（真の場合A→B、偽の場合B→A）
    pub direction: bool,
    /// プロバイダー名
    pub provider: String,
    /// 交換元コイン
    pub from: String,
    /// 交換先コイン
    pub target: String,
    /// 手数料率
    #[serde(rename = "fee_rate")]
    pub fee_rate: String,
    /// 入力量
    #[serde(rename = "amount_in")]
    pub amount_in: u64,
    /// 出力量
    #[serde(rename = "amount_out")]
    pub amount_out: u64,
    /// バージョン（オプション）
    pub version: Option<String>,
    /// 拡張詳細情報（オプション）
    #[serde(rename = "extended_details")]
    pub extended_details: Option<ExtendedDetails>,
}

/// 拡張詳細情報
#[derive(Debug, Deserialize, Clone)]
pub struct ExtendedDetails {
    // 各プロバイダー固有の追加情報
    #[serde(rename = "aftermath_pool_flatness")]
    pub aftermath_pool_flatness: Option<f64>,
    #[serde(rename = "aftermath_lp_supply_type")]
    pub aftermath_lp_supply_type: Option<String>,
    #[serde(rename = "turbos_fee_type")]
    pub turbos_fee_type: Option<String>,
    #[serde(rename = "after_sqrt_price")]
    pub after_sqrt_price: Option<u128>,
    #[serde(rename = "deepbookv3_deep_fee")]
    pub deepbookv3_deep_fee: Option<f64>,
    #[serde(rename = "scallop_scoin_treasury")]
    pub scallop_scoin_treasury: Option<String>,
    #[serde(rename = "haedal_pmm_base_price_seed")]
    pub haedal_pmm_base_price_seed: Option<String>,
    #[serde(rename = "haedal_pmm_quote_price_seed")]
    pub haedal_pmm_quote_price_seed: Option<String>,
    #[serde(rename = "steamm_bank_a")]
    pub steamm_bank_a: Option<String>,
    #[serde(rename = "steamm_bank_b")]
    pub steamm_bank_b: Option<String>,
    #[serde(rename = "steamm_lending_market")]
    pub steamm_lending_market: Option<String>,
    #[serde(rename = "steamm_lending_market_type")]
    pub steamm_lending_market_type: Option<String>,
    #[serde(rename = "steamm_btoken_a_type")]
    pub steamm_btoken_a_type: Option<String>,
    #[serde(rename = "steamm_btoken_b_type")]
    pub steamm_btoken_b_type: Option<String>,
    #[serde(rename = "steamm_lp_token_type")]
    pub steamm_lp_token_type: Option<String>,
}

/// ルーター情報
#[derive(Debug, Deserialize, Clone)]
pub struct Router {
    /// パスのリスト
    pub path: Vec<Path>,
    /// 入力量
    #[serde(rename = "amount_in")]
    pub amount_in: u64,
    /// 出力量
    #[serde(rename = "amount_out")]
    pub amount_out: u64,
    /// 初期価格
    #[serde(rename = "initial_price")]
    pub initial_price: String,
}

/// エラー情報
#[derive(Debug, Deserialize, Clone)]
pub struct RouterError {
    /// エラーコード
    pub code: u32,
    /// エラーメッセージ
    pub msg: String,
}

/// ルーター検索結果データ
#[derive(Debug, Deserialize, Clone)]
pub struct RouterData {
    /// 入力量
    #[serde(rename = "amount_in")]
    pub amount_in: u64,
    /// 出力量
    #[serde(rename = "amount_out")]
    pub amount_out: u64,
    /// 入力量ベースフラグ
    #[serde(rename = "by_amount_in", default)]
    pub by_amount_in: bool,
    /// ルーターのリスト
    pub routes: Vec<Router>,
    /// 流動性不足フラグ
    #[serde(rename = "insufficient_liquidity", default)]
    pub insufficient_liquidity: bool,
    /// パッケージマップ（オプション）
    pub packages: Option<HashMap<String, String>>,
    /// 深層手数料総額（オプション）
    #[serde(rename = "total_deep_fee")]
    pub total_deep_fee: Option<f64>,
    /// エラー情報（オプション）
    pub error: Option<RouterError>,
}

/// アグリゲーターレスポンス
#[derive(Debug, Deserialize)]
pub struct AggregatorResponse {
    /// ステータスコード
    pub code: u32,
    /// ステータスメッセージ
    pub msg: String,
    /// レスポンスデータ
    pub data: Option<RouterData>,
}
