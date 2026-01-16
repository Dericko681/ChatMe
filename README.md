# ChatMe - A Simple Chat Application in Rust

<table>
    <thead align="center">
        <tr border: 1px;>
            <td><b>🐛 Issues</b></td>
            <td><b>🔔 Open PRs</b></td>
            <td><b>🔕 Closed PRs</b></td>
        </tr>
     </thead>
    <tbody>
         <tr>
            <td><img alt="Issues" src="https://img.shields.io/github/issues/Dericko681/ChatMe?style=flat&logo=github"/></td>
            <td><img alt="Open Pull Requests" src="https://img.shields.io/github/issues-pr/Dericko681/ChatMe?style=flat&logo=github"/></td>
           <td><img alt="Close Pull Requests" src="https://img.shields.io/github/issues-pr-closed/Dericko681/ChatMe?style=flat&color=critical&logo=github"/></td>
        </tr>
    </tbody>
</table>

ChatMe is a chat application written in Rust that demonstrates concurrency and networking. The project explores Rust's standard library for threading, channels, and TCP communication.

> [!NOTE]
> Currently, the **Distributed Chat** is the primary functional feature. The **Local Chat** version is a planned feature and currently serves as a placeholder.

## Table of Contents

- [Project Overview](#project-overview)
- [Technologies Used](#technologies-used)
- [Distributed Chat (Networking)](#distributed-chat-networking)
    - [Features](#features)
    - [Setup Instructions](#setup-instructions)
- [Local Chat (Threading)](#local-chat-threading-planned)
- [Contributing](#contributing)
- [License](#license)

## Project Overview

The ChatMe project is designed to demonstrate basic concepts around multi-threading and communication protocols in Rust.

- **Distributed Chat (Networking version)**: Uses TCP sockets for communication between multiple clients and a central server. Supports private messaging and broadcasting.
- **Local Chat (Threading version)**: (Planned) A multi-threaded, single-process implementation using Rust's `mpsc` (multi-producer, single-consumer) channels for message passing.

## Technologies Used

- **Rust**: The core programming language.
- **Standard Library**: Utilized features including `std::thread`, `std::sync`, and `std::net`.

## Distributed Chat (Networking)

In this version, a server listens for incoming connections from clients. Clients can connect, choose a username, and send messages to others.

### Features

- **Central Server**: Handles multiple concurrent client connections.
- **Broadcasting**: Messages sent by a client are broadcast to all other connected users.
- **Private Messaging**: Send a message to a specific user using `@username message`.
- **Exit Command**: Type `/exit` to disconnect safely.

### Setup Instructions

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/Dericko681/ChatMe.git
   cd ChatMe
   ```

2. **Build the Project**:
   ```bash
   cargo build
   ```

3. **Run the Server**:
   In one terminal window, start the server:
   ```bash
   cargo run -- server
   ```
   The server will listen on `127.0.0.1:7878` by default.

4. **Run the Client**:
   In a separate terminal window, start the client:
   ```bash
   cargo run -- client
   ```
   Follow the prompts to enter your username and start chatting!

## Local Chat (Threading) [PLANNED]

This feature is currently under development. Once implemented, it will demonstrate:
- Multi-threaded design with sender and receiver running on separate threads.
- Communication between threads using Rust channels (`mpsc`).
- Simple message passing within a single process.

## Contributing

We welcome contributions! To contribute to ChatMe, follow these steps:

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature/your-feature`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to your branch (`git push origin feature/your-feature`).
6. Open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](file:///Users/admin/projects/Project/clones/ChatMe/LICENSE) file for details.

---

**Additional Notes**:
- Future improvements include message encryption, user authentication, and a TUI/GUI.
