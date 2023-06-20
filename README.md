## Testing & Build wasm

Run the following command to run the unit tests.

```bash
cargo test --all

# build lib
cd nodejs && RUSTFLAGS='-C link-arg=-s' nj-cli build --release
```
