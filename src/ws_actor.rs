use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Recipient, StreamHandler};
use actix_web_actors::ws;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub uuid: Option<String>,
    pub user_uuid: String,
    pub text: String,
    pub message_sent_at: Option<String>,
}

pub struct MyWs {
    pub chat_uuid: Uuid,
    pub user_uuid: Uuid,
    pub users: Addr<ChatServer>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(
            "Client {} connected to socket {}",
            self.user_uuid, self.chat_uuid
        );

        self.users.do_send(Connect {
            chat_uuid: self.chat_uuid,
            user_uuid: self.user_uuid,
            addr: ctx.address().recipient(),
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!(
            "Client {} disconnected to socket {}",
            self.user_uuid, self.chat_uuid
        );

        self.users.do_send(Disconnect {
            chat_uuid: self.chat_uuid,
            user_uuid: self.user_uuid,
        });
    }
}

// Ensure ChatMessage implements Message with a suitable result type
impl Message for ChatMessage {
    type Result = (); // Use () if no response is needed, or specify a type if a response is expected
}

/**
* Implement Handler for `MyWs` to handle `ChatMessage` messages
* This is where you will implement the logic to handle incoming `ChatMessage` messages
*/
impl Handler<ChatMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) -> Self::Result {
        // For debugging purposes, you can log all the messages here!
        let response = serde_json::to_string(&msg).unwrap();

        ctx.text(response);
    }
}

/**
* Implement `StreamHandler` for `MyWs` to handle incoming messages
* This is where you will parse incoming messages and decide how to handle them
*/
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(chat_message) = serde_json::from_str::<ChatMessage>(&text) {
                    let uuid = Uuid::new_v4();
                    let response_message = ChatMessage {
                        uuid: Some(uuid.to_string()),
                        user_uuid: chat_message.user_uuid.clone(),
                        text: chat_message.text,
                        message_sent_at: Some(chrono::Utc::now().naive_utc().to_string()),
                    };

                    // Todo: storing the message in the database

                    self.users.do_send(BroadcastMessage {
                        chat_uuid: self.chat_uuid,
                        message: response_message,
                    });
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

#[derive(Message)]
#[rtype(result = "()")]
struct Connect {
    chat_uuid: Uuid,
    user_uuid: Uuid,
    addr: Recipient<ChatMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    chat_uuid: Uuid,
    user_uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
struct BroadcastMessage {
    chat_uuid: Uuid, // Chat room UUID to identify which chat room to broadcast to
    message: ChatMessage,
}

pub struct ChatServer {
    chat_rooms: HashMap<Uuid, HashSet<(Uuid, Recipient<ChatMessage>)>>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            chat_rooms: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        info!("Adding client {} to chat {}", msg.user_uuid, msg.chat_uuid);

        self.chat_rooms
            .entry(msg.chat_uuid)
            .or_default()
            .insert((msg.user_uuid, msg.addr));
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        info!(
            "Removing client {} from chat {}",
            msg.user_uuid, msg.chat_uuid
        );

        if let Some(users) = self.chat_rooms.get_mut(&msg.chat_uuid) {
            users.retain(|(uuid, _)| *uuid != msg.user_uuid);
        }
    }
}

impl Handler<BroadcastMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Self::Context) {
        if let Some(users) = self.chat_rooms.get(&msg.chat_uuid) {
            for (_, user) in users {
                // For debugging purposes, you can log all recipients here
                user.do_send(msg.message.clone());
            }
        }
    }
}
