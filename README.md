# Rust Webapp

A simple web application built with Rust, demonstrating basic server functionalities using a thread pool to spawn predetermined numbers of threads while continuously deploying "workers" to handle each connection.

## Full Live Coding Video

<iframe width="560" height="315" src="https://www.youtube.com/embed/vL1_1LjVRgo?si=C59fDyZBLa_7JrxG" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

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
