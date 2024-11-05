#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(10, hiker::answer("|12|34|"));
    assert_eq!(20, hiker::answer("|19|34|"));
    assert_eq!(33, hiker::answer("|1/|3/|42|"));
}
