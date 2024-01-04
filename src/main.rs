use core::num;
use dotenv;
use rand::Rng;
use reqwest::*;
use std::io;
// use std::io::Result;

struct Player {
    name: String,
    score: u32,
    wins: u32,
}

trait Printable {
    fn print_summary(&self) -> String;
}

impl Printable for Player {
    fn print_summary(&self) -> String {
        format!("@{} of score {}", self.name, self.score)
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_no_updates_is_fine) => continue,
        }
    }
}

fn get_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut num_players = 0;
    loop {
        num_players = get_input("Enter number of players, greater than equal to 2: ");
        if num_players < 2 {
            print!("Invalid number");
            continue;
        } else {
            break;
        }
    }

    for n in 0..num_players {
        let player_name = get_input(format!("Player {} name", n).as_str());
        players.push(Player {
            name: player_name,
            score: 0,
            wins: 0,
        });
    }
    return players;
}

#[tokio::main]
// M-2: via API
async fn generate_number(max_range: u32) -> Result<u32> {
    // dotenv().ok();
    dotenv::from_path("./.env").expect(".ENV file is present");
    let url_token = std::env::var("RAND_URL").expect("RAND_URL must be set.");
    let body = reqwest::get(url_token.replace("{MAX}", &max_range.to_string()))
        .await?
        .text()
        .await?;

    let val = body.trim().parse::<u32>().expect("Error in parsing");
    println!("value = {:?}", val);

    Ok(val)
}

fn collect_guesses_into_proximities(
    players: &Vec<Player>,
    max_range: u32,
) -> Vec<(String, u32, u32)> {
    let mut player_proximities = Vec::<(String, u32, u32)>::new();
    let target = generate_number(100).expect("Failure in generating random value");
    // println!("target: {}", target);
    let mut index = 0;
    for player in players {
        println!("{}'s turn", player.name);
        let guess = get_input::<u32>(&format!("Guess the number (1-{max_range}):"));
        player_proximities.push((player.name.clone(), index, guess.abs_diff(target)));
        index += 1;
    }
    player_proximities
}

fn get_winner_name(players_vex: &Vec<(String, u32, u32)>) -> String {
    players_vex[0].0.to_owned()
}

fn main() {
    println!("Hello, world!");
    let mut players_list = get_players();
    loop {
        let mut player_prox_list = collect_guesses_into_proximities(&players_list, 100);
        player_prox_list.sort_by_key(|f| f.2);
        for player in &player_prox_list {
            print!("player prox {} {} \n", player.0, player.2)
        }

        print!("winner player {} \n", get_winner_name(&player_prox_list));

        println!("Continue play t(true) or f(false)");
        let mut loop_flag = String::new();
        io::stdin()
            .read_line(&mut loop_flag)
            .expect("Failed to read line");

        print!("loop flag {}", loop_flag);
        if loop_flag.trim() == "f" {
            print!("stop");
            break;
        }
    }

    // println!("Input maximum range for random number from");
    // let mut input_line = String::new();
    // io::stdin()
    //     .read_line(&mut input_line)
    //     .expect("Failed to read line");
    // let x: i32 = input_line.trim().parse().expect("Input not an integer");
    // let _ = bitcoin_score::get_bit_score();
    // println!("random number {}", x_rand);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random() -> Result<()> {
        let range: u32 = 10;
        let random_val: u32 = generate_number(range).unwrap();
        assert!(random_val > 0 && random_val <= range);
        Ok(())
    }

    #[test]
    fn test_player_print() -> Result<()> {
        let player = Player {
            name: "subbu".to_string(),
            score: 0,
            wins: 0,
        };
        assert_eq!(player.print_summary(), "@subbu of score 0");
        Ok(())
    }

    #[test]
    fn test_proximity_winner() -> Result<()> {
        let mut player_proximities = Vec::<(String, u32, u32)>::new();
        let names = vec!["a", "b", "c"];
        let scores = vec![50, 25, 17];
        for i in 0..3 {
            player_proximities.push((names[i].to_string(), 0, scores[i])); // If I don't de-reference with *, I get Vec<(&f64, &f64)>
        }
        player_proximities.sort_by_key(|f| f.2);

        assert_eq!(get_winner_name(&player_proximities), "c");
        Ok(())
    }
}
