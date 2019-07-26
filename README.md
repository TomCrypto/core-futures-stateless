**NOTE**: This is a WORKAROUND crate to bypass current standard library limitations. It will not be required in the future!

This crate is a libcore wrapper inspired from [here][1] which allows using `#![feature(async_await)]` in some restricted `#![no_std]` environments such as microcontrollers. In the current implementation the `task::Context` is a dummy context which does nothing and is not stored anywhere, so this crate is only useful if futures are woken up out-of-band e.g. through some interrupt request. For instance on Cortex-M the `PendSV` exception can be used to that effect.

A proper implementation would emulate thread-local-storage but I haven't needed this yet, and likely won't for as long as this workaround is needed.

# Usage

Put the following in your `Cargo.toml` to override libcore:

```toml
[dependencies.core]
package = "core-futures-stateless"
version = "0.1.0"
```

This crate also exposes an additional non-core method `task::stateless_waker()` which will create a stateless `task::Waker` (with no behaviour) that your executor can use to poll futures. This method can easily be implemented outside this crate but is provided for convenience and to reduce code size.

[1]: https://crates.io/crates/core-futures-tls
