mod password_generator;
mod password_security;

use password_generator::generate_password;
use password_security::SecurityLevel;

fn main() {
    let pass = "56";

    match SecurityLevel::determine_password_criteria(pass) {
        Ok(security_level) => {
            let length = security_level.security_value();
            let password = generate_password(length);

            println!("Generated password: {}", password);
        }
        Err(e) => println!("Error: {}", e),
    }
}
