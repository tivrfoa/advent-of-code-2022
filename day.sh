#!/bin/bash

# $1 year
# $2 day

year=$1
day=$2

if [ -e "src/y$year/day$day.rs" ]
then
	echo 'file already exists'
else

	touch inputs/$year/day$day.txt inputs/$year/day$day-sample.txt

	cp src/template.rs src/y$year/day$day.rs

	sed -i "s/dayX/${year}\/day${day}/g" src/y$year/day$day.rs
	sed -i "s/DayX/Day${day}/g" src/y$year/day$day.rs
fi

echo "pub mod day${day};" > "src/y${year}.rs"

replace_last_line() {
    local file_path="$1"
    local new_content="$2"

    # Get the total number of lines in the file
    local total_lines=$(wc -l < "$file_path")

    # Calculate the line number of the last line
    local last_line=$((total_lines))

    # Use sed to replace the last line with the new content and create a temporary file
    sed "${last_line}s/.*/$new_content/" "$file_path" > temp_file.txt

    # Replace the original file with the temporary file
    mv temp_file.txt "$file_path"
}

replace_last_line "src/lib.rs" "pub mod y$year;"

vim src/y$year/day$day.rs

