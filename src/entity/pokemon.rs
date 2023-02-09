#[derive(Copy, Clone, Debug)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

pub type Pt = PokemonType;

pub const ALL_POKEMON_TYPES: [PokemonType; 18] = [
    Pt::Normal,
    Pt::Fire,
    Pt::Water,
    Pt::Electric,
    Pt::Grass,
    Pt::Ice,
    Pt::Fighting,
    Pt::Poison,
    Pt::Ground,
    Pt::Flying,
    Pt::Psychic,
    Pt::Bug,
    Pt::Rock,
    Pt::Ghost,
    Pt::Dragon,
    Pt::Dark,
    Pt::Steel,
    Pt::Fairy,
];

pub fn calc_matchup_rate(att: &PokemonType, def: &PokemonType) -> f64 {
    return match att {
        Pt::Normal => match def {
            Pt::Rock => 0.5,
            Pt::Ghost => 0.0,
            Pt::Steel => 0.5,
            _ => 1.0,
        },
        Pt::Fire => match def {
            Pt::Fire => 0.5,
            Pt::Water => 0.5,
            Pt::Grass => 2.0,
            Pt::Ice => 2.0,
            Pt::Bug => 2.0,
            Pt::Rock => 0.5,
            Pt::Dragon => 0.5,
            Pt::Steel => 2.0,
            _ => 1.0,
        },
        Pt::Water => match def {
            Pt::Fire => 2.0,
            Pt::Water => 0.5,
            Pt::Grass => 0.5,
            Pt::Ground => 2.0,
            Pt::Rock => 2.0,
            Pt::Dragon => 0.5,
            _ => 1.0,
        },
        Pt::Electric => match def {
            Pt::Water => 2.0,
            Pt::Electric => 0.5,
            Pt::Grass => 0.5,
            Pt::Ground => 0.0,
            Pt::Flying => 2.0,
            Pt::Dragon => 0.5,
            _ => 1.0,
        },
        Pt::Grass => match def {
            Pt::Fire => 0.5,
            Pt::Water => 2.0,
            Pt::Grass => 0.5,
            Pt::Poison => 0.5,
            Pt::Ground => 2.0,
            Pt::Flying => 0.5,
            Pt::Bug => 0.5,
            Pt::Rock => 2.0,
            Pt::Dragon => 0.5,
            Pt::Steel => 0.5,
            _ => 1.0,
        },
        Pt::Ice => match def {
            Pt::Fire => 0.5,
            Pt::Water => 0.5,
            Pt::Grass => 2.0,
            Pt::Ice => 0.5,
            Pt::Ground => 2.0,
            Pt::Flying => 2.0,
            Pt::Dragon => 2.0,
            Pt::Steel => 0.5,
            _ => 1.0,
        },
        Pt::Fighting => match def {
            Pt::Normal => 2.0,
            Pt::Ice => 2.0,
            Pt::Poison => 0.5,
            Pt::Flying => 0.5,
            Pt::Psychic => 0.5,
            Pt::Bug => 0.5,
            Pt::Rock => 2.0,
            Pt::Ghost => 0.0,
            Pt::Dark => 2.0,
            Pt::Steel => 2.0,
            Pt::Fairy => 0.5,
            _ => 1.0,
        },
        Pt::Poison => match def {
            Pt::Grass => 2.0,
            Pt::Poison => 0.5,
            Pt::Ground => 0.5,
            Pt::Rock => 0.5,
            Pt::Ghost => 0.5,
            Pt::Steel => 0.0,
            Pt::Fairy => 2.0,
            _ => 1.0,
        },
        Pt::Ground => match def {
            Pt::Fire => 2.0,
            Pt::Electric => 2.0,
            Pt::Grass => 0.5,
            Pt::Poison => 2.0,
            Pt::Flying => 0.0,
            Pt::Bug => 0.5,
            Pt::Rock => 2.0,
            Pt::Steel => 2.0,
            _ => 1.0,
        },
        Pt::Flying => match def {
            Pt::Electric => 0.5,
            Pt::Grass => 2.0,
            Pt::Fighting => 2.0,
            Pt::Bug => 2.0,
            Pt::Rock => 0.5,
            Pt::Steel => 0.5,
            _ => 1.0,
        },
        Pt::Psychic => match def {
            Pt::Fighting => 2.0,
            Pt::Poison => 2.0,
            Pt::Psychic => 0.5,
            Pt::Dark => 0.0,
            Pt::Steel => 0.5,
            _ => 1.0,
        },
        Pt::Bug => match def {
            Pt::Fire => 0.5,
            Pt::Grass => 2.0,
            Pt::Fighting => 0.5,
            Pt::Poison => 0.5,
            Pt::Flying => 0.5,
            Pt::Psychic => 2.0,
            Pt::Ghost => 0.5,
            Pt::Dark => 2.0,
            Pt::Steel => 0.5,
            Pt::Fairy => 0.5,
            _ => 1.0,
        },
        Pt::Rock => match def {
            Pt::Fire => 2.0,
            Pt::Ice => 2.0,
            Pt::Fighting => 0.5,
            Pt::Ground => 0.5,
            Pt::Flying => 2.0,
            Pt::Bug => 2.0,
            Pt::Steel => 0.5,
            _ => 1.0,
        },
        Pt::Ghost => match def {
            Pt::Normal => 0.0,
            Pt::Psychic => 2.0,
            Pt::Ghost => 2.0,
            Pt::Dark => 0.5,
            _ => 1.0,
        },
        Pt::Dragon => match def {
            Pt::Dragon => 2.0,
            Pt::Steel => 0.5,
            Pt::Fairy => 0.0,
            _ => 1.0,
        },
        Pt::Dark => match def {
            Pt::Fighting => 0.5,
            Pt::Psychic => 2.0,
            Pt::Ghost => 2.0,
            Pt::Dark => 0.5,
            Pt::Fairy => 0.5,
            _ => 1.0,
        },
        Pt::Steel => match def {
            Pt::Fire => 0.5,
            Pt::Water => 0.5,
            Pt::Electric => 0.5,
            Pt::Ice => 2.0,
            Pt::Rock => 2.0,
            Pt::Steel => 0.5,
            Pt::Fairy => 2.0,
            _ => 1.0,
        },
        Pt::Fairy => match def {
            Pt::Fire => 0.5,
            Pt::Fighting => 2.0,
            Pt::Poison => 0.5,
            Pt::Dragon => 2.0,
            Pt::Dark => 2.0,
            Pt::Steel => 0.5,
            _ => 1.0,
        },
    };
}

#[test]
fn it_works() {
    let att = Pt::Ground;
    let defs = [Pt::Electric, Pt::Steel];
    let actual = calc_type_combination_matchup_rate(&att, &defs);

    assert_eq!(actual, 4.0);
}

pub fn calc_type_combination_matchup_rate(att: &PokemonType, defs: &[PokemonType]) -> f64 {
    let mut ans: f64 = 1.0;

    for def in defs {
        ans *= calc_matchup_rate(att, def);
    }

    return ans;
}
