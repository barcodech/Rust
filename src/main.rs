use std::process::Command;
fn os_commands() {
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
                    .args(["/C", "echo Hi there from Windows!"])
                    .output()
                    .expect("Failed to execute command.")
    } else {
        Command::new("sh")
                    .args(["-c", "echo Hi there from Linux!"])
                    .output()
                    .expect("Failed to execute command.")
    };
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte response.");
    println!("{}", cmd_output);
}
 
 
 
fn main() {
    os_commands();
}
