use std::env;
use std::process;

pub fn flags() {
    let args: Vec<String> = env::args().collect();
    let mut kx = "";

    if args.len() == 1 {
        kx = "all"
    } else {
        kx = args[1].as_str();
    }
    
    // help
    if kx == "--help" || kx == "-h" {
    println!("USAGE:
    kx                       : list the contexts
    kx <NAME>                : switch to context <NAME>
    kx -h,--help             : show help message
    kx -V,--version          : show version
        ");
        process::exit(0);
    }
    // version
    if kx == "--version" || kx == "-V" {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("{}",VERSION);
        process::exit(0);
    }

    if kx.starts_with("-") {
        println!("invalid option try with --help");
        process::exit(1);
    }
}
