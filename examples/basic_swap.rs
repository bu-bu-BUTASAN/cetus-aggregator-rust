/**
 * Cetus Aggregator Rustクライアントの基本的な使用例
 *
 * このサンプルは、SUIからCETUSへの交換ルートを検索する方法を示しています。
 */
use cetus_aggregator_rust::{AggregatorClient, AggregatorClientTrait, FindRouterParams};
use num_bigint::BigUint;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cetus Aggregator APIを使用したルート検索の例");
    println!("=============================================");

    // クライアントを初期化
    let client = AggregatorClient::new(None);
    println!(
        "クライアントを初期化しました: エンドポイント = {}",
        client.endpoint
    );

    // パラメータを準備
    let params = FindRouterParams {
        from: "0x2::sui::SUI".to_string(),
        target: "0x06864a6f921804860930db6ddbe2e16acdf8504495ea7481637a1c8b9a8fe54b::cetus::CETUS"
            .to_string(),
        amount: BigUint::from_str("1_000_000_000").unwrap(), // 1 SUI
        by_amount_in: true,
        depth: Some(3),       // 最大スワップ回数(3回まで)
        split_count: Some(1), // 最大分割数(3ルートまで)
        providers: Some(vec!["CETUS".to_string()]),
        ..Default::default()
    };

    println!("\n検索パラメータ:");
    println!("  交換元: {}", params.from);
    println!("  交換先: {}", params.target);
    println!(
        "  金額: {} (入力固定: {})",
        params.amount, params.by_amount_in
    );
    println!("  深さ: {:?}", params.depth);
    println!("  プロバイダー: {:?}", params.providers);

    println!("\nルート検索を実行中...");

    // ルート検索を実行
    match client.find_routers(params).await {
        Ok(Some(route_data)) => {
            println!("\n検索結果:");
            println!("  入力量: {}", route_data.amount_in);
            println!("  出力量: {}", route_data.amount_out);
            println!("  ルート数: {}", route_data.routes.len());

            // 各ルートの詳細を表示
            for (i, route) in route_data.routes.iter().enumerate() {
                println!("\nルート {}:", i + 1);
                println!("  初期価格: {}", route.initial_price);
                println!("  入力量: {}", route.amount_in);
                println!("  出力量: {}", route.amount_out);

                // 各パスの詳細を表示
                for (j, path) in route.path.iter().enumerate() {
                    println!("\n  パス {}:", j + 1);
                    println!("    プールID: {}", path.id);
                    println!("    プロバイダー: {}", path.provider);
                    println!("    方向: {}", if path.direction { "A->B" } else { "B->A" });
                    println!("    交換元: {}", path.from);
                    println!("    交換先: {}", path.target);
                    println!("    入力量: {}", path.amount_in);
                    println!("    出力量: {}", path.amount_out);
                    println!("    手数料率: {}", path.fee_rate);
                    if let Some(details) = &path.extended_details {
                        if let Some(after_sqrt_price) = &details.after_sqrt_price {
                            println!("    スクエアルート価格: {}", after_sqrt_price);
                        }
                    }
                    println!();
                }
            }
        }
        Ok(None) => {
            println!("\n適切なルートが見つかりませんでした。");
        }
        Err(e) => {
            println!("\nエラーが発生しました: {}", e);
        }
    }

    Ok(())
}
