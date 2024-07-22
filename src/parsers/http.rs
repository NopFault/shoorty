use actix_web::http::header;
use actix_web::HttpRequest;

pub struct RequestParser {
    req: HttpRequest,
}

impl RequestParser {
    pub fn new(req: &HttpRequest) -> RequestParser {
        RequestParser { req: req.clone() }
    }

    pub fn get_ip(&self) -> String {
        let peer_addr = self.req.peer_addr();

        match peer_addr {
            Some(data) => data.to_string(),
            None => "No IP".to_string(),
        }
    }

    pub fn get_ua(&self) -> String {
        let user_agent = self
            .req
            .headers()
            .get(header::USER_AGENT)
            .and_then(|h| h.to_str().ok());
        user_agent.unwrap_or("No User-Agent").to_string()
    }

    pub fn get_referer(&self) -> String {
        let user_agent = self
            .req
            .headers()
            .get(header::REFERER)
            .and_then(|h| h.to_str().ok());
        user_agent.unwrap_or("No Referer").to_string()
    }
}
