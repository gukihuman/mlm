# Mommy's Little Monsters

_version 0.1.0_

## About

A 2D action RPG built using the Bevy engine

## Links

-   Documentation: https://gukihuman.github.io/mlm-doc/
-   Documentation repository: https://github.com/gukihuman/mlm-doc/

## Setup

1. Install Rust [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Clone the repository and navigate into it

    ```bash
    git clone https://github.com/gukisan/mlm.git

    cd mlm
    ```

3. Run in dev mode _(takes a bit for the first time)_
    ```bash
    cargo run
    ```

## Build

Every time you build a standalone release, you need to comment dynamic_linking feature in dependencies section in `Cargo.toml`

```toml
[dependencies]
# bevy = { version = "0.14.2", features = ["dynamic_linking"] } # cargo run
bevy = { version = "0.14.2" } # cargo build --release
```

```bash
cargo build --release
```

## Dependencies

-   Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
-   Bevy: [https://bevyengine.org/](https://bevyengine.org/)

## Contributing

Contributions are welcome! Feel free to open issues or pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
