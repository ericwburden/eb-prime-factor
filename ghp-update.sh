#!/bin/bash

LIB='eb-prime-factor'

cargo doc \
    && echo "<meta http-equiv=refresh content=0;url=$LIB/index.html>" > target/doc/index.html \
    && ~/.local/bin/ghp-import -n target/doc
