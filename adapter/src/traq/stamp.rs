use kernel::{
    model::stamp::{NewStamp, StampType},
    traq::{error::TraqRepositoryError, stamp::StampTraqRepository},
};
use reqwest::StatusCode;
use serde::Serialize;

use super::TraqRepositoryImpl;

#[derive(Debug, Serialize)]
struct StampRequest {
    count: i32,
}

impl StampTraqRepository for TraqRepositoryImpl {
    async fn create(&self, stamp: NewStamp) -> Result<(), TraqRepositoryError> {
        let request_body = StampRequest { count: 1 };
        let response = reqwest::Client::new()
            .post(format!(
                "https://q.trap.jp/api/v3/messages/{}/stamps/{}",
                stamp.message_id.value,
                match stamp.stamp_type {
                    StampType::WhiteCheckMark => "93d376c3-80c9-4bb2-909b-2bbe2fbf9e93",
                }
            ))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| TraqRepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            code => Err(TraqRepositoryError::UnexpectedError(anyhow::anyhow!(
                "Failed to create stamp: {}(status code: {})",
                response.text().await.unwrap_or_default(),
                code
            ))),
        }
    }
}
