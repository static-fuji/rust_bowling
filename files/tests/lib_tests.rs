#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(10, hiker::answer("|12|34|"));
    assert_eq!(20, hiker::answer("|1/|34|"));
    assert_eq!(33, hiker::answer("|1/|3/|42|"));
    assert_eq!(31, hiker::answer("|1/|3/|4-|"));
    assert_eq!(10, hiker::answer("|X|"));
    assert_eq!(60, hiker::answer("X|X|X"));
    assert_eq!(270, hiker::answer("X|X|X|X|X|X|X|X|X|X"));
    assert_eq!(300, hiker::answer("X|X|X|X|X|X|X|X|X|X||XX"));
    assert_eq!(90, hiker::answer("9-|9-|9-|9-|9-|9-|9-|9-|9-|9-||"));
    assert_eq!(150, hiker::answer("5/|5/|5/|5/|5/|5/|5/|5/|5/|5/||5"));
    assert_eq!(167, hiker::answer("X|7/|9-|X|-8|8/|-6|X|X|X||81"));
    assert_eq!(30, hiker::answer("1/|X"));
}
