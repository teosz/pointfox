#!/bin/bash
# Script to compile and execute a c program in one step.

# Get file name without the .c extension
file_name=$(echo $1|sed 's/\(.*\)\.rs/\1/')

# Compile the program with -o option to specify the name of the binary
rustc -o $file_name.out $1

# If there were no compilation errors, run the program

./$file_name.out
rm -rf $file_name.out
