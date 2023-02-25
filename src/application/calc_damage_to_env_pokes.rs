use super::super::entity::pokemon::PokemonIndividual;
use super::interface::BoxPokemonRepositoryTrait;
use super::interface::EnvPokemonRepositoryTrait;

pub async fn calc_damage_to_env_pokes(
    box_pokemon_repository: &mut dyn BoxPokemonRepositoryTrait,
    env_pokemon_repository: &mut dyn EnvPokemonRepositoryTrait,
) {
    box_pokemon_repository.load().await;
    let box_pokemons: &Vec<PokemonIndividual> = box_pokemon_repository.get_all();

    env_pokemon_repository.load().await;
    let env_pokemons: &Vec<PokemonIndividual> = env_pokemon_repository.get_all();

    let mut answers: Vec<(i32, String, Vec<(i32, &str, &str)>)> = vec![];

    for env_pokemon in env_pokemons.iter() {
        let mut box_pokemon_damage_indexes = vec![];
        for box_pokemon in box_pokemons.iter() {
            let mut max_damage_index_permille: i32 = 0;
            let mut max_damage_move_name = "";

            for mv in box_pokemon.get_moves().iter() {
                let (mt, offensive_index) = box_pokemon.get_offensive_index(&mv);
                let defensive_index = env_pokemon.get_defensive_index(mt);
                let r = env_pokemon.calc_type_combination_matchup_rate(&mv.get_poke_type());

                // 乱数最悪の場合で考える
                let damage_index: f64 = (0.44 * offensive_index / defensive_index) * r * 0.85;
                let damage_index_permille: i32 = (damage_index * 1000.0).floor() as i32;

                if damage_index_permille > max_damage_index_permille {
                    max_damage_index_permille = damage_index_permille;
                    max_damage_move_name = mv.get_name();
                }
            }

            box_pokemon_damage_indexes.push((
                max_damage_index_permille,
                box_pokemon.get_comment(),
                max_damage_move_name,
            ));
        }

        box_pokemon_damage_indexes.sort();
        box_pokemon_damage_indexes.reverse();

        let max_damage_index_permille = box_pokemon_damage_indexes[0].0;
        let pair = (
            max_damage_index_permille,
            format!("{}", env_pokemon.get_comment()),
            box_pokemon_damage_indexes,
        );
        answers.push(pair);
    }

    answers.sort();
    for (damage, name, boxs) in answers.iter() {
        println!("{}\t{}\t{:?}", damage, name, boxs[0]);
    }
}
