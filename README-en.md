# 🌈 A Bevy of Us

> **A digital playground and toolkit designed for Plural Systems.**  
> Built with ❤️ in Rust & Bevy Engine.

## Introduction

**A Bevy of Us** is an open-source project designed specifically for Plural Systems (Plural Systems / DID / OSDD / Headmates).

While most System-focused software centers on “recording” and “scheduling,” we aim to provide **entertainment** and **connection**. This collection of mini-games helps System members collaborate, communicate, or simply enjoy time together within the same body.

The project name is a double entendre:
-   **Bevy** (n.): A group (typically referring to birds or people).
-   **Bevy**: The Rust game engine we use.

## Core Features

-   **Mini-Game Collection**
    -   Game mechanics specifically designed for “one body, multiple minds.”
    -   **Dual-Core Vision:** One screen, two perspectives. A split-screen reaction challenge for practicing co-fronting or duo collaboration.
    -   *(Planned)* **Memory Echo:** Asynchronous puzzle-solving where current members leave clues to help the next online member complete the challenge.
    -   *(Planned)* **Shared Garden:** A collaboratively maintained digital garden honoring each member's unique aesthetic.

-   **Web-First & Native**
    -   Built on **WebAssembly (WASM)**. No installation required—play directly in your browser.
    -   Also compilable into native executables for Windows/Linux/macOS.

## Tech Stack

-   **Language:** Rust
-   **Engine:** Bevy (ECS Architecture)
-   **Key Crates:**
    -   `bevy_hanabi` (Particle Effects)
    -   `bevy_vector_shapes` / `bevy_prototype_lyon` (Geometric Drawing)
    -   `bevy_pkv` (cross-platform persistent storage)

## Quick Start

If you want to run locally or contribute to development:

### Prerequisites
Ensure you have the [Rust Toolchain](https://rustup.rs/) installed.

### Running Locally
```bash
# Clone the repository
git clone https://github.com/your-username/bevy-of-us.git
cd bevy-of-us

# Run the native version
cargo run --features bevy/dynamic_linking
```

## Contributing

We not only want to make this game great, but also build an **inclusive community**.

-   **Developers:** Whether you're proficient in Bevy and Rust or not, we welcome PR submissions. Especially if you have unique insights into ECS architecture.
-   **Artists:** For rapid development, we currently rely on code-generated geometry. But in the future, we're eager to add a **custom character creation system**!
    If you excel at **pixel art**, **vector icons**, or **UI design** and are interested in creating modular components (like hairstyles, accessories, expressions) for a multi-entity system, please follow us! We need your help evolving our “squares and circles” into more expressive avatars.
-   **System:** Your feedback is crucial! If there's a specific game type you'd love to play (like one that helps with grounding or promotes communication), let us know in the Issues.

---

> *“We are many, and we are one.”*