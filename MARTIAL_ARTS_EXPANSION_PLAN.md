# GURPS Martial Arts Expansion Plan

## Overview

Systematically implement combat techniques from GURPS Martial Arts supplement based on publicly accessible online content.

## Goal

Add all martial arts techniques that can be verified from online GURPS wikis and community resources.

## Approach

**Research-Based Implementation:**
- Only implement techniques with complete, verified stats from online sources
- Do not invent or estimate stats
- Establish canonical foundation that can be expanded over time

## Progress Summary

**Started:** 2026-05-11
**Status:** IN PROGRESS (Foundation Established)
**Items Implemented:** 9 techniques
**Coverage:** ~5-10% of full Martial Arts supplement (canonical base only)

## Implementation Details

### Phase 1: Core Techniques (COMPLETE ✅)

Implemented 9 base techniques with full stats from various online sources:

1. **ArmLock** (MA 73)
   - Difficulty: Average
   - Prerequisites: Judo, Wrestling
   - Default: +0
   - Maximum: +4

2. **Breakfall** (MA 62)
   - Difficulty: Hard
   - Prerequisites: Acrobatics, Judo, Wrestling
   - Default: +0
   - Maximum: +5

3. **ChokeHold** (MA 77)
   - Difficulty: Hard
   - Prerequisites: Judo (best: -2), Wrestling (-3)
   - Default: Best -2
   - Maximum: Cannot exceed skill

4. **Disarming** (MA 82)
   - Difficulty: Hard
   - Prerequisites: Brawling, Boxing, Karate, Judo, Wrestling
   - Default: +0
   - Maximum: +5

5. **GroundFighting** (BS 231, MA 73)
   - Difficulty: Hard
   - Prerequisites: Brawling, Boxing, Karate, Judo, Wrestling
   - Default: -4
   - Maximum: Cannot exceed skill

6. **SpinningKick** (MA 85)
   - Difficulty: Hard
   - Prerequisites: Karate
   - Default: -5
   - Maximum: +0 (prerequisite level)

7. **TwoHandedPunch** (MA 86)
   - Difficulty: Average
   - Prerequisites: Brawling
   - Default: -2
   - Maximum: +0 (prerequisite level)

8. **TriangleChoke** (MA 86)
   - Difficulty: Hard
   - Prerequisites: Judo (best: -4), Wrestling (-5)
   - Default: Best -4
   - Maximum: +0 (prerequisite level)

9. **WhirlwindAttack** (MA 86)
   - Difficulty: Hard
   - Prerequisites: Boxing, Karate
   - Default: -5
   - Maximum: +0 (prerequisite level)

**File:** `src/combat/techniques.rs` (321 lines)
**Commit:** a0291ec

### Phase 2: Additional Techniques (PENDING)

**Status:** Awaiting discovery of more techniques with complete stats from online sources.

**Research needed for:**
- Weapon techniques (sword, staff, etc.)
- Grappling techniques beyond base set
- Strike variations and combinations
- Cinematic techniques
- Style-specific techniques

**Limitation:** Most technique stat tables are in copyrighted sourcebooks not reproduced online.

## Technical Implementation

### Enum Pattern
Following the established pattern for Skills and Spells:

```rust
pub enum Technique {
    ArmLock,
    Breakfall,
    // ... more variants
}
```

### Stat Methods
Each technique provides six methods:
- `difficulty()` → `TechniqueDifficulty` (Average/Hard)
- `prerequisites()` → `Vec<Skill>` (required base skills)
- `default_penalty()` → `i32` (penalty relative to prerequisite)
- `maximum_bonus()` → `Option<i32>` (training cap, None = cannot exceed)
- `is_cinematic()` → `bool` (requires cinematic campaign)
- `is_silly()` → `bool` (silly campaign only)

### Documentation
All techniques include:
- GURPS rule citations (BS page, MA page)
- Rustdoc with examples
- Difficulty and prerequisite information
- Default and maximum values

## Next Steps

1. **Opportunistic Addition:** Add techniques as they're discovered with complete stats
2. **Web Search:** Periodic searches for new wiki content or community resources
3. **Community Contribution:** If users have sourcebooks, they can contribute verified stats
4. **Future Expansion:** Consider other Martial Arts content:
   - Fighting styles
   - Combat options and maneuvers
   - Cinematic abilities
   - Training rules

## Notes

- Techniques represent specialized training that builds on base combat skills
- Average techniques cost 1 point per level to improve
- Hard techniques cost 2 points per level to improve
- Technique Mastery advantage allows exceeding normal maximum limits
- Some techniques cannot exceed their prerequisite skill level

## Resources

Online sources used for research:
- GURPS Wiki (gurps.fandom.com)
- GURPS Wikidot (gurpswiki.wikidot.com)
- Gaming blog posts and forums
- Community-maintained technique lists

---

**Last Updated:** 2026-05-11
