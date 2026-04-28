# GURPS Armor - Systematic Extraction

## Sources
- [ ] GURPS Basic Set 4E (BS 284-285)
- [ ] GURPS Low-Tech
- [ ] GURPS Low-Tech: Instant Armor
- [ ] GURPS High-Tech

## Current Status: 25 variants

## GURPS Basic Set - Armor Table (BS 284-285)

### Soft Armor
- ✓ NoArmor (DR 0)
- ✓ LeatherArmor (DR 1)
- ✓ HeavyLeather (DR 2)
- ✓ ClothArmor (DR 1)
- ✓ LightLeather (DR 1)
- ✗ Light Underarmor (TL8 - High-Tech)
- ✓ TacticalVest (DR 18)
- ✓ BallisticVest (DR 10)

### Mail
- ✓ Chainmail (DR 4)
- ✓ ScaleMail (DR 4)
- ✓ LightScale (DR 3)
- ✓ RingMail (DR 3)
- ✓ BandedMail (DR 4)
- ✓ SplintMail (DR 5)
- ✓ MailHauberk (DR 4)
- ✓ MailShirt (DR 4)

### Plate
- ✓ PlateArmor (DR 6)
- ✓ HeavyPlate (DR 8)
- ✓ HalfPlate (DR 5)
- ✓ BronzePlate (DR 5)
- ✗ Three-Quarter Plate (DR 6 - Low-Tech variant)
- ✗ Gothic Plate (DR 7 - Low-Tech variant)

### Shields (all in Armor enum)
- ✓ Buckler (DR 1)
- ✓ SmallShield (DR 1)
- ✓ MediumShield (DR 2)
- ✓ LargeShield (DR 3)
- ✓ TowerShield (DR 4)

### Helmets (Separate from body armor?)
- ✗ Leather Cap
- ✗ Pot Helm
- ✗ Great Helm
- ✗ Bascinet
- ✗ Sallet
- ✗ Close Helm

## Low-Tech Armor Additions

**Bronze Age (TL1):**
- ✗ Bronze Breastplate
- ✗ Bronze Corselet
- ✗ Linothorax (linen armor)
- ✗ Lamellar (DR 4)

**Iron Age (TL2):**
- ✗ Lorica Segmentata (Roman)
- ✗ Lorica Hamata (Roman mail)
- ✗ BrigandineArmor (DR 5)
- ✗ Coat of Plates

**Medieval (TL3):**
- ✗ Gambeson (DR 2)
- ✗ Aketon (DR 2)
- ✗ Jack of Plates (DR 4)
- ✗ Cuirass (breastplate only)

**Asian Armor:**
- ✗ O-Yoroi (Japanese great armor)
- ✗ Do-Maru (Japanese wrap armor)
- ✗ Lamellar Armor
- ✗ Mountain Pattern Armor (Chinese)

## High-Tech Armor (TL6-8)

**Modern:**
- ✗ Flak Jacket (DR 7)
- ✗ Combat Helmet (DR 12)
- ✗ Infantry Vest (DR 10)
- ✗ Assault Vest (DR 18)
- ✗ SWAT Suit (DR 20)

**Riot/Police:**
- ✗ Riot Shield
- ✗ Riot Helmet
- ✗ Stab Vest

## Ultra-Tech (TL9-12)
- ✗ Nanoweave (DR 20)
- ✗ Reflex Armor (DR 30)
- ✗ Combat Hardsuit (DR 50)
- ✗ Powered Armor (DR 70+)
- ✗ Battle Dress (DR 100+)

## Design Decision Needed

**Should shields be:**
1. Part of Armor enum?
2. Separate Shield enum/category?
3. Part of weapons (parrying tools)?

**Should helmets be:**
1. Included in body armor DR?
2. Separate items?
3. Modular add-ons?

## Action Items
1. Complete Basic Set armor types
2. Add shields (decide on categorization)
3. Add medieval armor variants
4. Add modern armor (TL7-8)
