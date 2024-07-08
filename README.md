# Rust Simple CLI Calculator

This project is a simple command-line interface (CLI) calculator written in Rust. It allows users to perform basic arithmetic operations such as addition, subtraction, multiplication, division, and modulus.

## Features

- Perform basic arithmetic operations: addition, subtraction, multiplication, division, and modulus.
- Simple and interactive command-line interface.

## Project Structure

The project consists of the following main components:

- **Calculator Struct**: Represents a calculator with two operands and an operator.
- **Operations Enum**: Defines the supported operations: Add, Subtract, Multiply, Divide, and Modulus.
- **CLI Module**: Handles user input from the command line.
- **Main Module**: Integrates the CLI and calculator functionalities to provide the overall calculator functionality.

## Code Overview

### Calculator Struct

The `Calculator` struct holds two operands (`operand1` and `operand2`) and an operator (`op`). The `calc` method performs the operation based on the operator.

### CLI Module

The `cli` module provides functions to read input from the command line. It uses the `rprompt` crate for interactive prompts.

### Main Module

The `main` module integrates the CLI and calculator functionalities to create an interactive CLI calculator.

## Dependencies

- [rprompt](https://crates.io/crates/rprompt): A crate for reading user input from the command line.

## Usage

1. Clone the repository:

    ```bash
    git clone https://github.com/GideonBature/rust-simple_cli_calculator
    ```

2. Navigate to the project directory:

    ```bash
    cd rust-simple_cli_calculator
    ```

3. Build the project:

    ```bash
    cargo build
    ```

4. Run the project:

    ```bash
    cargo run
    ```

Follow the prompts to enter the operands and the operator. The calculator will perform the operation and display the result.

## License

This project is licensed under the MIT License.
