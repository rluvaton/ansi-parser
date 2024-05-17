#!/bin/bash

./target/release/ansi-parser-extended parse --split-lines --from-line=400000 --to-line=500000 --file ../examples/fixtures/huge.ans >/dev/null 2>&1
