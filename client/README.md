client
===

A template for Embassy applications that attempts to connect to a server application on another board. This networking
communication is performed via the Uart. Visual Studio Code is assumed, but you should be able to use other editors
and IDEs.

The information provided here will apply reasonably to the server project also.

Development
---

You'll need the following toolchains:

```
rustup update
rustup target add thumbv7em-none-eabihf # for the nRF52840
rustup target add thumbv8m.main-none-eabihf # for the nRF9160
```

Testing the app
---

```
cargo test
```

Running the app on an nRF52840-dk
---

Ensure that probe-run is installed:

```
cargo install probe-run
```

We also use flip-link so that we are more able to detect stack overflows:

```
cargo install flip-link
```

...then:

```
PROBE_RUN_PROBE='1366:1015' \
DEFMT_LOG=debug \
PROBE_RUN_CHIP='nrf52840_xxAA' \
cargo run --no-default-features \
  --target thumbv7em-none-eabihf \
  --features "nrf52840-dk"
```

Here's an example that runs on the nRF9160 DK in secure mode instead:

```
PROBE_RUN_PROBE='1366:1055' \
DEFMT_LOG=debug \
PROBE_RUN_CHIP='nrf9160_xxAA' \
cargo run --no-default-features \
  --target thumbv8m.main-none-eabihf \
  --features "nrf9160-dk-s"
```

(change the `DEFMT_LOG` env as required or even omit it for no logging).

> If you are running wanting to run multiple devices from the same vendor and process type simultaneoulsy then you can use
> the serial number for `PROBE_RUN_PROBE`. This is provided by `probe-run` e.g.
> ```
> probe-run --list-probes
> the following probes were found:
> [0]: J-Link (J-Link) (VID: 1366, PID: 1015, Serial: 000683699482, JLink)
> [1]: J-Link (J-Link) (VID: 1366, PID: 1055, Serial: 000960066748, JLink)
> ```
>
> ...and supposing the first one was the probe we want:
> ```
> PROBE_RUN_PROBE='000683699482' \
> ...
> ```

Debugging the app
---

For debugging:

```
cargo install --git https://github.com/probe-rs/probe-rs probe-rs-debugger
```

...and then visit https://github.com/probe-rs/vscode#vs-code-probe-rs-debugger for instructions on 
how to install the VSCode plugin.

You should also install the `CodeLLDB` plugin from the Visual Studio Code market place so that you are
able to debug unit tests.

Releasing the app
---

Similar to running the debug app (can also use `build` instead of `run` to simply produce the firmware):

> By default, we configure the runner to measure stack space. Here, we
> override the runner configuration of `.cargo/config.toml`.

```
CARGO_TARGET_THUMBV7EM_NONE_EABIHF_RUNNER='probe-run' \
DEFMT_LOG=info \
PROBE_RUN_CHIP='nrf52840_xxAA' \
cargo build --target thumbv7em-none-eabihf --release
```

© Copyright [Titan Class P/L](https://www.titanclass.com.au/), 2021
