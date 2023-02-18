use super::super::entity::pokemon::BasicElement as Be;
use super::super::entity::pokemon::Move;
use super::super::entity::pokemon::MoveType;
use super::super::entity::pokemon::PokemonInstance;
use super::interface::BoxPokemonRepositoryTrait;
use super::interface::EnvPokemonRepositoryTrait;

pub fn calc_damage_to_env_pokes(
    box_pokemon_repository: &dyn BoxPokemonRepositoryTrait,
    env_pokemon_repository: &dyn EnvPokemonRepositoryTrait,
) {
    let box_pokemons: &Vec<PokemonInstance> = box_pokemon_repository.get_all();
    let env_pokemons: &Vec<PokemonInstance> = env_pokemon_repository.get_all();

    // TODO
    // (1) Type -> Elm
    // (2) 技をPokemonInstanceの中に入れてforで回すようにする
    // (3) interface層がロジックを知りすぎているので隠蔽する
    // (4) テラスタイプを考慮に入れる。インスタンスを手軽に生成するには?
    for env_pokemon in env_pokemons.iter() {
        let mut answers = vec![];
        for box_pokemon in box_pokemons.iter() {
            // 今は仮に、全員ゆきなだれのみとする
            // let aqua_tail = Move::new("アクアテール", Be::Water, MoveType::Physical, 10, 90, 0.95);
            let mv = Move::new("ゆきなだれ", Be::Ice, MoveType::Physical, 10, 60, 1.0);

            let (mt, offensive_index) = box_pokemon.get_offensive_index(&mv);
            let defensive_index = env_pokemon.get_defensive_index(mt);
            let r = env_pokemon.calc_type_combination_matchup_rate(&mv.get_poke_type());

            // 乱数最悪の場合で考える
            let damage_index: f64 = (0.44 * offensive_index / defensive_index) * r * 0.85;
            let damage_index_permille: i32 = (damage_index * 1000.0).floor() as i32;

            // println!("{}", box_pokemon.get_comment());
            // println!("{}", offensive_index);
            // println!("{}", defensive_index);
            // println!("");

            answers.push((damage_index_permille, box_pokemon.get_comment()));
        }

        answers.sort();
        answers.reverse();
        println!("{}", env_pokemon.get_comment());
        println!("{:?}", answers);
        println!("");
    }
}
