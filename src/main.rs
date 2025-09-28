use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    
    dotenvy::dotenv()?;

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    Ok()
}
