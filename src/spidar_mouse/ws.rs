use actix::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use super::messages::*;

pub async fn impl_ws(req: HttpRequest, stream: web::Payload) -> actix_web::Result<HttpResponse> {
    ws::start(SpidarWs, &req, stream)
}

struct SpidarWs;

impl Actor for SpidarWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SpidarWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let (result,message) = match msg {
            Ok(ws::Message::Text(text)) => {
                println!("text:{}",text);
                let json_struct: Result<RequestBody, serde_json::Error> = serde_json::from_str(&text);
                match json_struct {
                    Ok(t) => t.run(),
                    Err(e) => (false, e.to_string()),
                }
            },
            _ => (false, "invalid status".to_string()),
        };
        println!("{}", message);
        let json_str = serde_json::to_string(&ResponseBody{result,message}).unwrap();
        ctx.text(json_str)
    }
}
