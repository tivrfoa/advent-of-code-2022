#!/bin/bash

# $1 year
# $2 day

year=$1
day=$2

echo "mod day${day};" > "src/y${year}.rs"

touch inputs/$year/day$day.txt inputs/$year/day$day-sample.txt

cp src/template.rs src/y$year/day$day.rs

sed -i "s/dayX/${year}\/day${day}/g" src/y$year/day$day.rs
sed -i "s/DayX/Day${day}/g" src/y$year/day$day.rs

vim src/y$year/day$day.rs

