use std::env;
use std::process;
use cargo_metadata::MetadataCommand;

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
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let meta = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .current_dir(&path)
            .exec()
            .unwrap();
        
        let root = meta.root_package().unwrap();
        let version = &root.version;
        println!("{}",version);
        process::exit(0);
    }

    if kx.starts_with("-") {
        println!("invalid option try with --help");
        process::exit(1);
    }
}
