use std::io;
use rand::Rng;

fn main() {
    // generate random number
    let answer = rand::thread_rng().gen_range(1..101); 

    let mut answer_times = 0;

    loop {
        answer_times += 1;

        // input your answer
        println!("Please input your guess.");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line.");
        let input_num = buffer.trim().parse::<i32>().unwrap();

        // evaluate your answer
        if input_num == answer {
            println!("You win!");
            println!("You have guessed {} times.", answer_times);
            break;
        }
        if input_num < answer {
            println!("Too small!");
            continue;
        }
        if input_num > answer {
            println!("Too big!");
            continue;
        }

        // game over
        if answer_times == 10 {
            println!("You lose!");
            println!("The answer is {}.", answer);
            break;
        }
    }
}
