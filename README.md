# `screaming_float`

## What is it?

It's a variant of [`noisy_float`](https://crates.io/crates/noisy_float) that [screams](https://twitter.com/stephentyrone/status/1306637334012715008) whenever you create a `NaN`.

## How does it work?

Like this:

```Rust
use screaming_float::s64;

fn main() {
    s64(0.0) / s64(0.0); // screams
}
```

## Disclaimers

Please don't use this.
