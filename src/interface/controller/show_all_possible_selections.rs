use itertools::Itertools;
use lokicy::entity::pokemon;
use lokicy::entity::pokemon::Pt;

fn main() {
    let fixed_selection = [[Pt::Grass, Pt::Ice]];

    let others = [
        [Pt::Grass, Pt::Poison],
        [Pt::Electric, Pt::Steel],
        [Pt::Bug, Pt::Fighting],
        [Pt::Water, Pt::Flying],
        [Pt::Ground, Pt::Fighting],
    ];

    let mut answers = Vec::new();

    for other_selection in others.iter().combinations(3 - fixed_selection.len()) {
        let mut selection: Vec<[Pt; 2]> = Vec::new();
        let mut f: Vec<[Pt; 2]> = fixed_selection.to_vec();
        let mut o: Vec<[Pt; 2]> = other_selection.iter().map(|&p| p.clone()).collect();
        selection.append(&mut f);
        selection.append(&mut o);
        println!("selection: {:?}", &selection);

        let mut poor_size = 0;

        for att in &pokemon::ALL_POKEMON_TYPES {
            let mut min_rate = f64::INFINITY;

            for defs in &selection {
                let r = pokemon::calc_type_combination_matchup_rate(att, defs);
                min_rate = if min_rate < r { min_rate } else { r };
            }

            if min_rate >= 1.0 {
                poor_size += 1;
                println!("This party is poor at {:?} x{}", att, min_rate)
            }
        }

        let p = (selection, poor_size);
        answers.push(p);
        // println!("");
    }

    answers.sort_by(|a, b| (a.1).partial_cmp(&(b.1)).unwrap());

    for (selection, poor_size) in answers.iter() {
        println!("{:?} {}", selection, poor_size);
    }
}
