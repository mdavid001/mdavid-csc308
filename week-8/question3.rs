use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
use std::process::Command;

fn main() {
    loop {
        println!("\n=== NOTE TAKING APP ===");
        println!("1) Add a note");
        println!("2) View notes");
        println!("3) Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_note(),
            "2" => view_notes(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn add_note() {
    print!("Write your note: ");
    io::stdout().flush().unwrap();

    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();

    let timestamp = get_os_timestamp();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("Could not open notes.txt");

    writeln!(file, "[{}] {}", timestamp.trim(), note.trim()).unwrap();
    println!("Note saved!");
}

fn view_notes() {
    let mut content = String::new();

    match File::open("notes.txt") {
        Ok(mut f) => {
            f.read_to_string(&mut content).unwrap();
            println!("\n=== NOTES ===\n{}", content);
        }
        Err(_) => println!("No notes found."),
    }
}

fn get_os_timestamp() -> String {
    let output = Command::new("date")
        .arg("+%Y-%m-%d %H:%M:%S")
        .output()
        .expect("Failed to run `date`");

    String::from_utf8_lossy(&output.stdout).to_string()
}

