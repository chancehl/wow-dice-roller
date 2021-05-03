use rand::Rng;
use std::io;
use std::thread;
use std::time;

struct Player {
    name: String,
    earnings: i32,
}

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

fn convert_response_to_boolean(response: String) -> bool {
    match response.as_str() {
        "y" => return true,
        "yes" => return true,
        "Y" => return true,
        "YES" => return true,
        _ => return false,
    }
}

fn prompt_for_next_game() -> String {
    let mut input = String::new();

    println!("Play another game? (y/n): ");

    io::stdin()
        .read_line(&mut input)
        .expect("Did not receieve player response");

    let response = input.trim();

    return response.to_string();
}

fn mutate_earnings(winning_player: &mut Player, losing_player: &mut Player, wager: i32) {
    winning_player.earnings += wager;
    losing_player.earnings -= wager;
}

fn announce_round_results(
    winning_player: &Player,
    player_one: &Player,
    player_two: &Player,
    wager: i32,
) {
    println!(
        "{} wins {}g! ({} => {}g / {} => {}g)",
        winning_player.name,
        wager,
        player_one.name,
        player_one.earnings,
        player_two.name,
        player_two.earnings
    );
}

fn play_round(player_one: &mut Player, player_two: &mut Player) {
    let mut player = 0;
    let mut starting_point = 10000;
    let mut rng = rand::thread_rng();

    let wager = get_wager();

    while starting_point > 0 {
        // Generate new roll
        let roll = rng.gen_range(1..starting_point);

        // Print where we're at
        let current_player_name = if player == 0 {
            &player_one.name
        } else {
            &player_two.name
        };

        // Announce roll
        println!(
            "[{}] /roll 1-{} => {}",
            current_player_name, starting_point, roll
        );

        // Check for winner
        if roll == 1 {
            if player == 0 {
                // Mutate earnings
                mutate_earnings(player_one, player_two, wager);

                // Announce game state
                announce_round_results(&player_one, &player_one, &player_two, wager);
            } else {
                // Mutate earnings
                mutate_earnings(player_two, player_one, wager);

                // Announce game state
                announce_round_results(&player_two, &player_one, &player_two, wager);
            }

            // Exit loop
            return;
        }

        // Set starting point to the previous roll
        starting_point = roll;

        // Toggle player
        player = if player == 0 { 1 } else { 0 };

        // Sleep to make it suspenseful!
        thread::sleep(time::Duration::from_millis(250));
    }
}

fn play(player_names: &Vec<String>) {
    let mut rounds_played = 1;

    let player_one_name = &player_names[0];
    let player_two_name = &player_names[1];

    let player_one = &mut Player {
        name: player_one_name.to_string(),
        earnings: 0,
    };
    let player_two = &mut Player {
        name: player_two_name.to_string(),
        earnings: 0,
    };

    loop {
        play_round(player_one, player_two);

        let response = prompt_for_next_game();
        let should_continue = convert_response_to_boolean(response);

        if should_continue {
            rounds_played = rounds_played + 1;
        } else {
            break;
        }
    }

    if player_one.earnings == player_two.earnings {
        println!("It's a miracle! Nobody lost any money. Everyone can go home happy!")
    } else if player_one.earnings > player_two.earnings {
        println!(
            "Game over. {} has earned {}g after {} rounds.",
            player_one.name, player_one.earnings, rounds_played
        );
    } else {
        println!(
            "Game over. {} has earned {}g after {} rounds.",
            player_two.name, player_two.earnings, rounds_played
        );
    }
}

fn main() {
    let player_one = get_player_name(1);
    let player_two = get_player_name(2);

    let mut players = vec![];

    players.push(player_one);
    players.push(player_two);

    play(&players);
}
