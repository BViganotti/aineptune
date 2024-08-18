# Trading Data API

This project provides an API for processing and retrieving trading data using the Warp web framework in Rust.

## Features

- **Get Data**: Retrieve statistical data for a given trading symbol.
- **Post Data**: Add new trading data for a given symbol.
- **Concurrency**: Handles great concurrency thanks to Rayon and atomic operations.
- **Performance**: Access times are constant time, ensuring efficient data retrieval and updates.

## Endpoints

### `GET /data`

Retrieve statistical data for a given trading symbol.

#### Request

- **Method**: `GET`
- **Body**: JSON object containing:
  - `symbol` (String): The trading symbol to retrieve data for.
  - `k` (u32): A parameter to determine the number of data points to consider.

#### Response

- **Success**: JSON object containing statistical data (`min`, `max`, `last`, `avg`, `var`).
- **Failure**: JSON object with an error message.

### `POST /data`

Add new trading data for a given symbol.

#### Request

- **Method**: `POST`
- **Body**: JSON object containing:
  - `symbol` (String): The trading symbol to add data for.
  - `values` (Vec<f64>): A list of new data points to add.

#### Response

- **Success**: JSON object confirming the data was added.
- **Failure**: JSON object with an error message.

## Usage

### Prerequisites

- Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

### Running the Server

1. Clone the repository:
    ```sh
    git clone https://github.com/BViganotti/aineptune.git
    cd aineptune
    ```

2. Build and run the server:
    ```sh
    cargo run
    ```

3. The server will start and listen for requests on `http://127.0.0.1:8080`.

## Dependencies

- [warp](https://crates.io/crates/warp): A super-easy, composable, web server framework for warp speeds.
- [serde](https://crates.io/crates/serde): A framework for serializing and deserializing Rust data structures efficiently and generically.
- [tokio](https://crates.io/crates/tokio): An asynchronous runtime for the Rust programming language.
- [rayon](https://crates.io/crates/rayon): A data parallelism library for Rust, enabling efficient concurrent processing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.