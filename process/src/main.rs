use std::process::Command;

fn main() {
    println!("Check power!");

    // pmset -g batt
    let output = Command::new("pmset")
        .arg("-g")
        .arg("batt")
        .output()
        .unwrap();

    // println!("{:#?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
}
