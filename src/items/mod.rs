//! GURPS items system.
//!
//! # Overview
//!
//! This module enumerates all GURPS equipment (~400 items) across multiple
//! categories: weapons, armor, clothing, tools, containers, survival gear,
//! and magic items. Each item has universal properties (cost, weight, tech
//! level) and category-specific properties (weapon damage, armor DR, etc.).
//!
//! # Organization
//!
//! The module follows a delegation pattern similar to the spell system:
//! - `Item` enum contains ~400 variants across 8 categories
//! - Each category has its own module implementing property functions
//! - Universal properties (cost, weight, TL) available on all items
//! - Category-specific properties return `Option<T>`
//!
//! # Citations
//!
//! BS 266-289 - Equipment chapter
//! HT - High-Tech supplement
//! LT - Low-Tech supplement

mod armor;
mod clothing;
mod core;
mod types;

// Category modules
mod weapons_melee;
mod weapons_ranged;

pub use core::Item;
pub use types::{Capacity, Currency, ItemCategory, Quality, TechLevel, Weight};
