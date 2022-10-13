cmake_minimum_required(VERSION 3.10)

# Setup toolchain for arm cross compilation
set(CMAKE_TOOLCHAIN_FILE ${CMAKE_CURRENT_SOURCE_DIR}/cmake/toolchain.cmake)


##################################################################
# Configuration section
##################################################################

# Configure SDK root path
set(
    SDK_PATH
    "/home/developer/project/sdk"
)

# Setup project name, version and languages
project(
    dialog-example
    VERSION 1.0
    LANGUAGES ASM
)

##################################################################
# Build setup section
##################################################################

set(
  LINKER_SCRIPT_PATH
  "${CMAKE_CURRENT_BINARY_DIR}/ldscript_DA14531.lds"
)

add_custom_target(
  linker_script
  COMMAND ${CMAKE_C_COMPILER} -D__DA14531__ -I "${SDK_PATH}/sdk/common_project_files" -I "${SDK_PATH}/projects/target_apps/template/empty_peripheral_template/src/config" -E -P -c "${SDK_PATH}/sdk/common_project_files/ldscripts/ldscript_DA14531.lds.S" -o "${LINKER_SCRIPT_PATH}"
)
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${LINKER_SCRIPT_PATH} -lc -lnosys -lm")
link_directories(${SDK_PATH}/sdk/common_project_files/misc)

file(
  GLOB_RECURSE 
  FIRMWARE_SRC 
  ${PROJECT_SOURCE_DIR}/src/*.c
)

set(
    SDK_SRC
    "${SDK_PATH}/sdk/platform/arch/boot/GCC/ivtable_DA14531.S"
    "${SDK_PATH}/sdk/platform/arch/boot/GCC/startup_DA14531.S"
)

add_compile_definitions(
    __DA14531__
)


add_executable(
  ${PROJECT_NAME}
  ${SDK_SRC}
  ${FIRMWARE_SRC}
)

if("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
  set(
    RUST_RELEASE_FLAG
    --release
  )
  set(
    RUST_LIB_PATH
    "${CMAKE_CURRENT_SOURCE_DIR}/target/thumbv6m-none-eabi/release/librust.a"
  )
else()
set(
  RUST_LIB_PATH
  "${CMAKE_CURRENT_SOURCE_DIR}/target/thumbv6m-none-eabi/debug/librust.a"
)
endif()

add_custom_target(
  rust_lib
  COMMAND SDK_PATH=${SDK_PATH} cargo build ${RUST_RELEASE_FLAG}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

# Link pre-compiled (proprietary) libraries from SDK
target_link_libraries(
  ${PROJECT_NAME}
  "${RUST_LIB_PATH}"
  "${SDK_PATH}/sdk/platform/system_library/output/IAR/da14531.a"
)

add_dependencies(
  ${PROJECT_NAME}
  linker_script
  rust_lib
)

set_property(
    TARGET ${PROJECT_NAME} 
    APPEND
    PROPERTY ADDITIONAL_CLEAN_FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/target
)

##################################################################
# Additional modules
##################################################################

# Generates .hex and .bin binary files
include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/post_build.cmake)

