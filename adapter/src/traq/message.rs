use kernel::{
    model::{
        message::{Message, NewMessage},
        Id,
    },
    traq::{error::TraqRepositoryError, message::MessageTraqRepository},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::TraqRepositoryImpl;

#[derive(Debug, Serialize)]
struct MessageRequest {
    content: String,
    embed: bool,
}

#[derive(Deserialize, Debug)]
struct CreateMessageResponse {
    id: String,
    channel_id: String,
}

impl MessageTraqRepository for TraqRepositoryImpl {
    async fn create(&self, message: NewMessage) -> Result<Message, TraqRepositoryError> {
        let request_body = MessageRequest {
            content: message.content,
            embed: message.embed,
        };
        let response = reqwest::Client::new()
            .post(format!(
                "https://q.trap.jp/api/v3/channels/{}/messages",
                message.channel_id.value
            ))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| TraqRepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match response.status() {
            StatusCode::CREATED => {
                let response = response
                    .json::<CreateMessageResponse>()
                    .await
                    .map_err(|e| TraqRepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
                Ok(Message::new(
                    Id::new(response.id),
                    Id::new(response.channel_id),
                ))
            }
            code => Err(TraqRepositoryError::UnexpectedError(anyhow::anyhow!(
                "Failed to create message: {}(status code: {})",
                response.text().await.unwrap_or_default(),
                code
            ))),
        }
    }
}
