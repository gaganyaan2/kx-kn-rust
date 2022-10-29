extern crate yaml_rust;
use std::env;
use std::path::Path;
use std::process;

pub fn get_kubeconfig_file() -> String {
    let mut kubeconfig = "";
    let mut kubeconfig_env = "".to_string();
    
    // check KUBECONFIG env
    let kubeconfig_env_check = env::var("KUBECONFIG").ok();
    if kubeconfig_env_check != None {
        kubeconfig_env = env::var("KUBECONFIG").ok().unwrap().to_string();
    }
    // check config file in home dir
    let homedir = env::home_dir().unwrap().as_path().display().to_string();
    let home_kubeconfig = format!("{}{}",homedir, "/.kube/config");

    // set default config
    if kubeconfig_env.is_empty() {
        kubeconfig = &home_kubeconfig;
    } else {
        kubeconfig = &kubeconfig_env;
    }

    //check if config file exists
    if Path::new(kubeconfig).exists() {
        // println!("kubeconfig file exists {}",kubeconfig);
        return kubeconfig.to_string();
    } else {
        println!("{} : kubeconfig file does not exists ",kubeconfig);
        process::exit(1);
    }
    
}
