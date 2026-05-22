//! GURPS items system.
//!
//! # Overview
//!
//! This module provides a type-safe hierarchical item system with ~200 items
//! across 7 categories: melee weapons, ranged weapons, armor, clothing,
//! containers, tools, and survival gear. Each item has universal properties
//! (cost, weight, tech level) and category-specific properties (weapon damage,
//! armor DR, container capacity, etc.).
//!
//! # Organization
//!
//! The module uses nested enums for compile-time type safety:
//! - Top-level `Item` enum wraps 7 category enums
//! - Each category enum (MeleeWeapon, Armor, etc.) contains its variants
//! - Compiler enforces exhaustive matching - no runtime type errors possible
//! - Universal properties delegated to category implementations
//! - Category-specific properties return `Option<T>` from Item methods
//!
//! # Benefits
//!
//! - **Type safety**: Compiler prevents wrong-type access at compile time
//! - **Exhaustiveness**: No `_ => error!()` branches needed
//! - **Maintainability**: Adding items only requires updating enum + impl
//! - **Performance**: Zero-cost abstractions, no runtime overhead
//!
//! # Citations
//!
//! BS 266-289 - Equipment chapter
//! HT - High-Tech supplement
//! LT - Low-Tech supplement

// Category enums
mod armor;
mod clothing;
mod containers;
mod item;
mod melee_weapon;
mod ranged_weapon;
mod survival;
mod tools;
mod types;
mod weapons;

// Re-export all public types
pub use armor::Armor;
pub use clothing::Clothing;
pub use containers::Container;
pub use item::Item;
pub use melee_weapon::MeleeWeapon;
pub use ranged_weapon::RangedWeapon;
pub use survival::SurvivalGear;
pub use tools::Tool;
pub use types::{Capacity, Currency, Quality, TechLevel, Weight};
pub use weapons::{DamageType, Reach, WeaponDamage};
