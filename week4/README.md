# Mini Project week4

## Description

This is a simple project that takes a string and returns the number of words in the string.

## Steps to run

- `make format` to format code
  
- `make lint` to lint
  
- `cargo run "Hello World"` to run the program
  - Then you will get

  ```rust
  Hello 1
  World 1
  ```

- `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
  
- `make deploy` which is this `cargo lambda deploy`
