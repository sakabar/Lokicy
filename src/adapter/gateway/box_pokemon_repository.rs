use super::super::super::application::interface::BoxPokemonRepositoryTrait;
use super::super::super::entity::pokemon::AbilityElement as Ae;
use super::super::super::entity::pokemon::BasicElement as Be;
use super::super::super::entity::pokemon::MetaElement::Mae;
use super::super::super::entity::pokemon::MetaElement::Mbe;
use super::super::super::entity::pokemon::MetaElement::Mnone;
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
            PokemonSpecies::new(637, Mbe(Be::Bug), Mbe(Be::Fire), Mnone),
            191,
            72,
            85,
            205,
            127,
            120,
            "ウルガモス".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(130, Mbe(Be::Water), Mbe(Be::Flying), Mnone),
            202,
            194,
            99,
            72,
            120,
            101,
            "ギャラドス".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(713, Mbe(Be::Ice), Mnone, Mnone),
            202,
            185,
            205,
            57,
            66,
            48,
            "クレベース".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(936, Mbe(Be::Fire), Mbe(Be::Psychic), Mae(Ae::FlashFire)),
            191,
            72,
            122,
            194,
            100,
            95,
            "グレンアルマ".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(128, Mbe(Be::Fighting), Mbe(Be::Fire), Mnone),
            182,
            178,
            126,
            45,
            90,
            120,
            "ケンタロス(炎)".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(128, Mbe(Be::Fighting), Mbe(Be::Water), Mnone),
            182,
            178,
            126,
            45,
            90,
            120,
            "ケンタロス(水)".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(1000, Mbe(Be::Steel), Mbe(Be::Ghost), Mnone),
            194,
            58,
            116,
            203,
            111,
            104,
            "サーフゴー".to_string(),
        ),
        PokemonIndividual::new(
            PokemonSpecies::new(635, Mbe(Be::Dark), Mbe(Be::Dragon), Mae(Ae::Levitate)),
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
}
