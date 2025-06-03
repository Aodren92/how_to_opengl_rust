#!/bin/bash
git clone --depth 1 https://github.com/libsdl-org/SDL sdl
cd sdl
cmake -S . -B build
cmake --build build

git clone --depth 1 https://github.com/libsdl-org/SDL_image
cd sdl
cmake -DCMAKE_IGNORE_PATH=/usr/local/lib/cmake/SDL3 -DCMAKE_PREFIX_PATH=../how_to_opengl_rust/sdl/build .
