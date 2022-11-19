# rocket_error_stack

## Description
This crate provides [rocket_error_stack::Report]
and [rocket_error_stack::Result] as thin Wrappers around
[error_stack::Report] and [error_stack::Result] with
rockets [rocket::response::Responder] implemented.

## Install

```toml
[dependencies]
rocket = { version = "0.5.0-rc.2" }
error-stack = "0.2"
rocket_error_stack = "0.1"
```

## Note about required trait implementations

Your Reports will have to implement
[rocket_error_stack::StatusCodeReport] to
define the HTTP response code.

## Supported versions

This crate currently supports rocket
0.5.0-rc.2 and error-stack 0.2.

## Usage

```rust
use std::fmt;
use rocket::http::Status;
use error_stack::{Context, IntoReport, ResultExt};
use std::fs;

use rocket_error_stack::{Result, StatusCodeReport};

#[derive(Debug)]
struct SomeError(pub Status);

impl StatusCodeReport for SomeError {
  fn status(&self) -> Status {
      self.0
  }
}

impl fmt::Display for SomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error during request")
    }
}

impl Context for RequestError {}

#[get("/")]
fn get() -> Result<(), SomeError> {
  fs::read_to_string("nonexistent")
    .into_report()
    .attach_printable("Something went wrong!")
    .change_context(SomeError(Status::InternalServerError))?;
  Ok(())
}

```
