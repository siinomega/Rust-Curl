# 🕸️ Rust-Curl

**Rust-Curl** is a lightweight HTTP client built in Rust, inspired by `curl`, allowing users to send HTTP requests to URLs, manage headers, handle data, and save responses to files.

## ✏️ Features

- **Custom User-Agent:** Define a custom `User-Agent` header for requests.
- **Request Method Selection:** Specify the HTTP method (GET, POST, PUT, DELETE, etc.).
- **Data Inclusion:** Include data in the request body, ideal for POST requests.
- **HTTP Version Control:** Set the HTTP version to 1.1 or 2.0.
- **Response Saving:** Save the response content to a file for easy retrieval.
- **Color-Coded Output:** Provides clear messages and errors using color-coded feedback.

## 🛠️ Prerequisites

Before running Rust-Curl, ensure you have:

- [Rust](https://www.rust-lang.org/) (version 1.67+)
- [Tokio](https://tokio.rs/) (for async functionality)
- [Reqwest](https://docs.rs/reqwest/) (for HTTP requests)
- [Clap](https://docs.rs/clap/) (for command-line argument parsing)
- [Colored](https://docs.rs/colored/) (for terminal output styling)

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
reqwest = "0.11"
tokio = { version = "1.41.0", features = ["rt", "rt-multi-thread", "macros"] }
clap = "4.1.6"
colored = "2.1.0"
http = "1.1.0"
```
## 💻 Installation

1. CloneThe Repository :

   ```bash
   git clone https://github.com/siinomega/Rust-Curl.git
   ```
   
2. Navigate to the project directory :

   ```bash
   cd Rust-Curl
   ```
---
## ⚡ Contribuer

If you would like to contribute to this project, feel free to submit a pull request or report issues. Any contribution is welcome!

---
## 📄 Licence

This project is licensed under the [MIT](LICENSE).
---

Thank you for checking out this project! Feel free to reach out to me if you have any questions or suggestions.
