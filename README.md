# :mouse2: mini_uuid

**mini_uuid** is a Rust crate that converts UUIDs to a compact base64 representation, making them shorter and more efficient for storage and transmission!

**P.S, I made this in my free time for fun when I was bored one day, not sure how much I will iterate this, please don't use in production unless you know the code you are using**

## Features

You can use this crate with `v1`, `v4`, or `v7` uuids. Padding is optional, with the `padding` feature and you can enable url safety with the `url_safe` feature.

## Usage

```rust

use mini_uuid::MiniUuid;

fn main() {
  // Generate a UUID
  let uuid = uuid::Uuid::new_v4();

  // Convert to mini UUID (base64)
  let mini = MiniUuid::from_uuid(&uuid);
  // or just create it without the `uuid` crate
  let mini = MiniUuid::new();
  println!("Mini UUID: {}", mini.to_string());

  // Convert back to UUID
  let original_uuid = mini.to_uuid().unwrap();
  println!("Original UUID: {}", original_uuid);

}
```

## Methods

```rust
// Creates a `MiniUuid` from a `uuid::Uuid`
let mini = MiniUuid::from_uuid(uuid::Uuid::new_v4());

// Creates a `MiniUuid` from a string representing `uuid::Uuid`
let mini = MiniUuid::from_uuid_str("67e55044-10b1-426f-9247-bb680e5fe0c8");

// Creates a `MiniUuid` from a string by checking the `padding` and `url_safe` features
let mini = MiniUuid::from_string("/*INSERT MINI UUID*/");

// Creates a `MiniUuid` from a url safe base64 string with padding
let mini = MiniUuid::from_url_base64("Z+VQRBCxQm+SR7toDl/gyA==");

// Creates a `MiniUuid` from a url safe base64 string without padding
let mini = MiniUuid::from_url_base64_no_pad("Z+VQRBCxQm+SR7toDl/gyA");

// Creates a `MiniUuid` from a base64 string with padding
let mini = MiniUuid::from_base64("Z-VQRBCxQm-SR7toDl_gyA==");

// Creates a `MiniUuid` from a base64 string without padding
let mini = MiniUuid::from_base64_no_pad("Z-VQRBCxQm-SR7toDl_gyA");

// Converts `MiniUuid` to `uuid::Uuid`
let uuid = mini.to_uuid();

// Converts `MiniUuid` to a string by checking the `padding` and `url_safe` features
let uuid = mini.to_string();

// Converts `MiniUuid` to url safe base64 string with padding
let uuid = mini.to_url_base64();

// Converts `MiniUuid` to url safe base64 string without padding
let uuid = mini.to_url_base64_no_pad();

// Converts `MiniUuid` to base64 string with padding
let uuid = mini.to_base64();

// Converts `MiniUuid` to base64 string without padding
let uuid = mini.to_base64_no_pad();
```

## Installation

Add `mini_uuid` to your `Cargo.toml`:

```toml
[dependencies]
mini_uuid = "0.2.0"
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

Inspired by [uuid crate](https://github.com/uuid-rs/uuid) and [short_uuid crate](https://github.com/radim10/short-uuid)

Made with :heart: by [nanokeshtw](https://github.com/nanokeshtw)
