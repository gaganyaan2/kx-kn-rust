extern crate yaml_rust;
use std::fs;
use std::env;
use yaml_rust::YamlLoader;
use colored::Colorize;
use std::io::Write;
use std::process;

// use clap::Parser;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]

// struct Args {
//     #[arg(short, long)]
//     kx: String,

// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn kx(kubeconfig: &str) {
    // args
    // let args = Args::parse();

    let args: Vec<String> = env::args().collect();
    let kx = args[1].as_str();
    let read_kubeconfig = fs::read_to_string(kubeconfig)
    .expect("Something went wrong reading the file");

    let contexts = YamlLoader::load_from_str(&read_kubeconfig).unwrap();
    let context = &contexts[0];
    let current_context = context["current-context"].as_str().unwrap();
    let all_contexts = &context["contexts"];
    if kx == "all" {
        // get current context
        // println!("Current context: {}",current_context);
        // println!("context : {}",context["users"][0]["name"].as_str().unwrap());
        
        for context_name in all_contexts.as_vec().unwrap() {
            // println!("{:?}",context_name["name"].as_str().unwrap());
            if context_name["name"].as_str().unwrap() == current_context {
                println!("{}",context_name["name"].as_str().unwrap().green());
            } else {
                println!("{}",context_name["name"].as_str().unwrap());
            }
            
        }

    } else {
        let mut check_context_exists = false;
        for context_name in all_contexts.as_vec().unwrap() {
            if context_name["name"].as_str().unwrap() == kx {
                check_context_exists = true;
            }
        }

        if check_context_exists == true {
            let contents = fs::read_to_string(kubeconfig)
            .expect("Something went wrong reading the file");

            let mut value: serde_yaml::Value = serde_yaml::from_str(&contents).unwrap();
            *value.get_mut("current-context").unwrap() = kx.into();
            // serde_yaml::to_writer(std::io::stdout(), &value).unwrap();
            let writer = serde_yaml::to_string(&value).unwrap();
            let mut file = std::fs::File::create(kubeconfig).expect("create failed");
            file.write_all(writer.as_bytes()).expect("write failed");
            println!("Switched to context to \"{}\"",kx.green());

            // let mut file = File::create("/home/home/.kube/config").unwrap();
            // serde_yaml.to_writer(&mut file, &value);
        } else {
            println!("error: no context exists with the name: \"{}\"",kx.red());
            process::exit(1);
        }

    }
}
