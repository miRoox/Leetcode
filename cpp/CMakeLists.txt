cmake_minimum_required(VERSION 3.10)

project(Leetcode)

enable_testing()

include_directories(doctest)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_subdirectory(two-sum)
