use lokicy::entity::pokemon;

fn main() {
    for att in pokemon::ALL_POKEMON_TYPES {
        for def in pokemon::ALL_POKEMON_TYPES {
            let matchup_rate = pokemon::calc_matchup_rate(&att, &def);

            let ch = if matchup_rate == 2.0 {
                'O'
            } else if matchup_rate == 0.5 {
                '-'
            } else if matchup_rate == 0.0 {
                'X'
            } else if matchup_rate == 1.0 {
                ' '
            } else {
                'ぼ'
            };

            print!("{}|", ch);
        }
        println!("");
        println!("------------------------------------");
    }
}
