#!/bin/bash

# check if the correct number of arguments is provided
if [ "$#" -ne 3 ]; then
    echo "Use: $0 <platform> <number> <name>"
    exit 1
fi

# store arguments in variables
PLATFORM=$1
NUMBER=$2
NAME=$3

# create file and directory structure
FILE_PATH="src/$PLATFORM/problem_${NUMBER}_${NAME// /_}.rs"
mkdir -p "src/$PLATFORM"
touch "$FILE_PATH"

# add a basic template to the new file
echo 'fn main() {
    println!("Solution for '"$PLATFORM"' problem '"$NUMBER"'");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        // Add your tests here
        assert_eq!(2 + 2, 4);
    }
}' > "$FILE_PATH"

# add entry to Cargo.toml before [dependencies]
# grep and sed to insert the new entry before the line containing [dependencies]
NEW_BIN="[[bin]]\nname = \"${PLATFORM}_${NUMBER}\"\npath = \"$FILE_PATH\"\n"

if grep -q "\[dependencies\]" Cargo.toml; then
    # if [dependencies] exists, insert before it
    sed -i "/\[dependencies\]/i $NEW_BIN" Cargo.toml
else
    # if [dependencies] doesn't exist, add to the end
    echo -e "\n$NEW_BIN" >> Cargo.toml
fi

echo "Problem added successfully: $FILE_PATH"

# how to use this script:

# ```bash
# ./new_problem.sh leetcode 053 "Maximum Subarray"
# ```