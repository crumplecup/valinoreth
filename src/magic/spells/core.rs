//! GURPS magic spell core definitions.
//!
//! # GURPS Rules
//!
//! Spells are organized into colleges, have energy costs, prerequisites,
//! casting times, durations, and may allow resistance.
//!
//! # Citations
//!
//! - BS 239-253 - Magic system
//! - M 10-200 - Individual spells

use tracing::{debug, instrument};

use super::{air, body_control, earth, fire, healing, illusion_creation, knowledge, mind_control, movement, protection_warning, water};
use super::{Duration, EnergyCost, ResistanceType, SpellPrerequisite, SpellType};

/// Magic spell colleges.
///
/// # GURPS Rules
///
/// Spells are grouped into thematic colleges. Prerequisites often
/// require knowing other spells from the same college.
///
/// # Citations
///
/// M 10 - Spell colleges
///
/// # Examples
///
/// ```
/// use valinoreth::SpellCollege;
///
/// let college = SpellCollege::Knowledge;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumIter)]
pub enum SpellCollege {
    /// Air and wind manipulation spells. M 24
    Air,
    /// Physical enhancement and affliction spells. M 36
    BodyControl,
    /// Earth and stone manipulation spells. M 56
    Earth,
    /// Fire and heat manipulation spells. M 68
    Fire,
    /// Healing and restoration spells. M 90
    Healing,
    /// Illusion and creation spells. M 94
    IllusionCreation,
    /// Information and divination spells. M 106
    Knowledge,
    /// Mental influence and control spells. M 118
    MindControl,
    /// Movement and teleportation spells. M 146
    Movement,
    /// Protection and warning spells. M 162
    ProtectionWarning,
    /// Water and ice manipulation spells. M 186
    Water,
}

/// GURPS magic spells (Knowledge college canary implementation).
///
/// # GURPS Rules
///
/// Spells require skill rolls (3d6 ≤ skill), expend energy (FP),
/// and have prerequisites (Magery, other spells).
///
/// # Citations
///
/// BS 239-253 - Magic system
/// M 106-113 - Knowledge college
///
/// # Examples
///
/// ```
/// use valinoreth::Spell;
///
/// let spell = Spell::DetectMagic;
/// assert_eq!(spell.college(), valinoreth::SpellCollege::Knowledge);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumIter)]
pub enum Spell {
    // Air College - M 24-35
    /// Purifies air quality. M 33
    PurifyAirAir,
    /// Creates breathable air. M 28
    CreateAir,
    /// Removes air from area. M 28
    DestroyAir,
    /// Controls air movement. M 34
    ShapeAir,
    /// Eliminates smells. M 32
    NoSmell,
    /// Creates foul odor. M 34
    Stench,
    /// Levitates subject. M 35
    WalkOnAir,
    /// Electrical lightning bolt. M 30
    Lightning,
    /// Creates strong winds. M 35
    Windstorm,
    /// Blast of compressed air. M 25
    AirJet,
    /// Allows underwater breathing. M 27
    BreatheWater,
    /// Transform into gaseous form. M 26
    BodyOfAir,
    /// Sonic blast attack. M 28
    Concussion,
    /// Create sound effects. M 34
    Sound,
    /// Eliminates all sound. M 34
    Silence,
    /// Loud sonic attack. M 35
    Thunderclap,
    /// Spinning air barrier. M 26
    AirVortex,
    /// Survive without air. M 29
    EssentialAir,
    /// See through smoke and fog. M 24
    AirVision,
    /// Create any pleasant scent. M 24
    Odor,
    /// Remove oxygen from air. M 25
    DevitalizeAir,
    /// Creates barrier of wind. M 24
    WallOfWind,
    /// Windstorm with flying sand. M 27
    Sandstorm,

    // Body Control College - M 36-67
    /// Causes minor itching. M 59
    Itch,
    /// Causes muscle spasm. M 65
    Spasm,
    /// Inflicts pain on target. M 61
    Pain,
    /// Paralyzes target. M 61
    Paralyze,
    /// Prevents foot movement. M 63
    RootedFeet,
    /// Blinds target temporarily. M 65
    StrikeBlind,
    /// Deafens target temporarily. M 65
    StrikeDeaf,
    /// Silences target temporarily. M 65
    StrikeDumb,
    /// Increases speed and movement. M 58
    Haste,
    /// Decreases speed and movement. M 64
    Slow,
    /// Increases ST temporarily. M 66
    Strengthen,
    /// Decreases ST. M 67
    Weaken,
    /// Increases DX temporarily. M 58
    Grace,
    /// Decreases DX. M 51
    Clumsiness,
    /// Ignore pain penalties. M 63
    ResistPain,
    /// Resist disease effects. M 62
    ResistDisease,
    /// Cripples limb permanently. M 67
    WitherLimb,
    /// Direct HP damage. M 54
    Deathtouch,
    /// Transform into stone form. M 49
    BodyOfStone,
    /// Physical transformation. M 36
    AlterBody,
    /// Reshape flesh and bone. M 64
    ShapeFlesh,
    /// Increases ST temporarily. M 60
    Might,
    /// Increases HT temporarily. M 66
    Vigor,
    /// Manipulates fatigue. M 56
    Fatigue,
    /// Stuns target. M 65
    Stun,
    /// Makes target trip easily. M 66
    Tanglefoot,
    /// Complete paralysis. M 67
    TotalParalysis,
    /// Changes facial features. M 41
    AlterVisage,
    /// Grants climbing skill. M 35
    Climbing,

    // Earth College - M 56-67
    /// Manipulates earth and stone. M 64
    ShapeEarth,
    /// Hardens earth to stone. M 59
    EarthToStone,
    /// Softens stone to earth. M 66
    StoneToEarth,
    /// Creates earth or soil. M 57
    CreateEarth,
    /// Disintegrates earth and stone. M 58
    DestroyEarth,
    /// See through earth and stone. M 59
    EarthVision,
    /// Phase through stone. M 67
    WalkThroughEarth,
    /// Causes ground tremors. M 59
    Earthquake,
    /// Sculpts stone precisely. M 64
    ShapeStone,
    /// Transforms earth to air. M 58
    EarthToAir,
    /// Hurls stone projectiles. M 66
    StoneMissile,
    /// Creates solid stone. M 57
    CreateStone,
    /// Blast of abrasive sand. M 64
    SandJet,
    /// Turns flesh to stone. M 60
    FleshToStone,
    /// Reverses petrification. M 66
    StoneToFlesh,
    /// Strengthens limbs like iron. M 62
    IronArm,
    /// Traps target in earth. M 59
    Entombment,
    /// Summons earth elemental. M 58
    SummonEarthElemental,
    /// Cleanse earth and soil. M 63
    PurifyEarth,
    /// Survive without earth contact. M 60
    EssentialEarth,
    /// Transform body to living earth. M 48
    BodyOfEarth,
    /// Transform earth to water. M 59
    EarthToWater,
    /// Analyze metal composition. M 62
    IdentifyMetal,
    /// Sculpt and shape metal. M 64
    ShapeMetal,

    // Fire College - M 68-77
    /// Creates small flame. M 68
    IgniteFlame,
    /// Creates fire from nothing. M 69
    CreateFire,
    /// Puts out fires. M 71
    ExtinguishFire,
    /// Controls and sculpts fire. M 76
    ShapeFire,
    /// Hurled ball of fire. M 70
    Fireball,
    /// Fireball that explodes on impact. M 71
    ExplosiveFireball,
    /// Cone of flame from hands. M 70
    FlameJet,
    /// Heats objects or creatures. M 72
    Heat,
    /// Cools objects or creatures. M 69
    Cold,
    /// Protects from fire damage. M 75
    ResistFire,
    /// Protects from cold damage. M 75
    ResistCold,
    /// Removes smoke and bad air. M 74
    PurifyAir,
    /// Sphere of utter cold. M 69
    Frostbite,
    /// Wall of flames. M 76
    WallOfFire,
    /// Summons fire elemental. M 76
    SummonFireElemental,
    /// Permanent magical fire. M 74
    PermanentFlame,
    /// Seek and destroy fire. M 76
    SeekFireFire,
    /// Ignites target at range. M 72
    Ignition,

    // Healing College - M 90-105
    /// Transfers FP between casters. M 93
    LendEnergy,
    /// Converts HP to FP. M 94
    LendVitality,
    /// Restores lost FP. M 101
    RecoverEnergy,
    /// Heals 1-3 HP. M 96
    MinorHealing,
    /// Heals any amount of HP. M 94
    MajorHealing,
    /// Instant major healing. M 92
    GreatHealing,
    /// Wakes subject from sleep. M 90
    Awaken,
    /// Cures diseases. M 91
    CureDisease,
    /// Removes poison effects. M 98
    NeutralizePoison,
    /// Instant poison removal. M 92
    InstantNeutralizePoison,
    /// Stops bleeding wounds. M 103
    StopBleeding,
    /// Regenerates HP over time. M 101
    Regeneration,
    /// Restores lost limbs. M 102
    Restoration,
    /// Delays curse effects. M 103
    SuspendCurse,
    /// Removes curses permanently. M 101
    RemoveCurse,
    /// Cures magical ailments. M 91
    CurseRemoval,
    /// Heals crippled limbs. M 92
    Healing,
    /// Restores youth. M 102
    RestoreYouth,

    // Illusion & Creation College - M 94-105
    /// Creates simple sound. M 100
    SimpleIllusion,
    /// Creates visual image. M 97
    PerfectIllusion,
    /// Creates complex illusion. M 96
    ComplexIllusion,
    /// Makes illusion fully real. M 102
    MakeRealIllusion,
    /// Creates simple object. M 100
    Create,
    /// Makes created object permanent. M 99
    PermanentCreation,
    /// Hides object from sight. M 98
    Invisibility,
    /// Complete sensory concealment. M 98
    InvisibilityToAll,
    /// Disguises appearance. M 97
    Disguise,
    /// Alters appearance significantly. M 103
    ShapeShifting,
    /// Creates duplicate of self. M 98
    Duplicate,
    /// Creates illusory terrain. M 100
    Phantasm,
    /// Silent image projection. M 100
    SilentImage,
    /// Creates glowing light. M 99
    Light,
    /// Creates magical darkness. M 97
    Darkness,
    /// Hides aura from detection. M 98
    HideAura,
    /// Reflects spells back. M 101
    Mirror,

    // Mind Control College - M 118-144
    /// Confuses target briefly. M 122
    Daze,
    /// Puts target to sleep. M 139
    Sleep,
    /// Forces single-word command. M 121
    Command,
    /// Area daze effect. M 130
    MassDaze,
    /// Causes fear in target. M 124
    Fear,
    /// Intense paralyzing fear. M 141
    Terror,
    /// Makes target loyal to caster. M 130
    Loyalty,
    /// Makes target friendly. M 121
    Charm,
    /// Complete mental enslavement. M 123
    Enslave,
    /// Reads surface thoughts. M 132
    MindReading,
    /// Sends thoughts to target. M 133
    MindSending,
    /// Two-way mental communication. M 141
    Telepathy,
    /// Erases recent memories. M 125
    Forgetfulness,
    /// Implants false memory. M 124
    FalseMemory,
    /// Plants subconscious suggestion. M 140
    Suggestion,
    /// Forces target to speak truth. M 121
    CompelTruth,
    /// Blocks mind reading attempts. M 127
    HideThoughts,
    /// Protects against mental attacks. M 138
    ShieldMind,
    /// Takes over target's body. M 136
    Possession,
    /// Traps soul in object. M 140
    SoulJar,

    // Movement College - M 146-161
    /// Levitates object or person. M 154
    Levitation,
    /// Flies through the air. M 152
    Flight,
    /// Teleports short distance. M 159
    Blink,
    /// Teleports to known location. M 160
    Teleport,
    /// Creates teleport portal. M 159
    Teleportation,
    /// Increases movement speed. M 153
    HasteMovement,
    /// Slows movement. M 158
    SlowMovement,
    /// Allows wall climbing. M 151
    Cling,
    /// Jump to great heights. M 154
    Jump,
    /// Instantly swap positions. M 160
    Apportation,
    /// Summons object to hand. M 147
    Poltergeist,
    /// Opens locks remotely. M 155
    Lockmaster,
    /// Throws object with force. M 156
    TelekineticBlow,
    /// Stops movement completely. M 153
    HaltMovement,

    // Protection & Warning College - M 162-185
    /// Basic magical shield. M 182
    Shield,
    /// Deflects physical attacks. M 168
    Deflect,
    /// Reflects attacks back. M 177
    Reflection,
    /// Absorbs incoming energy. M 163
    Absorb,
    /// Warns of danger. M 185
    Warn,
    /// Detects hostile intent. M 164
    Sense,
    /// Protects from fire. M 175
    ResistFireProtection,
    /// Protects from cold. M 175
    ResistColdProtection,
    /// Protects from lightning. M 175
    ResistLightning,
    /// Protects from acid. M 175
    ResistAcid,
    /// General damage reduction. M 163
    ArmorEnchantment,
    /// Prevents teleportation. M 163
    AnchorSpell,
    /// Alerts to approaching beings. M 185
    WatchdogSpell,

    // Water College - M 186-200
    /// Purifies water quality. M 196
    PurifyWater,
    /// Creates fresh water. M 191
    CreateWater,
    /// Removes water from area. M 192
    DestroyWater,
    /// Controls water movement. M 199
    ShapeWater,
    /// Freezes water to ice. M 193
    Freeze,
    /// Boils water instantly. M 189
    Boil,
    /// Walk on water surface. M 200
    WalkOnWater,
    /// Blast of pressurized water. M 200
    WaterJet,
    /// Sphere of solid ice. M 194
    IceSphere,
    /// Creates slippery ice. M 194
    IceSlick,
    /// Transforms into water. M 189
    BodyOfWater,
    /// Removes water from target. M 192
    Dehydrate,
    /// Creates solid ice. M 191
    CreateIce,
    /// Condenses water from air. M 191
    CondenseSteam,
    /// Conjures ice weapon. M 194
    IceDagger,
    /// Enhanced swimming ability. M 199
    Swim,
    /// Summons water elemental. M 199
    SummonWaterElemental,
    /// Breathe underwater. M 189
    BreatheWaterWater,

    // Knowledge College - M 106-113
    /// Reveals presence of magic within radius. M 107
    DetectMagic,
    /// Analyzes magical items and effects. M 106
    AnalyzeMagic,
    /// Identifies specific spell being cast. M 109
    IdentifySpell,
    /// Locates specific object or person. M 112
    Seeker,
    /// Locates air or vacuum. M 111
    SeekAir,
    /// Locates earth or stone. M 111
    SeekEarth,
    /// Locates fire or heat. M 112
    SeekFire,
    /// Locates water or liquids. M 112
    SeekWater,
    /// Locates plants. M 112
    SeekPlant,
    /// Locates food. M 111
    SeekFood,
    /// Locates machines or technology. M 111
    SeekMachine,
    /// Reveals object's past. M 108
    History,
    /// Provides helpful insight or advice. M 113
    Wisdom,
    /// Reveals ancient historical events. M 106
    AncientHistory,
    /// Restores lost memories. M 110
    RecoverMemory,
    /// Answers yes/no questions about future. M 107
    Divination,
    /// Shows best path to destination. M 110
    Pathfinder,
    /// Makes wall transparent. M 108
    GlassWall,
    /// Tracks person or object. M 113
    Trace,
    /// Reveals character's nature and power. M 107
    Aura,
}

impl Spell {
    /// Returns the spell's college.
    ///
    /// # Citations
    ///
    /// M 106 - Knowledge college
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, SpellCollege};
    ///
    /// assert_eq!(Spell::DetectMagic.college(), SpellCollege::Knowledge);
    /// assert_eq!(Spell::Seeker.college(), SpellCollege::Knowledge);
    /// ```
    #[instrument]
    pub fn college(&self) -> SpellCollege {
        debug!("Getting spell college");

        let college = match self {
            // Air College
            Self::PurifyAirAir => SpellCollege::Air,
            Self::CreateAir => SpellCollege::Air,
            Self::DestroyAir => SpellCollege::Air,
            Self::ShapeAir => SpellCollege::Air,
            Self::NoSmell => SpellCollege::Air,
            Self::Stench => SpellCollege::Air,
            Self::WalkOnAir => SpellCollege::Air,
            Self::Lightning => SpellCollege::Air,
            Self::Windstorm => SpellCollege::Air,
            Self::AirJet => SpellCollege::Air,
            Self::BreatheWater => SpellCollege::Air,
            Self::BodyOfAir => SpellCollege::Air,
            Self::Concussion => SpellCollege::Air,
            Self::Sound => SpellCollege::Air,
            Self::Silence => SpellCollege::Air,
            Self::Thunderclap => SpellCollege::Air,
            Self::AirVortex => SpellCollege::Air,
            Self::EssentialAir => SpellCollege::Air,
            Self::AirVision => SpellCollege::Air,
            Self::Odor => SpellCollege::Air,
            Self::DevitalizeAir => SpellCollege::Air,
            Self::WallOfWind => SpellCollege::Air,
            Self::Sandstorm => SpellCollege::Air,

            // Body Control College
            Self::Itch => SpellCollege::BodyControl,
            Self::Spasm => SpellCollege::BodyControl,
            Self::Pain => SpellCollege::BodyControl,
            Self::Paralyze => SpellCollege::BodyControl,
            Self::RootedFeet => SpellCollege::BodyControl,
            Self::StrikeBlind => SpellCollege::BodyControl,
            Self::StrikeDeaf => SpellCollege::BodyControl,
            Self::StrikeDumb => SpellCollege::BodyControl,
            Self::Haste => SpellCollege::BodyControl,
            Self::Slow => SpellCollege::BodyControl,
            Self::Strengthen => SpellCollege::BodyControl,
            Self::Weaken => SpellCollege::BodyControl,
            Self::Grace => SpellCollege::BodyControl,
            Self::Clumsiness => SpellCollege::BodyControl,
            Self::ResistPain => SpellCollege::BodyControl,
            Self::ResistDisease => SpellCollege::BodyControl,
            Self::WitherLimb => SpellCollege::BodyControl,
            Self::Deathtouch => SpellCollege::BodyControl,
            Self::BodyOfStone => SpellCollege::BodyControl,
            Self::AlterBody => SpellCollege::BodyControl,
            Self::ShapeFlesh => SpellCollege::BodyControl,
            Self::Might => SpellCollege::BodyControl,
            Self::Vigor => SpellCollege::BodyControl,
            Self::Fatigue => SpellCollege::BodyControl,
            Self::Stun => SpellCollege::BodyControl,
            Self::Tanglefoot => SpellCollege::BodyControl,
            Self::TotalParalysis => SpellCollege::BodyControl,
            Self::AlterVisage => SpellCollege::BodyControl,
            Self::Climbing => SpellCollege::BodyControl,

            // Earth College
            Self::ShapeEarth => SpellCollege::Earth,
            Self::EarthToStone => SpellCollege::Earth,
            Self::StoneToEarth => SpellCollege::Earth,
            Self::CreateEarth => SpellCollege::Earth,
            Self::DestroyEarth => SpellCollege::Earth,
            Self::EarthVision => SpellCollege::Earth,
            Self::WalkThroughEarth => SpellCollege::Earth,
            Self::Earthquake => SpellCollege::Earth,
            Self::ShapeStone => SpellCollege::Earth,
            Self::EarthToAir => SpellCollege::Earth,
            Self::StoneMissile => SpellCollege::Earth,
            Self::CreateStone => SpellCollege::Earth,
            Self::SandJet => SpellCollege::Earth,
            Self::FleshToStone => SpellCollege::Earth,
            Self::StoneToFlesh => SpellCollege::Earth,
            Self::IronArm => SpellCollege::Earth,
            Self::Entombment => SpellCollege::Earth,
            Self::SummonEarthElemental => SpellCollege::Earth,
            Self::PurifyEarth => SpellCollege::Earth,
            Self::EssentialEarth => SpellCollege::Earth,
            Self::BodyOfEarth => SpellCollege::Earth,
            Self::EarthToWater => SpellCollege::Earth,
            Self::IdentifyMetal => SpellCollege::Earth,
            Self::ShapeMetal => SpellCollege::Earth,

            // Fire College
            Self::IgniteFlame => SpellCollege::Fire,
            Self::CreateFire => SpellCollege::Fire,
            Self::ExtinguishFire => SpellCollege::Fire,
            Self::ShapeFire => SpellCollege::Fire,
            Self::Fireball => SpellCollege::Fire,
            Self::ExplosiveFireball => SpellCollege::Fire,
            Self::FlameJet => SpellCollege::Fire,
            Self::Heat => SpellCollege::Fire,
            Self::Cold => SpellCollege::Fire,
            Self::ResistFire => SpellCollege::Fire,
            Self::ResistCold => SpellCollege::Fire,
            Self::PurifyAir => SpellCollege::Fire,
            Self::Frostbite => SpellCollege::Fire,
            Self::WallOfFire => SpellCollege::Fire,
            Self::SummonFireElemental => SpellCollege::Fire,
            Self::PermanentFlame => SpellCollege::Fire,
            Self::SeekFireFire => SpellCollege::Fire,
            Self::Ignition => SpellCollege::Fire,

            // Healing College
            Self::LendEnergy => SpellCollege::Healing,
            Self::LendVitality => SpellCollege::Healing,
            Self::RecoverEnergy => SpellCollege::Healing,
            Self::MinorHealing => SpellCollege::Healing,
            Self::MajorHealing => SpellCollege::Healing,
            Self::GreatHealing => SpellCollege::Healing,
            Self::Awaken => SpellCollege::Healing,
            Self::CureDisease => SpellCollege::Healing,
            Self::NeutralizePoison => SpellCollege::Healing,
            Self::InstantNeutralizePoison => SpellCollege::Healing,
            Self::StopBleeding => SpellCollege::Healing,
            Self::Regeneration => SpellCollege::Healing,
            Self::Restoration => SpellCollege::Healing,
            Self::SuspendCurse => SpellCollege::Healing,
            Self::RemoveCurse => SpellCollege::Healing,
            Self::CurseRemoval => SpellCollege::Healing,
            Self::Healing => SpellCollege::Healing,
            Self::RestoreYouth => SpellCollege::Healing,

            // Illusion & Creation College
            Self::SimpleIllusion => SpellCollege::IllusionCreation,
            Self::PerfectIllusion => SpellCollege::IllusionCreation,
            Self::ComplexIllusion => SpellCollege::IllusionCreation,
            Self::MakeRealIllusion => SpellCollege::IllusionCreation,
            Self::Create => SpellCollege::IllusionCreation,
            Self::PermanentCreation => SpellCollege::IllusionCreation,
            Self::Invisibility => SpellCollege::IllusionCreation,
            Self::InvisibilityToAll => SpellCollege::IllusionCreation,
            Self::Disguise => SpellCollege::IllusionCreation,
            Self::ShapeShifting => SpellCollege::IllusionCreation,
            Self::Duplicate => SpellCollege::IllusionCreation,
            Self::Phantasm => SpellCollege::IllusionCreation,
            Self::SilentImage => SpellCollege::IllusionCreation,
            Self::Light => SpellCollege::IllusionCreation,
            Self::Darkness => SpellCollege::IllusionCreation,
            Self::HideAura => SpellCollege::IllusionCreation,
            Self::Mirror => SpellCollege::IllusionCreation,

            // Mind Control College
            Self::Daze => SpellCollege::MindControl,
            Self::Sleep => SpellCollege::MindControl,
            Self::Command => SpellCollege::MindControl,
            Self::MassDaze => SpellCollege::MindControl,
            Self::Fear => SpellCollege::MindControl,
            Self::Terror => SpellCollege::MindControl,
            Self::Loyalty => SpellCollege::MindControl,
            Self::Charm => SpellCollege::MindControl,
            Self::Enslave => SpellCollege::MindControl,
            Self::MindReading => SpellCollege::MindControl,
            Self::MindSending => SpellCollege::MindControl,
            Self::Telepathy => SpellCollege::MindControl,
            Self::Forgetfulness => SpellCollege::MindControl,
            Self::FalseMemory => SpellCollege::MindControl,
            Self::Suggestion => SpellCollege::MindControl,
            Self::CompelTruth => SpellCollege::MindControl,
            Self::HideThoughts => SpellCollege::MindControl,
            Self::ShieldMind => SpellCollege::MindControl,
            Self::Possession => SpellCollege::MindControl,
            Self::SoulJar => SpellCollege::MindControl,

            // Movement College
            Self::Levitation => SpellCollege::Movement,
            Self::Flight => SpellCollege::Movement,
            Self::Blink => SpellCollege::Movement,
            Self::Teleport => SpellCollege::Movement,
            Self::Teleportation => SpellCollege::Movement,
            Self::HasteMovement => SpellCollege::Movement,
            Self::SlowMovement => SpellCollege::Movement,
            Self::Cling => SpellCollege::Movement,
            Self::Jump => SpellCollege::Movement,
            Self::Apportation => SpellCollege::Movement,
            Self::Poltergeist => SpellCollege::Movement,
            Self::Lockmaster => SpellCollege::Movement,
            Self::TelekineticBlow => SpellCollege::Movement,
            Self::HaltMovement => SpellCollege::Movement,

            // Protection & Warning College
            Self::Shield => SpellCollege::ProtectionWarning,
            Self::Deflect => SpellCollege::ProtectionWarning,
            Self::Reflection => SpellCollege::ProtectionWarning,
            Self::Absorb => SpellCollege::ProtectionWarning,
            Self::Warn => SpellCollege::ProtectionWarning,
            Self::Sense => SpellCollege::ProtectionWarning,
            Self::ResistFireProtection => SpellCollege::ProtectionWarning,
            Self::ResistColdProtection => SpellCollege::ProtectionWarning,
            Self::ResistLightning => SpellCollege::ProtectionWarning,
            Self::ResistAcid => SpellCollege::ProtectionWarning,
            Self::ArmorEnchantment => SpellCollege::ProtectionWarning,
            Self::AnchorSpell => SpellCollege::ProtectionWarning,
            Self::WatchdogSpell => SpellCollege::ProtectionWarning,

            // Water College
            Self::PurifyWater => SpellCollege::Water,
            Self::CreateWater => SpellCollege::Water,
            Self::DestroyWater => SpellCollege::Water,
            Self::ShapeWater => SpellCollege::Water,
            Self::Freeze => SpellCollege::Water,
            Self::Boil => SpellCollege::Water,
            Self::WalkOnWater => SpellCollege::Water,
            Self::WaterJet => SpellCollege::Water,
            Self::IceSphere => SpellCollege::Water,
            Self::IceSlick => SpellCollege::Water,
            Self::BodyOfWater => SpellCollege::Water,
            Self::Dehydrate => SpellCollege::Water,
            Self::CreateIce => SpellCollege::Water,
            Self::CondenseSteam => SpellCollege::Water,
            Self::IceDagger => SpellCollege::Water,
            Self::Swim => SpellCollege::Water,
            Self::SummonWaterElemental => SpellCollege::Water,
            Self::BreatheWaterWater => SpellCollege::Water,

            // Knowledge College
            Self::DetectMagic => SpellCollege::Knowledge,
            Self::AnalyzeMagic => SpellCollege::Knowledge,
            Self::IdentifySpell => SpellCollege::Knowledge,
            Self::Seeker => SpellCollege::Knowledge,
            Self::SeekAir => SpellCollege::Knowledge,
            Self::SeekEarth => SpellCollege::Knowledge,
            Self::SeekFire => SpellCollege::Knowledge,
            Self::SeekWater => SpellCollege::Knowledge,
            Self::SeekPlant => SpellCollege::Knowledge,
            Self::SeekFood => SpellCollege::Knowledge,
            Self::SeekMachine => SpellCollege::Knowledge,
            Self::History => SpellCollege::Knowledge,
            Self::Wisdom => SpellCollege::Knowledge,
            Self::AncientHistory => SpellCollege::Knowledge,
            Self::RecoverMemory => SpellCollege::Knowledge,
            Self::Divination => SpellCollege::Knowledge,
            Self::Pathfinder => SpellCollege::Knowledge,
            Self::GlassWall => SpellCollege::Knowledge,
            Self::Trace => SpellCollege::Knowledge,
            Self::Aura => SpellCollege::Knowledge,
        };

        debug!(?college, "Spell college retrieved");
        college
    }

    /// Returns base energy cost to cast.
    ///
    /// # GURPS Rules
    ///
    /// Energy cost varies by spell and may scale with effect.
    /// Most Information spells cost 1-3 FP.
    ///
    /// # Citations
    ///
    /// BS 241 - Energy cost
    /// M 106-113 - Knowledge spell costs
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, EnergyCost};
    ///
    /// assert_eq!(Spell::DetectMagic.base_energy_cost(), EnergyCost::Fixed(2));
    /// assert_eq!(Spell::Seeker.base_energy_cost(), EnergyCost::Fixed(3));
    /// ```
    #[instrument]
    pub fn base_energy_cost(&self) -> EnergyCost {
        debug!("Getting base energy cost");
        
        // Delegate to college modules
        match self.college() {
            SpellCollege::Air => return air::base_energy_cost(self),
            SpellCollege::BodyControl => return body_control::base_energy_cost(self),
            SpellCollege::Earth => return earth::base_energy_cost(self),
            SpellCollege::Fire => return fire::base_energy_cost(self),
            SpellCollege::Healing => return healing::base_energy_cost(self),
            SpellCollege::IllusionCreation => return illusion_creation::base_energy_cost(self),
            SpellCollege::Knowledge => return knowledge::base_energy_cost(self),
            SpellCollege::MindControl => return mind_control::base_energy_cost(self),
            SpellCollege::Movement => return movement::base_energy_cost(self),
            SpellCollege::ProtectionWarning => return protection_warning::base_energy_cost(self),
            SpellCollege::Water => return water::base_energy_cost(self),
        }
    }

    /// Returns casting time in seconds.
    ///
    /// # GURPS Rules
    ///
    /// Most spells take 1-4 seconds to cast. Complex divinations
    /// may take longer.
    ///
    /// # Citations
    ///
    /// BS 241 - Casting time
    /// M 106-113 - Knowledge spell casting times
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Spell;
    ///
    /// assert_eq!(Spell::DetectMagic.casting_time(), 1);
    /// assert_eq!(Spell::Wisdom.casting_time(), 5);
    /// ```
    #[instrument]
    pub fn casting_time(&self) -> i32 {
        debug!("Getting casting time");
        
        // Delegate to college modules
        match self.college() {
            SpellCollege::Air => return air::casting_time(self),
            SpellCollege::BodyControl => return body_control::casting_time(self),
            SpellCollege::Earth => return earth::casting_time(self),
            SpellCollege::Fire => return fire::casting_time(self),
            SpellCollege::Healing => return healing::casting_time(self),
            SpellCollege::IllusionCreation => return illusion_creation::casting_time(self),
            SpellCollege::Knowledge => return knowledge::casting_time(self),
            SpellCollege::MindControl => return mind_control::casting_time(self),
            SpellCollege::Movement => return movement::casting_time(self),
            SpellCollege::ProtectionWarning => return protection_warning::casting_time(self),
            SpellCollege::Water => return water::casting_time(self),
        }
    }

    /// Returns spell duration.
    ///
    /// # GURPS Rules
    ///
    /// Duration varies: instant (information revealed), concentration
    /// (active maintenance), or timed (minutes/hours).
    ///
    /// # Citations
    ///
    /// BS 241 - Duration
    /// M 106-113 - Knowledge spell durations
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, Duration};
    ///
    /// assert_eq!(Spell::DetectMagic.duration(), Duration::Concentration);
    /// assert_eq!(Spell::History.duration(), Duration::Instant);
    /// ```
    #[instrument]
    pub fn duration(&self) -> Duration {
        debug!("Getting spell duration");
        
        // Delegate to college modules
        match self.college() {
            SpellCollege::Air => return air::duration(self),
            SpellCollege::BodyControl => return body_control::duration(self),
            SpellCollege::Earth => return earth::duration(self),
            SpellCollege::Fire => return fire::duration(self),
            SpellCollege::Healing => return healing::duration(self),
            SpellCollege::IllusionCreation => return illusion_creation::duration(self),
            SpellCollege::Knowledge => return knowledge::duration(self),
            SpellCollege::MindControl => return mind_control::duration(self),
            SpellCollege::Movement => return movement::duration(self),
            SpellCollege::ProtectionWarning => return protection_warning::duration(self),
            SpellCollege::Water => return water::duration(self),
        }
    }

    /// Returns spell prerequisites.
    ///
    /// # GURPS Rules
    ///
    /// Prerequisites include Magery level and other spells that must
    /// be known first.
    ///
    /// # Citations
    ///
    /// BS 241 - Prerequisites
    /// M 106-113 - Knowledge spell prerequisites
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, SpellPrerequisite};
    ///
    /// let prereqs = Spell::DetectMagic.prerequisites();
    /// assert_eq!(prereqs.len(), 1);
    /// ```
    #[instrument]
    pub fn prerequisites(&self) -> Vec<SpellPrerequisite> {
        debug!("Getting spell prerequisites");
        
        // Delegate to college modules
        match self.college() {
            SpellCollege::Air => return air::prerequisites(self),
            SpellCollege::BodyControl => return body_control::prerequisites(self),
            SpellCollege::Earth => return earth::prerequisites(self),
            SpellCollege::Fire => return fire::prerequisites(self),
            SpellCollege::Healing => return healing::prerequisites(self),
            SpellCollege::IllusionCreation => return illusion_creation::prerequisites(self),
            SpellCollege::Knowledge => return knowledge::prerequisites(self),
            SpellCollege::MindControl => return mind_control::prerequisites(self),
            SpellCollege::Movement => return movement::prerequisites(self),
            SpellCollege::ProtectionWarning => return protection_warning::prerequisites(self),
            SpellCollege::Water => return water::prerequisites(self),
        }
    }

    /// Returns spell type (for casting mechanics).
    ///
    /// # GURPS Rules
    ///
    /// Knowledge spells are mostly Information type - they reveal
    /// data rather than affecting targets directly.
    ///
    /// # Citations
    ///
    /// BS 239 - Spell types
    /// M 106 - Information spells
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, SpellType};
    ///
    /// assert_eq!(Spell::DetectMagic.spell_type(), SpellType::Information);
    /// assert_eq!(Spell::GlassWall.spell_type(), SpellType::Area);
    /// ```
    #[instrument]
    pub fn spell_type(&self) -> SpellType {
        debug!("Getting spell type");
        
        // Delegate to college modules
        match self.college() {
            SpellCollege::Air => return air::spell_type(self),
            SpellCollege::BodyControl => return body_control::spell_type(self),
            SpellCollege::Earth => return earth::spell_type(self),
            SpellCollege::Fire => return fire::spell_type(self),
            SpellCollege::Healing => return healing::spell_type(self),
            SpellCollege::IllusionCreation => return illusion_creation::spell_type(self),
            SpellCollege::Knowledge => return knowledge::spell_type(self),
            SpellCollege::MindControl => return mind_control::spell_type(self),
            SpellCollege::Movement => return movement::spell_type(self),
            SpellCollege::ProtectionWarning => return protection_warning::spell_type(self),
            SpellCollege::Water => return water::spell_type(self),
        }
    }

    /// Returns resistance type if applicable.
    ///
    /// # GURPS Rules
    ///
    /// Most Knowledge spells don't allow resistance as they
    /// don't directly affect targets. Some may allow IQ resistance.
    ///
    /// # Citations
    ///
    /// BS 241 - Resistance
    /// M 106-113 - Knowledge spell resistance
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Spell;
    ///
    /// assert_eq!(Spell::DetectMagic.resistance(), None);
    /// assert_eq!(Spell::Aura.resistance(), None);
    /// ```
    #[instrument]
    pub fn resistance(&self) -> Option<ResistanceType> {
        debug!("Getting resistance type");
        
        // Delegate to college modules
        match self.college() {
            SpellCollege::Air => return air::resistance(self),
            SpellCollege::BodyControl => return body_control::resistance(self),
            SpellCollege::Earth => return earth::resistance(self),
            SpellCollege::Fire => return fire::resistance(self),
            SpellCollege::Healing => return healing::resistance(self),
            SpellCollege::IllusionCreation => return illusion_creation::resistance(self),
            SpellCollege::Knowledge => return knowledge::resistance(self),
            SpellCollege::MindControl => return mind_control::resistance(self),
            SpellCollege::Movement => return movement::resistance(self),
            SpellCollege::ProtectionWarning => return protection_warning::resistance(self),
            SpellCollege::Water => return water::resistance(self),
        }
    }

    /// Returns spell citation reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Spell;
    ///
    /// assert_eq!(Spell::DetectMagic.reference(), "M107");
    /// assert_eq!(Spell::Seeker.reference(), "M112");
    /// ```
    #[instrument]
    pub fn reference(&self) -> &'static str {
        debug!("Getting spell reference");
        
        // Delegate to college modules
        match self.college() {
            SpellCollege::Air => return air::reference(self),
            SpellCollege::BodyControl => return body_control::reference(self),
            SpellCollege::Earth => return earth::reference(self),
            SpellCollege::Fire => return fire::reference(self),
            SpellCollege::Healing => return healing::reference(self),
            SpellCollege::IllusionCreation => return illusion_creation::reference(self),
            SpellCollege::Knowledge => return knowledge::reference(self),
            SpellCollege::MindControl => return mind_control::reference(self),
            SpellCollege::Movement => return movement::reference(self),
            SpellCollege::ProtectionWarning => return protection_warning::reference(self),
            SpellCollege::Water => return water::reference(self),
        }
    }
}

