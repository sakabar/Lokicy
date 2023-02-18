use super::super::super::application::interface::EnvPokemonRepositoryTrait;
use super::super::super::entity::pokemon::PokemonClass;

use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use super::super::super::entity::pokemon::MetaElement::Mnone;

use super::super::super::entity::pokemon::PokemonInstance;

pub struct EnvPokemonRepository {
    data: Vec<PokemonInstance>,
}

impl EnvPokemonRepository {
    pub fn new() -> Self {
        Self {
            data: get_initial_data(),
        }
    }
}

impl EnvPokemonRepositoryTrait for EnvPokemonRepository {
    fn get_all(&self) -> &Vec<PokemonInstance> {
        &self.data
    }
}

fn get_initial_data() -> Vec<PokemonInstance> {
    return vec![
        PokemonInstance::new(
            PokemonClass::new(149, Mbe(Be::Dragon), Mbe(Be::Flying), Mnone),
            198,
            204,
            115,
            108,
            120,
            100,
            "カイリューHA".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(149, Mbe(Be::Normal), Mnone, Mnone),
            198,
            204,
            115,
            108,
            120,
            100,
            "カイリューHA(ノマテラ)".to_string(),
        ),
    ];
}
