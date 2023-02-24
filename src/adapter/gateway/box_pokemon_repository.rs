use super::super::super::application::interface::BoxPokemonRepositoryTrait;
use super::super::super::entity::pokemon::AbilityElement as Ae;
use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement::Mae;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use super::super::super::entity::pokemon::MetaElement::Mnone;
use super::super::super::entity::pokemon::Move;
use super::super::super::entity::pokemon::MoveType;
use super::super::super::entity::pokemon::PokemonIndividual;
use super::super::super::entity::pokemon::PokemonSpecies;

pub struct BoxPokemonRepository {
    data: Vec<PokemonIndividual>,
}

impl BoxPokemonRepository {
    pub fn new() -> Self {
        Self {
            data: get_initial_data(),
        }
    }
}

impl BoxPokemonRepositoryTrait for BoxPokemonRepository {
    fn get_all(&self) -> &Vec<PokemonIndividual> {
        &self.data
    }
}

fn get_initial_data() -> Vec<PokemonIndividual> {
    return vec![
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(637, Mbe(Be::Bug), Mbe(Be::Fire), Mnone),
            Be::Bug,
            false,
            191,
            72,
            85,
            205,
            127,
            120,
            vec![
                Move::new("オーバーヒート", Be::Fire, MoveType::Special, 135, 1.0, 10),
                Move::new("むしのさざめき", Be::Bug, MoveType::Special, 90, 1.0, 10),
            ],
            "ウルガモス".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(130, Mbe(Be::Water), Mbe(Be::Flying), Mnone),
            Be::Water,
            false,
            202,
            194,
            99,
            72,
            120,
            101,
            vec![
                Move::new("アクアテール", Be::Water, MoveType::Physical, 90, 0.95, 10),
                Move::new("じしん", Be::Ground, MoveType::Physical, 100, 1.0, 10),
                Move::new("ゆきなだれ", Be::Ice, MoveType::Physical, 60, 1.0, 10),
                Move::new("だいもんじ", Be::Fire, MoveType::Special, 110, 1.0, 10),
            ],
            "ギャラドス".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(713, Mbe(Be::Ice), Mnone, Mnone),
            Be::Ice,
            false,
            202,
            185,
            205,
            57,
            66,
            48,
            vec![Move::new(
                "つららおとし",
                Be::Ice,
                MoveType::Physical,
                85,
                1.0,
                10,
            )],
            "クレベース".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(
                936,
                Mbe(Be::Fire),
                Mbe(Be::Psychic),
                Mae(Ae::FlashFire),
            ),
            Be::Fire,
            false,
            191,
            72,
            122,
            194,
            100,
            95,
            vec![Move::new(
                "オーバーヒート",
                Be::Fire,
                MoveType::Special,
                135,
                1.0,
                10,
            )],
            "グレンアルマ".to_string(),
        ),
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
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(128, Mbe(Be::Fighting), Mbe(Be::Water), Mnone),
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
                Move::new(
                    "ウェーブタックル",
                    Be::Water,
                    MoveType::Physical,
                    120,
                    1.0,
                    10,
                ),
            ],
            "ケンタロス(水)".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(1000, Mbe(Be::Steel), Mbe(Be::Ghost), Mnone),
            Be::Ghost,
            false,
            194,
            58,
            116,
            203,
            111,
            104,
            vec![
                Move::new(
                    "ゴールドラッシュ",
                    Be::Steel,
                    MoveType::Special,
                    120,
                    1.0,
                    10,
                ),
                Move::new("シャドーボール", Be::Steel, MoveType::Special, 80, 1.0, 10),
            ],
            "サーフゴー".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new_by_number_elements(
                635,
                Mbe(Be::Dark),
                Mbe(Be::Dragon),
                Mae(Ae::Levitate),
            ),
            Be::Steel,
            false,
            199,
            112,
            111,
            194,
            110,
            118,
            vec![
                Move::new(
                    "りゅうせいぐん",
                    Be::Dragon,
                    MoveType::Special,
                    130,
                    1.0,
                    10,
                ),
                Move::new("あくのはどう", Be::Dark, MoveType::Special, 80, 1.0, 10),
                Move::new("だいもんじ", Be::Fire, MoveType::Special, 110, 1.0, 10),
                Move::new("ラスターカノン", Be::Steel, MoveType::Special, 80, 1.0, 10),
            ],
            "サザンドラ".to_string(),
        ),
        // 次、ヤバコイルから
    ];
}
