pub mod fs;

pub fn gen_id() -> String {
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect::<String>()
}
