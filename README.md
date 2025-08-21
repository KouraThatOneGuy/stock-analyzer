# README.md

# Stock Analyzer

This project is a simple stock analyzer that calculates the 3-day moving average of stock prices using a C library for the average calculation. It is built with Rust and provides terminal updates for stock price analysis.

## Project Structure

- `c_lib/average.c`: Implementation of the moving average function.
- `c_lib/average.h`: Header file declaring the moving_average function.
- `src/analyzer.rs`: Contains the analyze_stock function for analyzing stock prices.
- `src/ffi.rs`: Declares the external C function moving_average.
- `build.rs`: Compiles the C code in c_lib/average.c when building the Rust project.
- `Cargo.toml`: Configuration file for the Rust project.

## Setup Instructions

1. Ensure you have Rust and Cargo installed on your machine.
2. Clone the repository or download the project files.
3. Navigate to the project directory:
   ```
   cd stock-analyzer
   ```
4. Build the project:
   ```
   cargo build
   ```

## Usage

To run the application, execute the following command in your terminal:
```
cargo run
```

## Implementation Details

The application currently analyzes stock prices by calculating the 3-day moving average. To turn this into an application that automatically gives updates in your terminal, you will need to implement a loop in `src/main.rs` that periodically checks for stock price updates and calls the `analyze_stock` function, printing the results to the terminal.

Consider using the `tokio` crate for asynchronous tasks or `std::thread` for simple periodic execution.