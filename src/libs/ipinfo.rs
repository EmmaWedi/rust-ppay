use actix_web::{dev::ConnectionInfo, web, HttpRequest};

use crate::AppState;

pub struct IpInfo {
    pub addr: String,
    pub user_agent: String,
}

pub fn get_ip_info(state: &web::Data<AppState>, req: &HttpRequest, conn: &ConnectionInfo) -> IpInfo {

    let mut addr: String = String::from("");
    let mut user_agent = String::from("");

    let is_behind_proxy = state.config.get::<bool>("network.behind_proxy").unwrap();

    if is_behind_proxy {
        if let Some(val) = conn.realip_remote_addr() {
            let split = val.split(":");
            let vec: Vec<&str> = split.collect();
            if vec.len() >0 {
                addr = vec[0].to_string();
            }
        }
    } else {
        if let Some(val) = req.peer_addr() {
            addr = val.ip().to_string();
        };
    }

    if let Some(val) = req.headers().get("User-Agent") {
        user_agent = val.to_str().unwrap_or_default().to_string();
    };
    
    IpInfo { addr, user_agent }
}