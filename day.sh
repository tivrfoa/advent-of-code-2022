#!/bin/bash

# $1 day
# $2 year
cp src/template.rs src/y$2/day$1.rs

sed -i "s/ayX/ay${1}/g" src/y$2/day$1.rs

vim src/y$2/day$1.rs

