#!/bin/bash

cp src/template.rs src/day$1.rs

sed -i "s/ayX/ay${1}/g" src/day$1.rs

vim src/day$1.rs

