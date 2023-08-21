use std::{fs::File, io::Read, sync::Mutex, collections::HashMap};
use lazy_static::lazy_static;

const FILE_PREFIX: &str = "../frontend/";

#[derive(Debug)]
pub enum RouterErrorType {
    Type404,
}
#[derive(Debug)]
pub struct RouterError {
    pub error_type: RouterErrorType,
    file: String
} impl RouterError {
    pub fn new(error_type: RouterErrorType, file: String) -> Self{
        return RouterError {
            error_type,
            file
        }
    }
}

lazy_static! {
    static ref CACHED_FILES: Mutex<HashMap<String, File>> = Mutex::new(HashMap::new());
}

pub fn route(url: &str) -> Result<String, RouterError> {
    let file_route;
    match url {
        "/" => file_route = FILE_PREFIX.to_string() + "index.html",
        _ => return Err(RouterError::new(
                RouterErrorType::Type404,
                FILE_PREFIX.to_string() + "404.html"
                ))};

    let args = ARGS.lock().unwrap();
    let mut file: File;

    if args.cached {
        file = get_file_cache(&file_route[..]).unwrap(); 
    }
    else {
        file = File::open(file_route).unwrap();
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return Ok(contents);
}

pub fn get_file_cache(url: &str) -> Option<File>  {
    let file_exists = cache_check_file(url);
    match file_exists {
        Some(file) => return Some(file),
        None => {}
    }

    let file_result = File::open(url);
    
    match file_result {
        Ok(file) => {
            cache_insert_file(url, file.try_clone().unwrap());
            return Some(file);
        }
        Err(e) => {
            return None
        }
    }
}

fn cache_check_file(url: &str) -> Option<File> {
    let cache = CACHED_FILES.lock().unwrap();
    return cache.get(url).map(|file| file.try_clone().unwrap()) 
}

fn cache_insert_file(url: &str, file: File) {
    let mut cache = CACHED_FILES.lock().unwrap();
    cache.insert(url.to_owned(), file);
}

