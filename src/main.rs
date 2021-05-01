use rand::Rng;
use std::io;
use std::thread;
use std::time;

fn get_player_name(index: i32) -> String {
    let mut input = String::new();

    println!("Please enter a name for player {}: ", index);

    io::stdin()
        .read_line(&mut input)
        .expect("Did not receieve player name");

    let player_name = input.trim();

    return player_name.to_string();
}

fn get_wager() -> i32 {
    let mut input = String::new();

    println!("Please enter a wager: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Did not receive wager");

    let wager = input
        .trim()
        .parse()
        .expect("Failed to parse integer from input");

    return wager;
}

fn calculate_sleep_time(sleep_time: i32) -> time::Duration {
    if sleep_time >= 5000 {
        return time::Duration::from_millis(250);
    } else if sleep_time < 5000 && sleep_time >= 1000 {
        return time::Duration::from_millis(500);
    } else if sleep_time < 1000 && sleep_time >= 100 {
        return time::Duration::from_millis(750);
    } else if sleep_time < 100 && sleep_time >= 10 {
        return time::Duration::from_millis(1000);
    } else {
        return time::Duration::from_millis(1250);
    }
}

fn play(players: Vec<String>, wager: i32) {
    let mut player = 0;
    let mut starting_point = 10000;
    let mut rng = rand::thread_rng();

    while starting_point > 0 {
        // Generate new roll
        let roll = rng.gen_range(1..starting_point);

        // Print where we're at
        println!(
            "[{}]: /roll 1-{} => {}",
            players[player], starting_point, roll
        );

        // Check for winner
        if roll == 1 {
            if player == 0 {
                println!("{} wins {}g!", players[0], wager);
            } else {
                println!("{} wins {}g!", players[1], wager);
            }

            return;
        }

        // Set starting point to the previous roll
        starting_point = roll;

        // Toggle player
        if player == 0 {
            player = 1;
        } else {
            player = 0;
        }

        // Calculate sleep time
        let sleep_time = calculate_sleep_time(starting_point);

        // Sleep to make it suspenseful!
        thread::sleep(sleep_time);
    }
}

fn main() {
    let player_1 = get_player_name(1);
    let player_2 = get_player_name(2);
    let wager = get_wager();

    let mut players = vec![];

    players.push(player_1);
    players.push(player_2);

    play(players, wager);
}
