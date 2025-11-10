# Rust Webapp

A simple web application built with Rust, demonstrating basic server functionalities using a thread pool to spawn predetermined numbers of threads while continuously deploying "workers" to handle each connection.

## Full Live Coding Video

[Watch Me Code Live](https://youtu.be/vL1_1LjVRgo?si=wfhmmFcNbKDM8hxF)

## Getting Started

### Prerequisites
- Rust (version 1.70 or later) installed via [rustup](https://rustup.rs/)
- Cargo (comes with Rust)

### Installation
1. Clone the repository:
   ```
   git clone https://github.com/BrinkOfSecondBailout/Rust_Webapp.git
   cd Rust_Webapp
   ```
2. Build the project:
   ```
   cargo build
   ```

### Running the Application
1. Start the server:
   ```
   cargo run 
   ```
   The app will listen on `http://localhost:7878` by default.

2. Open your browser and navigate to `http://localhost:7878` to see the home page.

---
