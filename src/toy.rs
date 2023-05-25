mod toy_service;
use argon2::{self, Config};

fn toy() {
    println!("Bienvenido al puto juego\n");
    let mut b1 = toy_service::toy::Barco {hundido : false, casillas : vec![12, -12, 8]};
    println!("{}", b1.explotar());
    // Hashing the password
    let password = "user_password";
    let salt = generate_salt(); // Implement your own function to generate a random salt
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();
}