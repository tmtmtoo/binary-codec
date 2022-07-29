# binary-codec

_binary-codec_ is a type-safe and efficient binary encoding and decoding utilities.

## examples

- [bitmap](./tests/bitmap.rs)

## supports

- [x] core2 (no_std)
- [x] std
- [ ] tokio

## usage

Add this to your Cargo.toml:

### core2

```
[dependencies]
binary_codec = { version = "*", default_features = false, features = ["core2"] }
```

### std

```
[dependencies]
binary_codec = "*"
```
