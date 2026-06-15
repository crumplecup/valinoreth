use elicitation::Generator;
use valinoreth::ThreeDiceRoll;

#[test]
fn seeded_three_dice_generator_replays_advancing_stream() {
    let generator = ThreeDiceRoll::random_generator(42);
    let rolls: Vec<_> = (0..8).map(|_| generator.generate()).collect();

    let replay = ThreeDiceRoll::random_generator(42);
    let replayed_rolls: Vec<_> = (0..8).map(|_| replay.generate()).collect();

    assert_eq!(rolls, replayed_rolls);
    assert!(
        rolls.windows(2).any(|pair| pair[0] != pair[1]),
        "a reusable dice generator must advance instead of repeating the first roll"
    );
}
