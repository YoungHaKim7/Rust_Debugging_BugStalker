use std::{thread::sleep, time::Duration};

fn myprint(s: &str) {
    println!("{}", s)
}
fn main() {
    myprint("Hello, World");

    sleep(Duration::from_secs(1));

    myprint("bye!");
}
