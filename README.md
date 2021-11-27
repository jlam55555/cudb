# cudb

Simple MongoDB-like DBMS (rename pending to cuda++)

ECE464 Final project by Jonathan Lam, Derek Lee, Victor Zhang

API documentation can be found at [jlam55555.github.io/cudb][docs].

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
$ cargo build                                 # build project
$ cargo test -- --nocapture --test-threads=1  # run unit tests
$ cargo doc --no-deps                         # build documentation
$ cargo clean                                 # delete build artifacts
```

##### Open documentation in browser
```bash
$ cargo doc --no-deps --open
```

##### Build documentation to `docs/`
```bash
$ ./build_docs.sh
```

##### Test config
```bash
$ cargo test -- --nocapture --test-threads=1
```
- `--no-capture`: Show `stdout` even when the test fails.
- `--test-threads=1`: Prevent running in parallel. This is to prevent
  multiple concurrent access to the database file.
  
To run a single test:
```bash
$ cargo test [TEST_FN_NAME] -- --nocapture
```
where `[TEST_FN_NAME]` is one of the tests defined in the `tests/` directory, e.g.,
`test_pool_fetch`.

##### Build without unused warnings
```bash
$ RUSTFLAGS="$RUSTFLAGS -A unused_variables -A dead_code" cargo build
```
These flags can be used for the `run` and `test` targets as well.

[docs]: https://jlam55555.github.io/cudb/
