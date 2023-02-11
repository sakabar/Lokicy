use itertools::Itertools;
use lokicy::entity::pokemon;
use lokicy::entity::pokemon::MetaElement::Mbe;
use lokicy::entity::pokemon::MetaElement::Mae;
use lokicy::entity::pokemon::Move;
use lokicy::entity::pokemon::MoveType;
use lokicy::entity::pokemon::PokemonClass;
use lokicy::entity::pokemon::PokemonInstance;
use lokicy::entity::pokemon::BasicElement as Be;
use lokicy::entity::pokemon::AbilityElement as Ae;

fn main() {
    let yakemons: Vec<PokemonInstance> = vec![
        PokemonInstance::new(
            PokemonClass::new(637, Mbe(Be::Bug), Mbe(Be::Fire), Mae(Ae::None)),
            191,
            72,
            85,
            205,
            127,
            120,
            "ウルガモス".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(130, Mbe(Be::Water), Mbe(Be::Flying), Mae(Ae::None)),
            202,
            194,
            99,
            72,
            120,
            101,
            "ギャラドス".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(713, Mbe(Be::Ice), Mae(Ae::None), Mae(Ae::None)),
            202,
            185,
            205,
            57,
            66,
            48,
            "クレベース".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(936, Mbe(Be::Fire), Mbe(Be::Psychic), Mae(Ae::FlashFire)),
            191,
            72,
            122,
            194,
            100,
            95,
            "グレンアルマ".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(128, Mbe(Be::Fighting), Mbe(Be::Fire), Mae(Ae::None)),
            182,
            178,
            126,
            45,
            90,
            120,
            "ケンタロス(炎)".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(128, Mbe(Be::Fighting), Mbe(Be::Water), Mae(Ae::None)),
            182,
            178,
            126,
            45,
            90,
            120,
            "ケンタロス(水)".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(1000, Mbe(Be::Steel), Mbe(Be::Ghost), Mae(Ae::None)),
            194,
            58,
            116,
            203,
            111,
            104,
            "サーフゴー".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(635, Mbe(Be::Dark), Mbe(Be::Dragon), Mae(Ae::Levitate)),
            199,
            112,
            111,
            194,
            110,
            118,
            "サザンドラ".to_string(),
        ),
        // 次、ヤバコイルから
    ];

    let bokemons: Vec<PokemonInstance> = vec![
        PokemonInstance::new(
            PokemonClass::new(149, Mbe(Be::Dragon), Mbe(Be::Flying), Mae(Ae::None)),
            198,
            204,
            115,
            108,
            120,
            100,
            "カイリューHA".to_string(),
        ),
        PokemonInstance::new(
            PokemonClass::new(149, Mbe(Be::Normal), Mae(Ae::None), Mae(Ae::None)),
            198,
            204,
            115,
            108,
            120,
            100,
            "カイリューHA(ノマテラ)".to_string(),
        ),
    ];

    // TODO
    // (1) Type -> Elm
    // (2) 技をPokemonInstanceの中に入れてforで回すようにする
    // (3) interface層がロジックを知りすぎているので隠蔽する
    // (4) テラスタイプを考慮に入れる。インスタンスを手軽に生成するには?
    for bokemon in bokemons.iter() {
        let mut answers = vec![];
        for yakemon in yakemons.iter() {
            // 今は仮に、全員ゆきなだれのみとする
            // let aqua_tail = Move::new("アクアテール", Be::Water, MoveType::Physical, 10, 90, 0.95);
            let mv = Move::new("ゆきなだれ", Be::Ice, MoveType::Physical, 10, 60, 1.0);

            let (mt, offensive_index) = yakemon.get_offensive_index(&mv);
            let defensive_index = bokemon.get_defensive_index(mt);
            let r = bokemon.calc_type_combination_matchup_rate(&mv.get_poke_type());

            // 乱数最悪の場合で考える
            let damage_index: f64 = (0.44 * offensive_index / defensive_index) * r * 0.85;
            let damage_index_permille: i32 = (damage_index * 1000.0).floor() as i32;

            // println!("{}", yakemon.get_comment());
            // println!("{}", offensive_index);
            // println!("{}", defensive_index);
            // println!("");

            answers.push((damage_index_permille, yakemon.get_comment()));
        }

        answers.sort();
        answers.reverse();
        println!("{}", bokemon.get_comment());
        println!("{:?}", answers);
        println!("");
    }
}
