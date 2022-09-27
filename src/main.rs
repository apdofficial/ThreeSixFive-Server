mod test;

use actix::{Actor, ActorContext, AsyncContext, ContextFutureSpawner, StreamHandler, WrapFuture};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use strum_macros::EnumString;
use std::{str, thread};
use std::io::Read;
use std::str::FromStr;
use std::time::Duration;
use actix_web::http::header::ContentRangeSpec::Bytes;
use actix_web_actors::ws::WebsocketContext;
use async_std::task;
use strum_macros::Display;


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
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(msg)) => {
                ctx.ping(&msg);
            }
            Ok(ws::Message::Text(message)) => {
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
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn send_async_text(ctx: &mut WebsocketContext<MyWebSocket>, text: String) {
    ctx.text(text);
}

fn process_incoming_command(command: LandingPlatformCommand, ctx: &mut WebsocketContext<MyWebSocket>, actor: &mut MyWebSocket) {
    println!("Received: {}", command.to_string());
    match command {
        LandingPlatformCommand::OpenLid => {
            actor.landing_platform_state = LandingPlatformState::StateOpening;

            // task::spawn(async {
            //     ctx.text(actor.landing_platform_state.to_string())
            // });

            // let fut = async move {
            //     actor.landing_platform_state = LandingPlatformState::StateOpening;
            //     send_async_text(ctx, actor.landing_platform_state.to_string()).await;
            // };
            //
            // let fut = actix::fut::wrap_future::<_, Self>(fut);
            //
            // ctx.spawn(fut);
            //
            // let block = async move {
            //     actor.landing_platform_state = LandingPlatformState::StateOpening;
            //     send_async_text(ctx, actor.landing_platform_state.to_string()).await;
            // };
            // let block2 = async move {
            //     thread::sleep(Duration::from_millis(5000));
            //     actor.landing_platform_state = LandingPlatformState::StateOpen;
            //     send_async_text(ctx, actor.landing_platform_state.to_string()).await;
            // };
        }
        LandingPlatformCommand::CloseLid => {
            actor.landing_platform_state = LandingPlatformState::StateClosing;
            ctx.text(actor.landing_platform_state.to_string());
            thread::sleep(Duration::from_millis(5000));
            actor.landing_platform_state = LandingPlatformState::StateClosed;
            ctx.text(actor.landing_platform_state.to_string());
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("192.168.1.149", 8080))?
        .run()
        .await
}