# ChainScript

## Description

ChainScript is a Rust library designed to provide a flexible and secure framework for building and executing chained operations. It enables developers to define a series of functions, known as "links," that are executed sequentially, passing data from one link to the next. ChainScript focuses on providing a clear and composable API, robust error handling, and optional state management to facilitate complex workflows within Rust applications. It's particularly useful for scenarios requiring data processing pipelines, transaction management, or asynchronous task orchestration.

## Features

*   **Composable Chain Definition:** ChainScript allows you to define chains of operations using a fluent and intuitive API. You can easily add, remove, or rearrange links in your chain.
*   **Data Passing and Transformation:** Each link in the chain receives the output of the previous link as input, allowing for seamless data transformation and processing.
*   **Error Handling:** ChainScript provides a robust error handling mechanism, allowing you to define how errors are handled at each link in the chain. You can choose to halt execution, retry the link, or continue with a default value.
*   **State Management (Optional):** ChainScript allows for optional state management, enabling links to access and modify a shared state object throughout the chain's execution. This is useful for maintaining context and sharing data between links.
*   **Asynchronous Execution:** ChainScript supports asynchronous execution of links, allowing you to build highly performant and responsive applications.

## Installation

To use ChainScript in your Rust project, you'll need to add it as a dependency to your `Cargo.toml` file.



Next, you can install the dependencies by running the following command in your project directory:



This will download and compile the ChainScript library and its dependencies. Ensure you have a recent version of Rust installed. ChainScript is tested against the latest stable Rust version.

## Usage

Here's a simple example of how to use ChainScript to define and execute a chain of operations:



This example demonstrates how to create a chain of functions that parse a string, double the resulting integer, and add five to it. It also shows how to pass data between links and how to handle errors.

## Contributing

We welcome contributions to ChainScript! To contribute, please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Implement your changes and write tests to ensure they work correctly.
4.  Submit a pull request with a clear description of your changes.

Please ensure that your code adheres to the Rust coding style and that all tests pass before submitting your pull request.

## License

ChainScript is licensed under the MIT License. See the `LICENSE` file for more information.