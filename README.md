# run in netns

Run a command in a network namespace as an unprivileged user with this SUID binary.

Edit `NAMESPACE_PATH` in `src/main.rs` then build:

```
cargo build --release
```

Change ownership of the binary to a user that is able to access the namespace (usually root) and set the SUID bit:

```
chown root:root target/release/run-in-netns
chmod u+s target/release/run-in-netns
```

The target user must be able to read but not write to the binary:

```
chmod o+r-w target/release/run-in-netns
```

Use the binary as the unprivileged user:

```
target/release/run-in-netns ip addr
```
