use std::io::stdin;
use rand::Rng;

pub fn guess_number(){

    let number = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();

                match parsed{
                    Ok(guess) =>{
                        if guess < 1 || guess > 100
                        {
                            println!("guess out of range");
                        }
                        else if guess < number
                        {
                            println!("guess too low");
                        }
                        else if guess > number
                        {
                            println!("guess too high");
                        }
                        else
                        {
                            println!("YAY!!!");
                            break;
                        }
                    },
                    Err(e) =>{
                        println!("{}",e);
                    }
                }
            },
            Err(_) => {
                continue
            }
        }
    }
}