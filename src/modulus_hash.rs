pub(crate) fn module_hash(user_input: &str) {
    const M: u32 = 293;

    let mut text_sum: u32 = 0;
    for (i, letter) in user_input.chars().enumerate() {
        text_sum += letter as u32 * i as u32;
    }
    println!("Текст захэшированный делением на {}: {}", M, text_sum % M)
}
