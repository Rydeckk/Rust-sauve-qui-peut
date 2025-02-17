use super::*;

pub fn generate_acess_key() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    rand_string
}

pub fn generate_random_number(range: RangeInclusive<i32>) -> i32 {
    let mut rng = thread_rng();
    
    rng.gen_range(range)
    
}