use crate::auth::get_claim_from;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web_lab::middleware::Next;

fn is_authorized(req: &ServiceRequest, as_uid: Option<i64>) -> bool {
    if let Some(user_claim) = get_claim_from(req.request()) {
        match as_uid {
            Some(id) => return id == user_claim.id,
            None => return user_claim.id > 0,
        }
    }
    return false;
}

pub async fn root_gate(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let isroot = is_authorized(&req, Some(1));

    match isroot {
        true => Ok(next.call(req).await?),
        false => Err(actix_web::error::ErrorForbidden(
            "Ne tavo kiskis ne tu ir kiskis",
        )),
    }
}

pub async fn auth_gate(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let isauth = is_authorized(&req, None);

    match isauth {
        true => Ok(next.call(req).await?),
        false => Err(actix_web::error::ErrorForbidden(
            "Ne tavo kiskis ne tu ir kiskis",
        )),
    }
}
