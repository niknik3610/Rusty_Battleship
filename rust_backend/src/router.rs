use std::{fs::File, io::Read};

const FILE_PREFIX: &str = "../frontend/";

pub enum RouterErrorType {
    Type404,
}
pub struct RouterError {
    error_type: RouterErrorType,
    file: String
} impl RouterError {
    pub fn new(error_type: RouterErrorType, file: String) -> Self{
        return RouterError {
            error_type,
            file
        }
    }
}

pub fn route(url: &str) -> Result<String, RouterError> {
    let file_route;
    match url {
        "/" => file_route = FILE_PREFIX.to_string() + "index.html",
        _ => return Err(RouterError::new(
                RouterErrorType::Type404,
                FILE_PREFIX.to_string() + "404.html"
        ))};

    println!("{file_route}");

    //TODO: possibly change
    let mut file = File::open(file_route).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return Ok(contents);
}
