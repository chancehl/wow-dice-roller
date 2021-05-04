mod game;

use game::driver::play;
use game::io::get_player_name;

fn main() {
    let player_one = get_player_name(1);
    let player_two = get_player_name(2);

    let mut players = vec![];

    players.push(player_one);
    players.push(player_two);

    play(&players);
}
