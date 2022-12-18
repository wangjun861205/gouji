mod card;
mod desktop;
mod message;
mod server;
mod user;

use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web::{
    http::StatusCode,
    web::{get, Data, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws::{self, WebsocketContext};
use desktop::Desktop;
use message::{InnerMessage, Message, Response};
use serde_json;

pub struct WS {
    desktops: Data<Vec<Addr<Desktop>>>,
}

impl Actor for WS {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let msg: Message = serde_json::from_str(&text.to_string()).unwrap();
                match msg {
                    Message::Sit { uid, desktop_id } => {
                        self.desktops[0]
                            .try_send(InnerMessage::Sit {
                                uid: uid,
                                desktop_id: desktop_id,
                                ws_addr: ctx.address(),
                            })
                            .unwrap();
                    }
                    _ => {}
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<Response> for WS {
    type Result = Result<(), String>;
    fn handle(&mut self, msg: Response, ctx: &mut Self::Context) -> Self::Result {
        let s = serde_json::to_string(&msg).unwrap();
        ctx.text(s);
        Ok(())
    }
}

async fn index(req: HttpRequest, stream: Payload, desktops: Data<Vec<Addr<Desktop>>>) -> Result<HttpResponse, Error> {
    let w = WS { desktops };
    ws::start(w, &req, stream)?;
    Ok(HttpResponse::new(StatusCode::OK))
}

#[actix_web::main]
async fn main() {
    HttpServer::new(move || {
        let desktops = Data::new(vec![Desktop::new(); 10]);
        App::new().app_data(desktops).route("/", get().to(index))
    })
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
