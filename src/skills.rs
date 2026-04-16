//! Character skills and skill system.
//!
//! # GURPS Rules
//!
//! Skills represent trained abilities and knowledge. Each skill:
//! - Has a difficulty level (Easy, Average, Hard, Very Hard)
//! - Is based on an attribute (DX, IQ, HT, etc.)
//! - Costs points to improve
//!
//! # Citations
//!
//! - BS 170-185 - Skill system
//! - BS 186-253 - Skill descriptions

use crate::AttributeType;
use std::str::FromStr;

/// GURPS character skill.
///
/// # GURPS Rules
///
/// This enum contains all 278 skills from GURPS Basic Set.
/// Skills can be learned and improved with character points.
///
/// # Citations
///
/// BS 186-253 - Complete skill list
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum Skill {
    /// Financial record-keeping and auditing. BS 174
    Accounting,
    /// Gymnastic maneuvers and stunts. BS 174
    Acrobatics,
    /// Performance and deception. BS 174
    Acting,
    /// Managing organizations. BS 174
    Administration,
    /// Aerial maneuvers and stunts. BS 174
    Aerobatics,
    /// Operating lighter-than-air craft. BS 174
    Airshipman,
    /// Magical/chemical transmutation. BS 174
    Alchemy,
    /// Training and controlling animals. BS 175
    AnimalHandling,
    /// Study of human cultures. BS 175
    Anthropology,
    /// Underwater acrobatics. BS 175
    Aquabatics,
    /// Recovering and studying ancient artifacts. BS 175
    Archaeology,
    /// Building design. BS 175
    Architecture,
    /// Geographic and cultural knowledge of a region. BS 176
    AreaKnowledge,
    /// Maintaining and repairing armor and weapons. BS 176
    Armoury,
    /// Operating heavy weapons. BS 176
    Artillery,
    /// Creating visual art. BS 176
    Artist(Artist),
    /// Knowledge of stars and planets. BS 177
    Astronomy,
    /// Self-hypnosis techniques. BS 177
    Autohypnosis,
    /// Using axes and maces in combat. BS 177
    AxeMace,
    /// Shield attacks. BS 177
    Bashing,
    /// Operating powered armor. BS 178
    Battlesuit,
    /// Using energy weapons. BS 178
    BeamWeapons,
    /// Riding bicycles. BS 178
    Bicycling,
    /// Genetic engineering. BS 178
    Bioengineering,
    /// Life sciences. BS 178
    Biology,
    /// Fighting without vision. BS 178
    BlindFighting,
    /// Using blowguns. BS 178
    Blowpipe,
    /// Operating small watercraft. BS 179
    Boating,
    /// Controlling bodily functions. BS 179
    BodyControl,
    /// Reading body language. BS 179
    BodyLanguage,
    /// Internal spatial awareness. BS 179
    BodySense,
    /// Using bolas. BS 179
    Bolas,
    /// Carving bone and ivory. BS 179
    BoneCarving,
    /// Using bows. BS 179
    Bow,
    /// Unarmed combat with fists. BS 179
    Boxing,
    /// Hacking neural interfaces. BS 180
    BrainHacking,
    /// Psychological manipulation. BS 180
    Brainwashing,
    /// Unarmed combat. BS 180
    Brawling,
    /// Breaking objects with strikes. BS 180
    BreakingBlow,
    /// Controlling breathing. BS 180
    BreathControl,
    /// Using broadswords. BS 180
    Broadsword,
    /// Concealment and disguise of objects. BS 181
    Camouflage,
    /// Enthralling an audience. BS 181
    Captivate,
    /// Social drinking. BS 181
    Carousing,
    /// Woodworking. BS 181
    Carpentry,
    /// Map-making. BS 181
    Cartography,
    /// Science of matter and reactions. BS 181
    Chemistry,
    /// Scaling surfaces. BS 181
    Climbing,
    /// Using a cloak as a weapon. BS 181
    Cloak,
    /// Cinematic martial arts. BS 181
    CombatArt,
    /// Breaking into computer systems. BS 182
    ComputerHacking,
    /// Using computers. BS 182
    ComputerOperation,
    /// Writing software. BS 182
    ComputerProgramming,
    /// Appreciation of fine items. BS 182
    Connoisseur,
    /// Preparing food. BS 182
    Cooking,
    /// Creating forgeries. BS 182
    Counterfeiting,
    /// Operating large spacecraft. BS 182
    Crewman,
    /// Study of crime. BS 182
    Criminology,
    /// Using crossbows. BS 183
    Crossbow,
    /// Codes and ciphers. BS 183
    Cryptography,
    /// Knowledge of current events. BS 183
    CurrentAffairs(CurrentAffairs),
    /// Performing dances. BS 183
    Dancing,
    /// Spotting deception. BS 183
    DetectLies,
    /// Medical diagnosis. BS 183
    Diagnosis,
    /// Negotiation and treaties. BS 183
    Diplomacy,
    /// Altering appearance. BS 183
    Disguise,
    /// Operating diving equipment. BS 184
    DivingSuit,
    /// Lucid dreaming. BS 184
    Dreaming,
    /// Operating ground vehicles. BS 184
    Driving,
    /// Attacking from above. BS 184
    Dropping,
    /// Economic theory and markets. BS 184
    Economics,
    /// Electrical work. BS 184
    Electrician,
    /// Operating electronic devices. BS 184
    ElectronicsOperation(ElectronicsOperation),
    /// Repairing electronics. BS 185
    ElectronicsRepair,
    /// Engineering knowledge. BS 185
    Engineer,
    /// Supernatural influence. BS 185
    Enthrallment,
    /// Sexual arts. BS 185
    EroticArt,
    /// Getting free from bonds. BS 185
    Escape,
    /// Mystical healing. BS 185
    EsotericMedicine,
    /// Banishing spirits. BS 185
    Exorcism,
    /// Deep knowledge of a subject. BS 185
    ExpertSkill(ExpertSkill),
    /// Handling explosives. BS 185
    Explosives(Explosives),
    /// Training hunting birds. BS 186
    Falconry,
    /// Agriculture. BS 186
    Farming,
    /// Quickly drawing weapons. BS 186
    FastDraw,
    /// Deceptive persuasion. BS 186
    FastTalk,
    /// Petty theft and pickpocketing. BS 186
    Filch,
    /// Business and investment. BS 186
    Finance,
    /// Swallowing and breathing fire. BS 186
    FireEating,
    /// Emergency medical care. BS 186
    FirstAid,
    /// Catching fish. BS 187
    Fishing,
    /// Using flails. BS 187
    Flail,
    /// Flying with wings. BS 187
    Flight,
    /// Making stone tools. BS 187
    FlintKnapping,
    /// Cinematic leaping. BS 187
    FlyingLeap,
    /// Using energy swords. BS 187
    ForceSword,
    /// Using energy whips. BS 187
    ForceWhip,
    /// Breaking down doors and barriers. BS 187
    ForcedEntry,
    /// Crime scene investigation. BS 187
    Forensics,
    /// Creating fake documents. BS 187
    Forgery,
    /// Predicting the future. BS 187
    FortuneTelling,
    /// Directing artillery fire. BS 188
    ForwardObserver,
    /// Moving in zero gravity. BS 188
    FreeFall,
    /// Loading and unloading cargo. BS 188
    FreightHandling,
    /// Games of chance. BS 188
    Gambling,
    /// Playing competitive games. BS 188
    Games,
    /// Tending plants. BS 188
    Gardening,
    /// Strangling with a cord. BS 188
    Garrote,
    /// Physical geography. BS 188
    Geography,
    /// Study of rocks and minerals. BS 188
    Geology,
    /// Non-verbal communication. BS 188
    Gesture,
    /// Performing with a group. BS 189
    GroupPerformance,
    /// Operating heavy weapons. BS 189
    Gunner,
    /// Using firearms. BS 189
    Guns,
    /// Handling dangerous substances. BS 189
    HazardousMaterials,
    /// Coats of arms and lineage. BS 189
    Heraldry,
    /// Medicinal plants. BS 189
    HerbLore,
    /// Esoteric and supernatural knowledge. BS 189
    HiddenLore(HiddenLore),
    /// Long-distance walking. BS 189
    Hiking,
    /// Historical knowledge. BS 189
    History,
    /// Recreational skill. BS 189
    HobbySkill(HobbySkill),
    /// Concealing small items. BS 189
    Holdout,
    /// Domestic management. BS 189
    Housekeeping,
    /// Inducing hypnotic trances. BS 190
    Hypnotism,
    /// Resisting knockback. BS 190
    ImmovableStance,
    /// Using natural attacks. BS 190
    InnateAttack,
    /// Analyzing intelligence data. BS 190
    IntelligenceAnalysis,
    /// Questioning prisoners. BS 190
    Interrogation,
    /// Threatening others. BS 190
    Intimidation,
    /// Becoming invisible. BS 190
    InvisibilityArt,
    /// Working with gems and jewelry. BS 190
    Jeweler,
    /// Using jitte and sai. BS 190
    JitteSai,
    /// Throwing and grappling art. BS 191
    Judo,
    /// Leaping skill. BS 191
    Jumping,
    /// Striking martial art. BS 191
    Karate,
    /// Stunning shout. BS 191
    Kiai,
    /// Using knives in combat. BS 191
    Knife,
    /// Tying secure knots. BS 191
    KnotTying,
    /// Using kusari and chain weapons. BS 191
    Kusari,
    /// Using lances. BS 191
    Lance,
    /// Using lassos. BS 191
    Lasso,
    /// Legal knowledge. BS 191
    Law,
    /// Commanding others. BS 191
    Leadership,
    /// Working with leather. BS 192
    Leatherworking,
    /// Lifting heavy objects. BS 192
    Lifting,
    /// Walking on fragile surfaces. BS 192
    LightWalk,
    /// Language study. BS 192
    Linguistics,
    /// Reading lips. BS 192
    LipReading,
    /// Using liquid sprayers. BS 192
    LiquidProjector,
    /// Knowledge of literature. BS 192
    Literature,
    /// Opening locks. BS 192
    Lockpicking,
    /// Operating machine tools. BS 192
    Machinist,
    /// Using off-hand daggers. BS 193
    MainGauche,
    /// Cosmetics and disguise. BS 193
    Makeup,
    /// Analyzing markets. BS 193
    MarketAnalysis,
    /// Stone and brickwork. BS 193
    Masonry,
    /// Mathematical knowledge. BS 193
    Mathematics(Mathematics),
    /// Repairing machines. BS 193
    Mechanic,
    /// Mental discipline. BS 193
    Meditation,
    /// Using non-specific melee weapons. BS 193
    MeleeWeapon,
    /// Resisting mental attacks. BS 193
    MentalStrength,
    /// Trading and negotiation. BS 193
    Merchant,
    /// Working with metals. BS 194
    Metallurgy,
    /// Weather prediction. BS 194
    Meteorology,
    /// Imitating sounds. BS 194
    Mimicry(Mimicry),
    /// Blocking telepathy. BS 194
    MindBlock,
    /// Using monofilament whips. BS 194
    MonowireWhip,
    /// Riding animals. BS 194
    Mount,
    /// Composing music. BS 194
    MusicalComposition,
    /// Supernatural musical effects. BS 194
    MusicalInfluence,
    /// Playing instruments. BS 194
    MusicalInstrument,
    /// Natural history. BS 195
    Naturalist,
    /// Finding direction and position. BS 195
    Navigation,
    /// Operating hazmat suits. BS 195
    NBCSuit,
    /// Using nets as weapons. BS 195
    Net,
    /// Noticing details. BS 195
    Observation,
    /// Study of the supernatural. BS 195
    Occultism,
    /// Efficiently packing cargo. BS 195
    Packing,
    /// Study of fossils. BS 195
    Paleontology(Paleontology),
    /// Begging. BS 195
    Panhandling,
    /// Using parachutes. BS 195
    Parachuting,
    /// Deflecting missiles. BS 196
    ParryMissileWeapons,
    /// Entertaining an audience. BS 196
    Performance,
    /// Changing minds. BS 196
    Persuade,
    /// Preparing medicines. BS 196
    Pharmacy(Pharmacy),
    /// Philosophical knowledge. BS 196
    Philosophy,
    /// Taking photographs. BS 196
    Photography,
    /// Medical practice. BS 196
    Physician,
    /// Physical science. BS 196
    Physics,
    /// Study of body systems. BS 196
    Physiology,
    /// Stealing from pockets. BS 197
    Pickpocket,
    /// Operating aircraft and spacecraft. BS 197
    Piloting,
    /// Composing poetry. BS 197
    Poetry,
    /// Knowledge of toxins. BS 197
    Poisons,
    /// Using pole weapons. BS 197
    Polearm,
    /// Political knowledge and maneuvering. BS 197
    Politics,
    /// Devastating strikes. BS 197
    PowerBlow,
    /// Striking vital points. BS 197
    PressurePoints,
    /// Secrets of pressure points. BS 197
    PressureSecrets,
    /// Professional expertise. BS 197
    ProfessionalSkill(ProfessionalSkill),
    /// Political persuasion. BS 197
    Propaganda,
    /// Finding mineral deposits. BS 197
    Prospecting,
    /// Study of the mind. BS 197
    Psychology,
    /// Speaking to audiences. BS 197
    PublicSpeaking,
    /// Telekinetic pushing. BS 198
    Push,
    /// Using rapiers. BS 198
    Rapier,
    /// Performing religious ceremonies. BS 198
    ReligiousRitual,
    /// Finding information. BS 198
    Research,
    /// Controlling riding animals. BS 198
    Riding,
    /// Ceremonial magic. BS 198
    RitualMagic,
    /// Long-distance running. BS 198
    Running,
    /// Using sabers. BS 198
    Saber,
    /// Social expertise. BS 198
    SavoirFaire(SavoirFaire),
    /// Finding useful items. BS 199
    Scrounging,
    /// Using scuba gear. BS 199
    Scuba,
    /// Operating ships. BS 199
    Seamanship,
    /// Finding hidden objects. BS 199
    Search,
    /// Working with cloth. BS 199
    Sewing,
    /// Romantic attraction. BS 199
    SexAppeal,
    /// Following people. BS 199
    Shadowing,
    /// Using shields. BS 199
    Shield,
    /// Navigating large ships. BS 199
    Shiphandling,
    /// Using short swords. BS 199
    Shortsword,
    /// Vocal performance. BS 199
    Singing,
    /// Ice or roller skating. BS 200
    Skating,
    /// Skiing. BS 200
    Skiing,
    /// Manual dexterity tricks. BS 200
    SleightOfHand,
    /// Using slings. BS 200
    Sling,
    /// Using small swords. BS 200
    Smallsword,
    /// Metalworking. BS 200
    Smith,
    /// Concealing contraband. BS 200
    Smuggling,
    /// Study of societies. BS 200
    Sociology,
    /// Military skills. BS 200
    Soldier,
    /// Living in space. BS 200
    Spacer,
    /// Using spears. BS 200
    Spear,
    /// Using atlatls. BS 200
    SpearThrower,
    /// Rapid reading. BS 201
    SpeedReading,
    /// Athletic games. BS 201
    Sports,
    /// Using quarterstaffs. BS 201
    Staff,
    /// Theatrical combat. BS 201
    StageCombat,
    /// Moving quietly. BS 201
    Stealth,
    /// Urban underworld knowledge. BS 201
    Streetwise,
    /// Operating submarines. BS 201
    Submarine,
    /// Submarine crewing. BS 201
    Submariner,
    /// Planting suggestions. BS 201
    Suggest,
    /// Sumo wrestling art. BS 201
    SumoWrestling,
    /// Surgical procedures. BS 201
    Surgery,
    /// Wilderness survival. BS 201
    Survival,
    /// Influencing emotions. BS 202
    SwayEmotions,
    /// Swimming. BS 202
    Swimming,
    /// Drawing magical symbols. BS 202
    SymbolDrawing,
    /// Combat strategy. BS 202
    Tactics,
    /// Instructing others. BS 202
    Teaching,
    /// Driving wagons and teams. BS 202
    Teamster,
    /// Study of magic. BS 202
    Thaumatology,
    /// Religious study. BS 202
    Theology,
    /// Throwing objects. BS 202
    Throwing,
    /// Cinematic throwing. BS 202
    ThrowingArt,
    /// Using thrown weapons. BS 202
    ThrownWeapon,
    /// Using tonfa. BS 202
    Tonfa,
    /// Following tracks. BS 202
    Tracking,
    /// Building and disarming traps. BS 202
    Traps,
    /// Using two-handed axes and maces. BS 203
    TwoHandedAxeMace,
    /// Using two-handed flails. BS 203
    TwoHandedFlail,
    /// Using two-handed swords. BS 203
    TwoHandedSword,
    /// Typewriter and keyboard use. BS 203
    Typing,
    /// Surviving in cities. BS 203
    UrbanSurvival,
    /// Using vacuum suits. BS 203
    VaccSuit,
    /// Throwing voice. BS 203
    Ventriloquism,
    /// Animal medicine. BS 203
    Veterinary,
    /// Predicting weather. BS 203
    WeatherSense,
    /// Mad science. BS 203
    WeirdScience,
    /// Using whips. BS 203
    Whip,
    /// Grappling combat. BS 203
    Wrestling,
    /// Composing text. BS 203
    Writing,
    /// Meditative archery. BS 203
    ZenArchery,
}

impl Skill {
    /// The [`AttributeType`] on which the `Skill` is based.
    pub fn attribute(&self) -> AttributeType {
        match self {
            Self::Accounting => AttributeType::Intelligence,
            Self::Acrobatics => AttributeType::Dexterity,
            Self::Acting => AttributeType::Intelligence,
            Self::Administration => AttributeType::Intelligence,
            Self::Aerobatics => AttributeType::Dexterity,
            Self::Airshipman => AttributeType::Dexterity,
            Self::Alchemy => AttributeType::Intelligence,
            Self::AnimalHandling => AttributeType::Intelligence,
            Self::Anthropology => AttributeType::Intelligence,
            Self::Aquabatics => AttributeType::Dexterity,
            Self::Archaeology => AttributeType::Intelligence,
            Self::Architecture => AttributeType::Intelligence,
            Self::AreaKnowledge => AttributeType::Intelligence,
            Self::Armoury => AttributeType::Intelligence,
            Self::Artillery => AttributeType::Intelligence,
            Self::Artist(_) => AttributeType::Intelligence,
            Self::Astronomy => AttributeType::Intelligence,
            Self::Autohypnosis => AttributeType::Willpower,
            Self::AxeMace => AttributeType::Dexterity,
            Self::Bashing => AttributeType::Dexterity,
            Self::Battlesuit => AttributeType::Dexterity,
            Self::BeamWeapons => AttributeType::Dexterity,
            Self::Bicycling => AttributeType::Dexterity,
            Self::Bioengineering => AttributeType::Intelligence,
            Self::Biology => AttributeType::Intelligence,
            Self::BlindFighting => AttributeType::Perception,
            Self::Blowpipe => AttributeType::Dexterity,
            Self::Boating => AttributeType::Dexterity,
            Self::BodyControl => AttributeType::Dexterity,
            Self::BodyLanguage => AttributeType::Perception,
            Self::BodySense => AttributeType::Dexterity,
            Self::Bolas => AttributeType::Dexterity,
            Self::BoneCarving => AttributeType::Dexterity,
            Self::Bow => AttributeType::Dexterity,
            Self::Boxing => AttributeType::Dexterity,
            Self::BrainHacking => AttributeType::Intelligence,
            Self::Brainwashing => AttributeType::Intelligence,
            Self::Brawling => AttributeType::Dexterity,
            Self::BreakingBlow => AttributeType::Intelligence,
            Self::BreathControl => AttributeType::Health,
            Self::Broadsword => AttributeType::Dexterity,
            Self::Camouflage => AttributeType::Intelligence,
            Self::Captivate => AttributeType::Intelligence,
            Self::Carousing => AttributeType::Health,
            Self::Carpentry => AttributeType::Intelligence,
            Self::Cartography => AttributeType::Intelligence,
            Self::Chemistry => AttributeType::Intelligence,
            Self::Climbing => AttributeType::Dexterity,
            Self::Cloak => AttributeType::Dexterity,
            Self::CombatArt => AttributeType::Dexterity,
            Self::ComputerHacking => AttributeType::Intelligence,
            Self::ComputerOperation => AttributeType::Intelligence,
            Self::ComputerProgramming => AttributeType::Intelligence,
            Self::Connoisseur => AttributeType::Intelligence,
            Self::Cooking => AttributeType::Intelligence,
            Self::Counterfeiting => AttributeType::Dexterity,
            Self::Crewman => AttributeType::Intelligence,
            Self::Criminology => AttributeType::Intelligence,
            Self::Crossbow => AttributeType::Dexterity,
            Self::Cryptography => AttributeType::Intelligence,
            Self::CurrentAffairs(_) => AttributeType::Intelligence,
            Self::Dancing => AttributeType::Dexterity,
            Self::DetectLies => AttributeType::Perception,
            Self::Diagnosis => AttributeType::Intelligence,
            Self::Diplomacy => AttributeType::Intelligence,
            Self::Disguise => AttributeType::Intelligence,
            Self::DivingSuit => AttributeType::Dexterity,
            Self::Dreaming => AttributeType::Intelligence,
            Self::Driving => AttributeType::Dexterity,
            Self::Dropping => AttributeType::Dexterity,
            Self::Economics => AttributeType::Intelligence,
            Self::Electrician => AttributeType::Intelligence,
            Self::ElectronicsOperation(_) => AttributeType::Intelligence,
            Self::ElectronicsRepair => AttributeType::Intelligence,
            Self::Engineer => AttributeType::Intelligence,
            Self::Enthrallment => AttributeType::Intelligence,
            Self::EroticArt => AttributeType::Dexterity,
            Self::Escape => AttributeType::Dexterity,
            Self::EsotericMedicine => AttributeType::Intelligence,
            Self::Exorcism => AttributeType::Willpower,
            Self::ExpertSkill(_) => AttributeType::Intelligence,
            Self::Explosives(_) => AttributeType::Intelligence,
            Self::Falconry => AttributeType::Intelligence,
            Self::Farming => AttributeType::Intelligence,
            Self::FastDraw => AttributeType::Dexterity,
            Self::FastTalk => AttributeType::Intelligence,
            Self::Filch => AttributeType::Dexterity,
            Self::Finance => AttributeType::Intelligence,
            Self::FireEating => AttributeType::Dexterity,
            Self::FirstAid => AttributeType::Intelligence,
            Self::Fishing => AttributeType::Perception,
            Self::Flail => AttributeType::Dexterity,
            Self::Flight => AttributeType::Dexterity,
            Self::FlintKnapping => AttributeType::Dexterity,
            Self::FlyingLeap => AttributeType::Dexterity,
            Self::ForceSword => AttributeType::Dexterity,
            Self::ForceWhip => AttributeType::Dexterity,
            Self::ForcedEntry => AttributeType::Dexterity,
            Self::Forensics => AttributeType::Intelligence,
            Self::Forgery => AttributeType::Dexterity,
            Self::FortuneTelling => AttributeType::Intelligence,
            Self::ForwardObserver => AttributeType::Intelligence,
            Self::FreeFall => AttributeType::Dexterity,
            Self::FreightHandling => AttributeType::Intelligence,
            Self::Gambling => AttributeType::Intelligence,
            Self::Games => AttributeType::Intelligence,
            Self::Gardening => AttributeType::Intelligence,
            Self::Garrote => AttributeType::Dexterity,
            Self::Geography => AttributeType::Intelligence,
            Self::Geology => AttributeType::Intelligence,
            Self::Gesture => AttributeType::Intelligence,
            Self::GroupPerformance => AttributeType::Intelligence,
            Self::Gunner => AttributeType::Dexterity,
            Self::Guns => AttributeType::Dexterity,
            Self::HazardousMaterials => AttributeType::Intelligence,
            Self::Heraldry => AttributeType::Intelligence,
            Self::HerbLore => AttributeType::Intelligence,
            Self::HiddenLore(_) => AttributeType::Intelligence,
            Self::Hiking => AttributeType::Health,
            Self::History => AttributeType::Intelligence,
            Self::HobbySkill(skill) => skill.attribute(),
            Self::Holdout => AttributeType::Intelligence,
            Self::Housekeeping => AttributeType::Intelligence,
            Self::Hypnotism => AttributeType::Intelligence,
            Self::ImmovableStance => AttributeType::Dexterity,
            Self::InnateAttack => AttributeType::Dexterity,
            Self::IntelligenceAnalysis => AttributeType::Intelligence,
            Self::Interrogation => AttributeType::Intelligence,
            Self::Intimidation => AttributeType::Willpower,
            Self::InvisibilityArt => AttributeType::Dexterity,
            Self::Jeweler => AttributeType::Dexterity,
            Self::JitteSai => AttributeType::Dexterity,
            Self::Judo => AttributeType::Dexterity,
            Self::Jumping => AttributeType::Dexterity,
            Self::Karate => AttributeType::Dexterity,
            Self::Kiai => AttributeType::Health,
            Self::Knife => AttributeType::Dexterity,
            Self::KnotTying => AttributeType::Dexterity,
            Self::Kusari => AttributeType::Dexterity,
            Self::Lance => AttributeType::Dexterity,
            Self::Lasso => AttributeType::Dexterity,
            Self::Law => AttributeType::Intelligence,
            Self::Leadership => AttributeType::Intelligence,
            Self::Leatherworking => AttributeType::Dexterity,
            Self::Lifting => AttributeType::Strength,
            Self::LightWalk => AttributeType::Dexterity,
            Self::Linguistics => AttributeType::Intelligence,
            Self::LipReading => AttributeType::Perception,
            Self::LiquidProjector => AttributeType::Dexterity,
            Self::Literature => AttributeType::Intelligence,
            Self::Lockpicking => AttributeType::Dexterity,
            Self::Machinist => AttributeType::Intelligence,
            Self::MainGauche => AttributeType::Dexterity,
            Self::Makeup => AttributeType::Dexterity,
            Self::MarketAnalysis => AttributeType::Intelligence,
            Self::Masonry => AttributeType::Intelligence,
            Self::Mathematics(_) => AttributeType::Intelligence,
            Self::Mechanic => AttributeType::Intelligence,
            Self::Meditation => AttributeType::Willpower,
            Self::MeleeWeapon => AttributeType::Dexterity,
            Self::MentalStrength => AttributeType::Willpower,
            Self::Merchant => AttributeType::Intelligence,
            Self::Metallurgy => AttributeType::Intelligence,
            Self::Meteorology => AttributeType::Intelligence,
            Self::Mimicry(_) => AttributeType::Intelligence,
            Self::MindBlock => AttributeType::Willpower,
            Self::MonowireWhip => AttributeType::Dexterity,
            Self::Mount => AttributeType::Dexterity,
            Self::MusicalComposition => AttributeType::Intelligence,
            Self::MusicalInfluence => AttributeType::Intelligence,
            Self::MusicalInstrument => AttributeType::Dexterity,
            Self::Naturalist => AttributeType::Intelligence,
            Self::Navigation => AttributeType::Intelligence,
            Self::NBCSuit => AttributeType::Dexterity,
            Self::Net => AttributeType::Dexterity,
            Self::Observation => AttributeType::Perception,
            Self::Occultism => AttributeType::Intelligence,
            Self::Packing => AttributeType::Intelligence,
            Self::Paleontology(_) => AttributeType::Intelligence,
            Self::Panhandling => AttributeType::Intelligence,
            Self::Parachuting => AttributeType::Dexterity,
            Self::ParryMissileWeapons => AttributeType::Dexterity,
            Self::Performance => AttributeType::Intelligence,
            Self::Persuade => AttributeType::Intelligence,
            Self::Pharmacy(_) => AttributeType::Intelligence,
            Self::Philosophy => AttributeType::Intelligence,
            Self::Photography => AttributeType::Intelligence,
            Self::Physician => AttributeType::Intelligence,
            Self::Physics => AttributeType::Intelligence,
            Self::Physiology => AttributeType::Intelligence,
            Self::Pickpocket => AttributeType::Dexterity,
            Self::Piloting => AttributeType::Dexterity,
            Self::Poetry => AttributeType::Intelligence,
            Self::Poisons => AttributeType::Intelligence,
            Self::Polearm => AttributeType::Dexterity,
            Self::Politics => AttributeType::Intelligence,
            Self::PowerBlow => AttributeType::Health,
            Self::PressurePoints => AttributeType::Intelligence,
            Self::PressureSecrets => AttributeType::Intelligence,
            Self::ProfessionalSkill(skill) => skill.attribute(),
            Self::Propaganda => AttributeType::Intelligence,
            Self::Prospecting => AttributeType::Intelligence,
            Self::Psychology => AttributeType::Intelligence,
            Self::PublicSpeaking => AttributeType::Intelligence,
            Self::Push => AttributeType::Dexterity,
            Self::Rapier => AttributeType::Dexterity,
            Self::ReligiousRitual => AttributeType::Intelligence,
            Self::Research => AttributeType::Intelligence,
            Self::Riding => AttributeType::Dexterity,
            Self::RitualMagic => AttributeType::Intelligence,
            Self::Running => AttributeType::Health,
            Self::Saber => AttributeType::Dexterity,
            Self::SavoirFaire(_) => AttributeType::Intelligence,
            Self::Scrounging => AttributeType::Perception,
            Self::Scuba => AttributeType::Intelligence,
            Self::Seamanship => AttributeType::Intelligence,
            Self::Search => AttributeType::Perception,
            Self::Sewing => AttributeType::Dexterity,
            Self::SexAppeal => AttributeType::Health,
            Self::Shadowing => AttributeType::Intelligence,
            Self::Shield => AttributeType::Dexterity,
            Self::Shiphandling => AttributeType::Intelligence,
            Self::Shortsword => AttributeType::Dexterity,
            Self::Singing => AttributeType::Health,
            Self::Skating => AttributeType::Dexterity,
            Self::Skiing => AttributeType::Health,
            Self::SleightOfHand => AttributeType::Dexterity,
            Self::Sling => AttributeType::Dexterity,
            Self::Smallsword => AttributeType::Dexterity,
            Self::Smith => AttributeType::Intelligence,
            Self::Smuggling => AttributeType::Intelligence,
            Self::Sociology => AttributeType::Intelligence,
            Self::Soldier => AttributeType::Intelligence,
            Self::Spacer => AttributeType::Dexterity,
            Self::Spear => AttributeType::Dexterity,
            Self::SpearThrower => AttributeType::Dexterity,
            Self::SpeedReading => AttributeType::Intelligence,
            Self::Sports => AttributeType::Dexterity,
            Self::Staff => AttributeType::Dexterity,
            Self::StageCombat => AttributeType::Dexterity,
            Self::Stealth => AttributeType::Dexterity,
            Self::Streetwise => AttributeType::Intelligence,
            Self::Submarine => AttributeType::Dexterity,
            Self::Submariner => AttributeType::Dexterity,
            Self::Suggest => AttributeType::Intelligence,
            Self::SumoWrestling => AttributeType::Dexterity,
            Self::Surgery => AttributeType::Dexterity,
            Self::Survival => AttributeType::Intelligence,
            Self::SwayEmotions => AttributeType::Intelligence,
            Self::Swimming => AttributeType::Health,
            Self::SymbolDrawing => AttributeType::Intelligence,
            Self::Tactics => AttributeType::Intelligence,
            Self::Teaching => AttributeType::Intelligence,
            Self::Teamster => AttributeType::Intelligence,
            Self::Thaumatology => AttributeType::Intelligence,
            Self::Theology => AttributeType::Intelligence,
            Self::Throwing => AttributeType::Dexterity,
            Self::ThrowingArt => AttributeType::Dexterity,
            Self::ThrownWeapon => AttributeType::Dexterity,
            Self::Tonfa => AttributeType::Dexterity,
            Self::Tracking => AttributeType::Perception,
            Self::Traps => AttributeType::Intelligence,
            Self::TwoHandedAxeMace => AttributeType::Dexterity,
            Self::TwoHandedFlail => AttributeType::Dexterity,
            Self::TwoHandedSword => AttributeType::Dexterity,
            Self::Typing => AttributeType::Dexterity,
            Self::UrbanSurvival => AttributeType::Intelligence,
            Self::VaccSuit => AttributeType::Dexterity,
            Self::Ventriloquism => AttributeType::Intelligence,
            Self::Veterinary => AttributeType::Intelligence,
            Self::WeatherSense => AttributeType::Intelligence,
            Self::WeirdScience => AttributeType::Intelligence,
            Self::Whip => AttributeType::Dexterity,
            Self::Wrestling => AttributeType::Dexterity,
            Self::Writing => AttributeType::Intelligence,
            Self::ZenArchery => AttributeType::Intelligence,
        }
    }

    /// The [`Difficulty`] associated with the `Skill`.
    pub fn difficulty(&self) -> Difficulty {
        match self {
            Self::Accounting => Difficulty::Hard,
            Self::Acrobatics => Difficulty::Hard,
            Self::Acting => Difficulty::Average,
            Self::Administration => Difficulty::Average,
            Self::Aerobatics => Difficulty::Hard,
            Self::Airshipman => Difficulty::Average,
            Self::Alchemy => Difficulty::VeryHard,
            Self::AnimalHandling => Difficulty::Average,
            Self::Anthropology => Difficulty::Hard,
            Self::Aquabatics => Difficulty::Hard,
            Self::Archaeology => Difficulty::Hard,
            Self::Architecture => Difficulty::Hard,
            Self::AreaKnowledge => Difficulty::Easy,
            Self::Armoury => Difficulty::Average,
            Self::Artillery => Difficulty::Average,
            Self::Artist(_) => Difficulty::Hard,
            Self::Astronomy => Difficulty::Hard,
            Self::Autohypnosis => Difficulty::Hard,
            Self::AxeMace => Difficulty::Average,
            Self::Bashing => Difficulty::Easy,
            Self::Battlesuit => Difficulty::Average,
            Self::BeamWeapons => Difficulty::Easy,
            Self::Bicycling => Difficulty::Easy,
            Self::Bioengineering => Difficulty::VeryHard,
            Self::Biology => Difficulty::VeryHard,
            Self::BlindFighting => Difficulty::VeryHard,
            Self::Blowpipe => Difficulty::Hard,
            Self::Boating => Difficulty::Average,
            Self::BodyControl => Difficulty::VeryHard,
            Self::BodyLanguage => Difficulty::Average,
            Self::BodySense => Difficulty::Hard,
            Self::Bolas => Difficulty::Average,
            Self::BoneCarving => Difficulty::Average,
            Self::Bow => Difficulty::Hard,
            Self::Boxing => Difficulty::Average,
            Self::BrainHacking => Difficulty::VeryHard,
            Self::Brainwashing => Difficulty::Hard,
            Self::Brawling => Difficulty::Easy,
            Self::BreakingBlow => Difficulty::Hard,
            Self::BreathControl => Difficulty::Hard,
            Self::Broadsword => Difficulty::Average,
            Self::Camouflage => Difficulty::Easy,
            Self::Captivate => Difficulty::Hard,
            Self::Carousing => Difficulty::Easy,
            Self::Carpentry => Difficulty::Easy,
            Self::Cartography => Difficulty::Average,
            Self::Chemistry => Difficulty::Hard,
            Self::Climbing => Difficulty::Average,
            Self::Cloak => Difficulty::Average,
            Self::CombatArt => Difficulty::Hard,
            Self::ComputerHacking => Difficulty::VeryHard,
            Self::ComputerOperation => Difficulty::Easy,
            Self::ComputerProgramming => Difficulty::Hard,
            Self::Connoisseur => Difficulty::Average,
            Self::Cooking => Difficulty::Average,
            Self::Counterfeiting => Difficulty::Hard,
            Self::Crewman => Difficulty::Easy,
            Self::Criminology => Difficulty::Average,
            Self::Crossbow => Difficulty::Easy,
            Self::Cryptography => Difficulty::Hard,
            Self::CurrentAffairs(_) => Difficulty::Easy,
            Self::Dancing => Difficulty::Average,
            Self::DetectLies => Difficulty::Hard,
            Self::Diagnosis => Difficulty::Hard,
            Self::Diplomacy => Difficulty::Hard,
            Self::Disguise => Difficulty::Average,
            Self::DivingSuit => Difficulty::Average,
            Self::Dreaming => Difficulty::Hard,
            Self::Driving => Difficulty::Average,
            Self::Dropping => Difficulty::Hard,
            Self::Economics => Difficulty::Hard,
            Self::Electrician => Difficulty::Average,
            Self::ElectronicsOperation(_) => Difficulty::Average,
            Self::ElectronicsRepair => Difficulty::Average,
            Self::Engineer => Difficulty::Hard,
            Self::Enthrallment => Difficulty::Hard,
            Self::EroticArt => Difficulty::Average,
            Self::Escape => Difficulty::Hard,
            Self::EsotericMedicine => Difficulty::Hard,
            Self::Exorcism => Difficulty::Hard,
            Self::ExpertSkill(_) => Difficulty::Hard,
            Self::Explosives(_) => Difficulty::Average,
            Self::Falconry => Difficulty::Average,
            Self::Farming => Difficulty::Average,
            Self::FastDraw => Difficulty::Easy,
            Self::FastTalk => Difficulty::Average,
            Self::Filch => Difficulty::Average,
            Self::Finance => Difficulty::Hard,
            Self::FireEating => Difficulty::Average,
            Self::FirstAid => Difficulty::Easy,
            Self::Fishing => Difficulty::Easy,
            Self::Flail => Difficulty::Hard,
            Self::Flight => Difficulty::Average,
            Self::FlintKnapping => Difficulty::Average,
            Self::FlyingLeap => Difficulty::Hard,
            Self::ForceSword => Difficulty::Average,
            Self::ForceWhip => Difficulty::Average,
            Self::ForcedEntry => Difficulty::Easy,
            Self::Forensics => Difficulty::Hard,
            Self::Forgery => Difficulty::Hard,
            Self::FortuneTelling => Difficulty::Average,
            Self::ForwardObserver => Difficulty::Average,
            Self::FreeFall => Difficulty::Average,
            Self::FreightHandling => Difficulty::Average,
            Self::Gambling => Difficulty::Average,
            Self::Games => Difficulty::Easy,
            Self::Gardening => Difficulty::Easy,
            Self::Garrote => Difficulty::Easy,
            Self::Geography => Difficulty::Hard,
            Self::Geology => Difficulty::Hard,
            Self::Gesture => Difficulty::Easy,
            Self::GroupPerformance => Difficulty::Average,
            Self::Gunner => Difficulty::Easy,
            Self::Guns => Difficulty::Easy,
            Self::HazardousMaterials => Difficulty::Average,
            Self::Heraldry => Difficulty::Average,
            Self::HerbLore => Difficulty::VeryHard,
            Self::HiddenLore(_) => Difficulty::Average,
            Self::Hiking => Difficulty::Average,
            Self::History => Difficulty::Hard,
            Self::HobbySkill(_) => Difficulty::Easy,
            Self::Holdout => Difficulty::Average,
            Self::Housekeeping => Difficulty::Easy,
            Self::Hypnotism => Difficulty::Hard,
            Self::ImmovableStance => Difficulty::Hard,
            Self::InnateAttack => Difficulty::Easy,
            Self::IntelligenceAnalysis => Difficulty::Hard,
            Self::Interrogation => Difficulty::Average,
            Self::Intimidation => Difficulty::Average,
            Self::InvisibilityArt => Difficulty::VeryHard,
            Self::Jeweler => Difficulty::Hard,
            Self::JitteSai => Difficulty::Average,
            Self::Judo => Difficulty::Hard,
            Self::Jumping => Difficulty::Easy,
            Self::Karate => Difficulty::Hard,
            Self::Kiai => Difficulty::Hard,
            Self::Knife => Difficulty::Easy,
            Self::KnotTying => Difficulty::Easy,
            Self::Kusari => Difficulty::Hard,
            Self::Lance => Difficulty::Average,
            Self::Lasso => Difficulty::Average,
            Self::Law => Difficulty::Hard,
            Self::Leadership => Difficulty::Average,
            Self::Leatherworking => Difficulty::Average,
            Self::Lifting => Difficulty::Average,
            Self::LightWalk => Difficulty::Hard,
            Self::Linguistics => Difficulty::Hard,
            Self::LipReading => Difficulty::Average,
            Self::LiquidProjector => Difficulty::Easy,
            Self::Literature => Difficulty::Hard,
            Self::Lockpicking => Difficulty::Average,
            Self::Machinist => Difficulty::Average,
            Self::MainGauche => Difficulty::Average,
            Self::Makeup => Difficulty::Easy,
            Self::MarketAnalysis => Difficulty::Hard,
            Self::Masonry => Difficulty::Easy,
            Self::Mathematics(_) => Difficulty::Hard,
            Self::Mechanic => Difficulty::Average,
            Self::Meditation => Difficulty::Hard,
            Self::MeleeWeapon => Difficulty::Special,
            Self::MentalStrength => Difficulty::VeryHard,
            Self::Merchant => Difficulty::Average,
            Self::Metallurgy => Difficulty::Hard,
            Self::Meteorology => Difficulty::Average,
            Self::Mimicry(_) => Difficulty::Hard,
            Self::MindBlock => Difficulty::Average,
            Self::MonowireWhip => Difficulty::Hard,
            Self::Mount => Difficulty::Average,
            Self::MusicalComposition => Difficulty::Hard,
            Self::MusicalInfluence => Difficulty::VeryHard,
            Self::MusicalInstrument => Difficulty::Hard,
            Self::Naturalist => Difficulty::Hard,
            Self::Navigation => Difficulty::Average,
            Self::NBCSuit => Difficulty::Average,
            Self::Net => Difficulty::Hard,
            Self::Observation => Difficulty::Average,
            Self::Occultism => Difficulty::Average,
            Self::Packing => Difficulty::Average,
            Self::Paleontology(_) => Difficulty::Hard,
            Self::Panhandling => Difficulty::Average,
            Self::Parachuting => Difficulty::Average,
            Self::ParryMissileWeapons => Difficulty::Hard,
            Self::Performance => Difficulty::Average,
            Self::Persuade => Difficulty::Hard,
            Self::Pharmacy(_) => Difficulty::Hard,
            Self::Philosophy => Difficulty::Hard,
            Self::Photography => Difficulty::Average,
            Self::Physician => Difficulty::Hard,
            Self::Physics => Difficulty::VeryHard,
            Self::Physiology => Difficulty::Hard,
            Self::Pickpocket => Difficulty::Hard,
            Self::Piloting => Difficulty::Average,
            Self::Poetry => Difficulty::Average,
            Self::Poisons => Difficulty::Hard,
            Self::Polearm => Difficulty::Average,
            Self::Politics => Difficulty::Average,
            Self::PowerBlow => Difficulty::Hard,
            Self::PressurePoints => Difficulty::VeryHard,
            Self::PressureSecrets => Difficulty::VeryHard,
            Self::ProfessionalSkill(_) => Difficulty::Average,
            Self::Propaganda => Difficulty::Average,
            Self::Prospecting => Difficulty::Average,
            Self::Psychology => Difficulty::Hard,
            Self::PublicSpeaking => Difficulty::Average,
            Self::Push => Difficulty::Hard,
            Self::Rapier => Difficulty::Average,
            Self::ReligiousRitual => Difficulty::Hard,
            Self::Research => Difficulty::Average,
            Self::Riding => Difficulty::Average,
            Self::RitualMagic => Difficulty::VeryHard,
            Self::Running => Difficulty::Average,
            Self::Saber => Difficulty::Average,
            Self::SavoirFaire(_) => Difficulty::Easy,
            Self::Scrounging => Difficulty::Easy,
            Self::Scuba => Difficulty::Average,
            Self::Seamanship => Difficulty::Average,
            Self::Search => Difficulty::Average,
            Self::Sewing => Difficulty::Easy,
            Self::SexAppeal => Difficulty::Average,
            Self::Shadowing => Difficulty::Average,
            Self::Shield => Difficulty::Easy,
            Self::Shiphandling => Difficulty::Hard,
            Self::Shortsword => Difficulty::Average,
            Self::Singing => Difficulty::Easy,
            Self::Skating => Difficulty::Hard,
            Self::Skiing => Difficulty::Hard,
            Self::SleightOfHand => Difficulty::Hard,
            Self::Sling => Difficulty::Hard,
            Self::Smallsword => Difficulty::Average,
            Self::Smith => Difficulty::Average,
            Self::Smuggling => Difficulty::Average,
            Self::Sociology => Difficulty::Hard,
            Self::Soldier => Difficulty::Average,
            Self::Spacer => Difficulty::Average,
            Self::Spear => Difficulty::Average,
            Self::SpearThrower => Difficulty::Average,
            Self::SpeedReading => Difficulty::Average,
            Self::Sports => Difficulty::Average,
            Self::Staff => Difficulty::Average,
            Self::StageCombat => Difficulty::Average,
            Self::Stealth => Difficulty::Average,
            Self::Streetwise => Difficulty::Average,
            Self::Submarine => Difficulty::Average,
            Self::Submariner => Difficulty::Average,
            Self::Suggest => Difficulty::Hard,
            Self::SumoWrestling => Difficulty::Average,
            Self::Surgery => Difficulty::VeryHard,
            Self::Survival => Difficulty::Average,
            Self::SwayEmotions => Difficulty::Hard,
            Self::Swimming => Difficulty::Easy,
            Self::SymbolDrawing => Difficulty::Hard,
            Self::Tactics => Difficulty::Hard,
            Self::Teaching => Difficulty::Average,
            Self::Teamster => Difficulty::Average,
            Self::Thaumatology => Difficulty::VeryHard,
            Self::Theology => Difficulty::Hard,
            Self::Throwing => Difficulty::Hard,
            Self::ThrowingArt => Difficulty::Hard,
            Self::ThrownWeapon => Difficulty::Easy,
            Self::Tonfa => Difficulty::Average,
            Self::Tracking => Difficulty::Average,
            Self::Traps => Difficulty::Average,
            Self::TwoHandedAxeMace => Difficulty::Average,
            Self::TwoHandedFlail => Difficulty::Hard,
            Self::TwoHandedSword => Difficulty::Average,
            Self::Typing => Difficulty::Easy,
            Self::UrbanSurvival => Difficulty::Average,
            Self::VaccSuit => Difficulty::Average,
            Self::Ventriloquism => Difficulty::Hard,
            Self::Veterinary => Difficulty::Hard,
            Self::WeatherSense => Difficulty::Average,
            Self::WeirdScience => Difficulty::VeryHard,
            Self::Whip => Difficulty::Average,
            Self::Wrestling => Difficulty::Average,
            Self::Writing => Difficulty::Average,
            Self::ZenArchery => Difficulty::VeryHard,
        }
    }

    /// Book and page reference for the `Skill` description.
    pub fn reference(&self) -> &'static str {
        match self {
            Self::Accounting => "B174",
            Self::Acrobatics => "B174",
            Self::Acting => "B174",
            Self::Administration => "B174",
            Self::Aerobatics => "B174",
            Self::Airshipman => "B174",
            Self::Alchemy => "B174",
            Self::AnimalHandling => "B175",
            Self::Anthropology => "B175",
            Self::Aquabatics => "B175",
            Self::Archaeology => "B175",
            Self::Architecture => "B175",
            Self::AreaKnowledge => "B176",
            Self::Armoury => "B178",
            Self::Artillery => "B178",
            Self::Artist(_) => "B179",
            Self::Astronomy => "B179",
            Self::Autohypnosis => "B179",
            Self::AxeMace => "B208",
            Self::Bashing => "B179",
            Self::Battlesuit => "B179",
            Self::BeamWeapons => "B179",
            Self::Bicycling => "B180",
            Self::Bioengineering => "B180",
            Self::Biology => "B180",
            Self::BlindFighting => "B180",
            Self::Blowpipe => "B180",
            Self::Boating => "B180",
            Self::BodyControl => "B181",
            Self::BodyLanguage => "B181",
            Self::BodySense => "B181",
            Self::Bolas => "B181",
            Self::BoneCarving => "Lands Out of Time pg 11",
            Self::Bow => "B182",
            Self::Boxing => "B182",
            Self::BrainHacking => "B182",
            Self::Brainwashing => "B182",
            Self::Brawling => "B182",
            Self::BreakingBlow => "B182",
            Self::BreathControl => "B182",
            Self::Broadsword => "B208",
            Self::Camouflage => "B183",
            Self::Captivate => "B183",
            Self::Carousing => "B183",
            Self::Carpentry => "B183",
            Self::Cartography => "B183",
            Self::Chemistry => "B183",
            Self::Climbing => "B183",
            Self::Cloak => "B184",
            Self::CombatArt => "B184",
            Self::ComputerHacking => "B184",
            Self::ComputerOperation => "B184",
            Self::ComputerProgramming => "B184",
            Self::Connoisseur => "B185",
            Self::Cooking => "B185",
            Self::Counterfeiting => "B185",
            Self::Crewman => "B185",
            Self::Criminology => "B186",
            Self::Crossbow => "B186",
            Self::Cryptography => "B186",
            Self::CurrentAffairs(_) => "B186",
            Self::Dancing => "B187",
            Self::DetectLies => "B187",
            Self::Diagnosis => "B187",
            Self::Diplomacy => "B187",
            Self::Disguise => "B187",
            Self::DivingSuit => "B188",
            Self::Dreaming => "B188",
            Self::Driving => "B188",
            Self::Dropping => "B188",
            Self::Economics => "B188",
            Self::Electrician => "B189",
            Self::ElectronicsOperation(_) => "B189",
            Self::ElectronicsRepair => "B190",
            Self::Engineer => "B190",
            Self::Enthrallment => "B191",
            Self::EroticArt => "B191",
            Self::Escape => "B191",
            Self::EsotericMedicine => "B192",
            Self::Exorcism => "B192",
            Self::ExpertSkill(_) => "B193",
            Self::Explosives(_) => "B194",
            Self::Falconry => "B194",
            Self::Farming => "B194",
            Self::FastDraw => "B194",
            Self::FastTalk => "B195",
            Self::Filch => "B195",
            Self::Finance => "B195",
            Self::FireEating => "B195",
            Self::FirstAid => "B195",
            Self::Fishing => "B195",
            Self::Flail => "B208",
            Self::Flight => "B195",
            Self::FlintKnapping => "Lands Out of Time pg 11",
            Self::FlyingLeap => "B196",
            Self::ForceSword => "B196",
            Self::ForceWhip => "B196",
            Self::ForcedEntry => "B196",
            Self::Forensics => "B196",
            Self::Forgery => "B196",
            Self::FortuneTelling => "B197",
            Self::ForwardObserver => "B197",
            Self::FreeFall => "B197",
            Self::FreightHandling => "B197",
            Self::Gambling => "B197",
            Self::Games => "B197",
            Self::Gardening => "B198",
            Self::Garrote => "B198",
            Self::Geography => "B198",
            Self::Geology => "B198",
            Self::Gesture => "B198",
            Self::GroupPerformance => "B198",
            Self::Gunner => "B198",
            Self::Guns => "B198",
            Self::HazardousMaterials => "B199",
            Self::Heraldry => "B199",
            Self::HerbLore => "B199",
            Self::HiddenLore(_) => "B199",
            Self::Hiking => "B200",
            Self::History => "B200",
            Self::HobbySkill(_) => "B200",
            Self::Holdout => "B200",
            Self::Housekeeping => "B200",
            Self::Hypnotism => "B201",
            Self::ImmovableStance => "B201",
            Self::InnateAttack => "B201",
            Self::IntelligenceAnalysis => "B201",
            Self::Interrogation => "B202",
            Self::Intimidation => "B202",
            Self::InvisibilityArt => "B202",
            Self::Jeweler => "B202",
            Self::JitteSai => "B208",
            Self::Judo => "B203",
            Self::Jumping => "B203",
            Self::Karate => "B203",
            Self::Kiai => "B203",
            Self::Knife => "B208",
            Self::KnotTying => "B203",
            Self::Kusari => "B208",
            Self::Lance => "B204",
            Self::Lasso => "B204",
            Self::Law => "B204",
            Self::Leadership => "B204",
            Self::Leatherworking => "B204",
            Self::Lifting => "B205",
            Self::LightWalk => "B205",
            Self::Linguistics => "B205",
            Self::LipReading => "B205",
            Self::LiquidProjector => "B205",
            Self::Literature => "B205",
            Self::Lockpicking => "B206",
            Self::Machinist => "B206",
            Self::MainGauche => "B208",
            Self::Makeup => "B206",
            Self::MarketAnalysis => "B206",
            Self::Masonry => "B207",
            Self::Mathematics(_) => "B207",
            Self::Mechanic => "B207",
            Self::Meditation => "B207",
            Self::MeleeWeapon => "B208",
            Self::MentalStrength => "B209",
            Self::Merchant => "B209",
            Self::Metallurgy => "B209",
            Self::Meteorology => "B209",
            Self::Mimicry(_) => "B210",
            Self::MindBlock => "B210",
            Self::MonowireWhip => "B210",
            Self::Mount => "B210",
            Self::MusicalComposition => "B210",
            Self::MusicalInfluence => "B210",
            Self::MusicalInstrument => "B211",
            Self::Naturalist => "B211",
            Self::Navigation => "B211",
            Self::NBCSuit => "B211",
            Self::Net => "B211",
            Self::Observation => "B211",
            Self::Occultism => "B212",
            Self::Packing => "B212",
            Self::Paleontology(_) => "B212",
            Self::Panhandling => "B212",
            Self::Parachuting => "B212",
            Self::ParryMissileWeapons => "B212",
            Self::Performance => "B212",
            Self::Persuade => "B213",
            Self::Pharmacy(_) => "B213",
            Self::Philosophy => "B213",
            Self::Photography => "B213",
            Self::Physician => "B213",
            Self::Physics => "B213",
            Self::Physiology => "B213",
            Self::Pickpocket => "B213",
            Self::Piloting => "B214",
            Self::Poetry => "B214",
            Self::Poisons => "B214",
            Self::Polearm => "B208",
            Self::Politics => "B215",
            Self::PowerBlow => "B215",
            Self::PressurePoints => "B215",
            Self::PressureSecrets => "B215",
            Self::ProfessionalSkill(_) => "B215",
            Self::Propaganda => "B215",
            Self::Prospecting => "B216",
            Self::Psychology => "B216",
            Self::PublicSpeaking => "B216",
            Self::Push => "B216",
            Self::Rapier => "B208",
            Self::ReligiousRitual => "B216",
            Self::Research => "B217",
            Self::Riding => "B217",
            Self::RitualMagic => "B218",
            Self::Running => "B218",
            Self::Saber => "B208",
            Self::SavoirFaire(_) => "B218",
            Self::Scrounging => "B219",
            Self::Scuba => "B219",
            Self::Seamanship => "B220",
            Self::Search => "B219",
            Self::Sewing => "B219",
            Self::SexAppeal => "B219",
            Self::Shadowing => "B219",
            Self::Shield => "B220",
            Self::Shiphandling => "B220",
            Self::Shortsword => "B208",
            Self::Singing => "B220",
            Self::Skating => "B220",
            Self::Skiing => "B221",
            Self::SleightOfHand => "B221",
            Self::Sling => "B221",
            Self::Smallsword => "B208",
            Self::Smith => "B221",
            Self::Smuggling => "B221",
            Self::Sociology => "B221",
            Self::Soldier => "B221",
            Self::Spacer => "B222",
            Self::Spear => "B208",
            Self::SpearThrower => "B222",
            Self::SpeedReading => "B222",
            Self::Sports => "B222",
            Self::Staff => "B208",
            Self::StageCombat => "B222",
            Self::Stealth => "B222",
            Self::Streetwise => "B223",
            Self::Submarine => "B223",
            Self::Submariner => "B223",
            Self::Suggest => "B223",
            Self::SumoWrestling => "B223",
            Self::Surgery => "B223",
            Self::Survival => "B223",
            Self::SwayEmotions => "B224",
            Self::Swimming => "B224",
            Self::SymbolDrawing => "B224",
            Self::Tactics => "B224",
            Self::Teaching => "B224",
            Self::Teamster => "B225",
            Self::Thaumatology => "B225",
            Self::Theology => "B226",
            Self::Throwing => "B226",
            Self::ThrowingArt => "B226",
            Self::ThrownWeapon => "B226",
            Self::Tonfa => "B208",
            Self::Tracking => "B226",
            Self::Traps => "B226",
            Self::TwoHandedAxeMace => "B208",
            Self::TwoHandedFlail => "B208",
            Self::TwoHandedSword => "B208",
            Self::Typing => "B228",
            Self::UrbanSurvival => "B228",
            Self::VaccSuit => "B228",
            Self::Ventriloquism => "B228",
            Self::Veterinary => "B228",
            Self::WeatherSense => "B228",
            Self::WeirdScience => "B228",
            Self::Whip => "B208",
            Self::Wrestling => "B228",
            Self::Writing => "B228",
            Self::ZenArchery => "B228",
        }
    }
}

/// Human readable categories used to tag Skill variants for easier discovery and navigation.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
)]
pub enum Family {
    Animal,
    Arts,
    Athletic,
    Business,
    Combat,
    Craft,
    Criminal,
    Design,
    Entertainment,
    Esoteric,
    Everyman,
    Exploration,
    Humanities,
    Invention,
    Knowledge,
    Magical,
    Maintenance,
    Medical,
    Military,
    #[display("Natural Sciences")]
    NaturalSciences,
    Occult,
    Outdoor,
    Plant,
    Police,
    #[display("Ranged Combat")]
    RangedCombat,
    #[display("Ranged Weapon")]
    RangedWeapon,
    Repair,
    Scholarly,
    Social,
    #[display("Social Sciences")]
    SocialSciences,
    Spy,
    Street,
    Technical,
    Vehicle,
    Weapon,
}

impl Family {
    pub fn from_value(value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
pub enum SkillBase {
    #[from(AttributeType)]
    Attribute(AttributeType),
    #[from(Skill)]
    Skill(Skill),
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
pub struct SkillDefault {
    base: SkillBase,
    modifier: i64,
}

impl SkillDefault {
    pub fn new<T: Into<SkillBase>>(base: T, modifier: i64) -> Self {
        let base = base.into();
        Self { base, modifier }
    }
}

/// Specializations for the Artist skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Artist {
    #[default]
    Pottery,
    Sculpting,
    Woodworking,
}

/// Specializations for the Current Affairs skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum CurrentAffairs {
    #[default]
    HighCulture,
    PopularCulture,
    Business,
}

/// Specializations for the Electronics Operation skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum ElectronicsOperation {
    #[default]
    Media,
    Security,
    Medical,
    ElectronicWarfare,
    Surveillance,
}

/// Specializations for the Expert Skill skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum ExpertSkill {
    #[default]
    Epidemiology,
    MilitaryScience,
    Hydrology,
    NaturalPhilosophy,
    Psionics,
    Egyptology,
    PoliticalScience,
    Thanatology,
    Xenology,
    ComputerSecurity,
    Demolition,
    ExplosiveOrdnanceDisposal,
}

/// Specializations for the Explosives skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Explosives {
    #[default]
    Demolition,
    ExplosiveOrdnanceDisposal,
}

/// Specializations for the Hidden Lore skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum HiddenLore {
    #[default]
    DemonLore,
    FaerieLore,
    SpiritLore,
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum HobbySkill {
    #[default]
    Dexterity,
    Intelligence,
}

impl HobbySkill {
    pub fn attribute(&self) -> AttributeType {
        match self {
            Self::Dexterity => AttributeType::Dexterity,
            Self::Intelligence => AttributeType::Intelligence,
        }
    }
}

/// Specializations for the Mathematics skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Mathematics {
    #[default]
    Statistics,
    Surveying,
}

/// Specializations for the Mimicry skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Mimicry {
    #[default]
    AnimalSounds,
    BirdCalls,
}

/// Specializations for the Paleontology skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Paleontology {
    #[default]
    Paleobotany,
    Paleoanthropology,
}

/// Specializations for the Pharmacy skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Pharmacy {
    #[default]
    Herbal,
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum ProfessionalSkill {
    #[default]
    Dexterity,
    Intelligence,
}

impl ProfessionalSkill {
    pub fn attribute(&self) -> AttributeType {
        match self {
            Self::Dexterity => AttributeType::Dexterity,
            Self::Intelligence => AttributeType::Intelligence,
        }
    }
}

/// Specializations for the Savoir-Faire skill.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum SavoirFaire {
    #[default]
    HighSociety,
    Mafia,
    Servant,
    Military,
    Police,
}

/// Used to classify skills, spells, quick contests etc.
/// A measure of how quickly a [`Character`](crate::Character) can learn a [`Skill`].
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Difficulty {
    Easy,
    Average,
    Hard,
    VeryHard,
    Special,
}

impl Difficulty {
    /// Parses common abbreviations for `Difficulty` used in the text references.
    pub fn from_abbr(abbr: &str) -> Option<Self> {
        let lwr = abbr.to_lowercase();
        let value = match lwr.as_str() {
            "e" => Self::Easy,
            "a" => Self::Average,
            "h" => Self::Hard,
            "vh" => Self::VeryHard,
            "varies" => Self::Special,
            _ => return None,
        };
        Some(value)
    }
}
