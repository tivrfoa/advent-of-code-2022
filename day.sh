#!/bin/bash

# $1 day
# $2 year

echo "mod day${1};" > "src/y${2}.rs"

touch inputs/$2/day$1.txt inputs/$2/day$1-sample.txt

cp src/template.rs src/y$2/day$1.rs

sed -i "s/dayX/${2}\/day${1}/g" src/y$2/day$1.rs
sed -i "s/DayX/Day${1}/g" src/y$2/day$1.rs

vim src/y$2/day$1.rs

