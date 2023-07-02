use std::env;

fn fib(num_req: u32) -> u32{


    match num_req {


        0 => 1,
        1 => 1,
        _ => fib(num_req-1) + fib(num_req-2),

    }
}

fn main(){
        let args: Vec<_> = env::args().collect();

        let mut fib_count = 0;

        if args.len() < 2 {

            println!("Usage: ./fib INTEGER");
            return;

        }

        let num_req = match args[1].parse::<u32>(){

            Ok(num) => num,
            Err(_) => {

                println!("Invalid number");
                return;
            }

        };

        while fib_count < num_req {

            println!("{}", fib(fib_count));

            fib_count += 1;

        }
}
