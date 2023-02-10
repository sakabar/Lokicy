#[derive(Clone, Debug)]
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
                Pt::Rock | Pt::Steel => 0.5,
                Pt::Ghost => 0.0,
                _ => 1.0,
            },
            Pt::Fire => match &self {
                Pt::Grass | Pt::Ice | Pt::Bug | Pt::Steel => 2.0,
                Pt::Fire | Pt::Water | Pt::Rock | Pt::Dragon => 0.5,
                _ => 1.0,
            },
            Pt::Water => match &self {
                Pt::Fire | Pt::Ground | Pt::Rock => 2.0,
                Pt::Water | Pt::Grass | Pt::Dragon => 0.5,
                _ => 1.0,
            },
            Pt::Electric => match &self {
                Pt::Water | Pt::Flying => 2.0,
                Pt::Electric | Pt::Grass | Pt::Dragon => 0.5,
                Pt::Ground => 0.0,
                _ => 1.0,
            },
            Pt::Grass => match &self {
                Pt::Water | Pt::Ground | Pt::Rock => 2.0,
                Pt::Fire
                | Pt::Grass
                | Pt::Poison
                | Pt::Flying
                | Pt::Bug
                | Pt::Dragon
                | Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Ice => match &self {
                Pt::Grass | Pt::Ground | Pt::Flying | Pt::Dragon => 2.0,
                Pt::Fire | Pt::Water | Pt::Ice | Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Fighting => match &self {
                Pt::Normal | Pt::Ice | Pt::Rock | Pt::Dark | Pt::Steel => 2.0,
                Pt::Poison | Pt::Flying | Pt::Psychic | Pt::Bug | Pt::Fairy => 0.5,
                Pt::Ghost => 0.0,
                _ => 1.0,
            },
            Pt::Poison => match &self {
                Pt::Grass | Pt::Fairy => 2.0,
                Pt::Poison | Pt::Ground | Pt::Rock | Pt::Ghost => 0.5,
                Pt::Steel => 0.0,
                _ => 1.0,
            },
            Pt::Ground => match &self {
                Pt::Fire | Pt::Electric | Pt::Poison | Pt::Rock | Pt::Steel => 2.0,
                Pt::Grass | Pt::Bug => 0.5,
                Pt::Flying => 0.0,
                _ => 1.0,
            },
            Pt::Flying => match &self {
                Pt::Grass | Pt::Fighting | Pt::Bug => 2.0,
                Pt::Electric | Pt::Rock | Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Psychic => match &self {
                Pt::Fighting | Pt::Poison => 2.0,
                Pt::Psychic | Pt::Steel => 0.5,
                Pt::Dark => 0.0,
                _ => 1.0,
            },
            Pt::Bug => match &self {
                Pt::Grass | Pt::Psychic | Pt::Dark => 2.0,
                Pt::Fire
                | Pt::Fighting
                | Pt::Poison
                | Pt::Flying
                | Pt::Ghost
                | Pt::Steel
                | Pt::Fairy => 0.5,
                _ => 1.0,
            },
            Pt::Rock => match &self {
                Pt::Fire | Pt::Ice | Pt::Flying | Pt::Bug => 2.0,
                Pt::Fighting | Pt::Ground | Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Ghost => match &self {
                Pt::Psychic | Pt::Ghost => 2.0,
                Pt::Dark => 0.5,
                Pt::Normal => 0.0,
                _ => 1.0,
            },
            Pt::Dragon => match &self {
                Pt::Dragon => 2.0,
                Pt::Steel => 0.5,
                Pt::Fairy => 0.0,
                _ => 1.0,
            },
            Pt::Dark => match &self {
                Pt::Psychic | Pt::Ghost => 2.0,
                Pt::Fighting | Pt::Dark | Pt::Fairy => 0.5,
                _ => 1.0,
            },
            Pt::Steel => match &self {
                Pt::Ice | Pt::Rock | Pt::Fairy => 2.0,
                Pt::Fire | Pt::Water | Pt::Electric | Pt::Steel => 0.5,
                _ => 1.0,
            },
            Pt::Fairy => match &self {
                Pt::Fighting | Pt::Dragon | Pt::Dark => 2.0,
                Pt::Fire | Pt::Poison | Pt::Steel => 0.5,
                _ => 1.0,
            },
        };
    }
}

pub type Pt = PokemonType;

#[derive(Clone, Debug)]
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

// Enum Wrapper Pattern
#[derive(Clone, Debug)]
pub enum MetaType {
    Mpt(PokemonType),
    Mpta(PokemonTypeAbility),
}

impl MetaType {
    pub fn calc_matchup_rate(&self, att: &PokemonType) -> f64 {
        match self {
            MetaType::Mpt(def) => def.calc_matchup_rate(att),
            MetaType::Mpta(def) => def.calc_matchup_rate(att),
        }
    }
}

pub fn calc_type_combination_matchup_rate(att: &PokemonType, defs: &Vec<MetaType>) -> f64 {
    let mut ans: f64 = 1.0;

    for meta_def in defs.iter() {
        ans *= meta_def.calc_matchup_rate(att);
    }

    return ans;
}

#[test]
fn it_works_for_pokemon_type() {
    let att = Pt::Ground;
    let defs: Vec<MetaType> = vec![MetaType::Mpt(Pt::Electric), MetaType::Mpt(Pt::Steel)];
    let actual = calc_type_combination_matchup_rate(&att, &defs);

    assert_eq!(actual, 4.0);
}

#[test]
fn it_works_for_pokemon_type_ability() {
    let att = Pt::Ground;
    let defs: Vec<MetaType> = vec![
        MetaType::Mpt(Pt::Electric),
        MetaType::Mpt(Pt::Steel),
        MetaType::Mpta(Pta::EarthEater),
    ];
    let actual = calc_type_combination_matchup_rate(&att, &defs);

    assert_eq!(actual, 0.0);
}

// #[derive(Debug)]
// struct PokemonClass {
//     no: u8,
//     pt1: PokemonType,
// }
