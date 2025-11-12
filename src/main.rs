use rand::Rng;
use std::io;

struct Game {
    secret_number: u32,
    plays: u32,
    status: bool, // Define se o jogador acertou/jogador terminou(true = acertou)
    last_play: u32,
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut data_game = Game {
        secret_number: rng.gen_range(1..=100),
        plays: 0,
        status: false,
        last_play: 0,
    };
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("***Failed to read user input***");
    let mut val: u32 = input.trim().parse().expect("***Enter a valid number***");
    let mut status_game: bool = update(&mut data_game, val);
    while status_game != true {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("***Failed to read user input***");
        val = input.trim().parse().expect("***Enter a valid number***");
        status_game = update(&mut data_game, val);
    }
}

fn update(data: &mut Game, value: u32) -> bool{
    data.plays += 1;
    data.last_play = value;
    let mut status = data.status;

    if value > data.secret_number {
        println!("The secret number is less than the number entered.");
    }

    else if value < data.secret_number {
        println!("The secret number is greater than the number entered.");
    }
    
    else if value == data.secret_number {
        status = true;
        println!("!!!Game Finished!!!");
        println!("The secret_number is..... {}", data.secret_number);

        println!("!!!Match details!!!");
        println!(r#"
        Secret Number: {}
        Attempts: {}
        Last_Guess: {}
        "#, data.secret_number, data.plays, data.last_play);
    }
    return status
    
}