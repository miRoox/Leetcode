cmake_minimum_required(VERSION 3.10)

project(Leetcode)

include(ExternalProject)
find_package(Git REQUIRED)

ExternalProject_Add(
    doctest
    PREFIX ${CMAKE_BINARY_DIR}/doctest
    GIT_REPOSITORY https://github.com/onqtam/doctest.git
    TIMEOUT 10
    UPDATE_COMMAND ${GIT_EXECUTABLE} pull
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND ""
    LOG_DOWNLOAD ON
)

# Expose required variable (DOCTEST_INCLUDE_DIR) to parent scope
ExternalProject_Get_Property(doctest source_dir)
set(DOCTEST_INCLUDE_DIR ${source_dir}/doctest CACHE INTERNAL "Path to include folder for doctest")

enable_testing()

include_directories(${DOCTEST_INCLUDE_DIR})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_subdirectory(two-sum) # 1
add_subdirectory(add-two-numbers) # 2
add_subdirectory(regular-expression-matching) # 10
add_subdirectory(3sum) # 15
add_subdirectory(3sum-closest) # 16
add_subdirectory(merge-two-sorted-lists) # 21
add_subdirectory(next-permutation) # 31
add_subdirectory(rotate-image) # 48
add_subdirectory(jump-game) # 55
add_subdirectory(edit-distance) # 72
add_subdirectory(sort-list) # 148
add_subdirectory(power-of-two) # 231
add_subdirectory(counting-bits) # 338
add_subdirectory(top-k-frequent-elements) # 347
add_subdirectory(top-k-frequent-words) # 692
