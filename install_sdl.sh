#!/bin/bash
git clone --depth 1 https://github.com/libsdl-org/SDL sdl
cd sdl
cmake -S . -B build
cmake --build build
