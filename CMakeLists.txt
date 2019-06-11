cmake_minimum_required(VERSION 3.14)
project(leetcode)

set(CMAKE_CXX_STANDARD 17)

file(GLOB_RECURSE files *.cc)
foreach (file ${files})
    get_filename_component(testname ${file} NAME)
    string(REPLACE ".cc" "" testname ${testname})
    add_executable(${testname} ${file})
endforeach ()