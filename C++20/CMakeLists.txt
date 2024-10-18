cmake_minimum_required(VERSION 3.20 FATAL_ERROR)
project(BINARY_SEARCH LANGUAGES CXX)

option(RUN_UNIT_TESTS "Run unit tests" ON)
option(RUN_BENCHMARKS "Run benchmarks" ON)

# set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_BUILD_TYPE Release)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_CXX_FLAGS         "${CMAKE_CXX_FLAGS} -Wall -Wextra -Werror -Wconversion -Wno-sign-conversion -pedantic -Wredundant-move -Wnoexcept -funroll-loops -pthread -flto=auto")
set(CMAKE_CXX_FLAGS_DEBUG   "-Og -ggdb")
set(CMAKE_CXX_FLAGS_RELEASE "-O3 -DNDEBUG")

include(FetchContent)

# enable_testing()
find_package(GTest REQUIRED)
set(BENCHMARK_ENABLE_TESTING OFF)    # to suppress benchmark internal tests
FetchContent_Declare(googlebenchmark
                     GIT_REPOSITORY https://github.com/google/benchmark.git
                     GIT_TAG v1.9.0 )
FetchContent_MakeAvailable(googlebenchmark)

add_executable(binary_search
  src/main.cpp
  src/benchmark/benchmark.cpp
  src/benchmark/benchmark.C
  src/unittest/binary_search_tests.cpp
  src/unittest/utility_tests.cpp
)

target_compile_options(binary_search PUBLIC "-Wsign-conversion")

if(RUN_UNIT_TESTS)
  target_compile_definitions(binary_search PRIVATE RUN_UNIT_TESTS=1)
endif()

if(RUN_BENCHMARKS)
  target_compile_definitions(binary_search PRIVATE RUN_BENCHMARKS=1)
endif()

target_link_libraries(binary_search PRIVATE "${GTEST_LIBRARIES}")
target_link_libraries(binary_search PRIVATE "${GMOCK_LIBRARIES}")
target_link_libraries(binary_search PRIVATE benchmark::benchmark)

target_include_directories(binary_search PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_include_directories(binary_search PUBLIC "${benchmark_INCLUDE_DIRS}")
