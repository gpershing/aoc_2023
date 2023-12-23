use crate::{vector2::Vector2, dig_instruction::DigInstruction};

pub fn dig_count(instructions: impl Iterator<Item=DigInstruction>) -> u64 {
    let mut signed_area2 = 0i128;
    let mut boundary = 0u64;
    let mut at = Vector2::new(0, 0);
    instructions.for_each(|instr| {
        let vec = instr.direction.as_vector::<i128>() * (instr.count as i128);
        boundary += instr.count;
        let next = at + vec;
        signed_area2 += at.x * next.y - at.y * next.x;
        at = next;
    });
    assert_eq!(Vector2::new(0, 0), at);
    let area = (signed_area2 / 2).abs() as u64;
    // Pick's formula: A = i + b/2 - 1 => i = A + 1 - b/2
    let interior = area + 1 - boundary / 2;
    interior + boundary
}