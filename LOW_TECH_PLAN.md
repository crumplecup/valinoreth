# GURPS Low-Tech - Systematic Extraction Plan

## Overview
**Source:** GURPS Low-Tech 4E (TL0-4 equipment)
**Target:** ~205 items for historical/medieval completeness
**Current Total:** 231 items (BS complete)
**Target After Phase 2:** ~436 items

## Extraction Strategy

### Priority Order
1. **Melee Weapons** (~60 items) - Historical variants, polearms, agricultural weapons
2. **Ranged Weapons** (~40 items) - Historical bows, crossbows, siege weapons
3. **Armor** (~30 items) - Mail variants, historical plates, regional armor
4. **Tools** (~25 items) - Agricultural, crafting, trade tools
5. **Clothing** (~20 items) - Period clothing, regional garments
6. **Containers** (~15 items) - Historical containers, transport
7. **Survival Gear** (~15 items) - Period camping, travel equipment

## Category Breakdown

### Melee Weapons (~60 Low-Tech items)

**Axes & Hammers:**
- Battle Axe variants (francisca, bearded axe)
- War hammers (bec de corbin, lucerne hammer)
- Mauls and great hammers

**Polearms:**
- Bill, guisarme, partisan
- Bardiche, voulge, fauchard
- Awl pike, push pike

**Swords:**
- Migration period swords
- Viking swords, seax
- Dao, jian, katana variants
- Tulwar, shamshir, kilij

**Flails & Chains:**
- Military flails
- Grain flail (weapon)
- Three-section staff

**Agricultural Weapons:**
- Scythe (weapon)
- Sickle (weapon)
- Pitchfork (weapon)
- Billhook (weapon)

**Regional/Cultural:**
- Japanese: bo, jo, tonfa variants, jutte
- Chinese: dao variants, spear-sword
- Indian: urumi, talwar, chakram variants
- Middle Eastern: scimitar variants

### Ranged Weapons (~40 Low-Tech items)

**Bows:**
- Flatbow, hornbow
- Yumi (Japanese longbow)
- Horse bow variants

**Crossbows:**
- Windlass crossbow
- Cranequin crossbow
- Repeating crossbow (chu-ko-nu)

**Siege Weapons:**
- Ballista
- Catapult
- Trebuchet
- Onager

**Thrown:**
- Pilum (Roman javelin)
- Francisca (throwing axe)
- Angon (barbed javelin)

### Armor (~30 Low-Tech items)

**Mail Variants:**
- Byrnie (short mail)
- Hauberk variants
- Mail coif, mail mittens
- Aventail

**Lamellar & Scale:**
- Lamellar variants (by region)
- Scale variants
- Mountain pattern armor

**Plate Variants:**
- Cuirass (breastplate only)
- Coat of plates
- Brigandine variants
- Gothic plate, Milanese plate

**Regional Armor:**
- O-Yoroi (Japanese great armor)
- Do-Maru (Japanese wrap armor)
- Linothorax (Greek linen)
- Lorica Segmentata (Roman)

### Tools (~25 Low-Tech items)

**Agricultural:**
- Plow, harrow
- Scythe (tool), sickle (tool)
- Hoe, mattock, spade
- Pitchfork, hay fork
- Flail (grain threshing)

**Crafting:**
- Blacksmith's tools
- Armorer's tools
- Carpentry tools
- Mason's tools
- Weaver's loom
- Potter's wheel

**Trade:**
- Balance scales
- Weights & measures
- Merchant's kit

### General Equipment (~35 items)

**Clothing:**
- Tabard, surcoat
- Kimono, hakama
- Sari, dhoti
- Robes (various cultures)

**Containers:**
- Amphora, jar
- Water jar, oil jar
- Saddlebags
- Pack frame

**Survival:**
- Oil lamp (various types)
- Bronze mirror
- Whetstone
- Tinder box variants

## Extraction Batches

### Batch Size
- **Weapons:** 5-10 items per batch (complex properties)
- **Armor:** 3-5 items per batch (DR calculations)
- **Equipment:** 5-10 items per batch (simpler properties)

### Verification Per Batch
1. Add enum variants with docs
2. Implement all properties (cost, weight, TL, etc.)
3. Run `cargo check`
4. Run `cargo test`
5. Run `cargo clippy`
6. Update extraction checklist
7. Commit with detailed message

## Sources & References

**Primary:**
- GURPS Low-Tech 4E core book
- GURPS Low-Tech Companion series

**Secondary:**
- Historical weapon encyclopedias
- Museum collections (online)
- Academic sources on historical equipment

## Success Metrics

- [ ] All Low-Tech TL0-4 weapons extracted
- [ ] Historical armor variants complete
- [ ] Agricultural/trade tools complete
- [ ] Regional equipment variations included
- [ ] Total item count: ~436 (from 231)
- [ ] All tests passing
- [ ] Zero clippy warnings
- [ ] Full property implementations

## Notes

- Focus on items that appear in GURPS Low-Tech book
- Avoid excessive regional variants (keep practical)
- Prioritize commonly referenced items
- Maintain consistent naming conventions
- Cite Low-Tech page numbers where possible
