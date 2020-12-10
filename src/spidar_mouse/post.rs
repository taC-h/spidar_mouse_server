use actix_web::{web, HttpResponse};
use super::messages::*;

pub async fn impl_post(request_body: web::Json<RequestBody>) -> actix_web::Result<HttpResponse> {
    let (result,message) = request_body.into_inner().run();
    Ok(HttpResponse::Ok().json(ResponseBody {
        result,message
    }))
}