use lokicy::adapter::gateway::box_pokemon_repository::BoxPokemonRepository;
use lokicy::adapter::gateway::env_pokemon_repository::EnvPokemonRepository;
use lokicy::application::calc_damage_to_env_pokes::calc_damage_to_env_pokes;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut box_pokemon_repository = BoxPokemonRepository::new();
    let mut env_pokemon_repository = EnvPokemonRepository::new();
    calc_damage_to_env_pokes(&mut box_pokemon_repository, &mut env_pokemon_repository).await;
}
