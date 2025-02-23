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

pub fn debug_binary(vec: &Vec<u8>) -> String {
    vec.iter()
        .map(|byte| format!("{:08b}", byte)) // Convertir chaque octet en une string binaire de 8 bits
        .collect::<Vec<String>>() // Collecter dans un Vec<String>
        .join(" ") // Joindre les éléments avec des espaces
}