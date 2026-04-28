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
    /// Block and tackle, $50. BS 289
    BlockAndTackle,
    /// Chisel, $5. BS 289
    Chisel,
    /// Compass, $50. BS 289
    Compass,
    /// Crowbar, $20. BS 289
    Crowbar,
    /// File, $5. BS 289
    File,
    /// First aid kit, $50. BS 289
    FirstAidKit,
    /// Grapnel, $20. BS 289
    Grapnel,
    /// Hammer, $15. BS 289
    Hammer,
    /// Ladder, $30. BS 289
    Ladder,
    /// Lockpicks, $50. BS 289
    Lockpicks,
    /// Magnifying glass, $100. BS 289
    MagnifyingGlass,
    /// Parchment (10 sheets), $20. BS 289
    Parchment,
    /// Pickaxe, $15. BS 289
    Pickaxe,
    /// Pliers, $15. BS 289
    Pliers,
    /// Quill and ink, $10. BS 289
    QuillAndInk,
    /// Rope, $5. BS 289
    Rope,
    /// Saw, $150. BS 289
    Saw,
    /// Screwdriver, $5. BS 289
    Screwdriver,
    /// Shovel, $12. BS 289
    Shovel,
    /// Tool hatchet, $40. BS 289
    ToolHatchet,
    /// Toolkit, $200. BS 289
    Toolkit,
    /// Wrench, $20. BS 289
    Wrench,
}

impl Tool {
    /// Returns base cost for this tool.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting tool base cost");
        match self {
            Self::BlockAndTackle => Currency::dollars(50.0),
            Self::Chisel => Currency::dollars(5.0),
            Self::Compass => Currency::dollars(50.0),
            Self::Crowbar => Currency::dollars(20.0),
            Self::File => Currency::dollars(5.0),
            Self::FirstAidKit => Currency::dollars(50.0),
            Self::Grapnel => Currency::dollars(20.0),
            Self::Hammer => Currency::dollars(15.0),
            Self::Ladder => Currency::dollars(30.0),
            Self::Lockpicks => Currency::dollars(50.0),
            Self::MagnifyingGlass => Currency::dollars(100.0),
            Self::Parchment => Currency::dollars(20.0),
            Self::Pickaxe => Currency::dollars(15.0),
            Self::Pliers => Currency::dollars(15.0),
            Self::QuillAndInk => Currency::dollars(10.0),
            Self::Rope => Currency::dollars(5.0),
            Self::Saw => Currency::dollars(150.0),
            Self::Screwdriver => Currency::dollars(5.0),
            Self::Shovel => Currency::dollars(12.0),
            Self::ToolHatchet => Currency::dollars(40.0),
            Self::Toolkit => Currency::dollars(200.0),
            Self::Wrench => Currency::dollars(20.0),
        }
    }

    /// Returns weight for this tool.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting tool weight");
        match self {
            Self::BlockAndTackle => Weight::pounds(4.0),
            Self::Chisel => Weight::pounds(0.5),
            Self::Compass => Weight::pounds(0.5),
            Self::Crowbar => Weight::pounds(3.0),
            Self::File => Weight::pounds(0.5),
            Self::FirstAidKit => Weight::pounds(2.0),
            Self::Grapnel => Weight::pounds(2.0),
            Self::Hammer => Weight::pounds(2.0),
            Self::Ladder => Weight::pounds(20.0),
            Self::Lockpicks => Weight::pounds(0.1),
            Self::MagnifyingGlass => Weight::pounds(0.25),
            Self::Parchment => Weight::pounds(0.5),
            Self::Pickaxe => Weight::pounds(5.0),
            Self::Pliers => Weight::pounds(1.0),
            Self::QuillAndInk => Weight::pounds(0.1),
            Self::Rope => Weight::pounds(1.5),
            Self::Saw => Weight::pounds(3.0),
            Self::Screwdriver => Weight::pounds(0.25),
            Self::Shovel => Weight::pounds(6.0),
            Self::ToolHatchet => Weight::pounds(2.0),
            Self::Toolkit => Weight::pounds(10.0),
            Self::Wrench => Weight::pounds(2.0),
        }
    }

    /// Returns tech level for this tool.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting tool tech level");
        match self {
            Self::BlockAndTackle => TechLevel::new(1), // Bronze Age
            Self::Chisel => TechLevel::new(0),         // Stone Age
            Self::Compass => TechLevel::new(4),        // Age of Sail
            Self::Crowbar => TechLevel::new(1),        // Bronze Age
            Self::File => TechLevel::new(2),           // Medieval
            Self::FirstAidKit => TechLevel::new(5),    // Industrial
            Self::Grapnel => TechLevel::new(2),        // Medieval
            Self::Hammer => TechLevel::new(0),         // Stone Age
            Self::Ladder => TechLevel::new(0),         // Stone Age
            Self::Lockpicks => TechLevel::new(3),      // Medieval/Renaissance
            Self::MagnifyingGlass => TechLevel::new(4), // Renaissance
            Self::Parchment => TechLevel::new(2),      // Medieval
            Self::Pickaxe => TechLevel::new(0),        // Stone Age
            Self::Pliers => TechLevel::new(3),         // Renaissance
            Self::QuillAndInk => TechLevel::new(2),    // Medieval
            Self::Rope => TechLevel::new(0),           // Stone Age
            Self::Saw => TechLevel::new(2),            // Medieval
            Self::Screwdriver => TechLevel::new(3),    // Renaissance
            Self::Shovel => TechLevel::new(1),         // Bronze Age
            Self::ToolHatchet => TechLevel::new(0),    // Stone Age
            Self::Toolkit => TechLevel::new(1),        // Bronze Age (basic tools)
            Self::Wrench => TechLevel::new(4),         // Age of Sail
        }
    }
}
