#!/usr/bin/env bash
~/.cargo/bin/bindgen fastlz/fastlz.h \
  -o src/bindings.rs -- -Ifastlz \
