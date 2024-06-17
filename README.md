# rbot-messages: Bot Beats Game Message Handler

The `rbot-messages` crate serves as the message handler for the Bot Beats game, containing all the message types to and from the game. It includes functionalities for serializing and deserializing message types.

## Purpose

The primary role of this package is to operate discreetly in the background while the end user interacts with the `bot` crate. 

## Features

- Manages all communication between the game and external interfaces.
- Handles serialization and deserialization of message types.
- Can operate as a dynamic library or native code (`dll`/`dylib`), suitable for use in other languages via "C" bindings.

## Usage

To integrate this package into other projects or languages, it can be compiled into a dynamic library (`dll`/`dylib`) with compatible "C" bindings. For the average user, just use it through the `bot` crate.

## Getting Started

To incorporate `rbot-messages` into your project, include it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rbot-messages = "0.1.0"
```

## Contributions

Contributions to enhance or expand the functionality of `rbot-messages` are welcome. Feel free to submit pull requests or raise issues on the GitHub repository.

