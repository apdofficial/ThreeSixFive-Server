use std::borrow::Borrow;
use actix::{Actor, ActorContext, Arbiter, AsyncContext, Context, Handler, spawn, StreamHandler, WrapFuture};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware};
use actix_web_actors::ws;
use strum_macros::EnumString;
use std::str;
use std::str::FromStr;
use std::sync::{Mutex, RwLock};
use std::time::Duration;
use actix_web_actors::ws::{Message, WebsocketContext};
use strum_macros::Display;
use actix::prelude::*;
use tokio::time::sleep;

/// How often deltas are sent
const DELTAS_INTERVAL: Duration = Duration::from_millis(5000);

#[derive(Debug, Display, PartialEq, EnumString, Default)]
pub enum LandingPlatformState {
    StateOpen,
    StateOpening,
    StateClosed,
    StateClosing,
    #[default]
    Unknown
}

#[derive(Debug, Display, PartialEq, EnumString, Default)]
pub enum LandingPlatformCommand {
    OpenLid,
    CloseLid,
    GetLidStatus,
    #[default]
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

#[derive(Default)]
struct AppState {
    landing_platform_state: LandingPlatformState,
}

impl Actor for AppState {
    type Context = Context<Self>;
}

impl Supervised for AppState {}

impl SystemService for AppState {
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        println!("Service started");
    }
}

/// Define HTTP actor
struct MyWebSocket {
    landing_platform_state: LandingPlatformState,
}

impl MyWebSocket {

    fn dispatch_deltas(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(DELTAS_INTERVAL, |act, ctx| {
            let deltas = PixelDeltas { x: 110, y: 110 }; // TODO: read the data form the sensor here
            // ctx.binary(deltas.to_bytes())
            ctx.text(hex::encode(deltas.to_bytes()));
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

async fn index(req: HttpRequest, stream: web::Payload, data: Data<AppState>) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let res = ws::start(MyWebSocket { landing_platform_state: LandingPlatformState::Unknown  }, &req, stream);
    println!("{:?}", res);
    res
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    format!("Hello {name}!")
}


use actix_web::{web::Data,};
use redis_tang::{Builder, Pool, RedisManager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        landing_platform_state: LandingPlatformState::Unknown
    });

    HttpServer::new(move ||{
        App::new()
            .app_data( app_state.clone())
            .route("/", web::get().to(index))
            .service(greet)
            // enable logger
            .wrap(middleware::Logger::default())
    })
        .bind(("192.168.1.149", 8080))?
        .run()
        .await
}