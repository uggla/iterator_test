# Rust Iterator Test Game

This is a simple command-line fighting game written in Rust. It serves as a demonstration of how to implement and use iterators on custom collection types.

## Description

The game simulates battles between a team of players and a team of enemies. Each character has a certain amount of life points. In each round, a random player and a random enemy are chosen to fight. A dice roll determines the winner of the fight, and the loser takes damage. When a character's life reaches zero, they are removed from the game.

The game ends when either all players or all enemies are defeated, or when the user chooses to quit.

## Features

- **Player and Enemy Characters**: Simple structs representing the fighters.
- **CharacterCollection**: A generic collection to hold players or enemies, demonstrating custom iterator implementations (`iter`, `iter_mut`, `into_iter`).
- **Poisoned Trait**: A simple trait that can be applied to characters to affect their abilities (e.g., a poisoned player cannot gain score).
- **Turn-based Combat**: A game loop where random characters fight until one side is defeated.

## How to Run

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd iterator_test
    ```

2.  **Build the project:**
    ```bash
    cargo build
    ```

3.  **Run the game:**
    ```bash
    cargo run
    ```

    Follow the on-screen prompts to proceed with the fight or quit the game.

## Code Structure

-   `src/main.rs`: Contains the main game loop, the `CharacterCollection` implementation, and the fight logic.
-   `src/player.rs`: Defines the `Player` struct and its associated methods, including the `Poisoned` trait implementation.
-   `src/enemy.rs`: Defines the `Enemy` struct and its methods.
