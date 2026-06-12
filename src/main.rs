use colorful::Colorful;
use std::env::args;
use std::process::{Command, Stdio};

fn main() {
    let mut run_with_sudo: bool;
    let mut no_whoami: bool = false;
    let arguments: Vec<String> = args().collect();

    // making sure that at least 2 arguments are supplied
    // before setting them to variables
    if arguments.len() < 2 {
        println!(
            "{} please enter the device and mountpoint {}",
            "error:".red(),
            "(eg. automount /dev/sda1 /mnt/)".dark_gray()
        );
        std::process::exit(1);
    }

    let device = &arguments[1];
    let mountpoint = &arguments[2];

    let user = Command::new("whoami")
        .stdout(Stdio::piped())
        .output();

    // everyone has whoami this is the most pointless thing ever
    // you could've just ran with sudo when whoami fails
    // but noo you had to implement a failsafe
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
    dbg!("{}", &user);
    // user now contains just the current user name (without the trailing newline)

    if &user != "root" {
        run_with_sudo = true;
    } else {
        run_with_sudo = false;
    }
    dbg!("{} {} {}", &arguments, device, mountpoint);
}
