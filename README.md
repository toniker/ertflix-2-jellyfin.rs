# ERTFLIX to Jellyfin Middleware

This project serves as a middleware server application that facilitates communication between Jellyfin clients and the ERTFLIX backend. It provides a structured way to handle requests and responses, allowing seamless integration of media collections from both systems.

## Project Structure

```
ertflix-2-jellyfin
├── src
│   ├── api
│   │   ├── mod.rs          # Module declaration for the API layer
│   │   └── ertflix_client.rs # Implementation of the ERTFLIX client
│   ├── models
│   │   ├── mod.rs          # Module declaration for the models layer
│   │   ├── ertflix.rs      # Object representations for ERTFLIX collections
│   │   └── jellyfin.rs     # Object representations for Jellyfin collections
│   ├── routes
│   │   ├── mod.rs          # Module declaration for the routes layer
│   │   └── handlers.rs      # Route handlers for the server application
│   ├── services
│   │   ├── mod.rs          # Module declaration for the services layer
│   │   └── media_service.rs # Business logic for media-related operations
│   ├── config.rs           # Configuration settings for the application
│   ├── error.rs            # Custom error types and handling logic
│   └── main.rs             # Entry point of the application
├── .gitignore               # Specifies files and directories to ignore by Git
├── Cargo.toml               # Configuration file for the Rust package manager
└── README.md                # Documentation for the project
```

## Setup Instructions

1. **Clone the Repository**
   ```bash
   git clone <repository-url>
   cd ertflix-2-jellyfin
   ```

2. **Install Dependencies**
   Ensure you have Rust and Cargo installed. Then run:
   ```bash
   cargo build
   ```

3. **Run the Application**
   To start the server, execute:
   ```bash
   cargo run
   ```

## Usage

Once the server is running, it will listen for incoming requests from Jellyfin clients. The middleware will handle the requests, communicate with the ERTFLIX backend, and return the appropriate responses.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.