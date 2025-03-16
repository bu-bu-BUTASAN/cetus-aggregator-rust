/**
 * Cetus Aggregator API Rustクライアント
 *
 * このライブラリはCetus Aggregator APIと通信するためのRustクライアントを提供します。
 * Cetus Aggregatorは、Suiブロックチェーン上での暗号資産交換の最適なルートを見つけるサービスです。
 *
 * # 使用例
 *
 * ```rust
 * use cetus_aggregator_rust::{AggregatorClient, AggregatorClientTrait, FindRouterParams};
 * use num_bigint::BigUint;
 * use std::str::FromStr;
 *
 * #[tokio::main]
 * async fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     // クライアントを初期化
 *     let client = AggregatorClient::new(None);
 *     
 *     // パラメータを準備
 *     let params = FindRouterParams {
 *         from: "0x2::sui::SUI".to_string(),
 *         target: "0x06864a6f921804860930db6ddbe2e16acdf8504495ea7481637a1c8b9a8fe54b::cetus::CETUS".to_string(),
 *         amount: BigUint::from_str("1000000000").unwrap(),
 *         by_amount_in: true,
 *         depth: Some(3),
 *         split_count: Some(1),
 *         providers: Some(vec!["CETUS".to_string()]),
 *         ..Default::default()
 *     };
 *     
 *     // ルート検索を実行
 *     let routes = client.find_routers(params).await?;
 *     
 *     // 結果を表示
 *     if let Some(route_data) = routes {
 *         println!("入力量: {}", route_data.amount_in);
 *         println!("出力量: {}", route_data.amount_out);
 *     }
 *     
 *     Ok(())
 * }
 * ```
 */
// モジュールをエクスポート
pub mod client;
pub mod error;
pub mod models;

// 主要な型をルートレベルでエクスポート
pub use client::{AggregatorClient, AggregatorClientTrait};
pub use error::{AggregatorError, AggregatorServerErrorCode, Result};
pub use models::{
    AggregatorResponse, ExtendedDetails, FindRouterParams, Path, PreSwapLpChangeParams, Router,
    RouterData, RouterError,
};
