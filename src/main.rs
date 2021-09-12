/*
* Welcome is a simple prompt welcome program. You basically call this before your actual prompt
* when your shell starts and it will print based on your configuration
*
* */
use std::env;
use std::process::Command;

use std::str;

trait Section {
    fn to_string(&self) -> String;
}

fn run_command(command: String) -> std::io::Result<std::process::Output> {
    Command::new("sh").arg("-c").arg(command).output()
}

struct OS {}
impl Section for OS {
    fn to_string(&self) -> String {
        format!("Operating System: {}", env::consts::OS)
    }
}

// struct Network {}
// impl Section for Network {
//     fn to_string(&self) -> String {}
// }

struct Disk {}
impl Section for Disk {
    fn to_string(&self) -> String {
        let avail_home = fs2::available_space(home::home_dir().expect("cannot get home dir"))
            .expect("cannot get available_space of home");
        let avail_root = fs2::available_space("/").expect("cannot get avail space at root");

        format!(
            "Home: {}, Root: {}",
            pretty_size(&mut avail_home.clone()),
            pretty_size(&mut avail_root.clone())
        )
    }
}

fn pretty_size(size: &mut u64) -> String {
    let suffixes: Vec<&str> = vec!["B", "KB", "MB", "GB", "TB"];
    let mut suffix_index = 0;
    while size > &mut 1024 && suffix_index < 4 {
        suffix_index += 1;
        *size = *size / 1024;
    }
    return format!("{}{}", size, suffixes[suffix_index]);
}
struct Hostname {}
impl Section for Hostname {
    fn to_string(&self) -> String {
        let s: String = match run_command(String::from("uname -n")) {
            Ok(output) => String::from(
                str::from_utf8(&output.stdout)
                    .expect("cannot stringify output of command")
                    .to_string()
                    .trim_end(),
            ),
            Err(err) => panic!("{}", err),
        };
        format!("Hostname: {}", s)
    }
}

fn main() {
    let sections: Vec<&dyn Section> = vec![&OS {}, &Hostname {}, &Disk {}];
    for section in sections.into_iter() {
        println!("{}", section.to_string());
    }
}
