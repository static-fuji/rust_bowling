#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer(score_str: &str) -> u32 {
    let mut total_score = 0;
    let mut frame_index = 0;
    let frames: Vec<&str> = score_str.split('|').filter(|&s| !s.is_empty()).collect();

    while frame_index < frames.len() && frame_index < 10 {
        let frame = frames[frame_index];
        let first_roll = parse_roll(frame.chars().next().unwrap());
        let second_roll = parse_roll(frame.chars().nth(1).unwrap_or('0'));

        total_score += first_roll + second_roll;

        if frame.chars().nth(1).unwrap_or('0') == '/' {
            total_score += spare_bonus(&frames, frame_index) + 10 - first_roll;
        }

        frame_index += 1;
    }

    total_score
}

fn parse_roll(roll: char) -> u32 {
    match roll {
        '/'=>0,
        num => num.to_digit(10).unwrap_or(0),
    }
}

fn spare_bonus(frames: &[&str], frame_index: usize) -> u32 {
    if let Some(next_frame) = frames.get(frame_index + 1) {
        first_roll_value(next_frame)
    } else {
        0
    }
}

fn first_roll_value(frame: &str) -> u32 {
    parse_roll(frame.chars().next().unwrap_or('0'))
}
