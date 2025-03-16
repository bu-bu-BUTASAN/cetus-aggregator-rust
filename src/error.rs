/**
 * Cetus Aggregator APIのエラー定義
 *
 * このモジュールはAPIとの通信時に発生する可能性のあるエラーを定義します。
 */
use thiserror::Error;

/// アグリゲーターAPIのエラーコード
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregatorServerErrorCode {
    /// 計算エラー
    CalculateError = 10000,
    /// 入力数値が大きすぎる
    NumberTooLarge = 10001,
    /// ルートが見つからない
    NoRouter = 10002,
    /// 流動性不足
    InsufficientLiquidity = 10003,
    /// ハニーポットスキャム検出
    HoneyPot = 10004,
}

impl AggregatorServerErrorCode {
    /// エラーコードから対応するエラーコード列挙型を取得
    pub fn from_code(code: u32) -> Option<Self> {
        match code {
            10000 => Some(Self::CalculateError),
            10001 => Some(Self::NumberTooLarge),
            10002 => Some(Self::NoRouter),
            10003 => Some(Self::InsufficientLiquidity),
            10004 => Some(Self::HoneyPot),
            _ => None,
        }
    }

    /// エラーコードに対応するメッセージを取得
    pub fn message(&self) -> &'static str {
        match self {
            Self::CalculateError => "計算エラーが発生しました",
            Self::NumberTooLarge => "入力数値が大きすぎて対象の型に収まりません",
            Self::NoRouter => "適切なルートが見つかりませんでした",
            Self::InsufficientLiquidity => "流動性が不足しています",
            Self::HoneyPot => "対象トークンがハニーポットスキャムとして検出されました",
        }
    }
}

/// アグリゲーターAPIのエラー
#[derive(Error, Debug)]
pub enum AggregatorError {
    /// HTTPリクエストエラー
    #[error("HTTPリクエストエラー: {0}")]
    RequestError(#[from] reqwest::Error),

    /// JSONシリアライズ/デシリアライズエラー
    #[error("JSONエラー: {0}")]
    JsonError(#[from] serde_json::Error),

    /// APIエラー
    #[error("APIエラー ({code}): {message}")]
    ApiError {
        /// エラーコード
        code: u32,
        /// エラーメッセージ
        message: String,
    },

    /// サーバーエラー
    #[error("サーバーエラー: {0}")]
    ServerError(#[source] anyhow::Error),

    /// 入力パラメータエラー
    #[error("入力パラメータエラー: {0}")]
    InputError(String),

    /// その他のエラー
    #[error("その他のエラー: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result型のエイリアス
pub type Result<T> = std::result::Result<T, AggregatorError>;
