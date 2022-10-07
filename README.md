# Art Engine with Rust

Create `assets` directory and a new directory for every layer and put your assets in those directories.

Build binary:

```bash
cargo build --release
```

Now first, create images:

```bash
./target/release/art-engine image generate
```

When creating images finished, now you can create your metadata:

```bash
./target/release/art-engine metadata "Your Collection Name" "Your Description" "Your Base URI"
```

## TODO

- [x] Add command-line support
- [ ] Add odds for assets
- [ ] Better error handling mechanism
- [ ] Support Solana