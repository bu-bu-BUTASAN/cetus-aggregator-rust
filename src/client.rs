/**
 * Cetus Aggregator APIクライアント実装
 * 
 * このモジュールはCetus Aggregator APIと通信するためのクライアントを実装します。
 */
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde_json::json;

use crate::error::{AggregatorError, Result};
use crate::models::{AggregatorResponse, FindRouterParams, RouterData};

/// コイン識別子を完全な形式に変換する関数
/// 
/// この関数は、短縮形式のコイン識別子を完全な形式に変換します。
/// 現在の実装では単純に入力をそのまま返しますが、将来的には必要に応じて拡張できます。
fn completion_coin(coin: &str) -> String {
    // 現在の実装では単純に入力をそのまま返す
    // 実際のTypeScript実装に合わせて拡張する必要がある場合は、ここを修正
    coin.to_string()
}

/// アグリゲーターAPIクライアントのトレイト
#[async_trait]
pub trait AggregatorClientTrait {
    /// ルート検索を実行する
    /// 
    /// # 引数
    /// 
    /// * `params` - ルート検索のパラメータ
    /// 
    /// # 戻り値
    /// 
    /// 成功した場合はルーター検索結果データを含むOption、失敗した場合はエラーを返します。
    async fn find_routers(&self, params: FindRouterParams) -> Result<Option<RouterData>>;
}

/// アグリゲーターAPIクライアント実装
#[derive(Debug, Clone)]
pub struct AggregatorClient {
    /// APIエンドポイント
    pub endpoint: String,
    /// HTTPクライアント
    http_client: HttpClient,
}

impl AggregatorClient {
    /// 新しいクライアントを作成
    /// 
    /// # 引数
    /// 
    /// * `endpoint` - APIエンドポイント（Noneの場合はデフォルトエンドポイントを使用）
    /// 
    /// # 戻り値
    /// 
    /// 新しいAggregatorClientインスタンス
    pub fn new(endpoint: Option<String>) -> Self {
        let default_endpoint = "https://api-sui.cetus.zone/router_v2".to_string();
        let endpoint = endpoint.unwrap_or(default_endpoint);
        
        Self {
            endpoint,
            http_client: HttpClient::new(),
        }
    }
    
    /// GETリクエストによるルート検索
    /// 
    /// # 引数
    /// 
    /// * `params` - ルート検索のパラメータ
    /// 
    /// # 戻り値
    /// 
    /// 成功した場合はHTTPレスポンス、失敗した場合はエラーを返します。
    async fn get_router(&self, params: &FindRouterParams) -> Result<reqwest::Response> {
        let from_coin = completion_coin(&params.from);
        let target_coin = completion_coin(&params.target);
        
        // URLの基本部分を構築
        let mut url = format!(
            "{}/find_routes?from={}&target={}&amount={}&by_amount_in={}",
            self.endpoint,
            from_coin,
            target_coin,
            params.amount.to_string(),
            params.by_amount_in
        );
        
        // オプションパラメータを追加
        if let Some(depth) = params.depth {
            url.push_str(&format!("&depth={}", depth));
        }
        
        if let Some(ref split_algorithm) = params.split_algorithm {
            url.push_str(&format!("&split_algorithm={}", split_algorithm));
        }
        
        if let Some(split_factor) = params.split_factor {
            url.push_str(&format!("&split_factor={}", split_factor));
        }
        
        if let Some(split_count) = params.split_count {
            url.push_str(&format!("&split_count={}", split_count));
        }
        
        if let Some(ref providers) = params.providers {
            if !providers.is_empty() {
                url.push_str(&format!("&providers={}", providers.join(",")));
            }
        }
        
        // SDK バージョンを追加
        url.push_str("&v=1000327");
        
        // HTTPリクエストを実行
        match self.http_client.get(&url).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(AggregatorError::RequestError(e)),
        }
    }
    
    /// POSTリクエストによる流動性変更付きルート検索
    /// 
    /// # 引数
    /// 
    /// * `params` - ルート検索のパラメータ（流動性変更を含む）
    /// 
    /// # 戻り値
    /// 
    /// 成功した場合はHTTPレスポンス、失敗した場合はエラーを返します。
    async fn post_router_with_liquidity_changes(
        &self,
        params: &FindRouterParams,
    ) -> Result<reqwest::Response> {
        let from_coin = completion_coin(&params.from);
        let target_coin = completion_coin(&params.target);
        let url = format!("{}/find_routes", self.endpoint);
        
        // プロバイダーリストをカンマ区切り文字列に変換
        let providers_str = params.providers.as_ref().map(|p| p.join(","));
        
        // リクエストデータを構築
        let mut request_data = json!({
            "from": from_coin,
            "target": target_coin,
            "amount": params.amount.to_string(),
            "by_amount_in": params.by_amount_in,
        });
        
        if let Some(depth) = params.depth {
            request_data["depth"] = json!(depth);
        }
        
        if let Some(ref split_algorithm) = params.split_algorithm {
            request_data["split_algorithm"] = json!(split_algorithm);
        }
        
        if let Some(split_factor) = params.split_factor {
            request_data["split_factor"] = json!(split_factor);
        }
        
        if let Some(split_count) = params.split_count {
            request_data["split_count"] = json!(split_count);
        }
        
        if let Some(ref providers_str) = providers_str {
            request_data["providers"] = json!(providers_str);
        }
        
        // 流動性変更データを追加
        if let Some(ref liquidity_changes) = params.liquidity_changes {
            let changes = liquidity_changes.iter().map(|change| {
                json!({
                    "pool": change.pool_id,
                    "tick_lower": change.tick_lower,
                    "tick_upper": change.tick_upper,
                    "delta_liquidity": change.delta_liquidity,
                })
            }).collect::<Vec<_>>();
            
            request_data["liquidity_changes"] = json!(changes);
        }
        
        // POSTリクエストを送信
        match self.http_client
            .post(&url)
            .json(&request_data)
            .send()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(AggregatorError::RequestError(e)),
        }
    }
    
    /// レスポンスを解析してルーターデータを取得
    /// 
    /// # 引数
    /// 
    /// * `response` - HTTPレスポンス
    /// 
    /// # 戻り値
    /// 
    /// 成功した場合はルーターデータを含むOption、失敗した場合はエラーを返します。
    async fn parse_router_response(
        &self,
        response: reqwest::Response,
    ) -> Result<Option<RouterData>> {
        // レスポンスが成功したか確認
        if !response.status().is_success() {
            return Err(AggregatorError::ApiError {
                code: response.status().as_u16() as u32,
                message: format!("APIエラー: {}", response.status()),
            });
        }
        
        // レスポンス本文をJSONとして解析
        let data: AggregatorResponse = match response.json().await {
            Ok(data) => data,
            Err(e) => return Err(AggregatorError::RequestError(e)),
        };
        
        // エラーチェック
        if data.code != 0 && data.code != 200 {
            return Err(AggregatorError::ApiError {
                code: data.code,
                message: data.msg,
            });
        }
        
        // ルーターデータを返却
        Ok(data.data)
    }
}

#[async_trait]
impl AggregatorClientTrait for AggregatorClient {
    async fn find_routers(&self, params: FindRouterParams) -> Result<Option<RouterData>> {
        // 流動性変更があるかどうかでリクエスト方法を選択
        let response = if params.liquidity_changes.is_some() && !params.liquidity_changes.as_ref().unwrap().is_empty() {
            self.post_router_with_liquidity_changes(&params).await?
        } else {
            self.get_router(&params).await?
        };
        
        // レスポンスを解析して返却
        self.parse_router_response(response).await
    }
}