# Tests

## lib.rs

Remove the placeholder `add` and `test` functions in the root [`lib.rs`](./src/lib.rs) file, and replace with a `test_samples` function that calls our `point` function:

```rust
pub mod sample;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let result = sample::point(-0.5, 0.0, 1000);
        assert_eq!(result, 1000);
    }
}
```

## Try it

Run the tests and verify that they pass:

```bash
cargo test
```

## Return

[Return to the top-level README](./../../README.md)
