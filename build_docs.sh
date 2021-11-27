#!/bin/sh

# build documentation
cargo doc --no-deps

# move from default docs location to github pages location
rm -rf docs/
cp -r target/doc/ docs/

# build default landing/redirect page at document root
echo "<script>document.location.href='cudb/index.html';</script>" >docs/index.html
