use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("You input: {}", line);
                rl.add_history_entry(line);
            },
            Err(_) => {
                println!("Error occurred");
                break;
            },
        }
    }
}
