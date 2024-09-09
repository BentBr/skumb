use crate::json_serialization::web_socket::chat_message::ChatMessage;
use crate::json_serialization::web_socket::message::Data::ChatMessage as MessageEnum;
use crate::json_serialization::web_socket::message::{Data, Message as WsMessage};
use crate::json_serialization::web_socket::ping::{Knock, Ping};
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message as ActixMessage, Recipient, StreamHandler};
use actix_web_actors::ws;
use log::{error, info, warn};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct MyWs {
    pub chat_uuid: Uuid,
    pub user_uuid: Uuid,
    pub users: Addr<ChatServer>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Client {} connected to socket {}", self.user_uuid, self.chat_uuid);

        self.users.do_send(Connect {
            chat_uuid: self.chat_uuid,
            user_uuid: self.user_uuid,
            addr: ctx.address().recipient(),
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("Client {} disconnected to socket {}", self.user_uuid, self.chat_uuid);

        self.users.do_send(Disconnect {
            chat_uuid: self.chat_uuid,
            user_uuid: self.user_uuid,
        });
    }
}

// Ensure ChatMessage implements Message with a suitable result type
impl ActixMessage for WsMessage {
    type Result = (); // Use () if no response is needed, or specify a type if a response is expected
}

/**
* Implement Handler for `MyWs` to handle `ChatMessage` messages
* This is where you will implement the logic to handle incoming `ChatMessage` messages
*/
impl Handler<WsMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
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
                if let Ok(chat_message) = serde_json::from_str::<WsMessage>(&text) {
                    let response_message = match chat_message.data {
                        MessageEnum(message) => {
                            let uuid = Uuid::new_v4();

                            WsMessage::new(Data::ChatMessage(ChatMessage::new(
                                uuid.to_string(),
                                message.user_id,
                                message.text,
                                chrono::Utc::now().naive_utc().to_string(),
                            )))
                        }
                        Data::Connection(connection) => {
                            warn!("Response of connection message: {:?}", connection);
                            //Todo: add other message types and use them here
                            WsMessage::new(Data::Connection(connection))
                        }
                        Data::Ping(_ping) => {
                            //Todo: add other message types

                            warn!("PING: WebSocket error during parsing of message: {text}");

                            WsMessage::new(Data::Ping(Ping::new(Knock::Pong)))
                        }
                    };

                    // Todo: storing the message in the database

                    self.users.do_send(BroadcastMessage {
                        chat_uuid: self.chat_uuid,
                        message: response_message.clone(),
                    });

                    info!("Did send following message to chat room: {:?}", response_message);
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

#[derive(ActixMessage)]
#[rtype(result = "()")]
struct Connect {
    chat_uuid: Uuid,
    user_uuid: Uuid,
    addr: Recipient<WsMessage>,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
struct Disconnect {
    chat_uuid: Uuid,
    user_uuid: Uuid,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
struct BroadcastMessage {
    chat_uuid: Uuid, // Chat room UUID to identify which chat room to broadcast to
    message: WsMessage,
}

pub struct ChatServer {
    chat_rooms: HashMap<Uuid, HashSet<(Uuid, Recipient<WsMessage>)>>,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
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
        info!("Removing client {} from chat {}", msg.user_uuid, msg.chat_uuid);

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
