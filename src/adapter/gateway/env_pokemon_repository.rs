use super::super::super::application::interface::EnvPokemonRepositoryTrait;
use super::super::super::entity::pokemon::base_stats_to_stats;
use super::super::super::entity::pokemon::base_stats_to_stats_hp;
use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement as Me;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use super::super::super::entity::pokemon::MetaElement::Mnone;
use super::super::super::entity::pokemon::Move;
use super::super::super::entity::pokemon::MoveType;
use super::super::super::entity::pokemon::PokemonIndividual;
use super::super::super::entity::pokemon::PokemonSpecies;
use super::rustemon::read_base_stats;
use super::rustemon::read_pokemon_types;
use async_trait::async_trait;
use log::debug;

pub struct EnvPokemonRepository {
    data: Vec<PokemonIndividual>,
    rustemon_client: rustemon::client::RustemonClient,
}

impl EnvPokemonRepository {
    pub fn new() -> Self {
        Self {
            data: get_initial_data(),
            // data: vec![],
            rustemon_client: rustemon::client::RustemonClient::default(),
        }
    }
}

#[async_trait]
impl EnvPokemonRepositoryTrait for EnvPokemonRepository {
    fn get_all(&self) -> &Vec<PokemonIndividual> {
        &self.data
    }

    async fn load(&mut self) {
        let env_pokemons: Vec<(i64, Vec<(Be, f64)>)> = vec![
            // ハバタクカミ
            (
                987,
                vec![
                    (Be::Fairy, 0.472),
                    (Be::Normal, 0.184),
                    (Be::Fire, 0.157),
                    (Be::Ghost, 0.116),
                ],
            ),
            // カイリュー
            (149, vec![(Be::Normal, 0.566), (Be::Flying, 0.298)]),
            // サーフゴー
            (
                1000,
                vec![
                    (Be::Flying, 0.418),
                    (Be::Normal, 0.200),
                    (Be::Ghost, 0.131),
                    (Be::Fighting, 0.109),
                ],
            ),
            // セグレイブ
            (
                998,
                vec![(Be::Ground, 0.595), (Be::Electric, 0.177), (Be::Ice, 0.083)],
            ),
            // テツノドクガ
            (
                994,
                vec![
                    (Be::Grass, 0.450),
                    (Be::Fire, 0.230),
                    (Be::Water, 0.101),
                    (Be::Poison, 0.075),
                    (Be::Fairy, 0.067),
                ],
            ),
            // ドドゲザン
            (
                983,
                vec![(Be::Dark, 0.621), (Be::Flying, 0.161), (Be::Fairy, 0.106)],
            ),
            // テツノツツミ
            (
                991,
                vec![
                    (Be::Water, 0.401),
                    (Be::Ice, 0.310),
                    (Be::Ghost, 0.081),
                    (Be::Ground, 0.077),
                ],
            ),
            // ミミッキュ
            (
                778,
                vec![
                    (Be::Ghost, 0.373),
                    (Be::Fairy, 0.262),
                    (Be::Fighting, 0.169),
                    (Be::Normal, 0.080),
                ],
            ),
            // ヘイラッシャ
            (
                977,
                vec![
                    (Be::Grass, 0.298),
                    (Be::Fairy, 0.207),
                    (Be::Ground, 0.148),
                    (Be::Water, 0.097),
                    (Be::Steel, 0.052),
                ],
            ),
            // 水ロトム
            (
                0479,
                vec![
                    (Be::Fairy, 0.469),
                    (Be::Steel, 0.203),
                    (Be::Electric, 0.140),
                    (Be::Ghost, 0.075),
                ],
            ),
            // テツノブジン
            (
                1006,
                vec![
                    (Be::Ghost, 0.313),
                    (Be::Fairy, 0.210),
                    (Be::Steel, 0.177),
                    (Be::Fighting, 0.156),
                ],
            ),
            // ハッサム
            (212, vec![(Be::Steel, 0.631), (Be::Water, 0.166)]),
            // トドロクツキ
            (
                1005,
                vec![
                    (Be::Flying, 0.513),
                    (Be::Steel, 0.216),
                    (Be::Fire, 0.076),
                    (Be::Poison, 0.057),
                ],
            ),
            // ラウドボーン
            (
                911,
                vec![
                    (Be::Fairy, 0.356),
                    (Be::Normal, 0.260),
                    (Be::Fire, 0.178),
                    (Be::Water, 0.096),
                    (Be::Grass, 0.045),
                ],
            ),
            // キョジオーン
            (
                934,
                vec![
                    (Be::Ghost, 0.482),
                    (Be::Water, 0.173),
                    (Be::Flying, 0.149),
                    (Be::Poison, 0.069),
                    (Be::Fairy, 0.056),
                ],
            ),
            // ガブリアス
            (
                445,
                vec![
                    (Be::Steel, 0.297),
                    (Be::Fire, 0.213),
                    (Be::Ground, 0.174),
                    (Be::Fairy, 0.172),
                    (Be::Water, 0.047),
                ],
            ),
            // ドラパルト
            (
                887,
                vec![
                    (Be::Dragon, 0.221),
                    (Be::Ghost, 0.216),
                    (Be::Fire, 0.215),
                    (Be::Steel, 0.167),
                    (Be::Fairy, 0.062),
                ],
            ),
            // マスカーニャ
            (
                908,
                vec![(Be::Grass, 0.679), (Be::Dark, 0.130), (Be::Fire, 0.042)],
            ),
            // コノヨザル
            (
                979,
                vec![
                    (Be::Fire, 0.244),
                    (Be::Normal, 0.190),
                    (Be::Poison, 0.179),
                    (Be::Steel, 0.170),
                    (Be::Fighting, 0.059),
                ],
            ),
            // テツノカイナ
            (
                992,
                vec![
                    (Be::Ground, 0.217),
                    (Be::Flying, 0.205),
                    (Be::Electric, 0.205),
                    (Be::Fighting, 0.170),
                    (Be::Fairy, 0.068),
                ],
            ),
            // アーマーガア
            // (, vec![
            //     (Be::,0.),
            //     (Be::,0.),
            //     (Be::,0.),
            //     (Be::,0.),
            // ]),

            // 何位? ソウブレイズ
            (
                937,
                vec![
                    (Be::Bug, 1.0),
                    // (Be::,0.),
                    // (Be::,0.),
                    // (Be::,0.),
                ],
            ),
            // (, vec![
            //     (Be::,0.),
            //     (Be::,0.),
            //     (Be::,0.),
            //     (Be::,0.),
            // ]),

            // (, vec![
            //     (Be::,0.),
            //     (Be::,0.),
            //     (Be::,0.),
            //     (Be::,0.),
            // ]),
        ];

        // for pokemon_no in 1..=1008 {
        for env_pokemon in env_pokemons.iter() {
            let pokemon_no = env_pokemon.0;
            let tera_types_and_rates = &env_pokemon.1;

            debug!("{}", pokemon_no);
            let pokemon_result =
                rustemon::pokemon::pokemon::get_by_id(pokemon_no, &self.rustemon_client).await;
            let pokemon_species_result =
                rustemon::pokemon::pokemon_species::get_by_id(pokemon_no, &self.rustemon_client)
                    .await;

            let base_stats_hp: i32;
            let base_stats_attack: i32;
            let base_stats_defense: i32;
            let base_stats_special_attack: i32;
            let base_stats_special_defense: i32;
            let base_stats_speed: i32;

            let mut ja_name: String = "".to_string();
            match pokemon_species_result {
                Ok(pokemon_species) => {
                    for name_in_lang in pokemon_species.names.iter() {
                        if name_in_lang.language.name == "ja" {
                            ja_name = name_in_lang.name.to_string();
                        }
                    }
                }
                Err(_) => {}
            }

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

                    while elms.len() < 3 {
                        elms.push(Mnone);
                    }

                    for (tera_type, _tera_rate) in tera_types_and_rates.iter() {
                        for is_terastallized in [false, true] {
                            for individual in get_individuals(
                                pokemon_no,
                                &elms,
                                *tera_type,
                                is_terastallized,
                                base_stats_hp,
                                base_stats_attack,
                                base_stats_defense,
                                base_stats_special_attack,
                                base_stats_special_defense,
                                base_stats_speed,
                                &ja_name,
                            ) {
                                self.data.push(individual);
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
}

fn get_individuals(
    pokemon_no: i64,
    elms: &Vec<Me>,
    tera_type: Be,
    is_terastallized: bool,
    base_stats_hp: i32,
    base_stats_attack: i32,
    base_stats_defense: i32,
    base_stats_special_attack: i32,
    base_stats_special_defense: i32,
    base_stats_speed: i32,
    ja_name: &str,
) -> Vec<PokemonIndividual> {
    let mut ans: Vec<PokemonIndividual> = vec![];

    let name_acs = if is_terastallized {
        format!("{}_ACS_{:?}", ja_name, tera_type)
    } else {
        format!("{}_ACS_テラス前{:?}", ja_name, tera_type)
    };

    let individual_acs = PokemonIndividual::new(
        PokemonSpecies::new_by_number_elements(
            pokemon_no.try_into().unwrap(),
            elms[0].clone(),
            elms[1].clone(),
            elms[2].clone(),
        ),
        tera_type,
        is_terastallized,
        base_stats_to_stats_hp(base_stats_hp, 31, 0, 50),
        base_stats_to_stats(base_stats_attack, 31, 252, 50, 1.1),
        base_stats_to_stats(base_stats_defense, 31, 0, 50, 1.0),
        base_stats_to_stats(base_stats_special_attack, 31, 252, 50, 1.1),
        base_stats_to_stats(base_stats_special_defense, 31, 0, 50, 1.0),
        base_stats_to_stats(base_stats_speed, 31, 252, 50, 1.1),
        vec![],
        name_acs,
    );
    ans.push(individual_acs);

    let name_hac = if is_terastallized {
        format!("{}_HAC_{:?}", ja_name, tera_type)
    } else {
        format!("{}_HAC_テラス前{:?}", ja_name, tera_type)
    };

    let individual_hac = PokemonIndividual::new(
        PokemonSpecies::new_by_number_elements(
            pokemon_no.try_into().unwrap(),
            elms[0].clone(),
            elms[1].clone(),
            elms[2].clone(),
        ),
        tera_type,
        is_terastallized,
        base_stats_to_stats_hp(base_stats_hp, 31, 252, 50),
        base_stats_to_stats(base_stats_attack, 31, 252, 50, 1.1),
        base_stats_to_stats(base_stats_defense, 31, 0, 50, 1.0),
        base_stats_to_stats(base_stats_special_attack, 31, 252, 50, 1.1),
        base_stats_to_stats(base_stats_special_defense, 31, 0, 50, 1.0),
        base_stats_to_stats(base_stats_speed, 31, 0, 50, 0.9),
        vec![],
        name_hac,
    );
    ans.push(individual_hac);

    let name_hbd = if is_terastallized {
        format!("{}_HBD_{:?}", ja_name, tera_type)
    } else {
        format!("{}_HBD_テラス前{:?}", ja_name, tera_type)
    };

    let individual_hbd = PokemonIndividual::new(
        PokemonSpecies::new_by_number_elements(
            pokemon_no.try_into().unwrap(),
            elms[0].clone(),
            elms[1].clone(),
            elms[2].clone(),
        ),
        tera_type,
        is_terastallized,
        base_stats_to_stats_hp(base_stats_hp, 31, 252, 50),
        base_stats_to_stats(base_stats_attack, 31, 0, 50, 0.9),
        base_stats_to_stats(base_stats_defense, 31, 252, 50, 1.1),
        base_stats_to_stats(base_stats_special_attack, 31, 0, 50, 0.9),
        base_stats_to_stats(base_stats_special_defense, 31, 252, 50, 1.1),
        base_stats_to_stats(base_stats_speed, 31, 0, 50, 1.0),
        vec![],
        name_hbd,
    );
    ans.push(individual_hbd);

    return ans;
}

fn get_initial_data() -> Vec<PokemonIndividual> {
    return vec![
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(149, Mbe(Be::Dragon), Mbe(Be::Flying), Mnone),
            Be::Normal,
            false,
            198,
            204,
            115,
            108,
            120,
            100,
            vec![Move::new(
                "しんそく",
                Be::Normal,
                MoveType::Physical,
                80,
                1.0,
                10,
            )],
            "カイリューHA".to_string(),
        ),
        PokemonIndividual::new(
            // PokemonSpecies::new_by_number_elements(149, Mbe(Be::Dragon), Mbe(Be::Flying), Mnone),
            PokemonSpecies::new_by_number_elements(149, Mbe(Be::Normal), Mnone, Mnone),
            Be::Normal,
            true,
            198,
            204,
            115,
            108,
            120,
            100,
            vec![Move::new(
                "しんそく",
                Be::Normal,
                MoveType::Physical,
                80,
                1.0,
                10,
            )],
            "カイリューHA(ノマテラ)".to_string(),
        ),
    ];
}
