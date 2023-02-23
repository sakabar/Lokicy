use lokicy::adapter::gateway::box_pokemon_repository::BoxPokemonRepository;
use lokicy::adapter::gateway::env_pokemon_repository::EnvPokemonRepository;
use lokicy::application::calc_damage_to_env_pokes::calc_damage_to_env_pokes;

fn main() {
    let box_pokemon_repository = BoxPokemonRepository::new();
    let env_pokemon_repository = EnvPokemonRepository::new();
    calc_damage_to_env_pokes(&box_pokemon_repository, &env_pokemon_repository);
}
