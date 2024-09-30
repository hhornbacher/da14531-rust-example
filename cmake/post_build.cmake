# Generate raw binary file after compilation
add_custom_command(
  TARGET ${PROJECT_NAME} POST_BUILD
  COMMAND ${CMAKE_OBJCOPY} -O binary ${PROJECT_NAME} ${PROJECT_NAME}.bin
)
# Generate Intel hex binary file after compilation
add_custom_command(
  TARGET ${PROJECT_NAME} POST_BUILD
  COMMAND ${CMAKE_OBJCOPY} -O ihex ${PROJECT_NAME} ${PROJECT_NAME}.hex
)
# Generate size reports
if("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
  set(BUILD_TYPE release)
else()
  set(BUILD_TYPE debug)
endif()
