use std::process::{Command, Stdio};
use std::env::args;
use colorful::Colorful;

fn main() {
    let mut no_whoami = false;
    let arguments: Vec<String> = args().collect();
    if arguments.len() < 2 {
        println!("{} Please enter the device and mountpoint {}", "error:".red(), "(eg. automount /dev/sda1 /mnt/)".dark_gray());
        std::process::exit(1);
    }

    let device = &arguments[1];
    let mountpoint = &arguments[2];

    let user = Command::new("whoami")
        .stdout(Stdio::piped())
        .output();

    if user.is_ok() != true {
        println!("whoami is not installed, trying id -un");
        let user = Command::new("id").arg("-un").output();
        no_whoami = true;
    }

    if user.is_ok() != true && no_whoami {
        println!("id -un failed, running as sudo even if you are root");
    }

    let mut user = String::from_utf8(user.unwrap().stdout).unwrap();
    user.pop();
    dbg!("{}", user);

    dbg!("{} {} {}", &arguments, device, mountpoint);
}