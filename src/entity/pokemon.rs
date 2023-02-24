#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BasicElement {
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

impl BasicElement {
    pub fn calc_matchup_rate(&self, att: &BasicElement) -> f64 {
        return match att {
            Be::Normal => match &self {
                Be::Rock | Be::Steel => 0.5,
                Be::Ghost => 0.0,
                _ => 1.0,
            },
            Be::Fire => match &self {
                Be::Grass | Be::Ice | Be::Bug | Be::Steel => 2.0,
                Be::Fire | Be::Water | Be::Rock | Be::Dragon => 0.5,
                _ => 1.0,
            },
            Be::Water => match &self {
                Be::Fire | Be::Ground | Be::Rock => 2.0,
                Be::Water | Be::Grass | Be::Dragon => 0.5,
                _ => 1.0,
            },
            Be::Electric => match &self {
                Be::Water | Be::Flying => 2.0,
                Be::Electric | Be::Grass | Be::Dragon => 0.5,
                Be::Ground => 0.0,
                _ => 1.0,
            },
            Be::Grass => match &self {
                Be::Water | Be::Ground | Be::Rock => 2.0,
                Be::Fire
                | Be::Grass
                | Be::Poison
                | Be::Flying
                | Be::Bug
                | Be::Dragon
                | Be::Steel => 0.5,
                _ => 1.0,
            },
            Be::Ice => match &self {
                Be::Grass | Be::Ground | Be::Flying | Be::Dragon => 2.0,
                Be::Fire | Be::Water | Be::Ice | Be::Steel => 0.5,
                _ => 1.0,
            },
            Be::Fighting => match &self {
                Be::Normal | Be::Ice | Be::Rock | Be::Dark | Be::Steel => 2.0,
                Be::Poison | Be::Flying | Be::Psychic | Be::Bug | Be::Fairy => 0.5,
                Be::Ghost => 0.0,
                _ => 1.0,
            },
            Be::Poison => match &self {
                Be::Grass | Be::Fairy => 2.0,
                Be::Poison | Be::Ground | Be::Rock | Be::Ghost => 0.5,
                Be::Steel => 0.0,
                _ => 1.0,
            },
            Be::Ground => match &self {
                Be::Fire | Be::Electric | Be::Poison | Be::Rock | Be::Steel => 2.0,
                Be::Grass | Be::Bug => 0.5,
                Be::Flying => 0.0,
                _ => 1.0,
            },
            Be::Flying => match &self {
                Be::Grass | Be::Fighting | Be::Bug => 2.0,
                Be::Electric | Be::Rock | Be::Steel => 0.5,
                _ => 1.0,
            },
            Be::Psychic => match &self {
                Be::Fighting | Be::Poison => 2.0,
                Be::Psychic | Be::Steel => 0.5,
                Be::Dark => 0.0,
                _ => 1.0,
            },
            Be::Bug => match &self {
                Be::Grass | Be::Psychic | Be::Dark => 2.0,
                Be::Fire
                | Be::Fighting
                | Be::Poison
                | Be::Flying
                | Be::Ghost
                | Be::Steel
                | Be::Fairy => 0.5,
                _ => 1.0,
            },
            Be::Rock => match &self {
                Be::Fire | Be::Ice | Be::Flying | Be::Bug => 2.0,
                Be::Fighting | Be::Ground | Be::Steel => 0.5,
                _ => 1.0,
            },
            Be::Ghost => match &self {
                Be::Psychic | Be::Ghost => 2.0,
                Be::Dark => 0.5,
                Be::Normal => 0.0,
                _ => 1.0,
            },
            Be::Dragon => match &self {
                Be::Dragon => 2.0,
                Be::Steel => 0.5,
                Be::Fairy => 0.0,
                _ => 1.0,
            },
            Be::Dark => match &self {
                Be::Psychic | Be::Ghost => 2.0,
                Be::Fighting | Be::Dark | Be::Fairy => 0.5,
                _ => 1.0,
            },
            Be::Steel => match &self {
                Be::Ice | Be::Rock | Be::Fairy => 2.0,
                Be::Fire | Be::Water | Be::Electric | Be::Steel => 0.5,
                _ => 1.0,
            },
            Be::Fairy => match &self {
                Be::Fighting | Be::Dragon | Be::Dark => 2.0,
                Be::Fire | Be::Poison | Be::Steel => 0.5,
                _ => 1.0,
            },
        };
    }
}

type Be = BasicElement;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AbilityElement {
    Levitate,
    SapSipper,
    VoltAbsorb,
    WaterAbsorb,
    EarthEater,
    FlashFire,
}

type Ae = AbilityElement;

impl AbilityElement {
    pub fn calc_matchup_rate(&self, att: &BasicElement) -> f64 {
        return match &self {
            Ae::Levitate => match att {
                Be::Ground => 0.0,
                _ => 1.0,
            },
            Ae::SapSipper => match att {
                Be::Grass => 0.0,
                _ => 1.0,
            },
            Ae::VoltAbsorb => match att {
                Be::Electric => 0.0,
                _ => 1.0,
            },
            Ae::WaterAbsorb => match att {
                Be::Water => 0.0,
                _ => 1.0,
            },
            Ae::EarthEater => match att {
                Be::Ground => 0.0,
                _ => 1.0,
            },
            Ae::FlashFire => match att {
                Be::Fire => 0.0,
                _ => 1.0,
            },
        };
    }
}

pub const ALL_POKEMON_TYPES: [BasicElement; 18] = [
    Be::Normal,
    Be::Fire,
    Be::Water,
    Be::Electric,
    Be::Grass,
    Be::Ice,
    Be::Fighting,
    Be::Poison,
    Be::Ground,
    Be::Flying,
    Be::Psychic,
    Be::Bug,
    Be::Rock,
    Be::Ghost,
    Be::Dragon,
    Be::Dark,
    Be::Steel,
    Be::Fairy,
];

// Enum Wrapper Pattern
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MetaElement {
    Mnone,
    Mbe(BasicElement),
    Mae(AbilityElement),
}

impl MetaElement {
    pub fn calc_matchup_rate(&self, att: &BasicElement) -> f64 {
        match self {
            MetaElement::Mnone => 1.0,
            MetaElement::Mbe(def) => def.calc_matchup_rate(att),
            MetaElement::Mae(def) => def.calc_matchup_rate(att),
        }
    }
}

pub fn calc_type_combination_matchup_rate(att: &BasicElement, defs: &Vec<MetaElement>) -> f64 {
    let mut ans: f64 = 1.0;

    for meta_def in defs.iter() {
        ans *= meta_def.calc_matchup_rate(att);
    }

    return ans;
}

#[test]
fn it_works_for_pokemon_type() {
    let att = Be::Ground;
    let defs: Vec<MetaElement> = vec![MetaElement::Mbe(Be::Electric), MetaElement::Mbe(Be::Steel)];
    let actual = calc_type_combination_matchup_rate(&att, &defs);

    assert_eq!(actual, 4.0);
}

#[test]
fn it_works_for_pokemon_type_ability() {
    let att = Be::Ground;
    let defs: Vec<MetaElement> = vec![
        MetaElement::Mbe(Be::Electric),
        MetaElement::Mbe(Be::Steel),
        MetaElement::Mae(Ae::EarthEater),
    ];
    let actual = calc_type_combination_matchup_rate(&att, &defs);

    assert_eq!(actual, 0.0);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PokemonSpecies {
    no: u16,
    elm1: MetaElement,
    elm2: MetaElement,
    meta_elm: MetaElement,
    base_hit_point: i32,
    base_attack: i32,
    base_defence: i32,
    base_special_attack: i32,
    base_special_defence: i32,
    base_speed: i32,
}

impl PokemonSpecies {
    fn new(
        no: u16,
        elm1: MetaElement,
        elm2: MetaElement,
        meta_elm: MetaElement,
        base_hit_point: i32,
        base_attack: i32,
        base_defence: i32,
        base_special_attack: i32,
        base_special_defence: i32,
        base_speed: i32,
    ) -> Self {
        Self {
            no,
            elm1,
            elm2,
            meta_elm,
            base_hit_point,
            base_attack,
            base_defence,
            base_special_attack,
            base_special_defence,
            base_speed,
        }
    }

    pub fn new_by_number_elements(
        no: u16,
        elm1: MetaElement,
        elm2: MetaElement,
        meta_elm: MetaElement,
    ) -> Self {
        Self::new(no, elm1, elm2, meta_elm, 0, 0, 0, 0, 0, 0)
    }
}

#[derive(Debug)]
pub struct PokemonIndividual {
    poke_species: PokemonSpecies,
    tera_type: BasicElement,
    is_terastallized: bool,
    hit_point: i32,
    attack: i32,
    defence: i32,
    special_attack: i32,
    special_defence: i32,
    speed: i32,
    moves: Vec<Move>,
    comment: String,
}

impl PokemonIndividual {
    pub fn new(
        poke_species: PokemonSpecies,
        tera_type: BasicElement,
        is_terastallized: bool,
        hit_point: i32,
        attack: i32,
        defence: i32,
        special_attack: i32,
        special_defence: i32,
        speed: i32,
        moves: Vec<Move>,
        comment: String,
    ) -> Self {
        Self {
            poke_species,
            tera_type,
            is_terastallized,
            hit_point,
            attack,
            defence,
            special_attack,
            special_defence,
            speed,
            moves,
            comment,
        }
    }

    pub fn calc_type_combination_matchup_rate(&self, att: &BasicElement) -> f64 {
        if self.is_terastallized {
            let elms = vec![MetaElement::Mbe(self.tera_type)];

            calc_type_combination_matchup_rate(att, &elms)
        } else {
            let elms = vec![
                self.poke_species.elm1,
                self.poke_species.elm2,
                self.poke_species.meta_elm,
            ];

            calc_type_combination_matchup_rate(att, &elms)
        }
    }

    pub fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }

    pub fn get_comment(&self) -> &str {
        &self.comment
    }

    pub fn get_offensive_index<'a>(&'a self, mv: &'a Move) -> (&MoveType, f64) {
        let mt = mv.get_move_type();
        let val = match mt {
            MoveType::Physical => (self.attack * mv.get_power()) as f64,
            MoveType::Special => (self.special_attack * mv.get_power()) as f64,
            MoveType::Status => 0.0,
        };

        let r = if (MetaElement::Mbe(*mv.get_poke_type())) == self.poke_species.elm1
            || (MetaElement::Mbe(*mv.get_poke_type())) == self.poke_species.elm2
        {
            1.5
        } else {
            1.0
        };

        return (mt, val * r);
    }

    pub fn get_defensive_index(&self, move_type: &MoveType) -> f64 {
        match move_type {
            MoveType::Physical => (self.hit_point * self.defence) as f64,
            MoveType::Special => (self.hit_point * self.special_defence) as f64,
            MoveType::Status => 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveType {
    Physical,
    Special,
    Status,
}

#[derive(Debug)]
pub struct Move {
    name: String,
    poke_type: BasicElement,
    move_type: MoveType,
    pp: u8,
    power: i32,
    accuracy: f64,
}

impl Move {
    pub fn new(
        name: &str,
        poke_type: BasicElement,
        move_type: MoveType,
        power: i32,
        accuracy: f64,
        pp: u8,
    ) -> Self {
        Self {
            name: name.to_string(),
            poke_type,
            move_type,
            pp,
            power,
            accuracy,
        }
    }

    pub fn get_poke_type(&self) -> &BasicElement {
        &self.poke_type
    }

    pub fn get_move_type(&self) -> &MoveType {
        &self.move_type
    }

    pub fn get_power(&self) -> i32 {
        self.power
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
