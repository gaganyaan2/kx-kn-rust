use std::env;
use std::process;

pub fn flags() {
    let args: Vec<String> = env::args().collect();
    let mut kn = "";

    if args.len() == 1 {
        kn = "all"
    } else {
        kn = args[1].as_str();
    }
    
    // help
    if kn == "--help" || kn == "-h" {
    println!("USAGE:
    kn                       : list all namespaces
    kn <NAME>                : switch to namespace <NAME>
    kn -h,--help             : show help message
    kn -V,--version          : show version
        ");
        process::exit(0);
    }
    // version
    if kn == "--version" || kn == "-V" {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("{}",VERSION);
        process::exit(0);
    }

    if kn.starts_with("-") {
        println!("invalid option try with --help");
        process::exit(1);
    }
}