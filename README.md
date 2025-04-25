# 🧹 cargo-kit

```text
                                   _     _  _
  ___   __ _  _ __   __ _   ___   | | __(_)| |_
 / __| / _` || '__| / _` | / _ \  | |/ /| || __|
| (__ | (_| || |   | (_| || (_) | |   < | || |_
 \___| \__,_||_|    \__, | \___/  |_|\_\|_| \__|
                    |___/
```

This tool automates that setup process, making configuration simpler and faster.

## ✨ Features

`cargo-kit` can create or modify Cargo profiles in your `Cargo.toml` manifest and RUSTFLAGS in
the [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html#configuration-format) file, based on a
set of predefined templates:

- `fast-compile` - minimizes compilation times
  - Disables debuginfo generation and uses a faster linker.
  - In nightly mode, it also enables
    the [Cranelift codegen backend](https://nnethercote.github.io/perf-book/build-configuration.html#cranelift-codegen-back-end)
    and
    the [parallel frontend](https://nnethercote.github.io/perf-book/build-configuration.html#experimental-parallel-front-end).
- `fast-runtime` - maximizes runtime performance
  - Enables [LTO](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) and other settings designed to maximize
    runtime performance.
- `min-size` - minimizes binary size
  - Similar to `fast-runtime`, but uses optimization flags designed for small binary size.

You can also modify these templates in the interactive mode to build your own custom template.

## 🚀 Installation

To install **cargo-kit**, simply clone the repository and follow the instructions below:

```bash
git clone git@github.com:trinhminhtriet/cargo-kit.git
cd cargo-kit

cargo build --release
cp target/release/cargo-kit /usr/local/bin/cargo-kit

```

Alternatively, you can install the binary directly from the repository:

```bash
cargo install --path .
```

Running the below command will globally install the `cargo-kit` binary.

```bash
cargo install cargo-kit
```

Optionally, you can add `~/.cargo/bin` to your PATH if it's not already there

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 💡 Usage

- Interactive mode (CLI dialog that guides you through the process):

```sh
$ cargo kit
```

- Non-interactive mode (directly apply a predefined template to your Cargo workspace):

```sh
$ cargo kit apply <template> <profile>
# For example, apply `fast-runtime` template to the `dist` profile
$ cargo kit apply fast-runtime dist
```

You can enable additional configuration options that require a nightly compiler by running `cargo-kit` with a
nightly Cargo (e.g. `cargo +nightly kit`) or by using the `--nightly` flag.

Note that you should be executing `cargo kit` inside a directory that is a part of a Cargo workspace. It will then
apply the configuration options to that workspace.

### Caveats

- The configuration applied by this tool is quite opinionated and might not fit all use-cases
  perfectly. `cargo-kit` mostly serves to improve _discoverability_ of possible Cargo profile and config options, to
  help you find the ideal settings for your use-cases.
- `cargo-kit` currently only modifies `Cargo.toml` and `config.toml`. There are other things that can be configured
  to achieve e.g. even smaller binaries, but these are out of scope for this tool, at least at the moment.
- `cargo-kit` currently ignores Cargo settings that are not relevant to performance.
- Cargo config (`config.toml`) changes are applied to the global `build.hostflags` setting, because per-profile
  RUSTFLAGS are still [unstable](https://github.com/rust-lang/cargo/issues/10271).

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
