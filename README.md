# Port Map

![Rust Version](https://img.shields.io/badge/rust-1.54+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

This is a simple toy port scanner written in Rust. It's a small project that I created as my first attempt to learn Rust programming. The port scanner allows you to scan a target host for open ports within a specified range.

## Features

- Scans a target host for open ports within a specified range.
- Lightweight and easy-to-understand code, making it a great learning resource for Rust beginners.

## Getting Started

### Prerequisites

- Rust 1.54 or higher

### Usage

1. Clone the repository:
2. Run ```cargo run 127.0.0.1 -scan ``` for full scan or  ```cargo run 127.0.0.1 -scan -port ....``` for targeted scan
   Acknowledgements
   This project was inspired by my journey to learn Rust and the desire to create a simple yet functional tool to deepen my understanding of networking and systems programming.

### License
This project is licensed under the MIT License - see the LICENSE file for details.

### Disclaimer
This port scanner is purely a toy project and should not be used for any malicious activities. Always ensure you have proper authorization before scanning any target hosts.


This project was created as a part of my learning experience with Rust. I am not a professional Rust developer, and this code might not represent the best practices or optimal solutions. Feel free to contribute, provide feedback, or use it as a reference for your own learning journey in Rust programming. Happy coding!