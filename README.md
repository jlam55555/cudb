# cudb (a.k.a. cuda++)

Simple document-based noSQL DBMS modelled after MongoDB. (Has nothing to do with CUDA, has a lot to do with the <b>C</b>ooper <b>U</b>nion and <b>D</b>ata<b>B</b>ases.)

ECE464 Final project by Jonathan Lam, Derek Lee, Victor Zhang

[API documentation][docs]

[Functional and architectural overview][report]

---

### Build instructions
`cargo` is required to build this project.

```bash
$ cargo build                     # build library in src/
$ cargo run --example [EXAMPLE]   # run example examples/
$ cargo test -- --test-threads=1  # run unit tests in tests/
$ cargo doc --no-deps             # build documentation
$ cargo clean                     # delete build artifacts
```

##### Open documentation in browser
```bash
$ cargo doc --no-deps --open
```

##### Build documentation to `docs/`
```bash
$ ./build_docs.sh
```

##### Run example
See the [examples/][./examples/] directory. For example:
```bash
$ cargo run --example index_perf
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
$ cargo test [TEST_FN_NAME] -- --nocapture --test-threads=1
```
where `[TEST_FN_NAME]` is one of the tests defined in the `tests/` directory, e.g.,
`test_pool_fetch`.

[docs]: https://jlam55555.github.io/cudb/
[report]: http://files.lambdalambda.ninja/reports/21-22_fall/ece464_report_cudb.lam_lee_zhang.pdf
