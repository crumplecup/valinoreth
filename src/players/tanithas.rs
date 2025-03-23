use crate::{
    Advantage, Attributes, BaseDamage, Disadvantage, EiditicMemory, Encumbrance, Players, Stats,
};

impl Players {
    pub fn tanithas() {
        let attributes = Attributes::from_base(8, 8, 19, 8);
        let stats = Stats::from(attributes);
        let base_dmg = BaseDamage::from(attributes);
        let encumbrance = Encumbrance::from(&stats);

        let advantages = vec![
            Advantage::Ambidexterity,
            Advantage::BardicTalent(3),
            Advantage::EasyCasting(1),
            Advantage::EiditicMemory(EiditicMemory::Photographic),
            Advantage::LessSleep(2),
            Advantage::Magery(3),
            Advantage::ReducedConsumption(2),
            Advantage::StableCasting,
        ];
        #[allow(clippy::unnecessary_fold)]
        let adv_cost = advantages
            .iter()
            .map(|v| v.cost())
            .fold(0, |sum, val| sum + val);
        let disadvantages = vec![
            Disadvantage::Destiny(60),
            Disadvantage::SocialStigma(4),
            Disadvantage::Status(3),
        ];
        #[allow(clippy::unnecessary_fold)]
        let disadv_cost = disadvantages
            .iter()
            .map(|v| v.cost())
            .fold(0, |sum, val| sum + val);
        tracing::info!("Tanithas:");
        tracing::info!("Attributes: {attributes:#?}");
        tracing::info!("Basic Stats: {stats:#?}");
        tracing::info!("Basic Damage: {base_dmg:#?}");
        tracing::info!("Encumbrance: {encumbrance:#?}");
        tracing::info!("Advantages: {advantages:#?}");
        tracing::info!("Advantages cost: {adv_cost}");
        tracing::info!("Disadvantages: {disadvantages:#?}");
        tracing::info!("Disdvantages cost: {disadv_cost}");
    }
}
