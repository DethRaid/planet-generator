cmake_minimum_required(VERSION 3.8)
project(PlanetGenWrapper)

set(CMAKE_CXX_STANDARD_REQUIRED 17)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_LIST_DIR})


add_library(PlanetGenWrapper STATIC src/dynamic.cpp src/dynamic.h)
target_include_directories(PlanetGenWrapper PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src/)
target_link_libraries(PlanetGenWrapper ../target/debug/planet_gen)