cmake_minimum_required(VERSION 3.8)
project(test_float_overlap)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/main.rs
        target/debug/libtest_float_overlap.d
        Cargo.lock
        Cargo.toml
        test_float_overlap.iml)

add_executable(test_float_overlap ${SOURCE_FILES})