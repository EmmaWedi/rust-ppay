use regex::Regex;
use crate::libs::error;

pub fn not_none<T>(v: Option<T>, name: &str) -> Result<(), error::Error> {

    if let None = v {
        return Err(error::new_error(1002, &format!("{} cannot be empty", name)[..], 422));
    }

    Ok(())
}

pub fn required_str(v: &str, name: &str) -> Result<String, error::Error> {

    let v = v.to_string();

    if v.chars().count() == 0 {
        return Err(error::new_error(1002, &format!("{} is required", name)[..], 422));
    }

    Ok(v)
}

pub fn email(v: &str, name: &str) -> Result<String, error::Error> {

    let res_str = v.to_string();

    let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

    if !re.is_match(v) {
        return Err(error::new_error(1002, &format!("{} validation failed", name)[..], 422));
    }

    Ok(res_str)
}

pub fn mobile(v: &str, name: &str) -> Result<String, error::Error> {

    let re = Regex::new(r"^[0-9]{10}$").unwrap();

    if !re.is_match(v) {
        return Err(error::new_error(1002, &format!("{} validation failed", name)[..], 422));
    }

    let res_str = v.to_string();

    Ok(res_str)
}

pub fn uuid(v: &str, name: &str) -> Result<String, error::Error> {

    let re = Regex::new(r"^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$").unwrap();
    
    if !re.is_match(&(v.to_uppercase())[..]) {
        return Err(error::new_error(1002, &format!("{} is invalid", name)[..], 422));
    }

    let res_str = v.to_string();

    Ok(res_str)
}