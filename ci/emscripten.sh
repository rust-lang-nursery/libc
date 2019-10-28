#!/usr/bin/env bash
# Copyright 2017 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -ex

hide_output() {
  set +x
  on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap '$on_err' ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  "${@}" &> /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  rm -f /tmp/build.log
  set -x
}

git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
cd /emsdk-portable
hide_output ./emsdk install 1.38.46-upstream
./emsdk activate 1.38.46-upstream

# node 8 is required to run wasm
cd /
curl --retry 5 -L https://nodejs.org/dist/v12.3.1/node-v12.3.1-linux-x64.tar.xz | \
    tar -xJ
