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

add_subdirectory(two-sum)
add_subdirectory(add-two-numbers)
add_subdirectory(regular-expression-matching)
add_subdirectory(3sum)
add_subdirectory(3sum-closest)
add_subdirectory(merge-two-sorted-lists)
add_subdirectory(next-permutation)
add_subdirectory(rotate-image)
add_subdirectory(jump-game)
add_subdirectory(edit-distance)
add_subdirectory(sort-list)
add_subdirectory(power-of-two)
add_subdirectory(counting-bits)
add_subdirectory(top-k-frequent-elements)
add_subdirectory(top-k-frequent-words)
