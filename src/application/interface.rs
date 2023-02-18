use super::super::entity::pokemon::PokemonIndividual;

pub trait BoxPokemonRepositoryTrait {
    fn get_all(&self) -> &Vec<PokemonIndividual>;
}

pub trait EnvPokemonRepositoryTrait {
    fn get_all(&self) -> &Vec<PokemonIndividual>;
}
