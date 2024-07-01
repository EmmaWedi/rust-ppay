use sha2::{Digest, Sha256};

pub fn encrypt_password(password: &str, salt: &uuid::Uuid) -> String {

    let pwd = format!("{}{}", password, salt.to_string());

    let pwd = md5::compute(pwd);

    let pwd = format!("{:?}{}{}", pwd, password, salt.to_string());

    let pwd = Sha256::new().chain_update(pwd).finalize();
    
    format!("{:x}", pwd)
}