extern crate yaml_rust;
use std::fs;
use yaml_rust::YamlLoader;
use colored::Colorize;
use std::io::Write;
use std::process;

pub fn kx(kubeconfig: &str, kcontext: &str) {

    let read_kubeconfig = fs::read_to_string(kubeconfig)
    .expect("Something went wrong reading KUBECONFIG file");

    let contexts = YamlLoader::load_from_str(&read_kubeconfig).expect("KUBECONFIG file incorrect format");
    let context = &contexts[0];
    let mut current_context = "";

    //check if current-context present in kubeconfig file
    if !context["current-context"].as_str().is_none() {
        current_context = context["current-context"].as_str().unwrap();
    }
    
    let all_contexts = &context["contexts"];
    if kcontext == "" {
        // get current context
        for context_name in all_contexts.as_vec().unwrap() {
            if context_name["name"].as_str().unwrap() == current_context {
                println!("{}",context_name["name"].as_str().unwrap().green());
            } else {
                println!("{}",context_name["name"].as_str().unwrap());
            }
            
        }

    } else {
        // set current context
        let mut check_context_exists = false;
        for context_name in all_contexts.as_vec().unwrap() {
            if context_name["name"].as_str().unwrap() == kcontext {
                check_context_exists = true;
            }
        }

        if check_context_exists == true {
            let contents = fs::read_to_string(kubeconfig)
            .expect("Something went wrong reading KUBECONFIG file");

            let mut value: serde_yaml::Value = serde_yaml::from_str(&contents).unwrap();
            *value.get_mut("current-context").unwrap() = kcontext.into();
            let writer = serde_yaml::to_string(&value).unwrap();
            let mut file = std::fs::File::create(kubeconfig).expect("create failed");
            file.write_all(writer.as_bytes()).expect("write failed");
            println!("Switched context to \"{}\"",kcontext.green());
        } else {
            println!("error: no context exists with the name: \"{}\"",kcontext.red());
            process::exit(1);
        }

    }
}
