use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageCreatedEvent {
    pub event_time: String,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub user: User,
    pub channel_id: String,
    pub text: String,
    pub plain_text: String,
    pub embedded: Vec<Embedded>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub icon_id: String,
    pub bot: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    pub raw: String,
    pub r#type: String,
    pub id: String,
}

// Example JSON:

// {
//     "eventTime": "2019-05-08T13:33:51.690308239Z",
//     "message": {
//       "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
//       "user": {
//         "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
//         "name": "takashi_trap",
//         "displayName": "寺田 健二",
//         "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
//         "bot": false
//       },
//       "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e",
//       "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
//       "plainText": "@takashi_trap こんにちは",
//       "embedded": [
//         {
//           "raw": "@takashi_trap",
//           "type": "user",
//           "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
//         }
//       ],
//       "createdAt": "2019-05-08T13:33:51.632149265Z",
//       "updatedAt": "2019-05-08T13:33:51.632149265Z"
//     }
//   }
