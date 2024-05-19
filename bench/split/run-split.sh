#!/bin/bash

./target/release/ansi-parser-extended parse --split-lines --file ../examples/fixtures/huge.ans >/dev/null 2>&1
