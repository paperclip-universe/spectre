#[inline(always)]
pub fn is_allowed_character(character: char) -> bool {
    let char_num = character as u16;
    char_num != 167 && char_num >= 32 && char_num != 127
}

pub fn filter_allowed_characters(input: &str) -> String {
    input
        .chars()
        .filter(|character| is_allowed_character(*character))
        .collect::<String>()
}
