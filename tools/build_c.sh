#!/bin/bash

cd src/ffi/
if [ ! -d ./build/ ]; then mkdir ./build/; fi;
gcc -std=gnu17 -O3 -I ./include -c ./src/*.c -o ./build/library.o;
ar rcs ./build/liblibrary.a ./build/library.o;
