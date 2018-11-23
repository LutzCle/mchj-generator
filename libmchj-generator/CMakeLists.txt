cmake_minimum_required(VERSION 3.5)

project(MultiCoreHashJoinGenerator)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O3")

include_directories(${CMAKE_CURRENT_BINARY_DIR})

if(NOT HAVE_PTHREAD_ATTR_SETAFFINITY_NP)
    set(HAVE_PTHREAD_ATTR_SETAFFINITY_NP 1)
endif()
if(NOT CACHE_LINE_SIZE)
    set(CACHE_LINE_SIZE 64)
endif()
if(NOT RELATION_PADDING)
    set(RELATION_PADDING 0)
endif()

configure_file(config.h.cmake config.h @ONLY)

add_library(objlib OBJECT src/cpu_mapping.c src/generator.c src/genzipf.c)
set_property(TARGET objlib PROPERTY POSITION_INDEPENDENT_CODE 1)

add_library(mchj-generator SHARED $<TARGET_OBJECTS:objlib>)
add_library(mchj-generator-static STATIC $<TARGET_OBJECTS:objlib>)

set_target_properties(mchj-generator-static PROPERTIES OUTPUT_NAME mchj-generator)

install(TARGETS mchj-generator mchj-generator-static LIBRARY DESTINATION lib ARCHIVE DESTINATION lib)
