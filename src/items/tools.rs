//! Tool definitions and properties.
//!
//! # GURPS Rules
//!
//! Tools provide bonuses to specific tasks or enable certain actions.
//! Common tools include lockpicks, first aid kits, toolkits, rope,
//! grapnels, crowbars, and specialized equipment.
//!
//! # Citations
//!
//! BS 289 - Tools and equipment

use crate::{Currency, TechLevel, Weight};
use tracing::{debug, instrument};

/// Tool types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Tool {
    /// Abacus, $25. LT (Calculating device, TL1)
    Abacus,
    /// Anvil, $100. LT (Blacksmithing anvil, TL1)
    Anvil,
    /// Awl, $3. LT (Leatherworking piercer, TL0)
    Awl,
    /// Axe, $40. LT (Wood chopping tool, TL0)
    Axe,
    /// Balance scales, $100. LT (Weighing device, TL1)
    BalanceScales,
    /// Bellows, $30. LT (Forge air pump, TL1)
    Bellows,
    /// Block and tackle, $50. BS 289
    BlockAndTackle,
    /// Chain (10 ft), $80. LT (Heavy duty, TL2)
    Chain,
    /// Chisel, $5. BS 289
    Chisel,
    /// Compass, $50. BS 289
    Compass,
    /// Crowbar, $20. BS 289
    Crowbar,
    /// Draw knife, $8. LT (Woodworking tool, TL1)
    DrawKnife,
    /// File, $5. BS 289
    File,
    /// First aid kit, $50. BS 289
    FirstAidKit,
    /// Grapnel, $20. BS 289
    Grapnel,
    /// Grindstone, $50. LT (Wheel-mounted sharpening stone, TL1)
    Grindstone,
    /// Hammer, $15. BS 289
    Hammer,
    /// Hoe, $12. LT (Farming tool, TL0)
    Hoe,
    /// Hourglass, $75. LT (Timekeeping device, TL3)
    Hourglass,
    /// Ladder, $30. BS 289
    Ladder,
    /// Lever bar, $25. LT (Prying tool, TL1)
    LeverBar,
    /// Loom frame, $150. LT (Weaving loom, TL1)
    LoomFrame,
    /// Lockpicks, $50. BS 289
    Lockpicks,
    /// Magnifying glass, $100. BS 289
    MagnifyingGlass,
    /// Mattock, $15. LT (Digging/breaking tool, TL0)
    Mattock,
    /// Parchment (10 sheets), $20. BS 289
    Parchment,
    /// Pickaxe, $15. BS 289
    Pickaxe,
    /// Plane, $25. LT (Woodworking tool, TL2)
    Plane,
    /// Pliers, $15. BS 289
    Pliers,
    /// Plow, $100. LT (Field preparation, TL0)
    Plow,
    /// Potters wheel, $80. LT (Pottery wheel, TL1)
    PottersWheel,
    /// Quill and ink, $10. BS 289
    QuillAndInk,
    /// Rope, $5. BS 289
    Rope,
    /// Saw, $150. BS 289
    Saw,
    /// Scythe, $15. LT (Grain harvesting, TL1)
    Scythe,
    /// Screwdriver, $5. BS 289
    Screwdriver,
    /// Sealing wax, $5. LT (Document sealing, TL2)
    SealingWax,
    /// Shovel, $12. BS 289
    Shovel,
    /// Sickle, $8. LT (Small harvesting tool, TL0)
    Sickle,
    /// Spade, $12. LT (Digging tool, TL0)
    Spade,
    /// Spinning wheel, $120. LT (Thread making, TL2)
    SpinningWheel,
    /// Tongs, $10. LT (Forge/gripping tool, TL1)
    Tongs,
    /// Tool hatchet, $40. BS 289
    ToolHatchet,
    /// Toolkit, $200. BS 289
    Toolkit,
    /// Wedge, $5. LT (Splitting tool, TL0)
    Wedge,
    /// Whetstone, $5. LT (Sharpening stone, TL0)
    Whetstone,
    /// Wrench, $20. BS 289
    Wrench,
}

impl Tool {
    /// Returns base cost for this tool.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting tool base cost");
        match self {
            Self::Abacus => Currency::dollars(25.0),
            Self::Anvil => Currency::dollars(100.0),
            Self::Awl => Currency::dollars(3.0),
            Self::Axe => Currency::dollars(40.0),
            Self::BalanceScales => Currency::dollars(100.0),
            Self::Bellows => Currency::dollars(30.0),
            Self::BlockAndTackle => Currency::dollars(50.0),
            Self::Chain => Currency::dollars(80.0),
            Self::Chisel => Currency::dollars(5.0),
            Self::Compass => Currency::dollars(50.0),
            Self::Crowbar => Currency::dollars(20.0),
            Self::DrawKnife => Currency::dollars(8.0),
            Self::File => Currency::dollars(5.0),
            Self::FirstAidKit => Currency::dollars(50.0),
            Self::Grapnel => Currency::dollars(20.0),
            Self::Grindstone => Currency::dollars(50.0),
            Self::Hammer => Currency::dollars(15.0),
            Self::Hoe => Currency::dollars(12.0),
            Self::Hourglass => Currency::dollars(75.0),
            Self::Ladder => Currency::dollars(30.0),
            Self::LeverBar => Currency::dollars(25.0),
            Self::LoomFrame => Currency::dollars(150.0),
            Self::Lockpicks => Currency::dollars(50.0),
            Self::MagnifyingGlass => Currency::dollars(100.0),
            Self::Mattock => Currency::dollars(15.0),
            Self::Parchment => Currency::dollars(20.0),
            Self::Pickaxe => Currency::dollars(15.0),
            Self::Plane => Currency::dollars(25.0),
            Self::Pliers => Currency::dollars(15.0),
            Self::Plow => Currency::dollars(100.0),
            Self::PottersWheel => Currency::dollars(80.0),
            Self::QuillAndInk => Currency::dollars(10.0),
            Self::Rope => Currency::dollars(5.0),
            Self::Saw => Currency::dollars(150.0),
            Self::Scythe => Currency::dollars(15.0),
            Self::Screwdriver => Currency::dollars(5.0),
            Self::SealingWax => Currency::dollars(5.0),
            Self::Shovel => Currency::dollars(12.0),
            Self::Sickle => Currency::dollars(8.0),
            Self::Spade => Currency::dollars(12.0),
            Self::SpinningWheel => Currency::dollars(120.0),
            Self::Tongs => Currency::dollars(10.0),
            Self::ToolHatchet => Currency::dollars(40.0),
            Self::Toolkit => Currency::dollars(200.0),
            Self::Wedge => Currency::dollars(5.0),
            Self::Whetstone => Currency::dollars(5.0),
            Self::Wrench => Currency::dollars(20.0),
        }
    }

    /// Returns weight for this tool.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting tool weight");
        match self {
            Self::Abacus => Weight::pounds(2.0),
            Self::Anvil => Weight::pounds(50.0),
            Self::Awl => Weight::pounds(0.1),
            Self::Axe => Weight::pounds(4.0),
            Self::BalanceScales => Weight::pounds(5.0),
            Self::Bellows => Weight::pounds(8.0),
            Self::BlockAndTackle => Weight::pounds(4.0),
            Self::Chain => Weight::pounds(20.0),
            Self::Chisel => Weight::pounds(0.5),
            Self::Compass => Weight::pounds(0.5),
            Self::Crowbar => Weight::pounds(3.0),
            Self::DrawKnife => Weight::pounds(1.0),
            Self::File => Weight::pounds(0.5),
            Self::FirstAidKit => Weight::pounds(2.0),
            Self::Grapnel => Weight::pounds(2.0),
            Self::Grindstone => Weight::pounds(30.0),
            Self::Hammer => Weight::pounds(2.0),
            Self::Hoe => Weight::pounds(5.0),
            Self::Hourglass => Weight::pounds(1.0),
            Self::Ladder => Weight::pounds(20.0),
            Self::LeverBar => Weight::pounds(5.0),
            Self::LoomFrame => Weight::pounds(40.0),
            Self::Lockpicks => Weight::pounds(0.1),
            Self::MagnifyingGlass => Weight::pounds(0.25),
            Self::Mattock => Weight::pounds(6.0),
            Self::Parchment => Weight::pounds(0.5),
            Self::Pickaxe => Weight::pounds(5.0),
            Self::Plane => Weight::pounds(2.0),
            Self::Pliers => Weight::pounds(1.0),
            Self::Plow => Weight::pounds(60.0),
            Self::PottersWheel => Weight::pounds(50.0),
            Self::QuillAndInk => Weight::pounds(0.1),
            Self::Rope => Weight::pounds(1.5),
            Self::Saw => Weight::pounds(3.0),
            Self::Scythe => Weight::pounds(5.0),
            Self::Screwdriver => Weight::pounds(0.25),
            Self::SealingWax => Weight::pounds(0.1),
            Self::Shovel => Weight::pounds(6.0),
            Self::Sickle => Weight::pounds(2.0),
            Self::Spade => Weight::pounds(5.0),
            Self::SpinningWheel => Weight::pounds(30.0),
            Self::Tongs => Weight::pounds(2.0),
            Self::ToolHatchet => Weight::pounds(2.0),
            Self::Toolkit => Weight::pounds(10.0),
            Self::Wedge => Weight::pounds(3.0),
            Self::Whetstone => Weight::pounds(1.0),
            Self::Wrench => Weight::pounds(2.0),
        }
    }

    /// Returns tech level for this tool.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting tool tech level");
        match self {
            Self::Abacus => TechLevel::new(1),          // Bronze Age
            Self::Anvil => TechLevel::new(1),           // Bronze Age
            Self::Awl => TechLevel::new(0),             // Stone Age
            Self::Axe => TechLevel::new(0),             // Stone Age
            Self::BalanceScales => TechLevel::new(1),   // Bronze Age
            Self::Bellows => TechLevel::new(1),         // Bronze Age
            Self::BlockAndTackle => TechLevel::new(1),  // Bronze Age
            Self::Chain => TechLevel::new(2),           // Medieval
            Self::Chisel => TechLevel::new(0),          // Stone Age
            Self::Compass => TechLevel::new(4),         // Age of Sail
            Self::Crowbar => TechLevel::new(1),         // Bronze Age
            Self::DrawKnife => TechLevel::new(1),       // Bronze Age
            Self::File => TechLevel::new(2),            // Medieval
            Self::FirstAidKit => TechLevel::new(5),     // Industrial
            Self::Grapnel => TechLevel::new(2),         // Medieval
            Self::Grindstone => TechLevel::new(1),      // Bronze Age
            Self::Hammer => TechLevel::new(0),          // Stone Age
            Self::Hoe => TechLevel::new(0),             // Stone Age
            Self::Hourglass => TechLevel::new(3),       // Medieval
            Self::Ladder => TechLevel::new(0),          // Stone Age
            Self::LeverBar => TechLevel::new(1),        // Bronze Age
            Self::LoomFrame => TechLevel::new(1),       // Bronze Age
            Self::Lockpicks => TechLevel::new(3),       // Medieval/Renaissance
            Self::MagnifyingGlass => TechLevel::new(4), // Renaissance
            Self::Mattock => TechLevel::new(0),         // Stone Age
            Self::Parchment => TechLevel::new(2),       // Medieval
            Self::Pickaxe => TechLevel::new(0),         // Stone Age
            Self::Plane => TechLevel::new(2),           // Medieval
            Self::Pliers => TechLevel::new(3),          // Renaissance
            Self::Plow => TechLevel::new(0),            // Stone Age
            Self::PottersWheel => TechLevel::new(1),    // Bronze Age
            Self::QuillAndInk => TechLevel::new(2),     // Medieval
            Self::Rope => TechLevel::new(0),            // Stone Age
            Self::Saw => TechLevel::new(2),             // Medieval
            Self::Scythe => TechLevel::new(1),          // Bronze Age
            Self::Screwdriver => TechLevel::new(3),     // Renaissance
            Self::SealingWax => TechLevel::new(2),      // Medieval
            Self::Shovel => TechLevel::new(1),          // Bronze Age
            Self::Sickle => TechLevel::new(0),          // Stone Age
            Self::Spade => TechLevel::new(0),           // Stone Age
            Self::SpinningWheel => TechLevel::new(2),   // Medieval
            Self::Tongs => TechLevel::new(1),           // Bronze Age
            Self::ToolHatchet => TechLevel::new(0),     // Stone Age
            Self::Toolkit => TechLevel::new(1),         // Bronze Age (basic tools)
            Self::Wedge => TechLevel::new(0),           // Stone Age
            Self::Whetstone => TechLevel::new(0),       // Stone Age
            Self::Wrench => TechLevel::new(4),          // Age of Sail
        }
    }
}
