# uerr
`uerr` is a crate which provides stunning visual error handling.

# Showcase
Using the code below, we can display a simple error and show it to the user.

```rust
use uerr::UserError;

#[test]
fn sample_error() {
    UserError::from("could not open file")
        .and_reason("The system cannot find the file specified.")
        .and_help("Does this file exist?")
        .print_all("uerr/error: ");
}
```
### Output
```text
uerr/error: could not open file
 - caused by: The system cannot find the file specified.
 + help: Does this file exist?
```

## With multiple arguments
```text
program.exe: could not open file
 - caused by: The system cannot find the file specified.
     |        Filler reason.
 + help: Does this file exist?
     |   Filler help.
```
<details>
<summary>Click to see the code.</summary>

```rust
#[test]
fn sample_error() {
    UserError::from("could not open file")
        .and_reason("The system cannot find the file specified.")
        .and_reason("Filler reason.")
        .and_help("Does this file exist?")
        .and_help("Filler help.")
        .print_all("program.exe: ");
}
```
</details>

# Exiting with errors
The `UserError` struct also supports inline exiting.

```rust
#[test]
fn sample_error() {
    UserError::from("Sample Error")
        .print_all("my program: ")
        .exit(-1);
}
```