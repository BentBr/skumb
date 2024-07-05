use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use log::{error, warn};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub uuid: Option<String>,
    pub user_uuid: String,
    pub text: String,
    pub message_sent_at: Option<String>,
}

pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(chat_message) = serde_json::from_str::<ChatMessage>(&text) {
                    let uuid = Uuid::new_v4();
                    let response_message = ChatMessage {
                        uuid: Some(uuid.to_string()),
                        user_uuid: chat_message.user_uuid,
                        text: chat_message.text,
                        message_sent_at: Some(chrono::Utc::now().naive_utc().to_string()),
                    };

                    // Todo: storing the message in the database
                    // Echo the message back to the client
                    let response = serde_json::to_string(&response_message).unwrap();
                    ctx.text(response);
                } else {
                    warn!("WebSocket error during parsing of message: {text}");


                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Err(error) => error!("WebSocket error: {error}"),
            _ => (),
        }
    }
}
