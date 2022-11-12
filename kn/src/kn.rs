extern crate yaml_rust;
use kube::api::{Api, ResourceExt, ListParams};
use kube::Client;
use k8s_openapi::api::core::v1::Namespace;
use std::fs;
use yaml_rust::YamlLoader;
use colored::Colorize;
use std::io::Write;
use std::process;

#[tokio::main]
pub async fn kn(kubeconfig: &str, k_namespace: &str) {

    let read_kubeconfig = fs::read_to_string(kubeconfig)
    .expect("Something went wrong reading KUBECONFIG file");

    let contexts = YamlLoader::load_from_str(&read_kubeconfig).unwrap();
    let context = &contexts[0];
    let current_context = context["current-context"].as_str().unwrap();
    let all_contexts = &context["contexts"];
    let mut count = 0;
    let mut index_count = 0;
    let mut current_namespace = "";
    let client = Client::try_default().await.unwrap();
    let namespace: Api<Namespace> = Api::all(client);

    // get default namespace index
    for namespace_name in all_contexts.as_vec().unwrap() {
        if namespace_name["name"].as_str().unwrap() == current_context {
            index_count = count;
            if context["contexts"][index_count]["context"]["namespace"].as_str().is_none() {
                current_namespace = ""
            } else {
                current_namespace = context["contexts"][index_count]["context"]["namespace"].as_str().unwrap();
            }
            break;
        }
        count = count+1;
    }
    if k_namespace == "" {
        // print default namespace
        for n1 in namespace.list(&ListParams::default()).await.unwrap() {
            if current_namespace == n1.name_any() {
                println!("{}", n1.name_any().green());
            } else {
                println!("{}", n1.name_any());
            }
        }
    } else {
        // switch namespace
        let mut check_namespace_exists = false;
        for namespace_name in namespace.list(&ListParams::default()).await.unwrap() {
            if namespace_name.name_any() == k_namespace {
                check_namespace_exists = true;
            }
        }
        if check_namespace_exists == true {
            let contents = fs::read_to_string(kubeconfig)
            .expect("Something went wrong reading KUBECONFIG file");

            let mut value: serde_yaml::Value = serde_yaml::from_str(&contents).unwrap();
            let target = value
            .get_mut("contexts")
            .unwrap()
            .get_mut(index_count)
            .unwrap()
            .get_mut("context")
            .unwrap()
            .as_mapping_mut()
            .unwrap();

            if target.contains_key("namespace") {
                *value
                .get_mut("contexts").unwrap()
                .get_mut(index_count).unwrap()
                .get_mut("context").unwrap()
                .get_mut("namespace").unwrap() = k_namespace.into();
            } else {
                target.insert("namespace".into(), k_namespace.into());
            }
        
            let writer = serde_yaml::to_string(&value).unwrap();
            let mut file = std::fs::File::create(kubeconfig).expect("create failed");
            file.write_all(writer.as_bytes()).expect("write failed");
            println!("Switched namespace to \"{}\"",k_namespace.green());
        } else {
            println!("error: no namespace exists with the name: \"{}\"",k_namespace.red());
            process::exit(1);
        }
    }

}