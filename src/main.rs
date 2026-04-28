use std::env;
use std::time::Instant;

const APP_NAME: &str = "net-probe-02adfb";
const VERSION: &str = "0.4.0";

struct Handler {
    name: String,
    version: String,
}

impl Handler {
    fn new() -> Self {
        Self {
            name: APP_NAME.to_string(),
            version: VERSION.to_string(),
        }
    }

    fn process(&self, input: &str) -> String {
        println!("[{}] Processing: {}", self.name, input);
        format!("processed_{}_{}", input, self.name)
    }

    fn run(&self) {
        let start = Instant::now();
        println!("[{}] v{} starting...", self.name, self.version);
        let result = self.process("default");
        let elapsed = start.elapsed();
        println!("[{}] Result: {} ({:.2?})", self.name, result, elapsed);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        println!("{} v{}", APP_NAME, VERSION);
        return;
    }
    let handler = Handler::new();
    handler.run();
}
