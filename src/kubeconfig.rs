extern crate yaml_rust;
use std::fs;

pub fn run(){
    // home::home_dir();
    // println!("{}", path.display());
    let mut string = String::new();
    let message = match home::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("Impossible to get your home dir!"),
    };
    println!("{:?}", message);

    // let path_e = "/home/home/.kube/config";
    // println!("{}", path_e);

    // let contents = fs::read_to_string(path_e)
    //     .expect("Something went wrong reading the file");

    //println!("With text:\n{}", contents);

    
}