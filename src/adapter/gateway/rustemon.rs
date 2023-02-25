use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement as Me;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use log::warn;

pub fn rustemon_type_name_to_basic_element(basic_element_str: &str) -> Option<Be> {
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
        "fairy" => Some(Be::Fairy),
        _ => {
            warn!("Unknwon basic_element: {}", basic_element_str);
            None
        }
    };
}

pub fn read_base_stats(
    pokemon_stats: &Vec<rustemon::model::pokemon::PokemonStat>,
) -> (i32, i32, i32, i32, i32, i32) {
    let mut base_stats_hp: i32 = 0;
    let mut base_stats_attack: i32 = 0;
    let mut base_stats_defense: i32 = 0;
    let mut base_stats_special_attack: i32 = 0;
    let mut base_stats_special_defense: i32 = 0;
    let mut base_stats_speed: i32 = 0;

    for pokemon_stat in pokemon_stats.iter() {
        match pokemon_stat.stat.name.as_str() {
            "hp" => {
                base_stats_hp = pokemon_stat.base_stat as i32;
            }
            "attack" => {
                base_stats_attack = pokemon_stat.base_stat as i32;
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

    return (
        base_stats_hp,
        base_stats_attack,
        base_stats_defense,
        base_stats_special_attack,
        base_stats_special_defense,
        base_stats_speed,
    );
}

pub fn read_pokemon_types(pokemon_types: &Vec<rustemon::model::pokemon::PokemonType>) -> Vec<Me> {
    let mut elms = vec![];

    for pokemon_type in pokemon_types.iter() {
        let type_name: &str = &pokemon_type.type_.name;

        match rustemon_type_name_to_basic_element(type_name) {
            Some(elm) => {
                elms.push(Mbe(elm));
            }
            None => {}
        }
    }
    return elms;
}
