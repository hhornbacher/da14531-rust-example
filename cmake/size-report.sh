#!/usr/bin/env bash

PATH="/Applications/ARM/bin:$PATH"
PROJECT_NAME="$1"
BUILD_TYPE="$2"

TS=$(date +'%s')

REPORT_FILE=../reports/size-report-${TS}.txt

mkdir -p ../reports

echo "Binary size:" >${REPORT_FILE}
arm-none-eabi-size ${PROJECT_NAME} | tail -n 1 >>${REPORT_FILE}
echo -e "\nRust lib size:" >>${REPORT_FILE}
arm-none-eabi-size ../target/thumbv6m-none-eabi/${BUILD_TYPE}/librust.a | head -n 2 | tail -n 1 >>${REPORT_FILE}
