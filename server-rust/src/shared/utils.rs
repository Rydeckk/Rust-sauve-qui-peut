use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_acess_key() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    rand_string
}