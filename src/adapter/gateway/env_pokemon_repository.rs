use super::super::super::application::interface::EnvPokemonRepositoryTrait;
use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use super::super::super::entity::pokemon::MetaElement::Mnone;
use super::super::super::entity::pokemon::Move;
use super::super::super::entity::pokemon::MoveType;
use super::super::super::entity::pokemon::PokemonIndividual;
use super::super::super::entity::pokemon::PokemonSpecies;
use async_trait::async_trait;
use log::{debug, info, warn};

fn base_stats_to_stats_hp(
    base_stats: i32,
    individual_value: i32,
    effort_value: i32,
    level: i32,
) -> i32 {
    let x: i32 = (base_stats * 2 + individual_value + (effort_value / 4)) * level / 100;
    x + 10 + level
}

#[test]
fn it_works_for_base_stats_to_stats_hp() {
    // ハピナスのHP特化(Lv.50)
    let actual = base_stats_to_stats_hp(255, 31, 252, 50, 1.1);
    let expected = 362;
    assert_eq!(actual, expected);
}

// {(255×2+31+252/4)×50/100}+10+50=362

fn base_stats_to_stats(
    base_stats: i32,
    individual_value: i32,
    effort_value: i32,
    level: i32,
    natural: f32,
) -> i32 {
    let x = (base_stats * 2 + individual_value + (effort_value / 4)) * level / 100;
    ((x as f32 + 5.0) * natural).floor() as i32
}

#[test]
fn it_works_for_base_stats_to_stats() {
    // サンダースの素早さ特化(Lv.50)
    let actual = base_stats_to_stats(130, 31, 252, 50, 1.1);
    let expected = 200;
    assert_eq!(actual, expected);
}

pub struct EnvPokemonRepository {
    data: Vec<PokemonIndividual>,
    rustemon_client: rustemon::client::RustemonClient,
}

impl EnvPokemonRepository {
    pub fn new() -> Self {
        Self {
            // data:  get_initial_data(),
            data: vec![],
            rustemon_client: rustemon::client::RustemonClient::default(),
        }
    }
}

#[async_trait]
impl EnvPokemonRepositoryTrait for EnvPokemonRepository {
    fn get_all(&self) -> &Vec<PokemonIndividual> {
        &self.data
    }

    async fn load(&mut self) {
        for pokemon_no in 1..=1008 {
            debug!("{}", pokemon_no);
            let pokemon_result =
                rustemon::pokemon::pokemon::get_by_id(pokemon_no, &self.rustemon_client).await;
            let pokemon_species_result =
                rustemon::pokemon::pokemon_species::get_by_id(pokemon_no, &self.rustemon_client)
                    .await;

            let mut base_stats_hp: i32 = 0;
            let mut base_stats_atack: i32 = 0;
            let mut base_stats_defense: i32 = 0;
            let mut base_stats_special_attack: i32 = 0;
            let mut base_stats_special_defense: i32 = 0;
            let mut base_stats_speed: i32 = 0;

            let mut elms = vec![];

            let mut ja_name: String = "".to_string();
            match pokemon_species_result {
                Ok(pokemon_species) => {
                    for name_in_lang in pokemon_species.names.iter() {
                        if name_in_lang.language.name == "ja" {
                            ja_name = name_in_lang.name.to_string();
                        }
                    }
                }
                Err(_) => {}
            }

            match pokemon_result {
                Ok(pokemon) => {
                    for pokemon_stat in pokemon.stats.iter() {
                        match pokemon_stat.stat.name.as_str() {
                            "hp" => {
                                base_stats_hp = pokemon_stat.base_stat as i32;
                            }
                            "attack" => {
                                base_stats_atack = pokemon_stat.base_stat as i32;
                            }

                            "defense" => {
                                base_stats_defense = pokemon_stat.base_stat as i32;
                            }

                            "special-attack" => {
                                base_stats_special_attack = pokemon_stat.base_stat as i32;
                            }

                            "special-defense" => {
                                base_stats_special_defense = pokemon_stat.base_stat as i32;
                            }
                            "speed" => {
                                base_stats_speed = pokemon_stat.base_stat as i32;
                            }
                            _ => {}
                        }
                    }

                    for pokemon_types in pokemon.types.iter() {
                        let type_name: &str = &pokemon_types.type_.name;
                        match poke_api_type_name_to_basic_element(type_name) {
                            Some(elm) => {
                                elms.push(Mbe(elm));
                            }
                            None => {}
                        }
                    }

                    while elms.len() < 3 {
                        elms.push(Mnone);
                    }

                    let individual = PokemonIndividual::new(
                        PokemonSpecies::new_by_number_elements(
                            pokemon_no.try_into().unwrap(),
                            elms[0],
                            elms[1],
                            elms[2],
                        ),
                        base_stats_to_stats_hp(base_stats_hp, 31, 0, 50),
                        base_stats_to_stats(base_stats_atack, 31, 252, 50, 1.1),
                        base_stats_to_stats(base_stats_defense, 31, 0, 50, 1.0),
                        base_stats_to_stats(base_stats_special_attack, 31, 252, 50, 1.1),
                        base_stats_to_stats(base_stats_special_defense, 31, 0, 50, 1.0),
                        base_stats_to_stats(base_stats_speed, 31, 252, 50, 1.1),
                        vec![],
                        format!("{}_ACS", ja_name),
                    );
                    self.data.push(individual);

                    let individual_hac = PokemonIndividual::new(
                        PokemonSpecies::new_by_number_elements(
                            pokemon_no.try_into().unwrap(),
                            elms[0],
                            elms[1],
                            elms[2],
                        ),
                        base_stats_to_stats_hp(base_stats_hp, 31, 252, 50),
                        base_stats_to_stats(base_stats_atack, 31, 252, 50, 1.1),
                        base_stats_to_stats(base_stats_defense, 31, 0, 50, 1.0),
                        base_stats_to_stats(base_stats_special_attack, 31, 252, 50, 1.1),
                        base_stats_to_stats(base_stats_special_defense, 31, 0, 50, 1.0),
                        base_stats_to_stats(base_stats_speed, 31, 0, 50, 0.9),
                        vec![],
                        format!("{}_HAC", ja_name),
                    );
                    self.data.push(individual_hac);

                    let individual_hbd = PokemonIndividual::new(
                        PokemonSpecies::new_by_number_elements(
                            pokemon_no.try_into().unwrap(),
                            elms[0],
                            elms[1],
                            elms[2],
                        ),
                        base_stats_to_stats_hp(base_stats_hp, 31, 252, 50),
                        base_stats_to_stats(base_stats_atack, 31, 0, 50, 0.9),
                        base_stats_to_stats(base_stats_defense, 31, 252, 50, 1.1),
                        base_stats_to_stats(base_stats_special_attack, 31, 0, 50, 0.9),
                        base_stats_to_stats(base_stats_special_defense, 31, 252, 50, 1.1),
                        base_stats_to_stats(base_stats_speed, 31, 0, 50, 1.0),
                        vec![],
                        format!("{}_HBD", ja_name),
                    );
                    self.data.push(individual_hbd);
                }
                Err(_) => {}
            }
        }
    }
}

fn get_initial_data() -> Vec<PokemonIndividual> {
    return vec![
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(149, Mbe(Be::Dragon), Mbe(Be::Flying), Mnone),
            198,
            204,
            115,
            108,
            120,
            100,
            vec![Move::new(
                "しんそく",
                Be::Normal,
                MoveType::Physical,
                80,
                1.0,
                10,
            )],
            "カイリューHA".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(149, Mbe(Be::Normal), Mnone, Mnone),
            198,
            204,
            115,
            108,
            120,
            100,
            vec![Move::new(
                "しんそく",
                Be::Normal,
                MoveType::Physical,
                80,
                1.0,
                10,
            )],
            "カイリューHA(ノマテラ)".to_string(),
        ),
    ];
}

fn poke_api_type_name_to_basic_element(basic_element_str: &str) -> Option<Be> {
    return match basic_element_str {
        "normal" => Some(Be::Normal),
        "fire" => Some(Be::Fire),
        "water" => Some(Be::Water),
        "electric" => Some(Be::Electric),
        "grass" => Some(Be::Grass),
        "ice" => Some(Be::Ice),
        "fighting" => Some(Be::Fighting),
        "poison" => Some(Be::Poison),
        "ground" => Some(Be::Ground),
        "flying" => Some(Be::Flying),
        "psychic" => Some(Be::Psychic),
        "bug" => Some(Be::Bug),
        "rock" => Some(Be::Rock),
        "ghost" => Some(Be::Ghost),
        "dragon" => Some(Be::Dragon),
        "dark" => Some(Be::Dark),
        "steel" => Some(Be::Steel),
        "faily" => Some(Be::Fairy),
        _ => {
            warn!("Unknwon basic_element: {}", basic_element_str);
            None
        }
    };
}
