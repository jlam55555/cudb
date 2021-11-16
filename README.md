# cudb

Simple MongoDB-like DBMS (rename pending)

ECE464 Final project by Jonathan Lam, Derek Lee, Victor Zhang

---

### Functional design
TODO: which features are supported

##### API
TODO: how to interface with this system

---

### Architectural design
TODO: physical storage, block diagram

---

### Build instructions
`cargo` is required to build this project.

```bash
$ cargo build   # build project
$ cargo run     # build and run project
$ cargo clean   # delete build artifacts
```

##### Build without unused warnings
```bash
$ RUSTFLAGS="$RUSTFLAGS -A unused_variables -A dead_code" cargo build
```
