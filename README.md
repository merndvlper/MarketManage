# Market Management
🏪 Rust Market Management System
This is a terminal-based market management system written in Rust. It allows managing a basic inventory and client database, where users can add or delete products and clients, simulate product purchases, and persist data using JSON files. It is a beginner-friendly project for understanding data structures, file I/O, and basic business logic in Rust.



## 📦 Features

- Add / Delete clients and products
- Show all clients and products
- Simulate product purchasing with balance and stock validation
- Automatically saves and loads data from `client.json` and `product.json`
- Uses `serde` for JSON serialization/deserialization

## 🚀 Installation & Running

1. Make sure Rust is installed on your system. If not:  
   [https://rustup.rs/](https://rustup.rs/)

2. Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```bash
cargo run 
```

```pgsql
--- Market ---
1. Show the Clients
2. Show the Products
3. Buying
4. Exit
5. Add New Client
6. Add New Product
7. Delete Client
8. Delete Product
```

