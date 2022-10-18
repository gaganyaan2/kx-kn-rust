extern crate yaml_rust;
use std::fs;
use std::env;
use serde::{Serialize, Deserialize};


pub fn kubeconfig_from_env(){
    let kubeconfig_env = env::var("KUBECONFIG").expect("$KUBECONFIG is not set");
    println!("KUBECONFIG={}",kubeconfig_env);
}

pub fn kubeconfig_from_home_dir(){

    let home = env::var("HOME").expect("$HOME is not set");
    let home_kubeconfig = format!("{}{}",home.to_string(), "/.kube/config");
    println!("{:?}",home_kubeconfig);

    let mut contents = fs::read_to_string(home_kubeconfig)
        .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    // let mut value: serde_yaml::Value = serde_yaml::from_str(contents.to_string()).unwrap();
    // value["current-context"]["datadog"]["version"] = "1.38.8".into();
    // serde_yaml::to_writer(std::io::stdout(), &value).unwrap();
}
