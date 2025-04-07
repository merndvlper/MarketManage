use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Client {
    name: String,
    surrender_name: String,
    balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    price: f64,
    stock: u32,
}

struct Market {
    clients: HashMap<u32, Client>,
    products: HashMap<u32, Product>,
}

impl Market {
    fn new() -> Self {
        let clients: HashMap<u32, Client> = fs::read_to_string("client.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        let products: HashMap<u32, Product> = fs::read_to_string("product.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        Market { clients, products }
    }

    fn save_data_json(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.clients) {
            fs::write("client.json", json).expect("Client data could not be written");
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.products) {
            fs::write("product.json", json).expect("Product data could not be written");
        }
    }

    fn add_client(&mut self, id: u32, client: Client) {
        self.clients.insert(id, client);
        self.save_data_json();
    }

    fn add_product(&mut self, id: u32, product: Product) {
        self.products.insert(id, product);
        self.save_data_json();
    }

    fn delete_client(&mut self, id: u32) {
        if self.clients.remove(&id).is_some() {
            println!("Client deleted.");
        } else {
            println!("Client not found.");
        }
        self.save_data_json();
    }

    fn delete_product(&mut self, id: u32) {
        if self.products.remove(&id).is_some() {
            println!("Product deleted.");
        } else {
            println!("Product not found.");
        }
        self.save_data_json();
    }

    fn buy(&mut self, client_id: u32, product_id: u32) {
        if let (Some(client), Some(product)) = (
            self.clients.get_mut(&client_id),
            self.products.get_mut(&product_id),
        ) {
            if product.stock == 0 {
                println!("Product is out of stock!");
            } else if client.balance < product.price {
                println!("Insufficient funds!");
            } else {
                client.balance -= product.price;
                product.stock -= 1;
                println!(
                    "customer named {} {}, bought the '{}' product!",
                    client.name, client.surrender_name, product.name
                );
                self.save_data_json();
            }
        } else {
            println!("Client or product not found.");
        }
    }

    fn show_clients(&self) {
        for (id, client) in &self.clients {
            println!("ID: {} => {} {}, Balance: {:.2}", id, client.name, client.surrender_name, client.balance);
        }
    }

    fn show_product(&self) {
        for (id, product) in &self.products {
            println!("ID: {} => {} | Price: {:.2} | Stock: {}", id, product.name, product.price, product.stock);
        }
    }
}

fn main() {
    let mut market = Market::new();

    loop {
        println!("\n--- Market ---");
        println!("1. Show the Clients");
        println!("2. Show the Products");
        println!("3. Buying");
        println!("4. Exit");
        println!("5. Add New Client");
        println!("6. Add New Product");
        println!("7. Delete Client");
        println!("8. Delete Product");
        println!("Enter Your Choose: ");

        let mut choose = String::new();
        io::stdin().read_line(&mut choose).unwrap();
        let choose = choose.trim();

        match choose {
            "1" => market.show_clients(),
            "2" => market.show_product(),
            "3" => {
                let client_id = input_u32("Enter Client ID:");
                let product_id = input_u32("Enter Product ID:");
                market.buy(client_id, product_id);
            },
            "4" => {
                println!("Exiting...");
                break;
            },
            "5" => {
                let id = input_u32("New client ID:");
                let name = input_string("Name:");
                let surrender_name = input_string("Surrender Name:");
                let balance = input_f64("Balance:");

                market.add_client(id, Client { name, surrender_name, balance });
            },
            "6" => {
                let id = input_u32("New Product ID:");
                let name = input_string("Product Name:");
                let price = input_f64("Price:");
                let stock = input_u32("Stock:");

                market.add_product(id, Product { name, price, stock });
            },
            "7" => {
                let id = input_u32("Customer to be deleted ID:");
                market.delete_client(id);
            },
            "8" => {
                let id = input_u32("Product to be deleted ID:");
                market.delete_product(id);
            },
            _ => println!("Invalid Command!"),
        }
    }
}

fn input_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();
    data.trim().to_string()
}

fn input_u32(prompt: &str) -> u32 {
    println!("{}", prompt);
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();
    data.trim().parse().unwrap_or(0)
}

fn input_f64(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();
    data.trim().parse().unwrap_or(0.0)
}
