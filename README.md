# Cetus Aggregator Rust Client

Cetus Aggregator APIと通信するためのRustクライアントライブラリです。このライブラリを使用すると、Suiブロックチェーン上での暗号資産交換の最適なルートを検索できます。

## 機能

- 交換ルートの検索
- 複数のプロバイダーからの最適なルートの取得
- 流動性変更のシミュレーション
- 詳細なルート情報の取得

## インストール

Cargo.tomlに以下を追加してください：

```toml
[dependencies]
cetus-aggregator-rust = { git = "https://github.com/bu-bu-BUTASAN/cetus-aggregator-rust" }
```

## 使用例

### 基本的な使用方法

```rust
use cetus_aggregator_rust::{AggregatorClient, AggregatorClientTrait, FindRouterParams};
use num_bigint::BigUint;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // クライアントを初期化
    let client = AggregatorClient::new(None);
    
    // パラメータを準備
    let params = FindRouterParams {
        from: "0x2::sui::SUI".to_string(),
        target: "0x06864a6f921804860930db6ddbe2e16acdf8504495ea7481637a1c8b9a8fe54b::cetus::CETUS".to_string(),
        amount: BigUint::from_str("1000000000").unwrap(),
        by_amount_in: true,
        depth: Some(3),
        split_count: Some(1),
        providers: Some(vec!["CETUS".to_string()]),
        ..Default::default()
    };
    
    // ルート検索を実行
    let routes = client.find_routers(params).await?;
    
    // 結果を表示
    if let Some(route_data) = routes {
        println!("入力量: {}", route_data.amount_in);
        println!("出力量: {}", route_data.amount_out);
        
        for (i, route) in route_data.routes.iter().enumerate() {
            println!("ルート {}: 初期価格 = {}", i + 1, route.initial_price);
            
            for (j, path) in route.path.iter().enumerate() {
                println!("  パス {}: プロバイダー = {}", j + 1, path.provider);
                println!("    プールID: {}", path.id);
                println!("    方向: {}", if path.direction { "A->B" } else { "B->A" });
                println!("    交換元: {}", path.from);
                println!("    交換先: {}", path.target);
                println!("    入力量: {}", path.amount_in);
                println!("    出力量: {}", path.amount_out);
                println!("    手数料率: {}", path.fee_rate);
                // 拡張詳細情報（スクエアルート価格など）を表示
                if let Some(details) = &path.extended_details {
                    if let Some(after_sqrt_price) = &details.after_sqrt_price {
                        println!("    スクエアルート価格: {}", after_sqrt_price);
                    }
                }
            }
        }
    } else {
        println!("ルートが見つかりませんでした");
    }
    
    Ok(())
}
```

## パラメータの最適化

Cetus Aggregatorを使用する際、最適なルートを見つけるために重要なパラメータがいくつかあります。特に以下の2つのパラメータは結果に大きな影響を与えます：

### depth（深さ）

`depth`パラメータは、スワップの最大ホップ数（経由するプール数）を指定します。

- **推奨値**: `3`
- **意味**: 値が大きいほど、より多くのプールを経由した複雑なルートを見つけることができます
- **例**: `depth: Some(3)`の場合、SUI→USDC→USDT→CETUSのような3ホップのルートを見つけることができます
- **注意点**: 値が大きいほど計算コストも増加します

### split_count（分割数）

`split_count`パラメータは、取引を複数のルートに分割する数を指定します。

- **推奨値**: 通常の取引では`1`
- **意味**: 値が大きいほど、より多くのルートに分散して取引を行うことができます
- **例**: `split_count: Some(2)`の場合、取引を2つの異なるルートに分割します
- **注意点**: 値が大きいほどガス代も増加するため、小額の取引では`1`が推奨されます

これらのパラメータは公式SDKでも同様の推奨値が使用されており、最適なルートを見つけるために重要です。

### 高度な使用方法

```rust
// 流動性変更のシミュレーション付きルート検索
let params = FindRouterParams {
    from: "0x2::sui::SUI".to_string(),
    target: "0x...::cetus::CETUS".to_string(),
    amount: BigUint::from_str("1000000000").unwrap(),
    by_amount_in: true,
    depth: Some(2),
    providers: Some(vec!["CETUS".to_string(), "DEEPBOOK".to_string()]),
    liquidity_changes: Some(vec![
        PreSwapLpChangeParams {
            pool_id: "0x871d8a...".to_string(),
            tick_lower: 100,
            tick_upper: 394,
            delta_liquidity: -5498684,
        }
    ]),
    ..Default::default()
};

let routes = client.find_routers(params).await?;
```

## エラー処理

```rust
match client.find_routers(params).await {
    Ok(Some(route_data)) => {
        // ルートが見つかった場合の処理
    },
    Ok(None) => {
        // ルートが見つからなかった場合の処理
    },
    Err(e) => {
        match e {
            AggregatorError::ApiError { code, message } => {
                // APIエラーの処理
                println!("APIエラー ({}): {}", code, message);
            },
            AggregatorError::RequestError(e) => {
                // HTTPリクエストエラーの処理
                println!("リクエストエラー: {}", e);
            },
            _ => {
                // その他のエラーの処理
                println!("エラー: {}", e);
            }
        }
    }
}
```

## サンプルの実行

リポジトリをクローンした後、以下のコマンドでサンプルを実行できます：

```bash
cargo run --example basic_swap
```

## ライセンス

MIT

## 関連リンク

- [Cetus公式サイト](https://www.cetus.zone/)
- [Cetus開発者ドキュメント](https://cetus-1.gitbook.io/cetus-developer-docs/developer/cetus-plus-aggregator) 