use actix::*;
use actix_web::{http::HeaderValue, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::chat_server::ChatServer;

use crate::pb::ragdoll::ragdoll_internal_service_client::RagdollInternalServiceClient;
use crate::pb::ragdoll::VerificationTokenReq;
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
    ragdoll: web::Data<RagdollInternalServiceClient<tonic::transport::Channel>>,
) -> Result<HttpResponse, Error> {
    // TODO: improve logic
    let id = verification_token(req.headers().get("Authorization"), ragdoll.get_ref()).await;
    if id == 0 {
        return Ok(HttpResponse::Unauthorized().into());
    }
    ws::start(
        crate::session::WsChatSession::new(id, srv.get_ref().clone()),
        &req,
        stream,
    )
}

async fn verification_token(
    token: Option<&HeaderValue>,
    ragdoll: &RagdollInternalServiceClient<tonic::transport::Channel>,
) -> i64 {
    // TODO: improve logic

    if let Some(value) = token {
        if let Ok(token) = value.to_str() {
            let mut ragdoll = ragdoll.clone();
            return match ragdoll
                .verification_token(VerificationTokenReq {
                    token: token.into(),
                })
                .await
            {
                Ok(v) => {
                    return v.get_ref().id;
                }
                Err(err) => return 0,
            };
        }
    }
    return 0;
}
