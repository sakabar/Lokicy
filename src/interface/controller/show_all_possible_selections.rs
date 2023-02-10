use itertools::Itertools;
use lokicy::entity::pokemon;
use lokicy::entity::pokemon::MetaType;
use lokicy::entity::pokemon::MetaType::Mpt;
use lokicy::entity::pokemon::MetaType::Mpta;
use lokicy::entity::pokemon::Pt;
use lokicy::entity::pokemon::Pta;

fn main() {
    let fixed_selection: Vec<Vec<MetaType>> = vec![vec![Mpt(Pt::Grass), Mpt(Pt::Ice)]];

    let others: Vec<Vec<MetaType>> = vec![
        vec![Mpt(Pt::Grass), Mpt(Pt::Poison)],
        vec![Mpt(Pt::Electric), Mpt(Pt::Steel)],
        vec![Mpt(Pt::Bug), Mpt(Pt::Fighting), Mpta(Pta::Levitate)],
        vec![Mpt(Pt::Water), Mpt(Pt::Flying)],
        vec![Mpt(Pt::Ground), Mpt(Pt::Fighting)],
    ];

    let mut answers = Vec::new();

    for other_selection in others.iter().combinations(3 - fixed_selection.len()) {
        let mut selection: Vec<Vec<MetaType>> = Vec::new();
        let mut f: Vec<Vec<MetaType>> = fixed_selection
            .iter()
            .map(|v| v.iter().map(|t| t.clone()).collect())
            .collect();

        let mut o: Vec<Vec<MetaType>> = other_selection
            .iter()
            .map(|v| v.iter().map(|t| t.clone()).collect())
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
