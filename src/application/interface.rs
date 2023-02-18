use super::super::entity::pokemon::PokemonInstance;

pub trait BoxPokemonRepositoryTrait {
    fn get_all(&self) -> &Vec<PokemonInstance>;
}

pub trait EnvPokemonRepositoryTrait {
    fn get_all(&self) -> &Vec<PokemonInstance>;
}
