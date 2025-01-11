option(USE_SHIM BOOLEAN CACHE)
project(hollyphant-lib CXX)
cmake_minimum_required(VERSION 3.14)

find_package (Qt5 COMPONENTS Core Gui Qml)

set(CMAKE_AUTOMOC ON)

if (${USE_SHIM})
    set(${PROJECT_NAME}_MAIN_SRCS
        shim/main.cpp
    )
else()
    set(${PROJECT_NAME}_MAIN_SRCS
        shim/main.cpp
    )
endif()

add_library(${PROJECT_NAME} STATIC
    hollyphant/main.h
    hollyphant/hollyphant.h

    hollyphant.cpp
    ${${PROJECT_NAME}_MAIN_SRCS}
)

target_include_directories(${PROJECT_NAME} PUBLIC .)
target_link_libraries(${PROJECT_NAME} Qt5::Core Qt5::Gui Qt5::Qml qmlext)
