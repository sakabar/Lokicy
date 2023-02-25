use super::super::entity::pokemon::PokemonIndividual;
use async_trait::async_trait;

#[async_trait]
pub trait BoxPokemonRepositoryTrait {
    fn get_all(&self) -> &Vec<PokemonIndividual>;
    async fn load(&mut self);
}

#[async_trait]
pub trait EnvPokemonRepositoryTrait {
    fn get_all(&self) -> &Vec<PokemonIndividual>;
    async fn load(&mut self);
}
