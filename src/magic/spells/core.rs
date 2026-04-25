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

use super::{air, animal, body_control, communication_empathy, enchantment, earth, fire, food, gate, healing, illusion_creation, knowledge, light_darkness, making_breaking, meta_spells, mind_control, movement, necromantic, plant, protection_warning, sound, water};
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
    /// Animal control and beast spells. M 16
    Animal,
    /// Physical enhancement and affliction spells. M 36
    BodyControl,
    /// Communication and empathy spells. M 48
    CommunicationEmpathy,
    /// Earth and stone manipulation spells. M 56
    Earth,
    /// Enchantment and permanent magic. M 60
    Enchantment,
    /// Fire and heat manipulation spells. M 68
    Fire,
    /// Food creation and preservation spells. M 78
    Food,
    /// Portal and teleportation circle spells. M 82
    Gate,
    /// Healing and restoration spells. M 90
    Healing,
    /// Illusion and creation spells. M 94
    IllusionCreation,
    /// Information and divination spells. M 106
    Knowledge,
    /// Light and darkness manipulation spells. M 114
    LightDarkness,
    /// Object creation and destruction spells. M 116
    MakingBreaking,
    /// Spell manipulation and meta-magic. M 117
    MetaSpells,
    /// Mental influence and control spells. M 118
    MindControl,
    /// Movement and teleportation spells. M 146
    Movement,
    /// Death magic and undead spells. M 149
    Necromantic,
    /// Plant control and growth spells. M 155
    Plant,
    /// Protection and warning spells. M 162
    ProtectionWarning,
    /// Sound manipulation spells. M 171
    Sound,
    /// Technology and machine spells. M 176
    Technological,
    /// Water and ice manipulation spells. M 186
    Water,
    /// Weather control spells. M 193
    Weather,
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

    // Animal College - M 16-23
    /// Calms and befriends animals. M 30
    BeastSoother,
    /// Communicate with animals. M 29
    BeastSpeech,
    /// Control mammals. M 30
    MammalControl,
    /// Control birds. M 29
    BirdControl,
    /// Control reptiles. M 33
    ReptileControl,
    /// Control fish. M 30
    FishControl,
    /// Control insects. M 31
    InsectControl,
    /// Summon an animal. M 29
    BeastSummoning,
    /// Repel animals generally. M 32
    RepelAnimal,
    /// Repel mammals. M 32
    RepelMammal,
    /// Repel birds. M 32
    RepelBird,
    /// Repel reptiles. M 32
    RepelReptile,
    /// Repel fish. M 32
    RepelFish,
    /// Repel insects. M 32
    RepelInsect,
    /// Dominate animal completely. M 31
    Master,
    /// Ride any animal. M 33
    Rider,
    /// Transform into mammal. M 34
    Shapeshifting,
    /// Transform into bird. M 34
    BirdShapeshifting,
    /// Transform into reptile. M 34
    ReptileShapeshifting,
    /// Transform into fish. M 34
    FishShapeshifting,
    /// Transform into insect. M 34
    InsectShapeshifting,
    /// Advanced shapeshifting. M 31
    GreatShapeshifting,

    // Communication & Empathy College - M 48-55
    /// Detects emotional state. M 52
    SenseEmotion,
    /// Makes target agreeable. M 51
    Persuasion,
    /// Controls target's emotions. M 50
    EmotionControl,
    /// Detect lies and truth. M 49
    Truthsayer,
    /// Sense nearby life. M 52
    SenseLife,
    /// Sense hostile intent. M 52
    SenseFoes,
    /// Share language temporarily. M 50
    LendLanguage,
    /// Learn language temporarily. M 48
    BorrowLanguage,
    /// Permanent language knowledge. M 49
    GiftOfTongues,
    /// Literacy in any language. M 49
    GiftOfLetters,
    /// Understand written text. M 50
    Comprehend,
    /// Translate speech. M 54
    Translate,
    /// Permanent translation. M 54
    PermanentTranslation,
    /// Project voice over distance. M 51
    ProjectVoice,
    /// Silent communication. M 53
    SilentCommunication,
    /// Group telepathic link. M 49
    LinkMind,

    // Enchantment College - M 56-71
    /// Creates permanent magical items. M 56
    Enchant,
    /// Creates temporary magical effects. M 56
    TemporaryEnchantment,
    /// Curses an item or creature. M 57
    Hex,
    /// Inscribes spell on paper or parchment. M 57
    Scroll,
    /// Increases spell power. M 57
    Power,
    /// Creates energy storage gem. M 69
    Powerstone,
    /// Removes enchantments. M 58
    RemoveEnchantment,
    /// Protects from enchantments. M 58
    ResistEnchantment,
    /// Animates clay or stone construct. M 59
    Golem,
    /// Curses target with ill luck. M 60
    Malefice,
    /// Enslaves target's will. M 60
    Ensorcel,
    /// Blocks magical tracing. M 60
    ImpressionBlocker,
    /// Minor reality alteration. M 61
    LesserWish,
    /// Major reality alteration. M 61
    Wish,
    /// Ultimate reality alteration. M 61
    GreatWish,
    /// Creates mana storage stone. M 70
    Manastone,
    /// Enchants wizard's staff. M 70
    Staff,
    /// Enchants spell-storing wand. M 70
    Wand,
    /// Creates scrying crystal. M 71
    CrystalBall,
    /// Traps soul in gem. M 71
    SoulStone,

    // Food College - M 72-80
    /// Spoils and rots food. M 77
    Decay,
    /// Detects poison and spoilage. M 78
    TestFood,
    /// Removes toxins from food. M 78
    PurifyFood,
    /// Magically creates food. M 77
    CreateFood,
    /// Prevents food spoilage. M 78
    PreserveFood,
    /// Instantly cooks food. M 77
    Cook,
    /// Enhances food flavor. M 78
    Season,
    /// Changes food taste. M 77
    Flavor,
    /// Field-dresses game. M 78
    PrepareGame,
    /// Contaminates food with toxin. M 78
    PoisonFood,
    /// Reveals recipe of dish. M 77
    KnowRecipe,
    /// Ages food or drink. M 77
    Mature,
    /// Creates feast from ingredients. M 77
    Banquet,
    /// Creates minimal nutrition. M 77
    EssentialFood,
    /// Purifies and concentrates alcohol. M 77
    Distill,
    /// Ferments sugars into alcohol. M 77
    Ferment,
    /// Transforms water to wine. M 79
    WaterToWine,
    /// Inflicts hunger pangs. M 77
    Hunger,
    /// Causes extreme thirst. M 79
    Thirst,
    /// Protects organic material from decay. M 78
    Rotproof,

    // Gate College - M 80-86
    /// Instantly transport to known location. M 147
    Teleport,
    /// Short-range random teleport. M 80
    Blink,
    /// Move small objects at distance. M 80
    Apportation,
    /// Lift and move objects with magic. M 80
    Levitation,
    /// Teleport another person. M 147
    TeleportOther,
    /// Marks location for easy return. M 83
    Beacon,
    /// Detect teleportation events. M 84
    TraceTeleport,
    /// Redirect incoming teleports. M 84
    DivertTeleport,
    /// Summon being from other plane. M 82
    PlanarSummons,
    /// Visit other planes of existence. M 82
    PlanarVisit,
    /// Shift to parallel dimension. M 83
    PlaneShift,
    /// Send others to other planes. M 83
    PlaneShiftOther,
    /// Become partially ethereal. M 83
    Phase,
    /// Make others partially ethereal. M 83
    PhaseOther,
    /// Locate magical gates. M 85
    SeekGate,
    /// Manipulate existing gates. M 85
    ControlGate,
    /// Creates dimensional portal. M 85
    CreateGate,
    /// View through distant gates. M 85
    ScryGate,
    /// Hides object in pocket dimension. M 86
    HideObject,
    /// Creates protected pocket dimension. M 86
    Sanctuary,

    // Light & Darkness College - M 110-116
    /// Creates magical light. M 110
    Light,
    /// Creates magical darkness. M 110
    Darkness,
    /// Permanent light source. M 110
    ContinualLight,
    /// Changes object colors. M 110
    Colors,
    /// Eliminates shadows. M 110
    RemoveShadow,
    /// See in darkness. M 111
    NightVision,
    /// See in total darkness. M 111
    DarkVision,
    /// Makes target hard to see. M 113
    Blur,
    /// Detect invisible objects. M 113
    SeeInvisible,
    /// Creates reflective surface. M 112
    Mirror,
    /// Projects damaging light beam. M 112
    LightJet,
    /// Extinguishes all light. M 112
    Blackout,
    /// Creates wall of light. M 113
    WallOfLight,
    /// Creates wall of darkness. M 113
    WallOfDarkness,
    /// Blinds with bright flash. M 112
    Flash,
    /// Transform into living shadow. M 114
    BodyOfShadow,
    /// Become shadow-like. M 114
    ShadowForm,
    /// See heat signatures. M 111
    Infravision,

    // Making & Breaking College - M 116-125
    /// Repairs damaged objects. M 118
    Repair,
    /// Shatters brittle objects. M 122
    Shatter,
    /// Hardens and stiffens material. M 123
    Stiffen,
    /// Softens rigid material. M 123
    Soften,
    /// Restores rusted metal. M 121
    Restore,
    /// Causes metal to rust. M 121
    Rust,
    /// Transforms object's material. M 124
    TransformObject,
    /// Creates copy of object. M 117
    Copy,
    /// Colors and dyes objects. M 117
    Dye,
    /// Creates magical knot. M 119
    Knot,
    /// Fastens objects together. M 118
    Fasten,
    /// Extends object's length. M 118
    ExtendObject,
    /// Shrinks object's size. M 122
    ShrinkObject,
    /// Polishes surfaces. M 120
    Polish,
    /// Makes object explode. M 118
    Explode,
    /// Finds structural weaknesses. M 119
    FindWeakness,
    /// Creates simple object. M 116
    CreateObject,
    /// Destroys object completely. M 117
    DestroyObject,
    /// Advanced object reconstruction. M 120
    Rebuild,
    /// Writes magical inscriptions. M 119
    Inscribe,
    /// Gives object limited animation. M 116
    AnimateObject,

    // Meta-Spells College - M 126-133
    /// Cancels enemy spell being cast. M 126
    Counterspell,
    /// Removes existing spell effect. M 127
    DispelMagic,
    /// Temporarily pauses spell effect. M 132
    SuspendSpell,
    /// Allows continuous spell maintenance. M 130
    MaintainSpell,
    /// Lengthens spell duration. M 128
    ExtendSpell,
    /// Speeds up spell casting. M 129
    HasteSpell,
    /// Combines multiple spell effects. M 130
    Link,
    /// Makes spell effect permanent. M 131
    Permanency,
    /// Drains energy from target. M 132
    StealEnergy,
    /// Pulls mana from environment. M 127
    DrawPower,
    /// Stores spell in gem. M 132
    SpellStone,
    /// Bounces spell back. M 131
    Reflect,
    /// Creates protective barrier. M 133
    Ward,
    /// Increases spell effectiveness. M 126
    Augment,
    /// Decreases spell effectiveness. M 127
    Diminish,
    /// Allows spell delay. M 128
    Delay,

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
    /// Survive in extreme heat. M 71
    EssentialFlame,
    /// Transform body to living flame. M 48
    BodyOfFire,
    /// See through fire and smoke. M 70
    FireVision,
    /// Provides gentle warmth. M 77
    Warmth,
    /// Deflects energy attacks. M 69
    DeflectEnergy,
    /// Commands fire elemental. M 69
    ControlFireElemental,
    /// Creates permanent fire elemental. M 69
    CreateFireElemental,

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
    /// Returns dead to life. M 102
    Resurrection,
    /// Prevents infection. M 103
    Sterilize,
    /// Shares HP with willing target. M 103
    ShareVitality,
    /// Restore lost limb instantly. M 101
    RegrowLimb,
    /// Instant energy recovery. M 101
    InstantRecoverEnergy,
    /// Stops aging temporarily. M 103
    StopAging,

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
    /// Hides aura from detection. M 98
    HideAura,
    /// Commands illusions. M 96
    ControlIllusion,
    /// Removes illusions. M 97
    DispelIllusion,
    /// Commands creations. M 96
    ControlCreation,
    /// Removes creations. M 97
    DispelCreation,
    /// Protective illusion barrier. M 99
    IllusionShell,

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
    /// Induces berserk rage. M 119
    Berserk,
    /// Instills courage. M 120
    Bravery,
    /// Stuns target mentally. M 131
    MentalStun,
    /// Mass sleep effect. M 130
    MassSleep,
    /// Psychic damage attack. M 132
    MindWhip,

    // Movement College - M 146-161
    /// Flies through the air. M 152
    Flight,
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
    /// Summons object to hand. M 147
    Poltergeist,
    /// Opens locks remotely. M 155
    Lockmaster,
    /// Throws object with force. M 156
    TelekineticBlow,
    /// Stops movement completely. M 153
    HaltMovement,
    /// Deflects single missile. M 151
    DeflectMissile,
    /// Shields from all missiles. M 155
    MissileShield,
    /// Greatly increases speed. M 153
    GreatHaste,
    /// Reverses missile direction. M 157
    ReverseMissile,
    /// Perfect accuracy with missiles. M 160
    UnerringMissile,
    /// Walks through walls. M 161
    WalkThroughWalls,

    // Necromantic College - M 149-161
    /// Detects corpses and remains. M 149
    DeathVision,
    /// Animates single corpse. M 149
    Zombie,
    /// Animates skeleton. M 159
    Skeleton,
    /// Controls existing undead. M 151
    ControlZombie,
    /// Turns undead away. M 161
    TurnZombie,
    /// Creates permanent zombie. M 149
    PermanentZombie,
    /// Summons spirit. M 160
    SummonSpirit,
    /// Commands summoned spirit. M 160
    ControlSpirit,
    /// Banishes spirit. M 160
    BanishSpirit,
    /// Speaks with dead. M 160
    SpeakWithDead,
    /// Sees spirits and ghosts. M 159
    SpiritVision,
    /// Prevents resurrection. M 157
    PreventResurrection,
    /// Steals HP from target. M 151
    StealHealth,
    /// Ages target rapidly. M 149
    AgeSpell,
    /// Reverses aging. M 161
    Youth,
    /// Preserves corpse. M 157
    PreserveCorpse,
    /// Causes disease. M 149
    Pestilence,
    /// Transforms to undead form. M 150
    LichForm,
    /// Creates spectral servant. M 159
    SummonShade,
    /// Fear of death. M 150
    FearOfDeath,
    /// Sense death nearby. M 152
    SenseDeath,
    /// Drains vitality. M 151
    DrainVitality,
    /// Restores drained vitality. M 158
    RestoreVitality,
    /// Creates ghost. M 150
    CreateGhost,
    /// Binds ghost to location. M 150
    BindSpirit,
    /// Frees bound spirit. M 150
    FreeSpirit,
    /// Raises corpse as intelligent undead. M 158
    CreateVampire,
    /// Destroys undead utterly. M 149
    FinalDeath,
    /// Protects from undead. M 157
    WardAgainstUndead,
    /// Creates death energy. M 149
    NegativeEnergy,
    /// Converts to positive energy. M 157
    PositiveEnergy,
    /// Senses souls. M 159
    SenseSoul,
    /// Tears soul from body. M 160
    SoulTear,
    /// Repairs torn soul. M 158
    RestoreSoul,
    /// Necromantic blast. M 152
    NecromanticBlast,
    /// Withers body parts. M 161
    Wither,
    /// Heals withering. M 158
    RestoreBody,
    /// Mass animation. M 152
    AnimateDead,

    // Plant College - M 155-170
    /// Identifies plant species. M 155
    IdentifyPlant,
    /// Locates specific plant. M 156
    SeekPlantPlant,
    /// Accelerates plant growth. M 156
    PlantGrowth,
    /// Heals damaged plants. M 155
    HealPlant,
    /// Kills plants instantly. M 156
    WitherPlant,
    /// Shapes living wood. M 159
    ShapeWood,
    /// Commands plant creatures. M 155
    ControlPlant,
    /// Animates plants. M 155
    AnimatePlant,
    /// Creates entangling vines. M 156
    Entangle,
    /// Creates thorny barrier. M 159
    ThornWall,
    /// Enhances plant fertility. M 156
    FertileSoil,
    /// Ruins soil fertility. M 156
    Blight,
    /// Creates edible fruit. M 156
    FruitBearing,
    /// Summons plant elemental. M 160
    SummonPlantElemental,
    /// Controls plant elemental. M 155
    ControlPlantElemental,
    /// Creates permanent plant elemental. M 155
    CreatePlantElemental,
    /// Transforms into plant. M 160
    PlantForm,
    /// Speaks with plants. M 159
    SpeakWithPlants,
    /// Senses through plants. M 159
    PlantVision,
    /// Creates mobile plant servant. M 160
    Treant,
    /// Awakens plant intelligence. M 155
    AwakenPlant,
    /// Protects plants from harm. M 158
    ProtectPlant,
    /// Creates poison from plants. M 158
    PoisonExtract,
    /// Purifies plant toxins. M 158
    PurifyPlant,
    /// Creates medicinal herbs. M 157
    HerbLore,
    /// Commands entire forest. M 156
    ForestControl,
    /// Creates impenetrable hedge. M 157
    HedgeWall,
    /// Causes rapid decomposition. M 155
    Decompose,
    /// Prevents plant decomposition. M 158
    PreservePlant,
    /// Creates mobile vine servant. M 160
    VineServant,
    /// Transfers life to plants. M 160
    LifeToPlant,
    /// Extracts life from plants. M 157
    LifeFromPlant,

    // Sound College - M 171-178
    /// Prevents sound in area. M 175
    SilenceSound,
    /// Amplifies voice volume. M 173
    GreatVoice,
    /// Projects voice elsewhere. M 177
    Voices,
    /// Creates repeating echoes. M 172
    Echo,
    /// Perceives via sound waves. M 176
    SoundVision,
    /// Distorts speech clarity. M 173
    Garble,
    /// Creates animal roar. M 175
    Roar,
    /// Produces painful screech. M 176
    Screech,
    /// Muffles all sounds. M 174
    Hush,
    /// Grants enhanced hearing. M 174
    PerfectHearing,
    /// Creates sonic blast. M 176
    SonicBoom,
    /// Directs focused sound. M 176
    SoundJet,
    /// Creates sound barrier. M 176
    SoundWall,
    /// Causes intense vibration. M 177
    Vibration,
    /// Creates chaotic noise. M 172
    Cacophony,
    /// Produces harmonious music. M 173
    Harmony,
    /// Enables quiet communication. M 177
    Whisper,
    /// Blocks sound transmission. M 176
    Soundproof,
    /// Weaponizes sound waves. M 176
    SonicWeapon,
    /// Causes temporary deafness. M 172
    Deafen,
    /// Throws voice remotely. M 177
    Ventriloquism,
    /// Creates musical sounds. M 174
    Music,
    /// Produces rhythmic patterns. M 175
    Rhythm,
    /// Causes resonant frequency. M 175
    Resonance,
    /// Shields from sonic damage. M 176
    SonicShield,

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
    /// Grants damage resistance. M 163
    Armor,
    /// Creates physical barrier wall. M 169
    ForceWall,
    /// Creates physical barrier dome. M 169
    ForceDome,
    /// Physical and magical barrier wall. M 183
    UtterWall,
    /// Physical and magical barrier dome. M 184
    UtterDome,
    /// Blocks spell penetration. M 183
    SpellWall,
    /// Personal anti-spell shield. M 181
    SpellShield,
    /// Magically locks doors. M 170
    Magelock,
    /// Alerts when triggered. M 165
    Alarm,
    /// Creates confusion fog. M 171
    MysticMist,
    /// Blocks teleportation into area. M 182
    TeleportShield,
    /// Protects from weather. M 185
    WeatherDome,
    /// Improves blocking. M 166
    Block,
    /// Protects sleeping area. M 171
    Nightingale,

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
    /// Essence of water element. M 192
    EssentialWater,
    /// See through water clearly. M 200
    WaterVision,
    /// See through ice and fog. M 194
    IceVision,
    /// Creates obscuring fog. M 193
    Fog,
    /// Chills target with frost. M 193
    Frost,
    /// Creates erupting water. M 193
    Geyser,
    /// Hailstorm attack. M 193
    Hail,
    /// Creates spinning water vortex. M 200
    Whirlpool,
    /// Creates water vapor. M 191
    CreateSteam,
    /// Protects from water damage. M 198
    ResistWater,
    /// Breathe air while underwater. M 189
    BreatheAir,

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
    /// Remote viewing through magical eye. M 113
    WizardEye,
    /// Perfect recall of information. M 110
    Recall,
    /// Shows scenes from object's past. M 109
    ImagesOfThePast,
    /// Precise technical measurements. M 110
    Measurement,
    /// Reveals hidden writing. M 111
    RevealSecrets,
    /// Locates specific person. M 112
    SeekPerson,
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

            // Animal College
            Self::BeastSoother => SpellCollege::Animal,
            Self::BeastSpeech => SpellCollege::Animal,
            Self::MammalControl => SpellCollege::Animal,
            Self::BirdControl => SpellCollege::Animal,
            Self::ReptileControl => SpellCollege::Animal,
            Self::FishControl => SpellCollege::Animal,
            Self::InsectControl => SpellCollege::Animal,
            Self::BeastSummoning => SpellCollege::Animal,
            Self::RepelAnimal => SpellCollege::Animal,
            Self::RepelMammal => SpellCollege::Animal,
            Self::RepelBird => SpellCollege::Animal,
            Self::RepelReptile => SpellCollege::Animal,
            Self::RepelFish => SpellCollege::Animal,
            Self::RepelInsect => SpellCollege::Animal,
            Self::Master => SpellCollege::Animal,
            Self::Rider => SpellCollege::Animal,
            Self::Shapeshifting => SpellCollege::Animal,
            Self::BirdShapeshifting => SpellCollege::Animal,
            Self::ReptileShapeshifting => SpellCollege::Animal,
            Self::FishShapeshifting => SpellCollege::Animal,
            Self::InsectShapeshifting => SpellCollege::Animal,
            Self::GreatShapeshifting => SpellCollege::Animal,

            // Communication & Empathy College
            Self::SenseEmotion => SpellCollege::CommunicationEmpathy,
            Self::Persuasion => SpellCollege::CommunicationEmpathy,
            Self::EmotionControl => SpellCollege::CommunicationEmpathy,
            Self::Truthsayer => SpellCollege::CommunicationEmpathy,
            Self::SenseLife => SpellCollege::CommunicationEmpathy,
            Self::SenseFoes => SpellCollege::CommunicationEmpathy,
            Self::LendLanguage => SpellCollege::CommunicationEmpathy,
            Self::BorrowLanguage => SpellCollege::CommunicationEmpathy,
            Self::GiftOfTongues => SpellCollege::CommunicationEmpathy,
            Self::GiftOfLetters => SpellCollege::CommunicationEmpathy,
            Self::Comprehend => SpellCollege::CommunicationEmpathy,
            Self::Translate => SpellCollege::CommunicationEmpathy,
            Self::PermanentTranslation => SpellCollege::CommunicationEmpathy,
            Self::ProjectVoice => SpellCollege::CommunicationEmpathy,
            Self::SilentCommunication => SpellCollege::CommunicationEmpathy,
            Self::LinkMind => SpellCollege::CommunicationEmpathy,

            // Enchantment College
            Self::Enchant => SpellCollege::Enchantment,
            Self::TemporaryEnchantment => SpellCollege::Enchantment,
            Self::Hex => SpellCollege::Enchantment,
            Self::Scroll => SpellCollege::Enchantment,
            Self::Power => SpellCollege::Enchantment,
            Self::Powerstone => SpellCollege::Enchantment,
            Self::RemoveEnchantment => SpellCollege::Enchantment,
            Self::ResistEnchantment => SpellCollege::Enchantment,
            Self::Golem => SpellCollege::Enchantment,
            Self::Malefice => SpellCollege::Enchantment,
            Self::Ensorcel => SpellCollege::Enchantment,
            Self::ImpressionBlocker => SpellCollege::Enchantment,
            Self::LesserWish => SpellCollege::Enchantment,
            Self::Wish => SpellCollege::Enchantment,
            Self::GreatWish => SpellCollege::Enchantment,
            Self::Manastone => SpellCollege::Enchantment,
            Self::Staff => SpellCollege::Enchantment,
            Self::Wand => SpellCollege::Enchantment,
            Self::CrystalBall => SpellCollege::Enchantment,
            Self::SoulStone => SpellCollege::Enchantment,

            // Food College
            Self::Decay => SpellCollege::Food,
            Self::TestFood => SpellCollege::Food,
            Self::PurifyFood => SpellCollege::Food,
            Self::CreateFood => SpellCollege::Food,
            Self::PreserveFood => SpellCollege::Food,
            Self::Cook => SpellCollege::Food,
            Self::Season => SpellCollege::Food,
            Self::Flavor => SpellCollege::Food,
            Self::PrepareGame => SpellCollege::Food,
            Self::PoisonFood => SpellCollege::Food,
            Self::KnowRecipe => SpellCollege::Food,
            Self::Mature => SpellCollege::Food,
            Self::Banquet => SpellCollege::Food,
            Self::EssentialFood => SpellCollege::Food,
            Self::Distill => SpellCollege::Food,
            Self::Ferment => SpellCollege::Food,
            Self::WaterToWine => SpellCollege::Food,
            Self::Hunger => SpellCollege::Food,
            Self::Thirst => SpellCollege::Food,
            Self::Rotproof => SpellCollege::Food,

            // Gate College
            Self::Teleport => SpellCollege::Gate,
            Self::Blink => SpellCollege::Gate,
            Self::Apportation => SpellCollege::Gate,
            Self::Levitation => SpellCollege::Gate,
            Self::TeleportOther => SpellCollege::Gate,
            Self::Beacon => SpellCollege::Gate,
            Self::TraceTeleport => SpellCollege::Gate,
            Self::DivertTeleport => SpellCollege::Gate,
            Self::PlanarSummons => SpellCollege::Gate,
            Self::PlanarVisit => SpellCollege::Gate,
            Self::PlaneShift => SpellCollege::Gate,
            Self::PlaneShiftOther => SpellCollege::Gate,
            Self::Phase => SpellCollege::Gate,
            Self::PhaseOther => SpellCollege::Gate,
            Self::SeekGate => SpellCollege::Gate,
            Self::ControlGate => SpellCollege::Gate,
            Self::CreateGate => SpellCollege::Gate,
            Self::ScryGate => SpellCollege::Gate,
            Self::HideObject => SpellCollege::Gate,
            Self::Sanctuary => SpellCollege::Gate,

            // Light & Darkness College
            Self::Light => SpellCollege::LightDarkness,
            Self::Darkness => SpellCollege::LightDarkness,
            Self::ContinualLight => SpellCollege::LightDarkness,
            Self::Colors => SpellCollege::LightDarkness,
            Self::RemoveShadow => SpellCollege::LightDarkness,
            Self::NightVision => SpellCollege::LightDarkness,
            Self::DarkVision => SpellCollege::LightDarkness,
            Self::Blur => SpellCollege::LightDarkness,
            Self::SeeInvisible => SpellCollege::LightDarkness,
            Self::Mirror => SpellCollege::LightDarkness,
            Self::LightJet => SpellCollege::LightDarkness,
            Self::Blackout => SpellCollege::LightDarkness,
            Self::WallOfLight => SpellCollege::LightDarkness,
            Self::WallOfDarkness => SpellCollege::LightDarkness,
            Self::Flash => SpellCollege::LightDarkness,
            Self::BodyOfShadow => SpellCollege::LightDarkness,
            Self::ShadowForm => SpellCollege::LightDarkness,
            Self::Infravision => SpellCollege::LightDarkness,

            // Making & Breaking College
            Self::Repair => SpellCollege::MakingBreaking,
            Self::Shatter => SpellCollege::MakingBreaking,
            Self::Stiffen => SpellCollege::MakingBreaking,
            Self::Soften => SpellCollege::MakingBreaking,
            Self::Restore => SpellCollege::MakingBreaking,
            Self::Rust => SpellCollege::MakingBreaking,
            Self::TransformObject => SpellCollege::MakingBreaking,
            Self::Copy => SpellCollege::MakingBreaking,
            Self::Dye => SpellCollege::MakingBreaking,
            Self::Knot => SpellCollege::MakingBreaking,
            Self::Fasten => SpellCollege::MakingBreaking,
            Self::ExtendObject => SpellCollege::MakingBreaking,
            Self::ShrinkObject => SpellCollege::MakingBreaking,
            Self::Polish => SpellCollege::MakingBreaking,
            Self::Explode => SpellCollege::MakingBreaking,
            Self::FindWeakness => SpellCollege::MakingBreaking,
            Self::CreateObject => SpellCollege::MakingBreaking,
            Self::DestroyObject => SpellCollege::MakingBreaking,
            Self::Rebuild => SpellCollege::MakingBreaking,
            Self::Inscribe => SpellCollege::MakingBreaking,
            Self::AnimateObject => SpellCollege::MakingBreaking,

            // Meta-Spells College
            Self::Counterspell => SpellCollege::MetaSpells,
            Self::DispelMagic => SpellCollege::MetaSpells,
            Self::SuspendSpell => SpellCollege::MetaSpells,
            Self::MaintainSpell => SpellCollege::MetaSpells,
            Self::ExtendSpell => SpellCollege::MetaSpells,
            Self::HasteSpell => SpellCollege::MetaSpells,
            Self::Link => SpellCollege::MetaSpells,
            Self::Permanency => SpellCollege::MetaSpells,
            Self::StealEnergy => SpellCollege::MetaSpells,
            Self::DrawPower => SpellCollege::MetaSpells,
            Self::SpellStone => SpellCollege::MetaSpells,
            Self::Reflect => SpellCollege::MetaSpells,
            Self::Ward => SpellCollege::MetaSpells,
            Self::Augment => SpellCollege::MetaSpells,
            Self::Diminish => SpellCollege::MetaSpells,
            Self::Delay => SpellCollege::MetaSpells,

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
            Self::EssentialFlame => SpellCollege::Fire,
            Self::BodyOfFire => SpellCollege::Fire,
            Self::FireVision => SpellCollege::Fire,
            Self::Warmth => SpellCollege::Fire,
            Self::DeflectEnergy => SpellCollege::Fire,
            Self::ControlFireElemental => SpellCollege::Fire,
            Self::CreateFireElemental => SpellCollege::Fire,

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
            Self::Resurrection => SpellCollege::Healing,
            Self::Sterilize => SpellCollege::Healing,
            Self::ShareVitality => SpellCollege::Healing,
            Self::RegrowLimb => SpellCollege::Healing,
            Self::InstantRecoverEnergy => SpellCollege::Healing,
            Self::StopAging => SpellCollege::Healing,

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
            Self::HideAura => SpellCollege::IllusionCreation,
            Self::ControlIllusion => SpellCollege::IllusionCreation,
            Self::DispelIllusion => SpellCollege::IllusionCreation,
            Self::ControlCreation => SpellCollege::IllusionCreation,
            Self::DispelCreation => SpellCollege::IllusionCreation,
            Self::IllusionShell => SpellCollege::IllusionCreation,

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
            Self::Berserk => SpellCollege::MindControl,
            Self::Bravery => SpellCollege::MindControl,
            Self::MentalStun => SpellCollege::MindControl,
            Self::MassSleep => SpellCollege::MindControl,
            Self::MindWhip => SpellCollege::MindControl,

            // Movement College
            Self::Flight => SpellCollege::Movement,
            Self::Teleportation => SpellCollege::Movement,
            Self::HasteMovement => SpellCollege::Movement,
            Self::SlowMovement => SpellCollege::Movement,
            Self::Cling => SpellCollege::Movement,
            Self::Jump => SpellCollege::Movement,
            Self::Poltergeist => SpellCollege::Movement,
            Self::Lockmaster => SpellCollege::Movement,
            Self::TelekineticBlow => SpellCollege::Movement,
            Self::HaltMovement => SpellCollege::Movement,
            Self::DeflectMissile => SpellCollege::Movement,
            Self::MissileShield => SpellCollege::Movement,
            Self::GreatHaste => SpellCollege::Movement,
            Self::ReverseMissile => SpellCollege::Movement,
            Self::UnerringMissile => SpellCollege::Movement,
            Self::WalkThroughWalls => SpellCollege::Movement,

            // Necromantic College
            Self::DeathVision => SpellCollege::Necromantic,
            Self::Zombie => SpellCollege::Necromantic,
            Self::Skeleton => SpellCollege::Necromantic,
            Self::ControlZombie => SpellCollege::Necromantic,
            Self::TurnZombie => SpellCollege::Necromantic,
            Self::PermanentZombie => SpellCollege::Necromantic,
            Self::SummonSpirit => SpellCollege::Necromantic,
            Self::ControlSpirit => SpellCollege::Necromantic,
            Self::BanishSpirit => SpellCollege::Necromantic,
            Self::SpeakWithDead => SpellCollege::Necromantic,
            Self::SpiritVision => SpellCollege::Necromantic,
            Self::PreventResurrection => SpellCollege::Necromantic,
            Self::StealHealth => SpellCollege::Necromantic,
            Self::AgeSpell => SpellCollege::Necromantic,
            Self::Youth => SpellCollege::Necromantic,
            Self::PreserveCorpse => SpellCollege::Necromantic,
            Self::Pestilence => SpellCollege::Necromantic,
            Self::LichForm => SpellCollege::Necromantic,
            Self::SummonShade => SpellCollege::Necromantic,
            Self::FearOfDeath => SpellCollege::Necromantic,
            Self::SenseDeath => SpellCollege::Necromantic,
            Self::DrainVitality => SpellCollege::Necromantic,
            Self::RestoreVitality => SpellCollege::Necromantic,
            Self::CreateGhost => SpellCollege::Necromantic,
            Self::BindSpirit => SpellCollege::Necromantic,
            Self::FreeSpirit => SpellCollege::Necromantic,
            Self::CreateVampire => SpellCollege::Necromantic,
            Self::FinalDeath => SpellCollege::Necromantic,
            Self::WardAgainstUndead => SpellCollege::Necromantic,
            Self::NegativeEnergy => SpellCollege::Necromantic,
            Self::PositiveEnergy => SpellCollege::Necromantic,
            Self::SenseSoul => SpellCollege::Necromantic,
            Self::SoulTear => SpellCollege::Necromantic,
            Self::RestoreSoul => SpellCollege::Necromantic,
            Self::NecromanticBlast => SpellCollege::Necromantic,
            Self::Wither => SpellCollege::Necromantic,
            Self::RestoreBody => SpellCollege::Necromantic,
            Self::AnimateDead => SpellCollege::Necromantic,

            // Plant College
            Self::IdentifyPlant => SpellCollege::Plant,
            Self::SeekPlantPlant => SpellCollege::Plant,
            Self::PlantGrowth => SpellCollege::Plant,
            Self::HealPlant => SpellCollege::Plant,
            Self::WitherPlant => SpellCollege::Plant,
            Self::ShapeWood => SpellCollege::Plant,
            Self::ControlPlant => SpellCollege::Plant,
            Self::AnimatePlant => SpellCollege::Plant,
            Self::Entangle => SpellCollege::Plant,
            Self::ThornWall => SpellCollege::Plant,
            Self::FertileSoil => SpellCollege::Plant,
            Self::Blight => SpellCollege::Plant,
            Self::FruitBearing => SpellCollege::Plant,
            Self::SummonPlantElemental => SpellCollege::Plant,
            Self::ControlPlantElemental => SpellCollege::Plant,
            Self::CreatePlantElemental => SpellCollege::Plant,
            Self::PlantForm => SpellCollege::Plant,
            Self::SpeakWithPlants => SpellCollege::Plant,
            Self::PlantVision => SpellCollege::Plant,
            Self::Treant => SpellCollege::Plant,
            Self::AwakenPlant => SpellCollege::Plant,
            Self::ProtectPlant => SpellCollege::Plant,
            Self::PoisonExtract => SpellCollege::Plant,
            Self::PurifyPlant => SpellCollege::Plant,
            Self::HerbLore => SpellCollege::Plant,
            Self::ForestControl => SpellCollege::Plant,
            Self::HedgeWall => SpellCollege::Plant,
            Self::Decompose => SpellCollege::Plant,
            Self::PreservePlant => SpellCollege::Plant,
            Self::VineServant => SpellCollege::Plant,
            Self::LifeToPlant => SpellCollege::Plant,
            Self::LifeFromPlant => SpellCollege::Plant,

            // Sound College
            Self::SilenceSound => SpellCollege::Sound,
            Self::GreatVoice => SpellCollege::Sound,
            Self::Voices => SpellCollege::Sound,
            Self::Echo => SpellCollege::Sound,
            Self::SoundVision => SpellCollege::Sound,
            Self::Garble => SpellCollege::Sound,
            Self::Roar => SpellCollege::Sound,
            Self::Screech => SpellCollege::Sound,
            Self::Hush => SpellCollege::Sound,
            Self::PerfectHearing => SpellCollege::Sound,
            Self::SonicBoom => SpellCollege::Sound,
            Self::SoundJet => SpellCollege::Sound,
            Self::SoundWall => SpellCollege::Sound,
            Self::Vibration => SpellCollege::Sound,
            Self::Cacophony => SpellCollege::Sound,
            Self::Harmony => SpellCollege::Sound,
            Self::Whisper => SpellCollege::Sound,
            Self::Soundproof => SpellCollege::Sound,
            Self::SonicWeapon => SpellCollege::Sound,
            Self::Deafen => SpellCollege::Sound,
            Self::Ventriloquism => SpellCollege::Sound,
            Self::Music => SpellCollege::Sound,
            Self::Rhythm => SpellCollege::Sound,
            Self::Resonance => SpellCollege::Sound,
            Self::SonicShield => SpellCollege::Sound,

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
            Self::Armor => SpellCollege::ProtectionWarning,
            Self::ForceWall => SpellCollege::ProtectionWarning,
            Self::ForceDome => SpellCollege::ProtectionWarning,
            Self::UtterWall => SpellCollege::ProtectionWarning,
            Self::UtterDome => SpellCollege::ProtectionWarning,
            Self::SpellWall => SpellCollege::ProtectionWarning,
            Self::SpellShield => SpellCollege::ProtectionWarning,
            Self::Magelock => SpellCollege::ProtectionWarning,
            Self::Alarm => SpellCollege::ProtectionWarning,
            Self::MysticMist => SpellCollege::ProtectionWarning,
            Self::TeleportShield => SpellCollege::ProtectionWarning,
            Self::WeatherDome => SpellCollege::ProtectionWarning,
            Self::Block => SpellCollege::ProtectionWarning,
            Self::Nightingale => SpellCollege::ProtectionWarning,

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
            Self::EssentialWater => SpellCollege::Water,
            Self::WaterVision => SpellCollege::Water,
            Self::IceVision => SpellCollege::Water,
            Self::Fog => SpellCollege::Water,
            Self::Frost => SpellCollege::Water,
            Self::Geyser => SpellCollege::Water,
            Self::Hail => SpellCollege::Water,
            Self::Whirlpool => SpellCollege::Water,
            Self::CreateSteam => SpellCollege::Water,
            Self::ResistWater => SpellCollege::Water,
            Self::BreatheAir => SpellCollege::Water,

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
            Self::WizardEye => SpellCollege::Knowledge,
            Self::Recall => SpellCollege::Knowledge,
            Self::ImagesOfThePast => SpellCollege::Knowledge,
            Self::Measurement => SpellCollege::Knowledge,
            Self::RevealSecrets => SpellCollege::Knowledge,
            Self::SeekPerson => SpellCollege::Knowledge,
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
            SpellCollege::Animal => return animal::base_energy_cost(self),
            SpellCollege::BodyControl => return body_control::base_energy_cost(self),
            SpellCollege::CommunicationEmpathy => return communication_empathy::base_energy_cost(self),
            SpellCollege::Enchantment => return enchantment::base_energy_cost(self),
            SpellCollege::Earth => return earth::base_energy_cost(self),
            SpellCollege::Fire => return fire::base_energy_cost(self),
            SpellCollege::Food => return food::base_energy_cost(self),
            SpellCollege::Gate => return gate::base_energy_cost(self),
            SpellCollege::Healing => return healing::base_energy_cost(self),
            SpellCollege::IllusionCreation => return illusion_creation::base_energy_cost(self),
            SpellCollege::Knowledge => return knowledge::base_energy_cost(self),
            SpellCollege::LightDarkness => return light_darkness::base_energy_cost(self),
            SpellCollege::MakingBreaking => return making_breaking::base_energy_cost(self),
            SpellCollege::MetaSpells => return meta_spells::base_energy_cost(self),
            SpellCollege::MindControl => return mind_control::base_energy_cost(self),
            SpellCollege::Necromantic => return necromantic::base_energy_cost(self),
            SpellCollege::Plant => return plant::base_energy_cost(self),
            SpellCollege::Sound => return sound::base_energy_cost(self),
            SpellCollege::Movement => return movement::base_energy_cost(self),
            SpellCollege::ProtectionWarning => return protection_warning::base_energy_cost(self),
            SpellCollege::Water => return water::base_energy_cost(self),
            _ => panic!("Unimplemented college: {:?}", self.college()),
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
            SpellCollege::Animal => return animal::casting_time(self),
            SpellCollege::BodyControl => return body_control::casting_time(self),
            SpellCollege::CommunicationEmpathy => return communication_empathy::casting_time(self),
            SpellCollege::Enchantment => return enchantment::casting_time(self),
            SpellCollege::Earth => return earth::casting_time(self),
            SpellCollege::Fire => return fire::casting_time(self),
            SpellCollege::Food => return food::casting_time(self),
            SpellCollege::Gate => return gate::casting_time(self),
            SpellCollege::Healing => return healing::casting_time(self),
            SpellCollege::IllusionCreation => return illusion_creation::casting_time(self),
            SpellCollege::Knowledge => return knowledge::casting_time(self),
            SpellCollege::LightDarkness => return light_darkness::casting_time(self),
            SpellCollege::MakingBreaking => return making_breaking::casting_time(self),
            SpellCollege::MetaSpells => return meta_spells::casting_time(self),
            SpellCollege::MindControl => return mind_control::casting_time(self),
            SpellCollege::Necromantic => return necromantic::casting_time(self),
            SpellCollege::Plant => return plant::casting_time(self),
            SpellCollege::Sound => return sound::casting_time(self),
            SpellCollege::Movement => return movement::casting_time(self),
            SpellCollege::ProtectionWarning => return protection_warning::casting_time(self),
            SpellCollege::Water => return water::casting_time(self),
            _ => panic!("Unimplemented college: {:?}", self.college()),
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
            SpellCollege::Animal => return animal::duration(self),
            SpellCollege::BodyControl => return body_control::duration(self),
            SpellCollege::CommunicationEmpathy => return communication_empathy::duration(self),
            SpellCollege::Enchantment => return enchantment::duration(self),
            SpellCollege::Earth => return earth::duration(self),
            SpellCollege::Fire => return fire::duration(self),
            SpellCollege::Food => return food::duration(self),
            SpellCollege::Gate => return gate::duration(self),
            SpellCollege::Healing => return healing::duration(self),
            SpellCollege::IllusionCreation => return illusion_creation::duration(self),
            SpellCollege::Knowledge => return knowledge::duration(self),
            SpellCollege::LightDarkness => return light_darkness::duration(self),
            SpellCollege::MakingBreaking => return making_breaking::duration(self),
            SpellCollege::MetaSpells => return meta_spells::duration(self),
            SpellCollege::MindControl => return mind_control::duration(self),
            SpellCollege::Necromantic => return necromantic::duration(self),
            SpellCollege::Plant => return plant::duration(self),
            SpellCollege::Sound => return sound::duration(self),
            SpellCollege::Movement => return movement::duration(self),
            SpellCollege::ProtectionWarning => return protection_warning::duration(self),
            SpellCollege::Water => return water::duration(self),
            _ => panic!("Unimplemented college: {:?}", self.college()),
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
            SpellCollege::Animal => return animal::prerequisites(self),
            SpellCollege::BodyControl => return body_control::prerequisites(self),
            SpellCollege::CommunicationEmpathy => return communication_empathy::prerequisites(self),
            SpellCollege::Enchantment => return enchantment::prerequisites(self),
            SpellCollege::Earth => return earth::prerequisites(self),
            SpellCollege::Fire => return fire::prerequisites(self),
            SpellCollege::Food => return food::prerequisites(self),
            SpellCollege::Gate => return gate::prerequisites(self),
            SpellCollege::Healing => return healing::prerequisites(self),
            SpellCollege::IllusionCreation => return illusion_creation::prerequisites(self),
            SpellCollege::Knowledge => return knowledge::prerequisites(self),
            SpellCollege::LightDarkness => return light_darkness::prerequisites(self),
            SpellCollege::MakingBreaking => return making_breaking::prerequisites(self),
            SpellCollege::MetaSpells => return meta_spells::prerequisites(self),
            SpellCollege::MindControl => return mind_control::prerequisites(self),
            SpellCollege::Necromantic => return necromantic::prerequisites(self),
            SpellCollege::Plant => return plant::prerequisites(self),
            SpellCollege::Sound => return sound::prerequisites(self),
            SpellCollege::Movement => return movement::prerequisites(self),
            SpellCollege::ProtectionWarning => return protection_warning::prerequisites(self),
            SpellCollege::Water => return water::prerequisites(self),
            _ => panic!("Unimplemented college: {:?}", self.college()),
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
            SpellCollege::Animal => return animal::spell_type(self),
            SpellCollege::BodyControl => return body_control::spell_type(self),
            SpellCollege::CommunicationEmpathy => return communication_empathy::spell_type(self),
            SpellCollege::Enchantment => return enchantment::spell_type(self),
            SpellCollege::Earth => return earth::spell_type(self),
            SpellCollege::Fire => return fire::spell_type(self),
            SpellCollege::Food => return food::spell_type(self),
            SpellCollege::Gate => return gate::spell_type(self),
            SpellCollege::Healing => return healing::spell_type(self),
            SpellCollege::IllusionCreation => return illusion_creation::spell_type(self),
            SpellCollege::Knowledge => return knowledge::spell_type(self),
            SpellCollege::LightDarkness => return light_darkness::spell_type(self),
            SpellCollege::MakingBreaking => return making_breaking::spell_type(self),
            SpellCollege::MetaSpells => return meta_spells::spell_type(self),
            SpellCollege::MindControl => return mind_control::spell_type(self),
            SpellCollege::Necromantic => return necromantic::spell_type(self),
            SpellCollege::Plant => return plant::spell_type(self),
            SpellCollege::Sound => return sound::spell_type(self),
            SpellCollege::Movement => return movement::spell_type(self),
            SpellCollege::ProtectionWarning => return protection_warning::spell_type(self),
            SpellCollege::Water => return water::spell_type(self),
            _ => panic!("Unimplemented college: {:?}", self.college()),
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
            SpellCollege::Animal => return animal::resistance(self),
            SpellCollege::BodyControl => return body_control::resistance(self),
            SpellCollege::CommunicationEmpathy => return communication_empathy::resistance(self),
            SpellCollege::Enchantment => return enchantment::resistance(self),
            SpellCollege::Earth => return earth::resistance(self),
            SpellCollege::Fire => return fire::resistance(self),
            SpellCollege::Food => return food::resistance(self),
            SpellCollege::Gate => return gate::resistance(self),
            SpellCollege::Healing => return healing::resistance(self),
            SpellCollege::IllusionCreation => return illusion_creation::resistance(self),
            SpellCollege::Knowledge => return knowledge::resistance(self),
            SpellCollege::LightDarkness => return light_darkness::resistance(self),
            SpellCollege::MakingBreaking => return making_breaking::resistance(self),
            SpellCollege::MetaSpells => return meta_spells::resistance(self),
            SpellCollege::MindControl => return mind_control::resistance(self),
            SpellCollege::Necromantic => return necromantic::resistance(self),
            SpellCollege::Plant => return plant::resistance(self),
            SpellCollege::Sound => return sound::resistance(self),
            SpellCollege::Movement => return movement::resistance(self),
            SpellCollege::ProtectionWarning => return protection_warning::resistance(self),
            SpellCollege::Water => return water::resistance(self),
            _ => panic!("Unimplemented college: {:?}", self.college()),
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
            SpellCollege::Animal => return animal::reference(self),
            SpellCollege::BodyControl => return body_control::reference(self),
            SpellCollege::CommunicationEmpathy => return communication_empathy::reference(self),
            SpellCollege::Enchantment => return enchantment::reference(self),
            SpellCollege::Earth => return earth::reference(self),
            SpellCollege::Fire => return fire::reference(self),
            SpellCollege::Food => return food::reference(self),
            SpellCollege::Gate => return gate::reference(self),
            SpellCollege::Healing => return healing::reference(self),
            SpellCollege::IllusionCreation => return illusion_creation::reference(self),
            SpellCollege::Knowledge => return knowledge::reference(self),
            SpellCollege::LightDarkness => return light_darkness::reference(self),
            SpellCollege::MakingBreaking => return making_breaking::reference(self),
            SpellCollege::MetaSpells => return meta_spells::reference(self),
            SpellCollege::MindControl => return mind_control::reference(self),
            SpellCollege::Necromantic => return necromantic::reference(self),
            SpellCollege::Plant => return plant::reference(self),
            SpellCollege::Sound => return sound::reference(self),
            SpellCollege::Movement => return movement::reference(self),
            SpellCollege::ProtectionWarning => return protection_warning::reference(self),
            SpellCollege::Water => return water::reference(self),
            _ => panic!("Unimplemented college: {:?}", self.college()),
        }
    }
}

