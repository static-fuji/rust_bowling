#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer(score_str: &str) -> u32 {
    let mut total_score = 0;
    let mut frame_index = 0;
    let frames: Vec<&str> = score_str.split('|').filter(|&s| !s.is_empty()).collect();

    while frame_index < frames.len() && frame_index < 10 {
        let frame = frames[frame_index];

        if frame == "X" {
            total_score += 10 + strike_bonus(&frames, frame_index);
            frame_index += 1;
        } else {
            let frame = frames[frame_index];
            let first_roll = parse_roll(frame.chars().next().unwrap());
            let second_roll = parse_roll(frame.chars().nth(1).unwrap_or('0'));

            total_score += first_roll + second_roll;

            if frame.chars().nth(1).unwrap_or('0') == '/' {
                total_score += spare_bonus(&frames, frame_index) + 10 - first_roll;
            }
            frame_index += 1;
        }
    }

    total_score
}

fn strike_bonus(frames: &[&str], frame_index: usize) -> u32 {
    let mut bonus = 0;

    if let Some(next_frame) = frames.get(frame_index + 1) {
        let next_first_frame = parse_roll(next_frame.chars().next().unwrap());
        let next_second_frame_char = next_frame.chars().nth(1).unwrap_or('0');
        if next_frame == &"X" {
            bonus += 10;

            if let Some(next_next_frame) = frames.get(frame_index + 2) {
                bonus += first_roll_value(next_next_frame);
            }
        } else if next_second_frame_char == '/' {
            bonus += sum_rolls(next_frame) + 10 - next_first_frame;
        } else {
            bonus += sum_rolls(next_frame);
        }
    }

    bonus
}

fn sum_rolls(frame: &str) -> u32 {
    frame.chars().map(parse_roll).sum()
}

fn parse_roll(roll: char) -> u32 {
    match roll {
        '/' => 0,
        '-' => 0,
        'X' => 10,
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
