# oxk (Oxidized Kafka) 🦀

**oxk** is a blazing fast, modern CLI (Command Line Interface) for Apache Kafka, written in Rust. Designed for developers who need performance, simplicity, and low resource usage when interacting with Kafka clusters.

## 🚀 Features

- **Produce**: Send messages quickly via direct text or file input.
- **Consume**: Stream messages in real-time with Consumer Group control.
- **List**: Visualize available topics in the cluster.
- **High Performance**: Built on top of `librdkafka` and `tokio`.

## 📦 Installation

### Prerequisites

Ensure you have Rust and Cargo installed.

### Building Locally

```bash
# Clone the repository
git clone https://github.com/niwau/oxk.git
cd oxk

# Install the binary to your PATH
cargo install --path .
```

## 🛠️ Usage

The default broker endpoint is `localhost:9092`. You can change it using the `-b` or `--brokers` flag.

### 1\. List Topics

See which topics exist in your cluster.

```bash
oxk topic list

oxk -b "kafka-prod:9092" topic list
```

### 2\. Produce Messages

**Simple Send (Key/Value):**

```bash
oxk topic produce my-topic -k "id-123" -p "json payload here"
```

**Send from File:**
Useful for large payloads or complex JSONs.

```bash
oxk topic produce my-topic -f ./payload.json
```

### 3\. Consume Messages

Starts a consumer in the default group `oxk-group`.

```bash
oxk topic consume my-topic
```

**Custom Consumer Group:**

```bash
oxk topic consume my-topic -g "debug-group"
```

## 🤝 Contributing

Contributions are welcome\! Feel free to open Issues or Pull Requests.

1. Fork the project
2. Create your Feature Branch (`git checkout -b feature/MyFeature`)
3. Commit your changes (`git commit -m 'Add: MyFeature'`)
4. Push to the Branch (`git push origin feature/MyFeature`)
5. Open a Pull Request

## 📝 License

Distributed under the MIT License. See `LICENSE` for more information.
