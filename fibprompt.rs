use std::io::stdout;

fn fib(num_req: u32) -> u32{


    match num_req {


        0 => 1,
        1 => 1,
        _ => fib(num_req-1) + fib(num_req-2),

    }
}



fn main(){
    use std::io::{self, Write};

    let mut num = String::new();

    print!("Enter how many fibonacci numbers you would like to see: ");
    
    io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin().read_line(&mut num).expect("Failed to read line");


    let intnum = match num.trim().parse::<u32>(){

        Ok(num) => num,
        Err(_) => {

            println!("Invalid number!");
            return;

        }

    };

    let mut fib_count = 0;

    while fib_count < intnum{
            
            let mut lock = stdout().lock();
            write!(lock, "{} ", fib(fib_count)).expect("Error");
            fib_count += 1;

    }
    println!("");


}
