set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")

##################################################################
# Lookup program dependencies
##################################################################

function(lookup_program BINARY_NAME FILE_LINUX FILE_WINDOWS FILE_MAC VAR_OUT)
  message(STATUS "Looking for ${BINARY_NAME}...")

  find_file(${VAR_OUT}_LINUX "${FILE_LINUX}" PATHS ENV INCLUDE)
  find_file(${VAR_OUT}_MAC "${FILE_MAC}" PATHS ENV INCLUDE)
  find_file(${VAR_OUT}_WINDOWS "${FILE_WINDOWS}" PATHS ENV INCLUDE)


  if (EXISTS ${${VAR_OUT}_LINUX})
    set(${VAR_OUT} ${${VAR_OUT}_LINUX} CACHE INTERNAL "${VAR_OUT}")
  elseif (EXISTS ${${VAR_OUT}_MAC})
    set(${VAR_OUT} ${${VAR_OUT}_MAC} CACHE INTERNAL "${VAR_OUT}")
  elseif (EXISTS ${${VAR_OUT}_WINDOWS})
    set(${VAR_OUT} ${${VAR_OUT}_WINDOWS} CACHE INTERNAL "${VAR_OUT}")
  else()
    message(FATAL_ERROR "Not found: ${BINARY_NAME}")
  endif()

  message(STATUS "Found: ${${VAR_OUT}}")
endfunction()

lookup_program("GNU C for ARM" "arm-none-eabi-gcc" "arm-none-eabi-gcc" "arm-none-eabi-gcc.exe" ARM_GCC_COMPILER)

##################################################################
# Configure toolchain
##################################################################

get_filename_component(ARM_TOOLCHAIN_BIN_PATH ${ARM_GCC_COMPILER} DIRECTORY)
get_filename_component(ARM_TOOLCHAIN_BIN_GCC ${ARM_GCC_COMPILER} NAME_WE)

message(STATUS "ARM GCC Path: ${ARM_TOOLCHAIN_BIN_PATH}")

string(REGEX REPLACE "\-gcc" "-" CROSS_COMPILE ${ARM_TOOLCHAIN_BIN_GCC})

message(STATUS "ARM Cross Compile: ${CROSS_COMPILE}")

# The Generic system name is used for embedded targets (targets without OS) in CMake
set(CMAKE_SYSTEM_NAME Generic)

set(CMAKE_ASM_COMPILER {CROSS_COMPILE}gcc )
set(CMAKE_AR ${CROSS_COMPILE}ar)
set(CMAKE_ASM_COMPILER ${CROSS_COMPILE}gcc)
set(CMAKE_C_COMPILER ${CROSS_COMPILE}gcc)
set(CMAKE_CXX_COMPILER ${CROSS_COMPILE}g++)

set(CMAKE_OBJCOPY ${ARM_TOOLCHAIN_BIN_PATH}/${CROSS_COMPILE}objcopy
  CACHE FILEPATH "The toolchain objcopy command " FORCE )

set(CMAKE_OBJDUMP ${ARM_TOOLCHAIN_BIN_PATH}/${CROSS_COMPILE}objdump
  CACHE FILEPATH "The toolchain objdump command " FORCE )

set(COMMON_FLAGS "-mcpu=cortex-m0plus")
set(COMMON_FLAGS "${COMMON_FLAGS} -mthumb")
set(COMMON_FLAGS "${COMMON_FLAGS} -g1")

set(CMAKE_C_FLAGS "-Os")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Werror -Wno-maybe-uninitialized -Wno-error=address")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fmessage-length=0 -fsigned-char")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -ffunction-sections -fdata-sections")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${COMMON_FLAGS}")

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "")
set(CMAKE_ASM_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "")

set(CMAKE_EXE_LINKER_FLAGS   "${COMMON_FLAGS} --specs=nano.specs --specs=nosys.specs")
set(CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS} -Wl,--gc-sections")
