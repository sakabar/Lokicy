use super::super::super::application::interface::BoxPokemonRepositoryTrait;
use super::super::super::entity::pokemon::base_stats_to_stats;
use super::super::super::entity::pokemon::base_stats_to_stats_hp;
use super::super::super::entity::pokemon::AbilityElement as Ae;
use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement::Mae;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use super::super::super::entity::pokemon::MetaElement::Mnone;
use super::super::super::entity::pokemon::Move;
use super::super::super::entity::pokemon::MoveType;
use super::super::super::entity::pokemon::PokemonIndividual;
use super::super::super::entity::pokemon::PokemonSpecies;
use super::rustemon::read_base_stats;
use super::rustemon::read_pokemon_types;
use super::rustemon::rustemon_type_name_to_basic_element;
use async_trait::async_trait;
use log::{error, info};

pub struct BoxPokemonRepository {
    data: Vec<PokemonIndividual>,
    rustemon_client: rustemon::client::RustemonClient,
}

impl BoxPokemonRepository {
    pub fn new() -> Self {
        Self {
            // data: vec![],
            data: get_initial_data(),
            rustemon_client: rustemon::client::RustemonClient::default(),
        }
    }
}

#[async_trait]
impl BoxPokemonRepositoryTrait for BoxPokemonRepository {
    fn get_all(&self) -> &Vec<PokemonIndividual> {
        &self.data
    }

    async fn load(&mut self) {
        let inputs = vec![
            // (
            //     637,
            //     Be::Bug,
            //     None,
            //     (252, 0, 0, 252, 0, 0),
            //     (0.9, 1.0, 1.1, 1.0, 1.0),
            //     vec!["overheat", "bug-buzz"],
            //     "ウルガモス",
            // ),
            (
                130,
                Be::Water,
                None,
                (252, 252, 4, 0, 0, 0),
                (1.1, 1.0, 0.9, 1.0, 1.0),
                vec!["aqua-tail", "earthquake", "avalanche", "fire-blast"],
                "ギャラドス",
            ),
            // (
            //     713,
            //     Be::Ice,
            //     None,
            //     (252, 252, 4, 0, 0, 0),
            //     (1.1, 1.0, 0.9, 1.0, 1.0),
            //     vec!["icicle-crash"],
            //     "クレベース",
            // ),
            // (
            //     936,
            //     Be::Fire,
            //     Some(Ae::FlashFire),
            //     (244, 252, 12, 0, 0, 0),
            //     (0.9, 1.0, 1.1, 1.0, 1.0),
            //     vec!["overheat", "psychic"],
            //     "グレンアルマ",
            // ),
            // PokemonIndividual::new(
            //     PokemonSpecies::new_by_number_elements(128, Mbe(Be::Fighting), Mbe(Be::Fire), Mnone),
            //     Be::Fighting,
            //     false,
            //     182,
            //     178,
            //     126,
            //     45,
            //     90,
            //     120,
            //     vec![
            //         Move::new(
            //             "インファイト",
            //             Be::Fighting,
            //             MoveType::Physical,
            //             120,
            //             1.0,
            //             10,
            //         ),
            //         Move::new("フレアドライブ", Be::Fire, MoveType::Physical, 120, 1.0, 10),
            //     ],
            //     "ケンタロス(炎)".to_string(),
            // ),
            // PokemonIndividual::new(
            //     PokemonSpecies::new_by_number_elements(128, Mbe(Be::Fighting), Mbe(Be::Water), Mnone),
            //     Be::Fighting,
            //     false,
            //     182,
            //     178,
            //     126,
            //     45,
            //     90,
            //     120,
            //     vec![
            //         Move::new(
            //             "インファイト",
            //             Be::Fighting,
            //             MoveType::Physical,
            //             120,
            //             1.0,
            //             10,
            //         ),
            //         Move::new(
            //             "ウェーブタックル",
            //             Be::Water,
            //             MoveType::Physical,
            //             120,
            //             1.0,
            //             10,
            //         ),
            //     ],
            //     "ケンタロス(水)".to_string(),
            // ),
            // (
            //     1000,
            //     Be::Ghost,
            //     None,
            //     (252, 0, 4, 252, 0, 0),
            //     (0.9, 1.0, 1.1, 1.0, 1.0),
            //     vec!["make-it-rain", "shadow-ball", "focus-blast"],
            //     "サーフゴー",
            // ),
            // (
            //     635,
            //     Be::Steel,
            //     Some(Ae::Levitate),
            //     (252, 0, 0, 252, 4, 0),
            //     (0.9, 1.0, 1.1, 1.0, 1.0),
            //     vec!["draco-meteor", "dark-pulse", "fire-blast", "flash-cannon"],
            //     "サザンドラ",
            // ),
            (
                462,
                Be::Rock,
                None,
                (252, 0, 0, 252, 4, 0),
                (0.9, 1.0, 1.1, 1.0, 1.0),
                vec!["thunder", "flash-cannon", "volt-switch", "tera-blast"],
                "ジバコイル",
            ),
            // (
            //     959,
            //     Be::Steel,
            //     None,
            //     (244, 252, 12, 0, 0, 0),
            //     (1.1, 1.0, 0.9, 1.0, 1.0),
            //     vec!["gigaton-hammer", "play-rough", "bulldoze"],
            //     "デカヌチャン",
            // ),
            // Todo: ドータクン
            // ドオー
            // (
            //     980,
            //     Be::Poison,
            //     Some(Ae::WaterAbsorb),
            //     (108, 252, 148, 0, 0, 0),
            //     (1.1, 1.0, 0.9, 1.0, 1.0),
            //     vec!["gunk-shot", "earthquake", "stone-edge", "megahorn"],
            //     "ドオー",
            // ),
            // Todo: ドドゲザン
            // Todo: トリトドン
            // Todo: ハリテヤマ
            // Todo: バンギラス
            // Todo: ヘイラッシャ
            (
                373,
                Be::Flying,
                None,
                (252, 0, 4, 252, 0, 0),
                (0.9, 1.0, 1.1, 1.0, 1.0),
                vec!["draco-meteor", "hurricane", "fire-blast", "hydro-pump"],
                "ボーマンダ",
            ),
            // Todo: マリルリ
            // (
            //     373,
            //     Be::Flying,
            //     None,
            //     (252, 0, 0, 252, 4, 0),
            //     (0.9, 1.0, 1.1, 1.0, 1.0),
            //     vec!["leaf-storm", "sludge-bomb", "tera-blast", "foul-play"],
            //     "モロバレル",
            // ),
            (
                460,
                Be::Grass,
                None,
                (252, 0, 4, 252, 0, 0),
                (0.9, 1.0, 1.1, 1.0, 1.0),
                vec!["leaf-storm", "blizzard", "earth-power", "focus-blast"],
                "ユキノオー",
            ),
            (
                984,
                Be::Fighting,
                None,
                (252, 252, 4, 0, 0, 0),
                (1.1, 1.0, 0.9, 1.0, 1.0),
                vec!["close-combat", "headlong-rush", "stone-edge", "knock-off"],
                "イダイナキバ",
            ),
            // (
            //     988,
            //     Be::Bug,
            //     None,
            //     (252, 252, 4, 0, 0, 0),
            //     (1.1, 1.0, 0.9, 1.0, 1.0),
            //     vec!["close-combat", "first-impression","u-turn","dual-wingbeat"],
            //     "チヲハウハネ",
            // ),
        ];

        for input in inputs.iter() {
            let pokemon_no = input.0;
            let tera_type = input.1;
            let ability_element_opt = input.2;
            let (ev_h, ev_a, ev_b, ev_c, ev_d, ev_s) = input.3;
            let (natural_a, natural_b, natural_c, natural_d, natural_s) = input.4;
            let move_names = &input.5;
            let name_ja = input.6;

            let pokemon_result =
                rustemon::pokemon::pokemon::get_by_id(pokemon_no, &self.rustemon_client).await;

            let base_stats_hp: i32;
            let base_stats_attack: i32;
            let base_stats_defense: i32;
            let base_stats_special_attack: i32;
            let base_stats_special_defense: i32;
            let base_stats_speed: i32;

            let mut elms = vec![];

            match pokemon_result {
                Ok(pokemon) => {
                    (
                        base_stats_hp,
                        base_stats_attack,
                        base_stats_defense,
                        base_stats_special_attack,
                        base_stats_special_defense,
                        base_stats_speed,
                    ) = read_base_stats(&pokemon.stats);

                    let mut tmp_elms = read_pokemon_types(&pokemon.types);
                    elms.append(&mut tmp_elms);

                    match ability_element_opt {
                        Some(ae) => {
                            elms.push(Mae(ae));
                        }
                        None => {}
                    }

                    while elms.len() < 3 {
                        elms.push(Mnone);
                    }
                }
                Err(_) => {
                    panic!("{}", pokemon_no);
                }
            }

            let mut moves = vec![];
            for move_name in move_names.iter() {
                info!("{}", move_name);

                let move_result =
                    rustemon::moves::move_::get_by_name(move_name, &self.rustemon_client).await;
                match move_result {
                    Ok(rustemon_move) => {
                        // TODO: エラーハンドリング
                        let poke_type: Be =
                            rustemon_type_name_to_basic_element(rustemon_move.type_.name.as_str())
                                .unwrap();

                        let move_type = match rustemon_move.damage_class.name.as_str() {
                            "physical" => MoveType::Physical,
                            "special" => MoveType::Special,
                            "status" => MoveType::Status,
                            _ => {
                                panic!("Not found: {}", rustemon_move.damage_class.name)
                            }
                        };

                        let pp: u8 = rustemon_move.pp.unwrap_or(0) as u8;
                        let power: i32 = rustemon_move.power.unwrap_or(0) as i32;

                        // TODO: accuracyは整数で持つべき?
                        // TODO: 必中(ドゲザンとか)の扱いはどうなっている?
                        let accuracy: f64 = rustemon_move.accuracy.unwrap_or(0) as f64 / 100.0;

                        let mv = Move::new(move_name, poke_type, move_type, power, accuracy, pp);
                        moves.push(mv);
                    }
                    Err(_) => {
                        error!("Not found: {}", move_name);
                    }
                }
            }

            let is_terastallized = false;
            let individual = PokemonIndividual::new(
                PokemonSpecies::new_by_number_elements(
                    pokemon_no.try_into().unwrap(),
                    elms[0].clone(),
                    elms[1].clone(),
                    elms[2].clone(),
                ),
                tera_type,
                is_terastallized,
                base_stats_to_stats_hp(base_stats_hp, 31, ev_h, 50),
                base_stats_to_stats(base_stats_attack, 31, ev_a, 50, natural_a),
                base_stats_to_stats(base_stats_defense, 31, ev_b, 50, natural_b),
                base_stats_to_stats(base_stats_special_attack, 31, ev_c, 50, natural_c),
                base_stats_to_stats(base_stats_special_defense, 31, ev_d, 50, natural_d),
                base_stats_to_stats(base_stats_speed, 31, ev_s, 50, natural_s),
                moves,
                name_ja.to_string(),
            );

            self.data.push(individual);
        }
    }
}

fn get_initial_data() -> Vec<PokemonIndividual> {
    return vec![
        // PokemonIndividual::new(
        //     PokemonSpecies::new_by_number_elements(637, Mbe(Be::Bug), Mbe(Be::Fire), Mnone),
        //     Be::Bug,
        //     false,
        //     191,
        //     72,
        //     85,
        //     205,
        //     127,
        //     120,
        //     vec![
        //         Move::new("オーバーヒート", Be::Fire, MoveType::Special, 130, 1.0, 10),
        //         Move::new("むしのさざめき", Be::Bug, MoveType::Special, 90, 1.0, 10),
        //     ],
        //     "ウルガモス".to_string(),
        // ),
        // PokemonIndividual::new(
        //     PokemonSpecies::new_by_number_elements(130, Mbe(Be::Water), Mbe(Be::Flying), Mnone),
        //     Be::Water,
        //     false,
        //     202,
        //     194,
        //     99,
        //     72,
        //     120,
        //     101,
        //     vec![
        //         Move::new("アクアテール", Be::Water, MoveType::Physical, 90, 0.95, 10),
        //         Move::new("じしん", Be::Ground, MoveType::Physical, 100, 1.0, 10),
        //         Move::new("ゆきなだれ", Be::Ice, MoveType::Physical, 60, 1.0, 10),
        //         Move::new("だいもんじ", Be::Fire, MoveType::Special, 110, 1.0, 10),
        //     ],
        //     "ギャラドス".to_string(),
        // ),
        // PokemonIndividual::new(
        //     PokemonSpecies::new_by_number_elements(713, Mbe(Be::Ice), Mnone, Mnone),
        //     Be::Ice,
        //     false,
        //     202,
        //     185,
        //     205,
        //     57,
        //     66,
        //     48,
        //     vec![Move::new(
        //         "つららおとし",
        //         Be::Ice,
        //         MoveType::Physical,
        //         85,
        //         1.0,
        //         10,
        //     )],
        //     "クレベース".to_string(),
        // ),
        // PokemonIndividual::new(
        //     PokemonSpecies::new_by_number_elements(
        //         936,
        //         Mbe(Be::Fire),
        //         Mbe(Be::Psychic),
        //         Mae(Ae::FlashFire),
        //     ),
        //     Be::Fire,
        //     false,
        //     191,
        //     72,
        //     122,
        //     194,
        //     100,
        //     95,
        //     vec![Move::new(
        //         "オーバーヒート",
        //         Be::Fire,
        //         MoveType::Special,
        //         135,
        //         1.0,
        //         10,
        //     )],
        //     "グレンアルマ".to_string(),
        // ),
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(128, Mbe(Be::Fighting), Mbe(Be::Fire), Mnone),
            Be::Fighting,
            false,
            182,
            178,
            126,
            45,
            90,
            120,
            vec![
                Move::new(
                    "インファイト",
                    Be::Fighting,
                    MoveType::Physical,
                    120,
                    1.0,
                    10,
                ),
                Move::new("フレアドライブ", Be::Fire, MoveType::Physical, 120, 1.0, 10),
            ],
            "ケンタロス(炎)".to_string(),
        ),
        // PokemonIndividual::new(
        //     PokemonSpecies::new_by_number_elements(128, Mbe(Be::Fighting), Mbe(Be::Water), Mnone),
        //     Be::Fighting,
        //     false,
        //     182,
        //     178,
        //     126,
        //     45,
        //     90,
        //     120,
        //     vec![
        //         Move::new(
        //             "インファイト",
        //             Be::Fighting,
        //             MoveType::Physical,
        //             120,
        //             1.0,
        //             10,
        //         ),
        //         Move::new(
        //             "ウェーブタックル",
        //             Be::Water,
        //             MoveType::Physical,
        //             120,
        //             1.0,
        //             10,
        //         ),
        //     ],
        //     "ケンタロス(水)".to_string(),
        // ),
        // PokemonIndividual::new(
        //     PokemonSpecies::new_by_number_elements(1000, Mbe(Be::Steel), Mbe(Be::Ghost), Mnone),
        //     Be::Ghost,
        //     false,
        //     194,
        //     58,
        //     116,
        //     203,
        //     111,
        //     104,
        //     vec![
        //         Move::new(
        //             "ゴールドラッシュ",
        //             Be::Steel,
        //             MoveType::Special,
        //             120,
        //             1.0,
        //             10,
        //         ),
        //         Move::new("シャドーボール", Be::Steel, MoveType::Special, 80, 1.0, 10),
        //     ],
        //     "サーフゴー".to_string(),
        // ),
        // PokemonIndividual::new(
        //     PokemonSpecies::new_by_number_elements(
        //         635,
        //         Mbe(Be::Dark),
        //         Mbe(Be::Dragon),
        //         Mae(Ae::Levitate),
        //     ),
        //     Be::Steel,
        //     false,
        //     199,
        //     112,
        //     111,
        //     194,
        //     110,
        //     118,
        //     vec![
        //         Move::new(
        //             "りゅうせいぐん",
        //             Be::Dragon,
        //             MoveType::Special,
        //             130,
        //             1.0,
        //             10,
        //         ),
        //         Move::new("あくのはどう", Be::Dark, MoveType::Special, 80, 1.0, 10),
        //         Move::new("だいもんじ", Be::Fire, MoveType::Special, 110, 1.0, 10),
        //         Move::new("ラスターカノン", Be::Steel, MoveType::Special, 80, 1.0, 10),
        //     ],
        //     "サザンドラ".to_string(),
        // ),
        // 次、ヤバコイルから
    ];
}
