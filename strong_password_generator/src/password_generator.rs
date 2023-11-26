use rand::Rng;
pub fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(32..=127) as u8 as char)
        .collect()
}
