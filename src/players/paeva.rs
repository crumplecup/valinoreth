use crate::players::eponym;
use crate::{
    AbsoluteDirection, Advantage, Attributes, BaseDamage, Disadvantage, Duty, Encumbrance, Luck,
    SenseOfDuty, Stats, Wealth,
};

impl eponym::Players {
    pub fn paeva() {
        let attributes = Attributes::from_vec(vec![8, 12, 11, 9, 9, 11, 11, 9]);
        let stats = Stats::from(attributes);
        let base_dmg = BaseDamage::from(attributes);
        let encumbrance = Encumbrance::from(&stats);

        let advantages = vec![
            Advantage::AbsoluteDirection(AbsoluteDirection::Normal),
            Advantage::Ambidexterity,
            Advantage::AnimalEmpathy,
            Advantage::IndependentIncome(1),
            Advantage::Luck(Luck::Normal),
            Advantage::Status(2),
            Advantage::Wealth(Wealth::VeryWealthy),
            Advantage::Voice,
        ];

        #[allow(clippy::unnecessary_fold)]
        let adv_cost = advantages
            .iter()
            .map(|v| v.cost())
            .fold(0, |sum, val| sum + val);
        let disadvantages = vec![
            Disadvantage::Duty(Duty::FairlyOften),
            Disadvantage::Selfless,
            Disadvantage::SenseOfDuty(SenseOfDuty::LargeGroup),
        ];
        #[allow(clippy::unnecessary_fold)]
        let disadv_cost = disadvantages
            .iter()
            .map(|v| v.cost())
            .fold(0, |sum, val| sum + val);
        tracing::info!("Paeva:");
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
