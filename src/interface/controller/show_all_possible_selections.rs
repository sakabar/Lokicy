use itertools::Itertools;
use lokicy::entity::pokemon;
use lokicy::entity::pokemon::MetaType;
use lokicy::entity::pokemon::Pt;

fn main() {
    let fixed_selection = [vec![Pt::Grass, Pt::Ice]];

    let others = [
        vec![Pt::Grass, Pt::Poison],
        vec![Pt::Electric, Pt::Steel],
        vec![Pt::Bug, Pt::Fighting],
        vec![Pt::Water, Pt::Flying],
        vec![Pt::Ground, Pt::Fighting],
    ];

    let mut answers = Vec::new();

    for other_selection in others.iter().combinations(3 - fixed_selection.len()) {
        let mut selection: Vec<Vec<Box<dyn MetaType>>> = Vec::new();
        let mut f: Vec<Vec<Box<dyn MetaType>>> = fixed_selection
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&t| Box::new(t) as Box<dyn MetaType>)
                    .collect()
            })
            .collect();
        let mut o: Vec<Vec<Box<dyn MetaType>>> = other_selection
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&t| Box::new(t) as Box<dyn MetaType>)
                    .collect()
            })
            .collect();

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
