<h1 align="center" style="font-weight: 600">rust-os</h1>

[参考文档](https://os.phil-opp.com)

### how to use

```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# install rust nightly version in order to use unstable features
rustup install nightly
# set nightly as default
rustup default nightly
# install qemu on ubuntu
sudo apt-get install qemu-system
# build
cargo build
# run
cargo run
# test
cargo test
# run unit tests
cargo test --lib
# test single test (eg: should_panic)
cargo test --test should_panic
```