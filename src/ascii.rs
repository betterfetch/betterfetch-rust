use os_info::Info;

pub fn print_ascii_art(os: &Info) {
    let name = os.os_type().to_string().to_lowercase();
    if name.contains("ubuntu") {
        println!("{}", UBUNTU_ASCII);
    } else if name.contains("debian") {
        println!("{}", DEBIAN_ASCII);
    } else if name.contains("arch") {
        println!("{}", ARCH_ASCII);
    } else {
        println!("{}", GENERIC_ASCII);
    }
}

const GENERIC_ASCII: &str = r#"
 _          _   _             __      _       _     
| |__   ___| |_| |_ ___ _ __ / _| ___| |_ ___| |__  
| '_ \ / _ \ __| __/ _ \ '__| |_ / _ \ __/ __| '_ \ 
| |_) |  __/ |_| ||  __/ |  |  _|  __/ || (__| | | |
|_.__/ \___|\__|\__\___|_|  |_|  \___|\__\___|_| |_|                                                
"#;

const UBUNTU_ASCII: &str = r#"
 _   _ _                 _         
| | | | |__  _   _ _ __ | |_ _   _ 
| | | | '_ \| | | | '_ \| __| | | |
| |_| | |_) | |_| | | | | |_| |_| |
 \___/|_.__/ \__,_|_| |_|\__|\__,_|
                                
"#;

const DEBIAN_ASCII: &str = r#"
 ____       _     _             
|  _ \  ___| |__ (_) __ _ _ __  
| | | |/ _ \ '_ \| |/ _` | '_ \ 
| |_| |  __/ |_) | | (_| | | | |
|____/ \___|_.__/|_|\__,_|_| |_|
                                
"#;

const ARCH_ASCII: &str = r#"
    _             _     
   / \   _ __ ___| |__  
  / _ \ | '__/ __| '_ \ 
 / ___ \| | | (__| | | |
/_/   \_\_|  \___|_| |_|

"#;
