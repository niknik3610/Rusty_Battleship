use std::{fs::File, io::Read};

const FILE_PREFIX: &str = "../frontend/";

pub fn route(url: &str) -> String {
    let file_route;
    match url {
        "/" => file_route = FILE_PREFIX.to_string() + "index.html",
        _ => file_route = FILE_PREFIX.to_string() + "404.ts"
    }

    println!("{file_route}");

    //TODO: possibly change
    let mut file = File::open(file_route).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return contents;
}
