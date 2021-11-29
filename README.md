# cudb (a.k.a. cuda++)

Simple document-based noSQL DBMS modelled after MongoDB. (Has nothing to do with CUDA, has a lot to do with the <b>C</b>ooper <b>U</b>nion and <b>D</b>ata<b>B</b>ases.)

ECE464 Final project by Jonathan Lam, Derek Lee, Victor Zhang

[API documentation][docs]

---

### Examples

TODO

---

### Functional design

##### CRUD operations

TODO

##### Operations on collections

TODO

##### Documents and data types

TODO

##### Query semantics

TODO: policies on missing index values

---

### Architectural design

##### Persistence / filesystem backing store

TODO

##### Serialization

TODO

##### Documents and values

TODO

##### Indices

TODO: managing indices, matching queries to indices, b-trees

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
