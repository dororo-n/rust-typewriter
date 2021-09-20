mod prompts;
mod utils {
    pub mod typewriter;
}

fn main() {
    // prompts::prompt("you did it!", "good job!");
    utils::typewriter::write("what is this", 70, true, 4);
}
