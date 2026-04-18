//! GURPS magic spells.
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

        let cost = match self {
            // Air College
            Self::PurifyAirAir => EnergyCost::Fixed(1),
            Self::CreateAir => EnergyCost::Fixed(2),
            Self::DestroyAir => EnergyCost::Fixed(2),
            Self::ShapeAir => EnergyCost::Fixed(2),
            Self::NoSmell => EnergyCost::Fixed(1),
            Self::Stench => EnergyCost::Fixed(2),
            Self::WalkOnAir => EnergyCost::Fixed(3),
            Self::Lightning => EnergyCost::PerDie(1),
            Self::Windstorm => EnergyCost::Fixed(4),
            Self::AirJet => EnergyCost::PerDie(1),
            Self::BreatheWater => EnergyCost::Fixed(3),
            Self::BodyOfAir => EnergyCost::Fixed(5),
            Self::Concussion => EnergyCost::Fixed(2),
            Self::Sound => EnergyCost::Fixed(1),
            Self::Silence => EnergyCost::Fixed(2),
            Self::Thunderclap => EnergyCost::Fixed(3),
            Self::AirVortex => EnergyCost::Fixed(3),
            Self::EssentialAir => EnergyCost::Fixed(3),

            // Body Control College
            Self::Itch => EnergyCost::Fixed(2),
            Self::Spasm => EnergyCost::Fixed(3),
            Self::Pain => EnergyCost::Fixed(3),
            Self::Paralyze => EnergyCost::Fixed(5),
            Self::RootedFeet => EnergyCost::Fixed(3),
            Self::StrikeBlind => EnergyCost::Fixed(4),
            Self::StrikeDeaf => EnergyCost::Fixed(4),
            Self::StrikeDumb => EnergyCost::Fixed(4),
            Self::Haste => EnergyCost::Fixed(2),
            Self::Slow => EnergyCost::Fixed(3),
            Self::Strengthen => EnergyCost::Fixed(2),
            Self::Weaken => EnergyCost::Fixed(2),
            Self::Grace => EnergyCost::Fixed(2),
            Self::Clumsiness => EnergyCost::Fixed(2),
            Self::ResistPain => EnergyCost::Fixed(2),
            Self::ResistDisease => EnergyCost::Fixed(3),
            Self::WitherLimb => EnergyCost::Fixed(8),
            Self::Deathtouch => EnergyCost::PerDie(1),
            Self::BodyOfStone => EnergyCost::Fixed(5),
            Self::AlterBody => EnergyCost::Fixed(10),
            Self::ShapeFlesh => EnergyCost::Fixed(6),

            // Earth College
            Self::ShapeEarth => EnergyCost::Fixed(2),
            Self::EarthToStone => EnergyCost::Fixed(3),
            Self::StoneToEarth => EnergyCost::Fixed(3),
            Self::CreateEarth => EnergyCost::Fixed(3),
            Self::DestroyEarth => EnergyCost::Fixed(3),
            Self::EarthVision => EnergyCost::Fixed(2),
            Self::WalkThroughEarth => EnergyCost::Fixed(4),
            Self::Earthquake => EnergyCost::Fixed(6),
            Self::ShapeStone => EnergyCost::Fixed(2),
            Self::EarthToAir => EnergyCost::Fixed(5),
            Self::StoneMissile => EnergyCost::PerDie(1),
            Self::CreateStone => EnergyCost::Fixed(4),
            Self::SandJet => EnergyCost::PerDie(1),
            Self::FleshToStone => EnergyCost::Fixed(10),
            Self::StoneToFlesh => EnergyCost::Fixed(10),
            Self::IronArm => EnergyCost::Fixed(3),
            Self::Entombment => EnergyCost::Fixed(5),
            Self::SummonEarthElemental => EnergyCost::Fixed(10),

            // Fire College
            Self::IgniteFlame => EnergyCost::Fixed(1),
            Self::CreateFire => EnergyCost::Fixed(4),
            Self::ExtinguishFire => EnergyCost::Fixed(3),
            Self::ShapeFire => EnergyCost::Fixed(2),
            Self::Fireball => EnergyCost::PerDie(1),
            Self::ExplosiveFireball => EnergyCost::PerDie(2),
            Self::FlameJet => EnergyCost::PerDie(1),
            Self::Heat => EnergyCost::Fixed(2),
            Self::Cold => EnergyCost::Fixed(2),
            Self::ResistFire => EnergyCost::Fixed(2),
            Self::ResistCold => EnergyCost::Fixed(2),
            Self::PurifyAir => EnergyCost::Fixed(1),
            Self::Frostbite => EnergyCost::PerDie(1),
            Self::WallOfFire => EnergyCost::Fixed(3),
            Self::SummonFireElemental => EnergyCost::Fixed(8),
            Self::PermanentFlame => EnergyCost::Fixed(10),
            Self::SeekFireFire => EnergyCost::Fixed(2),
            Self::Ignition => EnergyCost::Fixed(3),

            // Healing College
            Self::LendEnergy => EnergyCost::PerFP(1),
            Self::LendVitality => EnergyCost::PerHP(1),
            Self::RecoverEnergy => EnergyCost::PerFP(2),
            Self::MinorHealing => EnergyCost::Fixed(1),
            Self::MajorHealing => EnergyCost::PerHP(2),
            Self::GreatHealing => EnergyCost::PerHP(4),
            Self::Awaken => EnergyCost::Fixed(1),
            Self::CureDisease => EnergyCost::Fixed(4),
            Self::NeutralizePoison => EnergyCost::Fixed(3),
            Self::InstantNeutralizePoison => EnergyCost::Fixed(6),
            Self::StopBleeding => EnergyCost::Fixed(2),
            Self::Regeneration => EnergyCost::PerHP(1),
            Self::Restoration => EnergyCost::Fixed(20),
            Self::SuspendCurse => EnergyCost::Fixed(3),
            Self::RemoveCurse => EnergyCost::Fixed(10),
            Self::CurseRemoval => EnergyCost::Fixed(5),
            Self::Healing => EnergyCost::PerHP(2),
            Self::RestoreYouth => EnergyCost::Fixed(100),

            // Illusion & Creation College
            Self::SimpleIllusion => EnergyCost::Fixed(1),
            Self::PerfectIllusion => EnergyCost::Fixed(3),
            Self::ComplexIllusion => EnergyCost::Fixed(2),
            Self::MakeRealIllusion => EnergyCost::Fixed(10),
            Self::Create => EnergyCost::Fixed(4),
            Self::PermanentCreation => EnergyCost::Fixed(8),
            Self::Invisibility => EnergyCost::Fixed(3),
            Self::InvisibilityToAll => EnergyCost::Fixed(5),
            Self::Disguise => EnergyCost::Fixed(2),
            Self::ShapeShifting => EnergyCost::Fixed(6),
            Self::Duplicate => EnergyCost::Fixed(4),
            Self::Phantasm => EnergyCost::Fixed(3),
            Self::SilentImage => EnergyCost::Fixed(1),
            Self::Light => EnergyCost::Fixed(1),
            Self::Darkness => EnergyCost::Fixed(2),
            Self::HideAura => EnergyCost::Fixed(2),
            Self::Mirror => EnergyCost::Fixed(4),

            // Mind Control College
            Self::Daze => EnergyCost::Fixed(2),
            Self::Sleep => EnergyCost::Fixed(4),
            Self::Command => EnergyCost::Fixed(2),
            Self::MassDaze => EnergyCost::Fixed(3),
            Self::Fear => EnergyCost::Fixed(3),
            Self::Terror => EnergyCost::Fixed(6),
            Self::Loyalty => EnergyCost::Fixed(10),
            Self::Charm => EnergyCost::Fixed(3),
            Self::Enslave => EnergyCost::Fixed(20),
            Self::MindReading => EnergyCost::Fixed(4),
            Self::MindSending => EnergyCost::Fixed(2),
            Self::Telepathy => EnergyCost::Fixed(4),
            Self::Forgetfulness => EnergyCost::Fixed(3),
            Self::FalseMemory => EnergyCost::Fixed(5),
            Self::Suggestion => EnergyCost::Fixed(3),
            Self::CompelTruth => EnergyCost::Fixed(2),
            Self::HideThoughts => EnergyCost::Fixed(3),
            Self::ShieldMind => EnergyCost::Fixed(4),
            Self::Possession => EnergyCost::Fixed(10),
            Self::SoulJar => EnergyCost::Fixed(20),

            // Movement College
            Self::Levitation => EnergyCost::Fixed(2),
            Self::Flight => EnergyCost::Fixed(4),
            Self::Blink => EnergyCost::Fixed(2),
            Self::Teleport => EnergyCost::Fixed(5),
            Self::Teleportation => EnergyCost::Fixed(8),
            Self::HasteMovement => EnergyCost::Fixed(3),
            Self::SlowMovement => EnergyCost::Fixed(3),
            Self::Cling => EnergyCost::Fixed(2),
            Self::Jump => EnergyCost::Fixed(2),
            Self::Apportation => EnergyCost::Fixed(3),
            Self::Poltergeist => EnergyCost::Fixed(1),
            Self::Lockmaster => EnergyCost::Fixed(2),
            Self::TelekineticBlow => EnergyCost::PerDie(1),
            Self::HaltMovement => EnergyCost::Fixed(3),

            // Protection & Warning College
            Self::Shield => EnergyCost::Fixed(2),
            Self::Deflect => EnergyCost::Fixed(2),
            Self::Reflection => EnergyCost::Fixed(3),
            Self::Absorb => EnergyCost::Fixed(3),
            Self::Warn => EnergyCost::Fixed(2),
            Self::Sense => EnergyCost::Fixed(1),
            Self::ResistFireProtection => EnergyCost::Fixed(3),
            Self::ResistColdProtection => EnergyCost::Fixed(3),
            Self::ResistLightning => EnergyCost::Fixed(3),
            Self::ResistAcid => EnergyCost::Fixed(3),
            Self::ArmorEnchantment => EnergyCost::Fixed(4),
            Self::AnchorSpell => EnergyCost::Fixed(3),
            Self::WatchdogSpell => EnergyCost::Fixed(2),

            // Water College
            Self::PurifyWater => EnergyCost::Fixed(1),
            Self::CreateWater => EnergyCost::Fixed(2),
            Self::DestroyWater => EnergyCost::Fixed(2),
            Self::ShapeWater => EnergyCost::Fixed(2),
            Self::Freeze => EnergyCost::Fixed(3),
            Self::Boil => EnergyCost::Fixed(2),
            Self::WalkOnWater => EnergyCost::Fixed(3),
            Self::WaterJet => EnergyCost::PerDie(1),
            Self::IceSphere => EnergyCost::PerDie(1),
            Self::IceSlick => EnergyCost::Fixed(2),
            Self::BodyOfWater => EnergyCost::Fixed(5),
            Self::Dehydrate => EnergyCost::Fixed(3),
            Self::CreateIce => EnergyCost::Fixed(3),
            Self::CondenseSteam => EnergyCost::Fixed(2),
            Self::IceDagger => EnergyCost::Fixed(2),
            Self::Swim => EnergyCost::Fixed(2),
            Self::SummonWaterElemental => EnergyCost::Fixed(10),
            Self::BreatheWaterWater => EnergyCost::Fixed(3),

            // Knowledge College
            Self::DetectMagic => EnergyCost::Fixed(2),
            Self::AnalyzeMagic => EnergyCost::Fixed(8),
            Self::IdentifySpell => EnergyCost::Fixed(2),
            Self::Seeker => EnergyCost::Fixed(3),
            Self::SeekAir => EnergyCost::Fixed(1),
            Self::SeekEarth => EnergyCost::Fixed(1),
            Self::SeekFire => EnergyCost::Fixed(1),
            Self::SeekWater => EnergyCost::Fixed(1),
            Self::SeekPlant => EnergyCost::Fixed(1),
            Self::SeekFood => EnergyCost::Fixed(1),
            Self::SeekMachine => EnergyCost::Fixed(1),
            Self::History => EnergyCost::Fixed(4),
            Self::Wisdom => EnergyCost::Fixed(8),
            Self::AncientHistory => EnergyCost::Fixed(5),
            Self::RecoverMemory => EnergyCost::Fixed(3),
            Self::Divination => EnergyCost::Fixed(4),
            Self::Pathfinder => EnergyCost::Fixed(3),
            Self::GlassWall => EnergyCost::Fixed(2),
            Self::Trace => EnergyCost::Fixed(2),
            Self::Aura => EnergyCost::Fixed(2),
        };

        debug!(?cost, "Energy cost retrieved");
        cost
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

        let time = match self {
            // Air College
            Self::PurifyAirAir => 1,
            Self::CreateAir => 1,
            Self::DestroyAir => 1,
            Self::ShapeAir => 1,
            Self::NoSmell => 1,
            Self::Stench => 1,
            Self::WalkOnAir => 2,
            Self::Lightning => 1,
            Self::Windstorm => 2,
            Self::AirJet => 1,
            Self::BreatheWater => 2,
            Self::BodyOfAir => 3,
            Self::Concussion => 1,
            Self::Sound => 1,
            Self::Silence => 1,
            Self::Thunderclap => 1,
            Self::AirVortex => 2,
            Self::EssentialAir => 2,

            // Body Control College
            Self::Itch => 1,
            Self::Spasm => 1,
            Self::Pain => 1,
            Self::Paralyze => 2,
            Self::RootedFeet => 1,
            Self::StrikeBlind => 1,
            Self::StrikeDeaf => 1,
            Self::StrikeDumb => 1,
            Self::Haste => 2,
            Self::Slow => 1,
            Self::Strengthen => 2,
            Self::Weaken => 1,
            Self::Grace => 2,
            Self::Clumsiness => 1,
            Self::ResistPain => 1,
            Self::ResistDisease => 1,
            Self::WitherLimb => 3,
            Self::Deathtouch => 1,
            Self::BodyOfStone => 3,
            Self::AlterBody => 5,
            Self::ShapeFlesh => 3,

            // Earth College
            Self::ShapeEarth => 1,
            Self::EarthToStone => 2,
            Self::StoneToEarth => 2,
            Self::CreateEarth => 1,
            Self::DestroyEarth => 1,
            Self::EarthVision => 1,
            Self::WalkThroughEarth => 2,
            Self::Earthquake => 3,
            Self::ShapeStone => 2,
            Self::EarthToAir => 3,
            Self::StoneMissile => 1,
            Self::CreateStone => 2,
            Self::SandJet => 1,
            Self::FleshToStone => 5,
            Self::StoneToFlesh => 5,
            Self::IronArm => 2,
            Self::Entombment => 2,
            Self::SummonEarthElemental => 5,

            // Fire College
            Self::IgniteFlame => 1,
            Self::CreateFire => 1,
            Self::ExtinguishFire => 1,
            Self::ShapeFire => 1,
            Self::Fireball => 1,
            Self::ExplosiveFireball => 2,
            Self::FlameJet => 1,
            Self::Heat => 1,
            Self::Cold => 1,
            Self::ResistFire => 1,
            Self::ResistCold => 1,
            Self::PurifyAir => 1,
            Self::Frostbite => 1,
            Self::WallOfFire => 2,
            Self::SummonFireElemental => 5,
            Self::PermanentFlame => 10,
            Self::SeekFireFire => 1,
            Self::Ignition => 1,

            // Healing College
            Self::LendEnergy => 1,
            Self::LendVitality => 1,
            Self::RecoverEnergy => 1,
            Self::MinorHealing => 1,
            Self::MajorHealing => 2,
            Self::GreatHealing => 1,
            Self::Awaken => 1,
            Self::CureDisease => 1,
            Self::NeutralizePoison => 1,
            Self::InstantNeutralizePoison => 1,
            Self::StopBleeding => 1,
            Self::Regeneration => 2,
            Self::Restoration => 10,
            Self::SuspendCurse => 1,
            Self::RemoveCurse => 5,
            Self::CurseRemoval => 2,
            Self::Healing => 2,
            Self::RestoreYouth => 60,

            // Illusion & Creation College
            Self::SimpleIllusion => 1,
            Self::PerfectIllusion => 2,
            Self::ComplexIllusion => 2,
            Self::MakeRealIllusion => 5,
            Self::Create => 2,
            Self::PermanentCreation => 5,
            Self::Invisibility => 2,
            Self::InvisibilityToAll => 3,
            Self::Disguise => 2,
            Self::ShapeShifting => 3,
            Self::Duplicate => 3,
            Self::Phantasm => 2,
            Self::SilentImage => 1,
            Self::Light => 1,
            Self::Darkness => 1,
            Self::HideAura => 1,
            Self::Mirror => 2,

            // Mind Control College
            Self::Daze => 1,
            Self::Sleep => 2,
            Self::Command => 1,
            Self::MassDaze => 2,
            Self::Fear => 1,
            Self::Terror => 2,
            Self::Loyalty => 5,
            Self::Charm => 2,
            Self::Enslave => 10,
            Self::MindReading => 2,
            Self::MindSending => 1,
            Self::Telepathy => 2,
            Self::Forgetfulness => 2,
            Self::FalseMemory => 3,
            Self::Suggestion => 2,
            Self::CompelTruth => 1,
            Self::HideThoughts => 1,
            Self::ShieldMind => 1,
            Self::Possession => 5,
            Self::SoulJar => 10,

            // Movement College
            Self::Levitation => 2,
            Self::Flight => 3,
            Self::Blink => 1,
            Self::Teleport => 2,
            Self::Teleportation => 5,
            Self::HasteMovement => 2,
            Self::SlowMovement => 2,
            Self::Cling => 2,
            Self::Jump => 1,
            Self::Apportation => 2,
            Self::Poltergeist => 1,
            Self::Lockmaster => 1,
            Self::TelekineticBlow => 1,
            Self::HaltMovement => 2,

            // Protection & Warning College
            Self::Shield => 1,
            Self::Deflect => 1,
            Self::Reflection => 2,
            Self::Absorb => 2,
            Self::Warn => 1,
            Self::Sense => 1,
            Self::ResistFireProtection => 2,
            Self::ResistColdProtection => 2,
            Self::ResistLightning => 2,
            Self::ResistAcid => 2,
            Self::ArmorEnchantment => 3,
            Self::AnchorSpell => 2,
            Self::WatchdogSpell => 2,

            // Water College
            Self::PurifyWater => 1,
            Self::CreateWater => 1,
            Self::DestroyWater => 1,
            Self::ShapeWater => 1,
            Self::Freeze => 2,
            Self::Boil => 1,
            Self::WalkOnWater => 2,
            Self::WaterJet => 1,
            Self::IceSphere => 1,
            Self::IceSlick => 1,
            Self::BodyOfWater => 3,
            Self::Dehydrate => 2,
            Self::CreateIce => 2,
            Self::CondenseSteam => 1,
            Self::IceDagger => 1,
            Self::Swim => 2,
            Self::SummonWaterElemental => 5,
            Self::BreatheWaterWater => 2,

            // Knowledge College
            Self::DetectMagic => 1,
            Self::AnalyzeMagic => 1,
            Self::IdentifySpell => 1,
            Self::Seeker => 2,
            Self::SeekAir => 1,
            Self::SeekEarth => 1,
            Self::SeekFire => 1,
            Self::SeekWater => 1,
            Self::SeekPlant => 1,
            Self::SeekFood => 1,
            Self::SeekMachine => 1,
            Self::History => 3,
            Self::Wisdom => 5,
            Self::AncientHistory => 4,
            Self::RecoverMemory => 2,
            Self::Divination => 5,
            Self::Pathfinder => 2,
            Self::GlassWall => 1,
            Self::Trace => 2,
            Self::Aura => 2,
        };

        debug!(time, "Casting time retrieved");
        time
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

        let duration = match self {
            // Air College
            Self::PurifyAirAir => Duration::Instant,
            Self::CreateAir => Duration::Concentration,
            Self::DestroyAir => Duration::Instant,
            Self::ShapeAir => Duration::Concentration,
            Self::NoSmell => Duration::Minutes(1),
            Self::Stench => Duration::Minutes(1),
            Self::WalkOnAir => Duration::Minutes(1),
            Self::Lightning => Duration::Instant,
            Self::Windstorm => Duration::Minutes(1),
            Self::AirJet => Duration::Instant,
            Self::BreatheWater => Duration::Minutes(10),
            Self::BodyOfAir => Duration::Minutes(1),
            Self::Concussion => Duration::Instant,
            Self::Sound => Duration::Minutes(1),
            Self::Silence => Duration::Minutes(1),
            Self::Thunderclap => Duration::Instant,
            Self::AirVortex => Duration::Minutes(1),
            Self::EssentialAir => Duration::Hours(1),

            // Body Control College
            Self::Itch => Duration::Minutes(1),
            Self::Spasm => Duration::Minutes(1),
            Self::Pain => Duration::Minutes(1),
            Self::Paralyze => Duration::Minutes(1),
            Self::RootedFeet => Duration::Minutes(1),
            Self::StrikeBlind => Duration::Minutes(1),
            Self::StrikeDeaf => Duration::Minutes(1),
            Self::StrikeDumb => Duration::Minutes(1),
            Self::Haste => Duration::Minutes(1),
            Self::Slow => Duration::Minutes(1),
            Self::Strengthen => Duration::Minutes(1),
            Self::Weaken => Duration::Minutes(1),
            Self::Grace => Duration::Minutes(1),
            Self::Clumsiness => Duration::Minutes(1),
            Self::ResistPain => Duration::Minutes(10),
            Self::ResistDisease => Duration::Hours(1),
            Self::WitherLimb => Duration::Permanent,
            Self::Deathtouch => Duration::Instant,
            Self::BodyOfStone => Duration::Minutes(1),
            Self::AlterBody => Duration::Permanent,
            Self::ShapeFlesh => Duration::Minutes(1),

            // Earth College
            Self::ShapeEarth => Duration::Concentration,
            Self::EarthToStone => Duration::Permanent,
            Self::StoneToEarth => Duration::Permanent,
            Self::CreateEarth => Duration::Permanent,
            Self::DestroyEarth => Duration::Instant,
            Self::EarthVision => Duration::Concentration,
            Self::WalkThroughEarth => Duration::Minutes(1),
            Self::Earthquake => Duration::Instant,
            Self::ShapeStone => Duration::Permanent,
            Self::EarthToAir => Duration::Permanent,
            Self::StoneMissile => Duration::Instant,
            Self::CreateStone => Duration::Permanent,
            Self::SandJet => Duration::Instant,
            Self::FleshToStone => Duration::Permanent,
            Self::StoneToFlesh => Duration::Permanent,
            Self::IronArm => Duration::Minutes(1),
            Self::Entombment => Duration::Minutes(1),
            Self::SummonEarthElemental => Duration::Minutes(1),

            // Fire College
            Self::IgniteFlame => Duration::Instant,
            Self::CreateFire => Duration::Concentration,
            Self::ExtinguishFire => Duration::Instant,
            Self::ShapeFire => Duration::Concentration,
            Self::Fireball => Duration::Instant,
            Self::ExplosiveFireball => Duration::Instant,
            Self::FlameJet => Duration::Instant,
            Self::Heat => Duration::Minutes(1),
            Self::Cold => Duration::Minutes(1),
            Self::ResistFire => Duration::Minutes(1),
            Self::ResistCold => Duration::Minutes(1),
            Self::PurifyAir => Duration::Instant,
            Self::Frostbite => Duration::Instant,
            Self::WallOfFire => Duration::Minutes(1),
            Self::SummonFireElemental => Duration::Minutes(1),
            Self::PermanentFlame => Duration::Permanent,
            Self::SeekFireFire => Duration::Concentration,
            Self::Ignition => Duration::Instant,

            // Healing College
            Self::LendEnergy => Duration::Concentration,
            Self::LendVitality => Duration::Concentration,
            Self::RecoverEnergy => Duration::Instant,
            Self::MinorHealing => Duration::Instant,
            Self::MajorHealing => Duration::Instant,
            Self::GreatHealing => Duration::Instant,
            Self::Awaken => Duration::Instant,
            Self::CureDisease => Duration::Instant,
            Self::NeutralizePoison => Duration::Instant,
            Self::InstantNeutralizePoison => Duration::Instant,
            Self::StopBleeding => Duration::Instant,
            Self::Regeneration => Duration::Minutes(1),
            Self::Restoration => Duration::Permanent,
            Self::SuspendCurse => Duration::Minutes(10),
            Self::RemoveCurse => Duration::Permanent,
            Self::CurseRemoval => Duration::Instant,
            Self::Healing => Duration::Instant,
            Self::RestoreYouth => Duration::Permanent,

            // Illusion & Creation College
            Self::SimpleIllusion => Duration::Concentration,
            Self::PerfectIllusion => Duration::Concentration,
            Self::ComplexIllusion => Duration::Concentration,
            Self::MakeRealIllusion => Duration::Permanent,
            Self::Create => Duration::Minutes(10),
            Self::PermanentCreation => Duration::Permanent,
            Self::Invisibility => Duration::Minutes(1),
            Self::InvisibilityToAll => Duration::Minutes(1),
            Self::Disguise => Duration::Minutes(10),
            Self::ShapeShifting => Duration::Minutes(1),
            Self::Duplicate => Duration::Minutes(1),
            Self::Phantasm => Duration::Concentration,
            Self::SilentImage => Duration::Concentration,
            Self::Light => Duration::Minutes(1),
            Self::Darkness => Duration::Minutes(1),
            Self::HideAura => Duration::Hours(1),
            Self::Mirror => Duration::Minutes(1),

            // Mind Control College
            Self::Daze => Duration::Minutes(1),
            Self::Sleep => Duration::Minutes(1),
            Self::Command => Duration::Minutes(1),
            Self::MassDaze => Duration::Minutes(1),
            Self::Fear => Duration::Minutes(1),
            Self::Terror => Duration::Minutes(10),
            Self::Loyalty => Duration::Hours(1),
            Self::Charm => Duration::Hours(1),
            Self::Enslave => Duration::Permanent,
            Self::MindReading => Duration::Concentration,
            Self::MindSending => Duration::Instant,
            Self::Telepathy => Duration::Concentration,
            Self::Forgetfulness => Duration::Permanent,
            Self::FalseMemory => Duration::Permanent,
            Self::Suggestion => Duration::Permanent,
            Self::CompelTruth => Duration::Minutes(1),
            Self::HideThoughts => Duration::Minutes(10),
            Self::ShieldMind => Duration::Minutes(1),
            Self::Possession => Duration::Concentration,
            Self::SoulJar => Duration::Permanent,

            // Movement College
            Self::Levitation => Duration::Minutes(1),
            Self::Flight => Duration::Minutes(1),
            Self::Blink => Duration::Instant,
            Self::Teleport => Duration::Instant,
            Self::Teleportation => Duration::Minutes(1),
            Self::HasteMovement => Duration::Minutes(1),
            Self::SlowMovement => Duration::Minutes(1),
            Self::Cling => Duration::Minutes(1),
            Self::Jump => Duration::Instant,
            Self::Apportation => Duration::Instant,
            Self::Poltergeist => Duration::Concentration,
            Self::Lockmaster => Duration::Permanent,
            Self::TelekineticBlow => Duration::Instant,
            Self::HaltMovement => Duration::Minutes(1),

            // Protection & Warning College
            Self::Shield => Duration::Minutes(1),
            Self::Deflect => Duration::Minutes(1),
            Self::Reflection => Duration::Minutes(1),
            Self::Absorb => Duration::Minutes(1),
            Self::Warn => Duration::Hours(1),
            Self::Sense => Duration::Concentration,
            Self::ResistFireProtection => Duration::Minutes(10),
            Self::ResistColdProtection => Duration::Minutes(10),
            Self::ResistLightning => Duration::Minutes(10),
            Self::ResistAcid => Duration::Minutes(10),
            Self::ArmorEnchantment => Duration::Permanent,
            Self::AnchorSpell => Duration::Hours(1),
            Self::WatchdogSpell => Duration::Hours(8),

            // Water College
            Self::PurifyWater => Duration::Instant,
            Self::CreateWater => Duration::Permanent,
            Self::DestroyWater => Duration::Instant,
            Self::ShapeWater => Duration::Concentration,
            Self::Freeze => Duration::Permanent,
            Self::Boil => Duration::Instant,
            Self::WalkOnWater => Duration::Minutes(1),
            Self::WaterJet => Duration::Instant,
            Self::IceSphere => Duration::Instant,
            Self::IceSlick => Duration::Permanent,
            Self::BodyOfWater => Duration::Minutes(1),
            Self::Dehydrate => Duration::Instant,
            Self::CreateIce => Duration::Permanent,
            Self::CondenseSteam => Duration::Permanent,
            Self::IceDagger => Duration::Minutes(1),
            Self::Swim => Duration::Minutes(10),
            Self::SummonWaterElemental => Duration::Minutes(1),
            Self::BreatheWaterWater => Duration::Minutes(10),

            // Knowledge College
            Self::DetectMagic => Duration::Concentration,
            Self::AnalyzeMagic => Duration::Instant,
            Self::IdentifySpell => Duration::Instant,
            Self::Seeker => Duration::Instant,
            Self::SeekAir => Duration::Concentration,
            Self::SeekEarth => Duration::Concentration,
            Self::SeekFire => Duration::Concentration,
            Self::SeekWater => Duration::Concentration,
            Self::SeekPlant => Duration::Concentration,
            Self::SeekFood => Duration::Concentration,
            Self::SeekMachine => Duration::Concentration,
            Self::History => Duration::Instant,
            Self::Wisdom => Duration::Instant,
            Self::AncientHistory => Duration::Instant,
            Self::RecoverMemory => Duration::Instant,
            Self::Divination => Duration::Instant,
            Self::Pathfinder => Duration::Instant,
            Self::GlassWall => Duration::Minutes(1),
            Self::Trace => Duration::Concentration,
            Self::Aura => Duration::Instant,
        };

        debug!(?duration, "Duration retrieved");
        duration
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

        let prereqs = match self {
            // Air College
            Self::PurifyAirAir => vec![SpellPrerequisite::Magery(0)],
            Self::CreateAir => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::PurifyAirAir),
            ],
            Self::DestroyAir => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::CreateAir),
            ],
            Self::ShapeAir => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::CreateAir),
            ],
            Self::NoSmell => vec![SpellPrerequisite::Magery(0)],
            Self::Stench => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::NoSmell),
            ],
            Self::WalkOnAir => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
            ],
            Self::Lightning => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
            ],
            Self::Windstorm => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeAir),
            ],
            Self::AirJet => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeAir),
            ],
            Self::BreatheWater => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
            ],
            Self::BodyOfAir => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 6),
            ],
            Self::Concussion => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
            ],
            Self::Sound => vec![SpellPrerequisite::Magery(0)],
            Self::Silence => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Sound),
            ],
            Self::Thunderclap => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Sound),
            ],
            Self::AirVortex => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeAir),
            ],
            Self::EssentialAir => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 6),
            ],

            // Body Control College
            Self::Itch => vec![SpellPrerequisite::Magery(0)],
            Self::Spasm => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Itch),
            ],
            Self::Pain => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Spasm),
            ],
            Self::Paralyze => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Pain),
            ],
            Self::RootedFeet => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
            ],
            Self::StrikeBlind => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
            ],
            Self::StrikeDeaf => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
            ],
            Self::StrikeDumb => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
            ],
            Self::Haste => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
            ],
            Self::Slow => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Haste),
            ],
            Self::Strengthen => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
            ],
            Self::Weaken => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Strengthen),
            ],
            Self::Grace => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
            ],
            Self::Clumsiness => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Grace),
            ],
            Self::ResistPain => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Pain),
            ],
            Self::ResistDisease => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
            ],
            Self::WitherLimb => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 8),
            ],
            Self::Deathtouch => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 10),
            ],
            Self::BodyOfStone => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
            ],
            Self::AlterBody => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 10),
            ],
            Self::ShapeFlesh => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 8),
            ],

            // Earth College
            Self::ShapeEarth => vec![SpellPrerequisite::Magery(0)],
            Self::EarthToStone => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::ShapeEarth),
            ],
            Self::StoneToEarth => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::EarthToStone),
            ],
            Self::CreateEarth => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::ShapeEarth),
            ],
            Self::DestroyEarth => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::CreateEarth),
            ],
            Self::EarthVision => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::ShapeEarth),
            ],
            Self::WalkThroughEarth => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 4),
            ],
            Self::Earthquake => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 8),
            ],
            Self::ShapeStone => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::EarthToStone),
            ],
            Self::EarthToAir => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 6),
            ],
            Self::StoneMissile => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeStone),
            ],
            Self::CreateStone => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::EarthToStone),
            ],
            Self::SandJet => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeEarth),
            ],
            Self::FleshToStone => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 6),
            ],
            Self::StoneToFlesh => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::FleshToStone),
            ],
            Self::IronArm => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 4),
            ],
            Self::Entombment => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 4),
            ],
            Self::SummonEarthElemental => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 8),
            ],

            // Fire College
            Self::IgniteFlame => vec![SpellPrerequisite::Magery(0)],
            Self::CreateFire => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::IgniteFlame),
            ],
            Self::ExtinguishFire => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::IgniteFlame),
            ],
            Self::ShapeFire => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::IgniteFlame),
            ],
            Self::Fireball => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::CreateFire),
            ],
            Self::ExplosiveFireball => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Fireball),
            ],
            Self::FlameJet => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeFire),
            ],
            Self::Heat => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::IgniteFlame),
            ],
            Self::Cold => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Heat),
            ],
            Self::ResistFire => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Heat),
            ],
            Self::ResistCold => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Cold),
            ],
            Self::PurifyAir => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::CreateFire),
            ],
            Self::Frostbite => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Cold),
            ],
            Self::WallOfFire => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeFire),
            ],
            Self::SummonFireElemental => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Fire, 8),
            ],
            Self::PermanentFlame => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Fire, 4),
            ],
            Self::SeekFireFire => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::IgniteFlame),
            ],
            Self::Ignition => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Heat),
            ],

            // Healing College
            Self::LendEnergy => vec![SpellPrerequisite::Magery(0)],
            Self::LendVitality => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::LendEnergy),
            ],
            Self::RecoverEnergy => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::LendEnergy),
            ],
            Self::MinorHealing => vec![SpellPrerequisite::Magery(0)],
            Self::MajorHealing => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::MinorHealing),
            ],
            Self::GreatHealing => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::MajorHealing),
            ],
            Self::Awaken => vec![SpellPrerequisite::Magery(0)],
            Self::CureDisease => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 4),
            ],
            Self::NeutralizePoison => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::MinorHealing),
            ],
            Self::InstantNeutralizePoison => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::NeutralizePoison),
            ],
            Self::StopBleeding => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::MinorHealing),
            ],
            Self::Regeneration => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::MajorHealing),
            ],
            Self::Restoration => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 8),
            ],
            Self::SuspendCurse => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 4),
            ],
            Self::RemoveCurse => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::SuspendCurse),
            ],
            Self::CurseRemoval => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 4),
            ],
            Self::Healing => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::MajorHealing),
            ],
            Self::RestoreYouth => vec![
                SpellPrerequisite::Magery(3),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 12),
            ],

            // Illusion & Creation College
            Self::SimpleIllusion => vec![SpellPrerequisite::Magery(0)],
            Self::PerfectIllusion => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::SimpleIllusion),
            ],
            Self::ComplexIllusion => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::PerfectIllusion),
            ],
            Self::MakeRealIllusion => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::ComplexIllusion),
            ],
            Self::Create => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 4),
            ],
            Self::PermanentCreation => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Create),
            ],
            Self::Invisibility => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 4),
            ],
            Self::InvisibilityToAll => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Invisibility),
            ],
            Self::Disguise => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::SimpleIllusion),
            ],
            Self::ShapeShifting => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 6),
            ],
            Self::Duplicate => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 6),
            ],
            Self::Phantasm => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ComplexIllusion),
            ],
            Self::SilentImage => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::SimpleIllusion),
            ],
            Self::Light => vec![SpellPrerequisite::Magery(0)],
            Self::Darkness => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Light),
            ],
            Self::HideAura => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 4),
            ],
            Self::Mirror => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 8),
            ],

            // Mind Control College
            Self::Daze => vec![SpellPrerequisite::Magery(0)],
            Self::Sleep => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Daze),
            ],
            Self::Command => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Daze),
            ],
            Self::MassDaze => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Daze),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
            ],
            Self::Fear => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Daze),
            ],
            Self::Terror => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Fear),
            ],
            Self::Loyalty => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 8),
            ],
            Self::Charm => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
            ],
            Self::Enslave => vec![
                SpellPrerequisite::Magery(3),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 12),
            ],
            Self::MindReading => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
            ],
            Self::MindSending => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::MindReading),
            ],
            Self::Telepathy => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::MindReading),
                SpellPrerequisite::Spell(Self::MindSending),
            ],
            Self::Forgetfulness => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
            ],
            Self::FalseMemory => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Forgetfulness),
            ],
            Self::Suggestion => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 6),
            ],
            Self::CompelTruth => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
            ],
            Self::HideThoughts => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::MindReading),
            ],
            Self::ShieldMind => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 6),
            ],
            Self::Possession => vec![
                SpellPrerequisite::Magery(3),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 10),
            ],
            Self::SoulJar => vec![
                SpellPrerequisite::Magery(3),
                SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 12),
            ],

            // Movement College
            Self::Levitation => vec![SpellPrerequisite::Magery(0)],
            Self::Flight => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Levitation),
            ],
            Self::Blink => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Movement, 4),
            ],
            Self::Teleport => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Blink),
            ],
            Self::Teleportation => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Teleport),
            ],
            Self::HasteMovement => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Movement, 4),
            ],
            Self::SlowMovement => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::HasteMovement),
            ],
            Self::Cling => vec![SpellPrerequisite::Magery(0)],
            Self::Jump => vec![SpellPrerequisite::Magery(0)],
            Self::Apportation => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Movement, 4),
            ],
            Self::Poltergeist => vec![SpellPrerequisite::Magery(0)],
            Self::Lockmaster => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Poltergeist),
            ],
            Self::TelekineticBlow => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Poltergeist),
            ],
            Self::HaltMovement => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Movement, 4),
            ],

            // Protection & Warning College
            Self::Shield => vec![SpellPrerequisite::Magery(0)],
            Self::Deflect => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Shield),
            ],
            Self::Reflection => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::Spell(Self::Deflect),
            ],
            Self::Absorb => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::ProtectionWarning, 4),
            ],
            Self::Warn => vec![SpellPrerequisite::Magery(0)],
            Self::Sense => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Warn),
            ],
            Self::ResistFireProtection => vec![SpellPrerequisite::Magery(0)],
            Self::ResistColdProtection => vec![SpellPrerequisite::Magery(0)],
            Self::ResistLightning => vec![SpellPrerequisite::Magery(0)],
            Self::ResistAcid => vec![SpellPrerequisite::Magery(0)],
            Self::ArmorEnchantment => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::ProtectionWarning, 6),
            ],
            Self::AnchorSpell => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::ProtectionWarning, 4),
            ],
            Self::WatchdogSpell => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Warn),
            ],

            // Water College
            Self::PurifyWater => vec![SpellPrerequisite::Magery(0)],
            Self::CreateWater => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::PurifyWater),
            ],
            Self::DestroyWater => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::CreateWater),
            ],
            Self::ShapeWater => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::CreateWater),
            ],
            Self::Freeze => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::CreateWater),
            ],
            Self::Boil => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::CreateWater),
            ],
            Self::WalkOnWater => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 4),
            ],
            Self::WaterJet => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::ShapeWater),
            ],
            Self::IceSphere => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Freeze),
            ],
            Self::IceSlick => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::Freeze),
            ],
            Self::BodyOfWater => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 6),
            ],
            Self::Dehydrate => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DestroyWater),
            ],
            Self::CreateIce => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Freeze),
            ],
            Self::CondenseSteam => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::CreateWater),
            ],
            Self::IceDagger => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::CreateIce),
            ],
            Self::Swim => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::ShapeWater),
            ],
            Self::SummonWaterElemental => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 8),
            ],
            Self::BreatheWaterWater => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 4),
            ],

            // Knowledge College
            Self::DetectMagic => vec![SpellPrerequisite::Magery(0)],
            Self::AnalyzeMagic => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 6),
            ],
            Self::IdentifySpell => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
            Self::Seeker => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
            Self::SeekAir => vec![SpellPrerequisite::Magery(0)],
            Self::SeekEarth => vec![SpellPrerequisite::Magery(0)],
            Self::SeekFire => vec![SpellPrerequisite::Magery(0)],
            Self::SeekWater => vec![SpellPrerequisite::Magery(0)],
            Self::SeekPlant => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::SeekEarth),
            ],
            Self::SeekFood => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::SeekWater),
            ],
            Self::SeekMachine => vec![SpellPrerequisite::Magery(0)],
            Self::History => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4),
            ],
            Self::Wisdom => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 12),
                SpellPrerequisite::IQ(14),
            ],
            Self::AncientHistory => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::History),
            ],
            Self::RecoverMemory => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4),
            ],
            Self::Divination => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 6),
            ],
            Self::Pathfinder => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Seeker),
            ],
            Self::GlassWall => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
            Self::Trace => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Seeker),
            ],
            Self::Aura => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
        };

        debug!(count = prereqs.len(), "Prerequisites retrieved");
        prereqs
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

        let spell_type = match self {
            // Air College
            Self::PurifyAirAir => SpellType::Area,
            Self::CreateAir => SpellType::Regular,
            Self::DestroyAir => SpellType::Area,
            Self::ShapeAir => SpellType::Regular,
            Self::NoSmell => SpellType::Area,
            Self::Stench => SpellType::Area,
            Self::WalkOnAir => SpellType::Regular,
            Self::Lightning => SpellType::Missile,
            Self::Windstorm => SpellType::Area,
            Self::AirJet => SpellType::Missile,
            Self::BreatheWater => SpellType::Regular,
            Self::BodyOfAir => SpellType::Regular,
            Self::Concussion => SpellType::Regular,
            Self::Sound => SpellType::Regular,
            Self::Silence => SpellType::Area,
            Self::Thunderclap => SpellType::Regular,
            Self::AirVortex => SpellType::Area,
            Self::EssentialAir => SpellType::Regular,

            // Body Control College
            Self::Itch => SpellType::Regular,
            Self::Spasm => SpellType::Regular,
            Self::Pain => SpellType::Regular,
            Self::Paralyze => SpellType::Regular,
            Self::RootedFeet => SpellType::Regular,
            Self::StrikeBlind => SpellType::Regular,
            Self::StrikeDeaf => SpellType::Regular,
            Self::StrikeDumb => SpellType::Regular,
            Self::Haste => SpellType::Regular,
            Self::Slow => SpellType::Regular,
            Self::Strengthen => SpellType::Regular,
            Self::Weaken => SpellType::Regular,
            Self::Grace => SpellType::Regular,
            Self::Clumsiness => SpellType::Regular,
            Self::ResistPain => SpellType::Regular,
            Self::ResistDisease => SpellType::Regular,
            Self::WitherLimb => SpellType::Regular,
            Self::Deathtouch => SpellType::Melee,
            Self::BodyOfStone => SpellType::Regular,
            Self::AlterBody => SpellType::Regular,
            Self::ShapeFlesh => SpellType::Regular,

            // Earth College
            Self::ShapeEarth => SpellType::Regular,
            Self::EarthToStone => SpellType::Area,
            Self::StoneToEarth => SpellType::Area,
            Self::CreateEarth => SpellType::Regular,
            Self::DestroyEarth => SpellType::Area,
            Self::EarthVision => SpellType::Information,
            Self::WalkThroughEarth => SpellType::Regular,
            Self::Earthquake => SpellType::Area,
            Self::ShapeStone => SpellType::Regular,
            Self::EarthToAir => SpellType::Regular,
            Self::StoneMissile => SpellType::Missile,
            Self::CreateStone => SpellType::Regular,
            Self::SandJet => SpellType::Missile,
            Self::FleshToStone => SpellType::Regular,
            Self::StoneToFlesh => SpellType::Regular,
            Self::IronArm => SpellType::Regular,
            Self::Entombment => SpellType::Regular,
            Self::SummonEarthElemental => SpellType::Regular,

            // Fire College
            Self::IgniteFlame => SpellType::Regular,
            Self::CreateFire => SpellType::Regular,
            Self::ExtinguishFire => SpellType::Area,
            Self::ShapeFire => SpellType::Regular,
            Self::Fireball => SpellType::Missile,
            Self::ExplosiveFireball => SpellType::Missile,
            Self::FlameJet => SpellType::Missile,
            Self::Heat => SpellType::Regular,
            Self::Cold => SpellType::Regular,
            Self::ResistFire => SpellType::Regular,
            Self::ResistCold => SpellType::Regular,
            Self::PurifyAir => SpellType::Area,
            Self::Frostbite => SpellType::Missile,
            Self::WallOfFire => SpellType::Area,
            Self::SummonFireElemental => SpellType::Regular,
            Self::PermanentFlame => SpellType::Regular,
            Self::SeekFireFire => SpellType::Information,
            Self::Ignition => SpellType::Regular,

            // Healing College
            Self::LendEnergy => SpellType::Regular,
            Self::LendVitality => SpellType::Regular,
            Self::RecoverEnergy => SpellType::Regular,
            Self::MinorHealing => SpellType::Regular,
            Self::MajorHealing => SpellType::Regular,
            Self::GreatHealing => SpellType::Regular,
            Self::Awaken => SpellType::Regular,
            Self::CureDisease => SpellType::Regular,
            Self::NeutralizePoison => SpellType::Regular,
            Self::InstantNeutralizePoison => SpellType::Regular,
            Self::StopBleeding => SpellType::Regular,
            Self::Regeneration => SpellType::Regular,
            Self::Restoration => SpellType::Regular,
            Self::SuspendCurse => SpellType::Regular,
            Self::RemoveCurse => SpellType::Regular,
            Self::CurseRemoval => SpellType::Regular,
            Self::Healing => SpellType::Regular,
            Self::RestoreYouth => SpellType::Regular,

            // Illusion & Creation College
            Self::SimpleIllusion => SpellType::Regular,
            Self::PerfectIllusion => SpellType::Regular,
            Self::ComplexIllusion => SpellType::Regular,
            Self::MakeRealIllusion => SpellType::Regular,
            Self::Create => SpellType::Regular,
            Self::PermanentCreation => SpellType::Regular,
            Self::Invisibility => SpellType::Regular,
            Self::InvisibilityToAll => SpellType::Regular,
            Self::Disguise => SpellType::Regular,
            Self::ShapeShifting => SpellType::Regular,
            Self::Duplicate => SpellType::Regular,
            Self::Phantasm => SpellType::Area,
            Self::SilentImage => SpellType::Regular,
            Self::Light => SpellType::Regular,
            Self::Darkness => SpellType::Area,
            Self::HideAura => SpellType::Regular,
            Self::Mirror => SpellType::Blocking,

            // Mind Control College
            Self::Daze => SpellType::Regular,
            Self::Sleep => SpellType::Regular,
            Self::Command => SpellType::Regular,
            Self::MassDaze => SpellType::Area,
            Self::Fear => SpellType::Regular,
            Self::Terror => SpellType::Regular,
            Self::Loyalty => SpellType::Regular,
            Self::Charm => SpellType::Regular,
            Self::Enslave => SpellType::Regular,
            Self::MindReading => SpellType::Information,
            Self::MindSending => SpellType::Regular,
            Self::Telepathy => SpellType::Regular,
            Self::Forgetfulness => SpellType::Regular,
            Self::FalseMemory => SpellType::Regular,
            Self::Suggestion => SpellType::Regular,
            Self::CompelTruth => SpellType::Regular,
            Self::HideThoughts => SpellType::Blocking,
            Self::ShieldMind => SpellType::Blocking,
            Self::Possession => SpellType::Regular,
            Self::SoulJar => SpellType::Regular,

            // Movement College
            Self::Levitation => SpellType::Regular,
            Self::Flight => SpellType::Regular,
            Self::Blink => SpellType::Regular,
            Self::Teleport => SpellType::Regular,
            Self::Teleportation => SpellType::Regular,
            Self::HasteMovement => SpellType::Regular,
            Self::SlowMovement => SpellType::Regular,
            Self::Cling => SpellType::Regular,
            Self::Jump => SpellType::Regular,
            Self::Apportation => SpellType::Regular,
            Self::Poltergeist => SpellType::Regular,
            Self::Lockmaster => SpellType::Regular,
            Self::TelekineticBlow => SpellType::Missile,
            Self::HaltMovement => SpellType::Regular,

            // Protection & Warning College
            Self::Shield => SpellType::Blocking,
            Self::Deflect => SpellType::Blocking,
            Self::Reflection => SpellType::Blocking,
            Self::Absorb => SpellType::Blocking,
            Self::Warn => SpellType::Information,
            Self::Sense => SpellType::Information,
            Self::ResistFireProtection => SpellType::Regular,
            Self::ResistColdProtection => SpellType::Regular,
            Self::ResistLightning => SpellType::Regular,
            Self::ResistAcid => SpellType::Regular,
            Self::ArmorEnchantment => SpellType::Regular,
            Self::AnchorSpell => SpellType::Area,
            Self::WatchdogSpell => SpellType::Information,

            // Water College
            Self::PurifyWater => SpellType::Area,
            Self::CreateWater => SpellType::Regular,
            Self::DestroyWater => SpellType::Area,
            Self::ShapeWater => SpellType::Regular,
            Self::Freeze => SpellType::Area,
            Self::Boil => SpellType::Area,
            Self::WalkOnWater => SpellType::Regular,
            Self::WaterJet => SpellType::Missile,
            Self::IceSphere => SpellType::Missile,
            Self::IceSlick => SpellType::Area,
            Self::BodyOfWater => SpellType::Regular,
            Self::Dehydrate => SpellType::Regular,
            Self::CreateIce => SpellType::Regular,
            Self::CondenseSteam => SpellType::Regular,
            Self::IceDagger => SpellType::Melee,
            Self::Swim => SpellType::Regular,
            Self::SummonWaterElemental => SpellType::Regular,
            Self::BreatheWaterWater => SpellType::Regular,

            // Knowledge College
            Self::DetectMagic => SpellType::Information,
            Self::AnalyzeMagic => SpellType::Information,
            Self::IdentifySpell => SpellType::Information,
            Self::Seeker => SpellType::Information,
            Self::SeekAir => SpellType::Information,
            Self::SeekEarth => SpellType::Information,
            Self::SeekFire => SpellType::Information,
            Self::SeekWater => SpellType::Information,
            Self::SeekPlant => SpellType::Information,
            Self::SeekFood => SpellType::Information,
            Self::SeekMachine => SpellType::Information,
            Self::History => SpellType::Information,
            Self::Wisdom => SpellType::Information,
            Self::AncientHistory => SpellType::Information,
            Self::RecoverMemory => SpellType::Regular,
            Self::Divination => SpellType::Information,
            Self::Pathfinder => SpellType::Information,
            Self::GlassWall => SpellType::Area,
            Self::Trace => SpellType::Information,
            Self::Aura => SpellType::Information,
        };

        debug!(?spell_type, "Spell type retrieved");
        spell_type
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

        let resistance = match self {
            // Air College
            Self::PurifyAirAir => None,
            Self::CreateAir => None,
            Self::DestroyAir => None,
            Self::ShapeAir => None,
            Self::NoSmell => None,
            Self::Stench => None,
            Self::WalkOnAir => None,
            Self::Lightning => None,
            Self::Windstorm => None,
            Self::AirJet => None,
            Self::BreatheWater => None,
            Self::BodyOfAir => None,
            Self::Concussion => Some(ResistanceType::HT),
            Self::Sound => None,
            Self::Silence => None,
            Self::Thunderclap => Some(ResistanceType::HT),
            Self::AirVortex => None,
            Self::EssentialAir => None,

            // Body Control College
            Self::Itch => Some(ResistanceType::HT),
            Self::Spasm => Some(ResistanceType::HT),
            Self::Pain => Some(ResistanceType::HT),
            Self::Paralyze => Some(ResistanceType::HT),
            Self::RootedFeet => Some(ResistanceType::HT),
            Self::StrikeBlind => Some(ResistanceType::HT),
            Self::StrikeDeaf => Some(ResistanceType::HT),
            Self::StrikeDumb => Some(ResistanceType::HT),
            Self::Haste => None,
            Self::Slow => Some(ResistanceType::HT),
            Self::Strengthen => None,
            Self::Weaken => Some(ResistanceType::HT),
            Self::Grace => None,
            Self::Clumsiness => Some(ResistanceType::HT),
            Self::ResistPain => None,
            Self::ResistDisease => None,
            Self::WitherLimb => Some(ResistanceType::HT),
            Self::Deathtouch => Some(ResistanceType::HT),
            Self::BodyOfStone => None,
            Self::AlterBody => Some(ResistanceType::HT),
            Self::ShapeFlesh => Some(ResistanceType::HT),

            // Earth College
            Self::ShapeEarth => None,
            Self::EarthToStone => None,
            Self::StoneToEarth => None,
            Self::CreateEarth => None,
            Self::DestroyEarth => None,
            Self::EarthVision => None,
            Self::WalkThroughEarth => None,
            Self::Earthquake => None,
            Self::ShapeStone => None,
            Self::EarthToAir => None,
            Self::StoneMissile => None,
            Self::CreateStone => None,
            Self::SandJet => None,
            Self::FleshToStone => Some(ResistanceType::HT),
            Self::StoneToFlesh => None,
            Self::IronArm => None,
            Self::Entombment => Some(ResistanceType::HT),
            Self::SummonEarthElemental => None,

            // Fire College
            Self::IgniteFlame => None,
            Self::CreateFire => None,
            Self::ExtinguishFire => None,
            Self::ShapeFire => None,
            Self::Fireball => None,
            Self::ExplosiveFireball => None,
            Self::FlameJet => None,
            Self::Heat => Some(ResistanceType::HT),
            Self::Cold => Some(ResistanceType::HT),
            Self::ResistFire => None,
            Self::ResistCold => None,
            Self::PurifyAir => None,
            Self::Frostbite => None,
            Self::WallOfFire => None,
            Self::SummonFireElemental => None,
            Self::PermanentFlame => None,
            Self::SeekFireFire => None,
            Self::Ignition => None,

            // Healing College
            Self::LendEnergy => None,
            Self::LendVitality => None,
            Self::RecoverEnergy => None,
            Self::MinorHealing => None,
            Self::MajorHealing => None,
            Self::GreatHealing => None,
            Self::Awaken => None,
            Self::CureDisease => None,
            Self::NeutralizePoison => None,
            Self::InstantNeutralizePoison => None,
            Self::StopBleeding => None,
            Self::Regeneration => None,
            Self::Restoration => None,
            Self::SuspendCurse => None,
            Self::RemoveCurse => None,
            Self::CurseRemoval => None,
            Self::Healing => None,
            Self::RestoreYouth => None,

            // Illusion & Creation College
            Self::SimpleIllusion => Some(ResistanceType::IQ),
            Self::PerfectIllusion => Some(ResistanceType::IQ),
            Self::ComplexIllusion => Some(ResistanceType::IQ),
            Self::MakeRealIllusion => None,
            Self::Create => None,
            Self::PermanentCreation => None,
            Self::Invisibility => None,
            Self::InvisibilityToAll => None,
            Self::Disguise => Some(ResistanceType::IQ),
            Self::ShapeShifting => None,
            Self::Duplicate => Some(ResistanceType::IQ),
            Self::Phantasm => Some(ResistanceType::IQ),
            Self::SilentImage => Some(ResistanceType::IQ),
            Self::Light => None,
            Self::Darkness => None,
            Self::HideAura => None,
            Self::Mirror => None,

            // Mind Control College
            Self::Daze => Some(ResistanceType::Will),
            Self::Sleep => Some(ResistanceType::Will),
            Self::Command => Some(ResistanceType::Will),
            Self::MassDaze => Some(ResistanceType::Will),
            Self::Fear => Some(ResistanceType::Will),
            Self::Terror => Some(ResistanceType::Will),
            Self::Loyalty => Some(ResistanceType::Will),
            Self::Charm => Some(ResistanceType::Will),
            Self::Enslave => Some(ResistanceType::Will),
            Self::MindReading => Some(ResistanceType::Will),
            Self::MindSending => None,
            Self::Telepathy => None,
            Self::Forgetfulness => Some(ResistanceType::Will),
            Self::FalseMemory => Some(ResistanceType::Will),
            Self::Suggestion => Some(ResistanceType::Will),
            Self::CompelTruth => Some(ResistanceType::Will),
            Self::HideThoughts => None,
            Self::ShieldMind => None,
            Self::Possession => Some(ResistanceType::Will),
            Self::SoulJar => Some(ResistanceType::Will),

            // Movement College
            Self::Levitation => None,
            Self::Flight => None,
            Self::Blink => None,
            Self::Teleport => None,
            Self::Teleportation => None,
            Self::HasteMovement => None,
            Self::SlowMovement => Some(ResistanceType::HT),
            Self::Cling => None,
            Self::Jump => None,
            Self::Apportation => None,
            Self::Poltergeist => None,
            Self::Lockmaster => None,
            Self::TelekineticBlow => None,
            Self::HaltMovement => Some(ResistanceType::HT),

            // Protection & Warning College
            Self::Shield => None,
            Self::Deflect => None,
            Self::Reflection => None,
            Self::Absorb => None,
            Self::Warn => None,
            Self::Sense => None,
            Self::ResistFireProtection => None,
            Self::ResistColdProtection => None,
            Self::ResistLightning => None,
            Self::ResistAcid => None,
            Self::ArmorEnchantment => None,
            Self::AnchorSpell => None,
            Self::WatchdogSpell => None,

            // Water College
            Self::PurifyWater => None,
            Self::CreateWater => None,
            Self::DestroyWater => None,
            Self::ShapeWater => None,
            Self::Freeze => None,
            Self::Boil => None,
            Self::WalkOnWater => None,
            Self::WaterJet => None,
            Self::IceSphere => None,
            Self::IceSlick => None,
            Self::BodyOfWater => None,
            Self::Dehydrate => Some(ResistanceType::HT),
            Self::CreateIce => None,
            Self::CondenseSteam => None,
            Self::IceDagger => None,
            Self::Swim => None,
            Self::SummonWaterElemental => None,
            Self::BreatheWaterWater => None,

            // Knowledge College
            Self::DetectMagic => None,
            Self::AnalyzeMagic => None,
            Self::IdentifySpell => None,
            Self::Seeker => None,
            Self::SeekAir => None,
            Self::SeekEarth => None,
            Self::SeekFire => None,
            Self::SeekWater => None,
            Self::SeekPlant => None,
            Self::SeekFood => None,
            Self::SeekMachine => None,
            Self::History => None,
            Self::Wisdom => None,
            Self::AncientHistory => None,
            Self::RecoverMemory => Some(ResistanceType::IQ),
            Self::Divination => None,
            Self::Pathfinder => None,
            Self::GlassWall => None,
            Self::Trace => None,
            Self::Aura => None,
        };

        debug!(?resistance, "Resistance retrieved");
        resistance
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

        let reference = match self {
            // Air College
            Self::PurifyAirAir => "M33",
            Self::CreateAir => "M28",
            Self::DestroyAir => "M28",
            Self::ShapeAir => "M34",
            Self::NoSmell => "M32",
            Self::Stench => "M34",
            Self::WalkOnAir => "M35",
            Self::Lightning => "M30",
            Self::Windstorm => "M35",
            Self::AirJet => "M25",
            Self::BreatheWater => "M27",
            Self::BodyOfAir => "M26",
            Self::Concussion => "M28",
            Self::Sound => "M34",
            Self::Silence => "M34",
            Self::Thunderclap => "M35",
            Self::AirVortex => "M26",
            Self::EssentialAir => "M29",

            // Body Control College
            Self::Itch => "M59",
            Self::Spasm => "M65",
            Self::Pain => "M61",
            Self::Paralyze => "M61",
            Self::RootedFeet => "M63",
            Self::StrikeBlind => "M65",
            Self::StrikeDeaf => "M65",
            Self::StrikeDumb => "M65",
            Self::Haste => "M58",
            Self::Slow => "M64",
            Self::Strengthen => "M66",
            Self::Weaken => "M67",
            Self::Grace => "M58",
            Self::Clumsiness => "M51",
            Self::ResistPain => "M63",
            Self::ResistDisease => "M62",
            Self::WitherLimb => "M67",
            Self::Deathtouch => "M54",
            Self::BodyOfStone => "M49",
            Self::AlterBody => "M36",
            Self::ShapeFlesh => "M64",

            // Earth College
            Self::ShapeEarth => "M64",
            Self::EarthToStone => "M59",
            Self::StoneToEarth => "M66",
            Self::CreateEarth => "M57",
            Self::DestroyEarth => "M58",
            Self::EarthVision => "M59",
            Self::WalkThroughEarth => "M67",
            Self::Earthquake => "M59",
            Self::ShapeStone => "M64",
            Self::EarthToAir => "M58",
            Self::StoneMissile => "M66",
            Self::CreateStone => "M57",
            Self::SandJet => "M64",
            Self::FleshToStone => "M60",
            Self::StoneToFlesh => "M66",
            Self::IronArm => "M62",
            Self::Entombment => "M59",
            Self::SummonEarthElemental => "M58",

            // Fire College
            Self::IgniteFlame => "M68",
            Self::CreateFire => "M69",
            Self::ExtinguishFire => "M71",
            Self::ShapeFire => "M76",
            Self::Fireball => "M70",
            Self::ExplosiveFireball => "M71",
            Self::FlameJet => "M70",
            Self::Heat => "M72",
            Self::Cold => "M69",
            Self::ResistFire => "M75",
            Self::ResistCold => "M75",
            Self::PurifyAir => "M74",
            Self::Frostbite => "M69",
            Self::WallOfFire => "M76",
            Self::SummonFireElemental => "M76",
            Self::PermanentFlame => "M74",
            Self::SeekFireFire => "M76",
            Self::Ignition => "M72",

            // Healing College
            Self::LendEnergy => "M93",
            Self::LendVitality => "M94",
            Self::RecoverEnergy => "M101",
            Self::MinorHealing => "M96",
            Self::MajorHealing => "M94",
            Self::GreatHealing => "M92",
            Self::Awaken => "M90",
            Self::CureDisease => "M91",
            Self::NeutralizePoison => "M98",
            Self::InstantNeutralizePoison => "M92",
            Self::StopBleeding => "M103",
            Self::Regeneration => "M101",
            Self::Restoration => "M102",
            Self::SuspendCurse => "M103",
            Self::RemoveCurse => "M101",
            Self::CurseRemoval => "M91",
            Self::Healing => "M92",
            Self::RestoreYouth => "M102",

            // Illusion & Creation College
            Self::SimpleIllusion => "M100",
            Self::PerfectIllusion => "M97",
            Self::ComplexIllusion => "M96",
            Self::MakeRealIllusion => "M102",
            Self::Create => "M100",
            Self::PermanentCreation => "M99",
            Self::Invisibility => "M98",
            Self::InvisibilityToAll => "M98",
            Self::Disguise => "M97",
            Self::ShapeShifting => "M103",
            Self::Duplicate => "M98",
            Self::Phantasm => "M100",
            Self::SilentImage => "M100",
            Self::Light => "M99",
            Self::Darkness => "M97",
            Self::HideAura => "M98",
            Self::Mirror => "M101",

            // Mind Control College
            Self::Daze => "M122",
            Self::Sleep => "M139",
            Self::Command => "M121",
            Self::MassDaze => "M130",
            Self::Fear => "M124",
            Self::Terror => "M141",
            Self::Loyalty => "M130",
            Self::Charm => "M121",
            Self::Enslave => "M123",
            Self::MindReading => "M132",
            Self::MindSending => "M133",
            Self::Telepathy => "M141",
            Self::Forgetfulness => "M125",
            Self::FalseMemory => "M124",
            Self::Suggestion => "M140",
            Self::CompelTruth => "M121",
            Self::HideThoughts => "M127",
            Self::ShieldMind => "M138",
            Self::Possession => "M136",
            Self::SoulJar => "M140",

            // Movement College
            Self::Levitation => "M154",
            Self::Flight => "M152",
            Self::Blink => "M159",
            Self::Teleport => "M160",
            Self::Teleportation => "M159",
            Self::HasteMovement => "M153",
            Self::SlowMovement => "M158",
            Self::Cling => "M151",
            Self::Jump => "M154",
            Self::Apportation => "M160",
            Self::Poltergeist => "M147",
            Self::Lockmaster => "M155",
            Self::TelekineticBlow => "M156",
            Self::HaltMovement => "M153",

            // Protection & Warning College
            Self::Shield => "M182",
            Self::Deflect => "M168",
            Self::Reflection => "M177",
            Self::Absorb => "M163",
            Self::Warn => "M185",
            Self::Sense => "M164",
            Self::ResistFireProtection => "M175",
            Self::ResistColdProtection => "M175",
            Self::ResistLightning => "M175",
            Self::ResistAcid => "M175",
            Self::ArmorEnchantment => "M163",
            Self::AnchorSpell => "M163",
            Self::WatchdogSpell => "M185",

            // Water College
            Self::PurifyWater => "M196",
            Self::CreateWater => "M191",
            Self::DestroyWater => "M192",
            Self::ShapeWater => "M199",
            Self::Freeze => "M193",
            Self::Boil => "M189",
            Self::WalkOnWater => "M200",
            Self::WaterJet => "M200",
            Self::IceSphere => "M194",
            Self::IceSlick => "M194",
            Self::BodyOfWater => "M189",
            Self::Dehydrate => "M192",
            Self::CreateIce => "M191",
            Self::CondenseSteam => "M191",
            Self::IceDagger => "M194",
            Self::Swim => "M199",
            Self::SummonWaterElemental => "M199",
            Self::BreatheWaterWater => "M189",

            // Knowledge College
            Self::DetectMagic => "M107",
            Self::AnalyzeMagic => "M106",
            Self::IdentifySpell => "M109",
            Self::Seeker => "M112",
            Self::SeekAir => "M111",
            Self::SeekEarth => "M111",
            Self::SeekFire => "M112",
            Self::SeekWater => "M112",
            Self::SeekPlant => "M112",
            Self::SeekFood => "M111",
            Self::SeekMachine => "M111",
            Self::History => "M108",
            Self::Wisdom => "M113",
            Self::AncientHistory => "M106",
            Self::RecoverMemory => "M110",
            Self::Divination => "M107",
            Self::Pathfinder => "M110",
            Self::GlassWall => "M108",
            Self::Trace => "M113",
            Self::Aura => "M107",
        };

        debug!(reference, "Reference retrieved");
        reference
    }
}

/// Energy cost structure for spells.
///
/// # GURPS Rules
///
/// Energy costs can be fixed or scale with spell effect
/// (per die of damage, per HP healed, per yard of range, etc.).
///
/// # Citations
///
/// BS 241 - Energy cost
/// M 10 - Cost notation
///
/// # Examples
///
/// ```
/// use valinoreth::EnergyCost;
///
/// let fixed = EnergyCost::Fixed(2);
/// let scaling = EnergyCost::PerDie(1);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnergyCost {
    /// Fixed energy cost
    Fixed(i32),
    /// Cost per die of damage
    PerDie(i32),
    /// Cost per HP restored/damaged
    PerHP(i32),
    /// Cost per FP transferred
    PerFP(i32),
    /// Cost per yard of range/radius
    PerYard(i32),
}

/// Spell prerequisite.
///
/// # GURPS Rules
///
/// Spells require Magery levels, other spells, minimum attributes,
/// or a count of spells known in a college.
///
/// # Citations
///
/// BS 241 - Prerequisites
/// M 10 - Prerequisite notation
///
/// # Examples
///
/// ```
/// use valinoreth::{SpellPrerequisite, Spell, SpellCollege};
///
/// let magery = SpellPrerequisite::Magery(1);
/// let spell = SpellPrerequisite::Spell(Spell::DetectMagic);
/// let college = SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4);
/// let iq = SpellPrerequisite::IQ(12);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpellPrerequisite {
    /// Magery level required (0 = any, 1+ = specific level)
    Magery(usize),
    /// Specific spell must be known
    Spell(Spell),
    /// Number of spells in college must be known
    SpellsInCollege(SpellCollege, usize),
    /// Minimum IQ required
    IQ(i32),
}

/// Spell type for casting mechanics.
///
/// # GURPS Rules
///
/// Different spell types have different casting mechanics:
/// - Regular: affects single target
/// - Area: affects area/multiple targets
/// - Missile: ranged attack requiring hit roll
/// - Information: reveals data
///
/// # Citations
///
/// BS 239 - Spell types
///
/// # Examples
///
/// ```
/// use valinoreth::SpellType;
///
/// let info = SpellType::Information;
/// let area = SpellType::Area;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpellType {
    /// Standard single-target spell
    Regular,
    /// Affects area or multiple targets
    Area,
    /// Ranged attack (requires to-hit roll)
    Missile,
    /// Melee touch attack
    Melee,
    /// Blocks or resists other spells
    Blocking,
    /// Reveals information
    Information,
}

/// Spell duration.
///
/// # GURPS Rules
///
/// Durations vary from instant effects to maintained concentrations
/// to timed effects.
///
/// # Citations
///
/// BS 241 - Duration
///
/// # Examples
///
/// ```
/// use valinoreth::Duration;
///
/// let instant = Duration::Instant;
/// let maintained = Duration::Concentration;
/// let timed = Duration::Minutes(10);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Duration {
    /// Instant effect
    Instant,
    /// Requires active concentration
    Concentration,
    /// Lasts for minutes
    Minutes(i32),
    /// Lasts for hours
    Hours(i32),
    /// Lasts until dispelled
    Permanent,
}

/// Resistance type for spells.
///
/// # GURPS Rules
///
/// Some spells allow resistance rolls to avoid or reduce effects.
/// Resistance is typically vs Will, HT, or IQ.
///
/// # Citations
///
/// BS 241 - Resistance
///
/// # Examples
///
/// ```
/// use valinoreth::ResistanceType;
///
/// let will = ResistanceType::Will;
/// let iq = ResistanceType::IQ;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResistanceType {
    /// Resistance vs Will
    Will,
    /// Resistance vs Health
    HT,
    /// Resistance vs Intelligence
    IQ,
    /// Special resistance (varies)
    Special,
}
