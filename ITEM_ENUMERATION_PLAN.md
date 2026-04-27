# GURPS Item Enumeration Plan

## Goal
Systematically enumerate ALL items from GURPS sources to achieve wiki-complete coverage.

## Current Status (201 items)
- MeleeWeapon: 55 variants
- RangedWeapon: 31 variants
- Armor: 25 variants
- Clothing: 30 variants
- Container: 20 variants
- Tool: 20 variants
- SurvivalGear: 20 variants

## Source Books (Priority Order)

### 1. GURPS Basic Set (4th Edition)
**Status: IN PROGRESS**
- [ ] BS 266-289: Equipment Chapter
  - [ ] BS 271-276: Melee Weapons Table (partially done)
  - [ ] BS 276-278: Ranged Weapons Table (partially done)
  - [ ] BS 284-285: Armor Table (partially done)
  - [ ] BS 288: Containers
  - [ ] BS 289: Other Equipment
- [ ] Document: Every item listed gets enumerated

### 2. GURPS Low-Tech (4th Edition)
**Status: NOT STARTED**
- [ ] TL0-4 Equipment
- [ ] Complete weapon tables
- [ ] Complete armor tables
- [ ] Tools, clothing, survival gear for historical settings

### 3. GURPS High-Tech (4th Edition)
**Status: NOT STARTED**
- [ ] TL5-8 Equipment
- [ ] Modern and futuristic weapons
- [ ] Modern armor
- [ ] Technical equipment

### 4. GURPS Ultra-Tech (4th Edition)
**Status: NOT STARTED**
- [ ] TL9-12 Equipment
- [ ] Advanced weapons
- [ ] Powered armor
- [ ] Sci-fi equipment

### 5. Other Supplements (As Needed)
- GURPS Martial Arts (additional weapons)
- GURPS Loadouts series (comprehensive equipment lists)
- Setting-specific books (Camelot, WWII, etc.)

## Systematic Process

### For Each Source Book:
1. **Open to equipment chapter/section**
2. **Go page by page** - don't skip around
3. **Extract every item in every table**
4. **Note page number and citation** in code comments
5. **Mark section as DONE** in this file
6. **Move to next section** - no backtracking

### For Each Item:
```rust
/// Item Name, stats. BS XXX (or LT XXX, HT XXX)
ItemName,
```

### Quality Control:
- At end of each source, verify item count matches table row counts
- Cross-reference indices/equipment lists in book
- Mark book as COMPLETE only when 100% extracted

## Next Actions

1. **Complete Basic Set systematic sweep**
   - Start at BS 271, go line by line through equipment tables
   - End at BS 289
   - Mark each page range as done

2. **Then move to Low-Tech**
   - Start at beginning of equipment chapter
   - Sweep through completely
   - Low-Tech has hundreds of items we're missing

3. **Then High-Tech**, etc.

## Tracking Progress

Update this file after each sweep session:
- Mark page ranges as [x] DONE
- Note item counts added
- Document any skipped items (with reason)

## Categories to Add

Based on comprehensive sweep, may need:
- Ammunition (separate category)
- Electronics (High-Tech)
- Drugs/Medicine (separate category)
- Vehicles (separate module?)
- Food/Provisions (expand SurvivalGear?)

---

**Last Updated:** 2026-04-26
**Current Phase:** Basic Set - Melee Weapons Table Sweep
