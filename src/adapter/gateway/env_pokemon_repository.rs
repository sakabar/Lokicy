use super::super::super::application::interface::EnvPokemonRepositoryTrait;
use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use super::super::super::entity::pokemon::MetaElement::Mnone;
use super::super::super::entity::pokemon::PokemonIndividual;
use super::super::super::entity::pokemon::PokemonSpecies;

pub struct EnvPokemonRepository {
    data: Vec<PokemonIndividual>,
}

impl EnvPokemonRepository {
    pub fn new() -> Self {
        Self {
            data: get_initial_data(),
        }
    }
}

impl EnvPokemonRepositoryTrait for EnvPokemonRepository {
    fn get_all(&self) -> &Vec<PokemonIndividual> {
        &self.data
    }
}

fn get_initial_data() -> Vec<PokemonIndividual> {
    return vec![
        PokemonIndividual::new(
            PokemonSpecies::new(149, Mbe(Be::Dragon), Mbe(Be::Flying), Mnone),
            198,
            204,
            115,
            108,
            120,
            100,
            "カイリューHA".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(149, Mbe(Be::Normal), Mnone, Mnone),
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
