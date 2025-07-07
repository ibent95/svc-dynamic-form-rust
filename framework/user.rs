// Representasi class bernama "User"
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

impl User {
    // Constructor ala Rust (seperti `new()` di class OOP)
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    // Fungsi contoh instance method
    pub fn greet(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }
}

// Implementasi Default agar bisa menggunakan User::default()
impl Default for User {
    fn default() -> Self {
        Self {
            name: "Anonymous".to_string(),
            age: 0,
        }
    }
}

fn main() {
    // Instansiasi menggunakan constructor `new`
    let user1 = User::new("Ibnu".to_string(), 30);
    user1.greet();

    // Instansiasi menggunakan Default
    let user2 = User::default();
    user2.greet();

    // Kombinasi nilai custom + default
    let user3 = User {
        name: "Rustacean".to_string(),
        ..Default::default()
    };
    user3.greet();

    // Cetak struct sebagai debug
    println!("User3 struct: {:?}", user3);
}
