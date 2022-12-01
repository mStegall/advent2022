use std::env;

mod problem1;
mod problem2;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) => {
            if arg == "1"{
                problem1::main()
            } else if arg == "2"{
                problem2::main()
            }
        },
        None => println!("select a problem")
    }   
}