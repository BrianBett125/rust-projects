use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::HashMap;

// ------------------ SIMPLE ENCRYPTION ------------------

fn xor_encrypt_decrypt(data: &str, key: &str) -> String {
    data.chars()
        .zip(key.chars().cycle())
        .map(|(d, k)| ((d as u8) ^ (k as u8)) as char)
        .collect()
}

// ------------------ STRUCTS ------------------

#[derive(Clone)]
struct Credential {
    site: String,
    username: String,
    password: String,
}

struct Vault {
    data: HashMap<String, Credential>,
}

// ------------------ IMPLEMENTATION ------------------

impl Vault {
    fn new() -> Self {
        Vault {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, site: String, username: String, password: String) {
        let cred = Credential {
            site: site.clone(),
            username,
            password,
        };
        self.data.insert(site, cred);
        println!("✅ Credential saved.");
    }

    fn list(&self) {
        if self.data.is_empty() {
            println!("📭 No stored credentials.");
            return;
        }

        println!("🔐 Stored Sites:");
        for key in self.data.keys() {
            println!("- {}", key);
        }
    }

    fn view(&self, site: &str) {
        match self.data.get(site) {
            Some(c) => {
                println!("🌐 Site: {}", c.site);
                println!("👤 Username: {}", c.username);
                println!("🔑 Password: {}", c.password);
            }
            None => println!("⚠️ Not found."),
        }
    }

    fn delete(&mut self, site: &str) {
        if self.data.remove(site).is_some() {
            println!("🗑️ Deleted.");
        } else {
            println!("⚠️ Not found.");
        }
    }

    fn save(&self, filename: &str, key: &str) {
        let mut file = File::create(filename).expect("Error creating file");

        for cred in self.data.values() {
            let line = format!("{},{},{}\n", cred.site, cred.username, cred.password);
            let encrypted = xor_encrypt_decrypt(&line, key);
            file.write_all(encrypted.as_bytes()).unwrap();
        }

        println!("💾 Vault saved securely.");
    }

    fn load(&mut self, filename: &str, key: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("📂 No vault found, starting fresh.");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let decrypted = xor_encrypt_decrypt(&contents, key);

        for line in decrypted.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                self.add(
                    parts[0].to_string(),
                    parts[1].to_string(),
                    parts[2].to_string(),
                );
            }
        }

        println!("🔓 Vault loaded.");
    }
}

// ------------------ AUTH ------------------

fn authenticate() -> String {
    println!("Enter master password:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// ------------------ MAIN ------------------

fn main() {
    let filename = "vault.db";
    let mut vault = Vault::new();

    let key = authenticate();
    vault.load(filename, &key);

    loop {
        println!("\n==== PASSWORD VAULT ====");
        println!("1. Add Credential");
        println!("2. List Sites");
        println!("3. View Credential");
        println!("4. Delete Credential");
        println!("5. Save & Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut site = String::new();
                let mut user = String::new();
                let mut pass = String::new();

                println!("Site:");
                io::stdin().read_line(&mut site).unwrap();

                println!("Username:");
                io::stdin().read_line(&mut user).unwrap();

                println!("Password:");
                io::stdin().read_line(&mut pass).unwrap();

                vault.add(
                    site.trim().to_string(),
                    user.trim().to_string(),
                    pass.trim().to_string(),
                );
            }
            "2" => vault.list(),
            "3" => {
                println!("Enter site:");
                let mut site = String::new();
                io::stdin().read_line(&mut site).unwrap();
                vault.view(site.trim());
            }
            "4" => {
                println!("Enter site to delete:");
                let mut site = String::new();
                io::stdin().read_line(&mut site).unwrap();
                vault.delete(site.trim());
            }
            "5" => {
                vault.save(filename, &key);
                println!("👋 Goodbye.");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}
