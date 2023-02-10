use std::fmt;

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

impl PokemonType {
    pub fn calc_matchup_rate(&self, att: &PokemonType) -> f64 {
        return match att {
            Pt::Normal => match &self {
                Pt::Rock => 0.5,
                Pt::Ghost => 0.0,
                Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Fire => match &self {
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
            Pt::Water => match &self {
                Pt::Fire => 2.0,
                Pt::Water => 0.5,
                Pt::Grass => 0.5,
                Pt::Ground => 2.0,
                Pt::Rock => 2.0,
                Pt::Dragon => 0.5,
                _ => 1.0,
            },
            Pt::Electric => match &self {
                Pt::Water => 2.0,
                Pt::Electric => 0.5,
                Pt::Grass => 0.5,
                Pt::Ground => 0.0,
                Pt::Flying => 2.0,
                Pt::Dragon => 0.5,
                _ => 1.0,
            },
            Pt::Grass => match &self {
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
            Pt::Ice => match &self {
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
            Pt::Fighting => match &self {
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
            Pt::Poison => match &self {
                Pt::Grass => 2.0,
                Pt::Poison => 0.5,
                Pt::Ground => 0.5,
                Pt::Rock => 0.5,
                Pt::Ghost => 0.5,
                Pt::Steel => 0.0,
                Pt::Fairy => 2.0,
                _ => 1.0,
            },
            Pt::Ground => match &self {
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
            Pt::Flying => match &self {
                Pt::Electric => 0.5,
                Pt::Grass => 2.0,
                Pt::Fighting => 2.0,
                Pt::Bug => 2.0,
                Pt::Rock => 0.5,
                Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Psychic => match &self {
                Pt::Fighting => 2.0,
                Pt::Poison => 2.0,
                Pt::Psychic => 0.5,
                Pt::Dark => 0.0,
                Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Bug => match &self {
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
            Pt::Rock => match &self {
                Pt::Fire => 2.0,
                Pt::Ice => 2.0,
                Pt::Fighting => 0.5,
                Pt::Ground => 0.5,
                Pt::Flying => 2.0,
                Pt::Bug => 2.0,
                Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Ghost => match &self {
                Pt::Normal => 0.0,
                Pt::Psychic => 2.0,
                Pt::Ghost => 2.0,
                Pt::Dark => 0.5,
                _ => 1.0,
            },
            Pt::Dragon => match &self {
                Pt::Dragon => 2.0,
                Pt::Steel => 0.5,
                Pt::Fairy => 0.0,
                _ => 1.0,
            },
            Pt::Dark => match &self {
                Pt::Fighting => 0.5,
                Pt::Psychic => 2.0,
                Pt::Ghost => 2.0,
                Pt::Dark => 0.5,
                Pt::Fairy => 0.5,
                _ => 1.0,
            },
            Pt::Steel => match &self {
                Pt::Fire => 0.5,
                Pt::Water => 0.5,
                Pt::Electric => 0.5,
                Pt::Ice => 2.0,
                Pt::Rock => 2.0,
                Pt::Steel => 0.5,
                Pt::Fairy => 2.0,
                _ => 1.0,
            },
            Pt::Fairy => match &self {
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
}

pub type Pt = PokemonType;

#[derive(Copy, Clone, Debug)]
pub enum PokemonTypeAbility {
    Levitate,
    SapSipper,
    VoltAbsorb,
    WaterAbsorb,
    EarthEater,
    FlashFire,
}

impl PokemonTypeAbility {
    pub fn calc_matchup_rate(&self, att: &PokemonType) -> f64 {
        return match &self {
            Pta::Levitate => match att {
                Pt::Ground => 0.0,
                _ => 1.0,
            },
            Pta::SapSipper => match att {
                Pt::Grass => 0.0,
                _ => 1.0,
            },
            Pta::VoltAbsorb => match att {
                Pt::Electric => 0.0,
                _ => 1.0,
            },
            Pta::WaterAbsorb => match att {
                Pt::Water => 0.0,
                _ => 1.0,
            },
            Pta::EarthEater => match att {
                Pt::Ground => 0.0,
                _ => 1.0,
            },
            Pta::FlashFire => match att {
                Pt::Fire => 0.0,
                _ => 1.0,
            },
        };
    }
}

pub type Pta = PokemonTypeAbility;

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

pub fn calc_type_combination_matchup_rate(att: &PokemonType, defs: &Vec<Box<dyn MetaType>>) -> f64 {
    let mut ans: f64 = 1.0;

    for def in defs.iter() {
        ans *= def.calc_matchup_rate(att);
    }

    return ans;
}

#[test]
fn it_works_for_pokemon_type() {
    let att = Pt::Ground;
    let defs: [Metatype] = [Pt::Electric, Pt::Steel];
    let actual = calc_type_combination_matchup_rate(&att, &defs);

    assert_eq!(actual, 4.0);
}

#[test]
fn it_works_for_pokemon_type_ability() {
    let att = Pt::Ground;
    let defs: [Metatype] = [Pt::Electric, Pt::Steel, Pta::EarthEater];
    let actual = calc_type_combination_matchup_rate(&att, &defs);

    assert_eq!(actual, 0.0);
}

pub trait MetaType {
    fn calc_matchup_rate(&self, att: &PokemonType) -> f64;
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl MetaType for PokemonType {
    fn calc_matchup_rate(&self, att: &PokemonType) -> f64 {
        self.calc_matchup_rate(att)
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for dyn MetaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MetaType for PokemonTypeAbility {
    fn calc_matchup_rate(&self, att: &PokemonType) -> f64 {
        self.calc_matchup_rate(att)
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// #[derive(Debug)]
// struct PokemonClass {
//     no: u8,
//     pt1: PokemonType,
// }
