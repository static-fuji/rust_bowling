#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer(score_str: &str) -> u32 {
    let mut total_score = 0;
    let mut frame_index = 0;
    let frames: Vec<&str> = score_str.split('|').filter(|&s| !s.is_empty()).collect();

    while frame_index < frames.len() && frame_index < 10 {
        let frame = frames[frame_index];
        let first_roll = frame.chars().nth(0).unwrap();
        let second_roll = frame.chars().nth(1).unwrap_or('0');

        total_score += first_roll.to_digit(10).unwrap_or(0) + second_roll.to_digit(10).unwrap_or(0);

        frame_index += 1;
    }
    total_score
}
