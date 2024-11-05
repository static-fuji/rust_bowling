#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(10, hiker::answer("|12|34|"));
}
