//! Tests for Combat VSM transitions and invariants.

use valinoreth::{
    apply_damage, begin_turn, combat_consistent, declare_attack, declared_attack_target, end_turn,
    initialize_combat, resolve_attack, resolve_defense, CombatConsistent, CombatState,
    CombatantSlot, CombatantState, DeclaredAttackTarget,
};

// Helper to create test combatants
fn create_test_combatants() -> Vec<CombatantState> {
    vec![
        CombatantState {
            id: "fighter".to_string(),
            team: "heroes".to_string(),
            current_hp: 12,
            max_hp: 12,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 6,
            incapacitated: false,
        },
        CombatantState {
            id: "wizard".to_string(),
            team: "heroes".to_string(),
            current_hp: 8,
            max_hp: 8,
            current_fp: 12,
            max_fp: 12,
            basic_speed: 5,
            incapacitated: false,
        },
        CombatantState {
            id: "orc".to_string(),
            team: "enemies".to_string(),
            current_hp: 10,
            max_hp: 10,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 5,
            incapacitated: false,
        },
    ]
}

fn declared_target(attacker: usize, target: usize) -> DeclaredAttackTarget {
    use elicitation::Established;

    declared_attack_target(
        CombatantSlot::new(attacker),
        CombatantSlot::new(target),
        Established::assert(),
    )
}

#[test]
fn test_initialize_combat_creates_active_state() {
    use elicitation::Established;

    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let combatants = create_test_combatants();
    let init_proof = Established::assert();

    let (new_state, new_proof) = initialize_combat(state, proof, combatants, init_proof);

    match &new_state {
        CombatState::Active {
            combatants,
            turn_order,
            current_actor,
            round,
        } => {
            assert_eq!(combatants.len(), 3);
            assert_eq!(turn_order.len(), 3);
            assert_eq!(*current_actor, 0);
            assert_eq!(*round, 1);

            // Verify turn order is sorted by Basic Speed (descending)
            // Fighter (6) should be first, then wizard/orc (both 5)
            assert_eq!(combatants[turn_order[0]].id, "fighter");
            assert_eq!(combatants[turn_order[0]].basic_speed, 6);
        }
        _ => panic!("Expected Active state"),
    }

    // Verify invariant is maintained
    assert!(combat_consistent(&new_state));
    let _ = new_proof; // Proof is valid
}

#[test]
fn test_turn_order_sorted_by_basic_speed() {
    use elicitation::Established;

    let combatants = vec![
        CombatantState {
            id: "slow".to_string(),
            team: "a".to_string(),
            current_hp: 10,
            max_hp: 10,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 3,
            incapacitated: false,
        },
        CombatantState {
            id: "fast".to_string(),
            team: "b".to_string(),
            current_hp: 10,
            max_hp: 10,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 8,
            incapacitated: false,
        },
        CombatantState {
            id: "medium".to_string(),
            team: "c".to_string(),
            current_hp: 10,
            max_hp: 10,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 5,
            incapacitated: false,
        },
    ];

    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (new_state, _) = initialize_combat(state, proof, combatants, init_proof);

    if let CombatState::Active {
        combatants,
        turn_order,
        ..
    } = new_state
    {
        // Verify order: fast (8) → medium (5) → slow (3)
        assert_eq!(combatants[turn_order[0]].id, "fast");
        assert_eq!(combatants[turn_order[1]].id, "medium");
        assert_eq!(combatants[turn_order[2]].id, "slow");
    } else {
        panic!("Expected Active state");
    }
}

#[test]
fn test_begin_turn_maintains_state() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);
    let turn_proof = Established::assert();

    let (new_state, new_proof) = begin_turn(state.clone(), proof, turn_proof);

    // State should be unchanged
    assert_eq!(new_state, state);
    assert!(combat_consistent(&new_state));
    let _ = new_proof;
}

#[test]
fn test_declare_attack_maintains_state() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);
    let target = declared_target(0, 1);

    let (new_state, new_proof) = declare_attack(state.clone(), proof, target);

    // State should be unchanged (attack declaration tracked via proof)
    assert_eq!(new_state, state);
    assert!(combat_consistent(&new_state));
    let _ = new_proof;
}

#[test]
fn test_resolve_attack_maintains_state() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);
    let resolved_proof = Established::assert();

    let (new_state, new_proof) = resolve_attack(state.clone(), proof, resolved_proof);

    // State unchanged (mechanics handled by GameMaster)
    assert_eq!(new_state, state);
    assert!(combat_consistent(&new_state));
    let _ = new_proof;
}

#[test]
fn test_resolve_defense_maintains_state() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);
    let resolved_proof = Established::assert();

    let (new_state, new_proof) = resolve_defense(state.clone(), proof, resolved_proof);

    // State unchanged (mechanics handled by GameMaster)
    assert_eq!(new_state, state);
    assert!(combat_consistent(&new_state));
    let _ = new_proof;
}

#[test]
fn test_apply_damage_reduces_hp() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);

    let target = declared_target(0, 1);
    let (state, proof) = declare_attack(state, proof, target);

    // Apply 5 damage to declared combatant slot 1.
    let damage_proof = Established::assert();
    let (new_state, new_proof) = apply_damage(state, proof, target, 5, damage_proof);

    if let CombatState::Active { combatants, .. } = &new_state {
        let target = &combatants[1];

        // Original HP was 8 for wizard, should now be 3
        assert_eq!(target.current_hp, 3);
        assert!(!target.incapacitated);
    } else {
        panic!("Expected Active state");
    }

    assert!(combat_consistent(&new_state));
    let _ = new_proof;
}

#[test]
fn test_apply_damage_sets_incapacitated() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);

    let target = declared_target(0, 1);
    let (state, proof) = declare_attack(state, proof, target);

    // Apply 10 damage to declared combatant slot 1 (should incapacitate wizard with 8 HP)
    let damage_proof = Established::assert();
    let (new_state, new_proof) = apply_damage(state, proof, target, 10, damage_proof);

    if let CombatState::Active { combatants, .. } = &new_state {
        let target = &combatants[1];

        assert_eq!(target.current_hp, -2);
        assert!(target.incapacitated);
    } else {
        panic!("Expected Active state");
    }

    assert!(combat_consistent(&new_state));
    let _ = new_proof;
}

#[test]
fn test_apply_damage_hits_declared_combatant_slot_not_turn_order_slot() {
    use elicitation::Established;

    let combatants = vec![
        CombatantState {
            id: "attacker".to_string(),
            team: "heroes".to_string(),
            current_hp: 12,
            max_hp: 12,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 4,
            incapacitated: false,
        },
        CombatantState {
            id: "intended-target".to_string(),
            team: "enemies".to_string(),
            current_hp: 10,
            max_hp: 10,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 3,
            incapacitated: false,
        },
        CombatantState {
            id: "fast-bystander".to_string(),
            team: "enemies".to_string(),
            current_hp: 10,
            max_hp: 10,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 9,
            incapacitated: false,
        },
    ];

    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);

    if let CombatState::Active { turn_order, .. } = &state {
        assert_eq!(turn_order.as_slice(), &[2, 0, 1]);
    } else {
        panic!("Expected Active state");
    }

    let target = declared_target(0, 1);
    let (state, proof) = declare_attack(state, proof, target);

    let damage_proof = Established::assert();
    let (state, _proof) = apply_damage(state, proof, target, 4, damage_proof);

    if let CombatState::Active { combatants, .. } = &state {
        assert_eq!(combatants[0].current_hp, 12);
        assert_eq!(combatants[1].current_hp, 6);
        assert_eq!(combatants[2].current_hp, 10);
    } else {
        panic!("Expected Active state");
    }

    assert!(combat_consistent(&state));
}

#[test]
fn test_end_turn_advances_to_next_combatant() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);

    // Start at current_actor = 0, round = 1
    if let CombatState::Active {
        current_actor,
        round,
        ..
    } = &state
    {
        assert_eq!(*current_actor, 0);
        assert_eq!(*round, 1);
    }

    let turn_proof = Established::assert();
    let (new_state, new_proof) = end_turn(state, proof, turn_proof);

    // Should advance to current_actor = 1, same round
    if let CombatState::Active {
        current_actor,
        round,
        ..
    } = new_state
    {
        assert_eq!(current_actor, 1);
        assert_eq!(round, 1);
    } else {
        panic!("Expected Active state");
    }

    assert!(combat_consistent(&new_state));
    let _ = new_proof;
}

#[test]
fn test_end_turn_wraps_to_new_round() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (mut state, mut proof) = initialize_combat(state, proof, combatants, init_proof);

    // Advance through all 3 combatants
    for _ in 0..3 {
        let turn_proof = Established::assert();
        (state, proof) = end_turn(state, proof, turn_proof);
    }

    // Should wrap to current_actor = 0, round = 2
    if let CombatState::Active {
        current_actor,
        round,
        ..
    } = state
    {
        assert_eq!(current_actor, 0);
        assert_eq!(round, 2);
    } else {
        panic!("Expected Active state");
    }

    assert!(combat_consistent(&state));
    let _ = proof;
}

#[test]
fn test_complete_combat_flow() {
    use elicitation::Established;

    // Initialize combat
    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();
    let init_proof = Established::assert();

    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);

    // Begin turn
    let turn_proof = Established::assert();
    let (state, proof) = begin_turn(state, proof, turn_proof);

    // Declare attack (attacker 0 -> target 2)
    let target = declared_target(0, 2);
    let (state, proof) = declare_attack(state, proof, target);

    // Resolve attack
    let resolved_proof = Established::assert();
    let (state, proof) = resolve_attack(state, proof, resolved_proof);

    // Resolve defense
    let defense_proof = Established::assert();
    let (state, proof) = resolve_defense(state, proof, defense_proof);

    // Apply damage (6 points to declared combatant slot 2)
    let damage_proof = Established::assert();
    let (state, proof) = apply_damage(state, proof, target, 6, damage_proof);

    // Verify damage was applied
    if let CombatState::Active {
        combatants,
        current_actor,
        round,
        ..
    } = &state
    {
        let target = &combatants[2];

        assert_eq!(target.current_hp, 4); // 10 - 6 = 4
        assert!(!target.incapacitated);
        assert_eq!(*current_actor, 0);
        assert_eq!(*round, 1);
    } else {
        panic!("Expected Active state");
    }

    // End turn
    let turn_proof = Established::assert();
    let (state, proof) = end_turn(state, proof, turn_proof);

    // Verify we advanced to next combatant
    if let CombatState::Active {
        current_actor,
        round,
        ..
    } = state
    {
        assert_eq!(current_actor, 1);
        assert_eq!(round, 1);
    } else {
        panic!("Expected Active state");
    }

    assert!(combat_consistent(&state));
    let _ = proof;
}

#[test]
fn test_invariant_maintained_through_transitions() {
    use elicitation::Established;

    let combatants = create_test_combatants();
    let state = CombatState::Uninitialized;
    let proof = Established::<CombatConsistent>::assert();

    // Check initial state
    assert!(combat_consistent(&state));

    // Initialize
    let init_proof = Established::assert();
    let (state, proof) = initialize_combat(state, proof, combatants, init_proof);
    assert!(combat_consistent(&state));

    // Multiple transitions
    let turn_proof = Established::assert();
    let (state, proof) = begin_turn(state, proof, turn_proof);
    assert!(combat_consistent(&state));

    let target = declared_target(0, 1);
    let (state, proof) = declare_attack(state, proof, target);
    assert!(combat_consistent(&state));

    let damage_proof = Established::assert();
    let (state, proof) = apply_damage(state, proof, target, 3, damage_proof);
    assert!(combat_consistent(&state));

    let turn_proof = Established::assert();
    let (state, proof) = end_turn(state, proof, turn_proof);
    assert!(combat_consistent(&state));

    let _ = proof;
}
