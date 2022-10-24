extern crate yaml_rust;
use std::fs;
use std::env;
use yaml_rust::YamlLoader;
use colored::Colorize;
// use clap::Parser;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]

// struct Args {
//     #[arg(short, long)]
//     kx: String,

// }

pub fn kx(kubeconfig: &str) {
    // args
    // let args = Args::parse();

    let args: Vec<String> = env::args().collect();
    let kx = args[1].as_str();

    if kx == "all" {
        // get current context
        let read_kubeconfig = fs::read_to_string(kubeconfig)
            .expect("Something went wrong reading the file");

        let contexts = YamlLoader::load_from_str(&read_kubeconfig).unwrap();
        let context = &contexts[0];
        let current_context = context["current-context"].as_str().unwrap();
        // println!("Current context: {}",current_context);
        // println!("context : {}",context["users"][0]["name"].as_str().unwrap());

        let all_contexts = &context["users"];
        for context_name in all_contexts.as_vec().unwrap() {

            if context_name["name"].as_str().unwrap() == current_context {
                println!("{}",context_name["name"].as_str().unwrap().green());
            } else {
                println!("{}",context_name["name"].as_str().unwrap());
            }
            
        }

    } else {
        println!("setting default context to {}",kx.green());
    }
}