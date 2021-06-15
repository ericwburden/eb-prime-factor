#!/bin/bash

LIB='eb_prime_factor'

cargo doc \
    && echo "<meta http-equiv=refresh content=0;url=$LIB/index.html>" > target/doc/index.html \
    && ~/.local/bin/ghp-import -n target/doc \
    && git push origin gh-pages:gh-pages
