use actix::{Actor, ActorContext, Arbiter, AsyncContext, Context, Handler, spawn, StreamHandler, WrapFuture};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use strum_macros::EnumString;
use std::str;
use std::str::FromStr;
use std::time::Duration;
use actix_web_actors::ws::{Message, WebsocketContext};
use strum_macros::Display;
use actix::prelude::*;
use tokio::time::sleep;

/// How often deltas are sent
const DELTAS_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Debug, Display, PartialEq, EnumString)]
pub enum LandingPlatformState {
    StateOpen,
    StateOpening,
    StateClosed,
    StateClosing,
    Unknown
}

#[derive(Debug, Display, PartialEq, EnumString)]
pub enum LandingPlatformCommand {
    OpenLid,
    CloseLid,
    GetLidStatus,
    Unknown
}

struct PixelDeltas{
    x: i8,
    y: i8
}

impl PixelDeltas {
    fn to_bytes(&self) -> Vec<u8> {
        [self.x.to_le_bytes(), self.y.to_le_bytes()].concat()
    }

}

/// Define HTTP actor
struct MyWebSocket {
    landing_platform_state: LandingPlatformState,
}

impl MyWebSocket {

    fn dispatch_deltas(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(DELTAS_INTERVAL, |act, ctx| {
            let deltas = PixelDeltas { x: 0, y: 0 };
            ctx.binary(deltas.to_bytes())
        });
    }

}

impl Actor for MyWebSocket {
    type Context = WebsocketContext<Self>;

    /// Method is called on actor start. We start dispatch deltas task here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.dispatch_deltas(ctx);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<Message, ws::ProtocolError>> for MyWebSocket {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(Message::Pong(msg)) => {
                ctx.ping(&msg);
            }
            Ok(Message::Text(message)) => {
                match str::from_utf8(message.as_ref()) {
                    Ok(string_message) => {
                        match LandingPlatformCommand::from_str(string_message){
                            Ok(command) => { process_incoming_command(command, ctx, self) }
                            _ => {}
                        };
                    },
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

            },
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TextMessage {
    message: String,
}

impl Handler<TextMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.message);
    }
}

fn process_incoming_command(command: LandingPlatformCommand, ctx: &mut WebsocketContext<MyWebSocket>, actor: &mut MyWebSocket) {
    println!("Received: {}", command.to_string());
    match command {
        LandingPlatformCommand::OpenLid => {
            let recipient = ctx.address().recipient();
            actor.landing_platform_state = LandingPlatformState::StateOpening;
            let landing_platform_state = actor.landing_platform_state.to_string();
            spawn(async move {
                recipient.send(TextMessage { message: landing_platform_state }).await.unwrap();
            });

            let recipient = ctx.address().recipient();
            actor.landing_platform_state = LandingPlatformState::StateOpen;
            let landing_platform_state = actor.landing_platform_state.to_string();
            spawn(async move {
                sleep(Duration::from_secs(5)).await;
                recipient.send(TextMessage { message: landing_platform_state }).await.unwrap();
            });
        }
        LandingPlatformCommand::CloseLid => {
            let recipient = ctx.address().recipient();
            actor.landing_platform_state = LandingPlatformState::StateClosing;
            let landing_platform_state = actor.landing_platform_state.to_string();
            spawn(async move {
                recipient.send(TextMessage { message: landing_platform_state }).await.unwrap();
            });

            let recipient = ctx.address().recipient();
            actor.landing_platform_state = LandingPlatformState::StateClosed;
            let landing_platform_state = actor.landing_platform_state.to_string();
            spawn(async move {
                sleep(Duration::from_secs(5)).await;
                recipient.send(TextMessage { message: landing_platform_state }).await.unwrap();
            });
        }
        LandingPlatformCommand::GetLidStatus => {
            ctx.text(actor.landing_platform_state.to_string());
        }
        LandingPlatformCommand::Unknown => {

        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let res = ws::start(MyWebSocket { landing_platform_state: LandingPlatformState::Unknown }, &req, stream);
    println!("{:?}", res);
    res
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(index))
            .service(greet)
    }).bind(("192.168.1.149", 8080))?.run().await
}