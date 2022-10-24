extern crate yaml_rust;
use std::env;
use std::path::Path;
use std::process;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn get_kubeconfig_file() -> String {
    let mut kubeconfig = "";
    // check KUBECONFIG env
    let kubeconfig_env = env::var("KUBECONFIG").ok().unwrap().to_string();
    // check config file in home dir
    let home = env::var("HOME").ok().unwrap().to_string();
    let home_kubeconfig = format!("{}{}",home, "/.kube/config");

    // set default config
    if kubeconfig_env.is_empty() {
        kubeconfig = &home_kubeconfig;
    } else {
        kubeconfig = &kubeconfig_env;
    }

    //check if config file exists
    if Path::new(kubeconfig).exists() {
        println!("kubeconfig file exists {}",kubeconfig);
        return kubeconfig.to_string();
    } else {
        println!("{} : kubeconfig file does not exists ",kubeconfig);
        process::exit(1);
    }
    
}
