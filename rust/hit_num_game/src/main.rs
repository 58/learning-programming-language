use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;

fn main() {
    // display the number of times you have guessed
    const FILE_NAME: &str = "hit_num_game.log";
    let path = Path::new(FILE_NAME);
    let display = path.display();
    let mut answer_times = 0;

    if path.exists() {
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut answered_times = String::new();
        match file.read_to_string(&mut answered_times) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }

        println!("You have guessed {} times.", answered_times);
    } else {
        match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(_) => (),
        };
    }

    // generate random number
    let answer = rand::thread_rng().gen_range(1..101); 

    loop {
        answer_times += 1;

        // input your answer
        println!("Please input your guess.");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line.");
        let input_num = buffer.trim().parse::<i32>().unwrap();

        // game over
        if answer_times == 10 {
            println!("You lose!");
            println!("The answer is {}.", answer);
            break;
        }

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
    }

    // write the number of times you have guessed
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(answer_times.to_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}
