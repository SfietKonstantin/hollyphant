cmake_minimum_required(VERSION 3.14)
project(hollyphant-lib CXX)
option(USE_SHIM BOOLEAN CACHE)

include(FindPkgConfig)

find_package (Qt5 COMPONENTS Core Gui Qml REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(SQLite3 REQUIRED)
pkg_search_module(SAILFISH sailfishapp)

set(CMAKE_AUTOMOC ON)

if (${USE_SHIM})
    set(${PROJECT_NAME}_MAIN_SRCS
        src/shim/main.cpp
    )
else()
    # Use Rust
    if ("${SAILFISH_FOUND}")
        corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES hollyphant-ffi FLAGS -j1)
        # We need to help Rust with OpenSSL and cross-compilation
        corrosion_set_env_vars(hollyphant_ffi
            "CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}"
            "OPENSSL_DIR=/usr"
            "OPENSSL_LIB_DIR=/usr/lib"
            "OPENSSL_INCLUDE_DIR=/usr/include"
            "ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_DIR=/usr"
            "ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_LIB_DIR=/usr/lib"
            "ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_INCLUDE_DIR=/usr/include"
            "AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_DIR=/usr"
            "AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib64"
            "AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR=/usr/include"
        )
    else()
        corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES hollyphant-ffi)
        corrosion_set_env_vars(hollyphant_ffi
            "CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}"
        )
    endif()
    corrosion_link_libraries(hollyphant_ffi ${PROJECT_NAME})

    include_directories(src "${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/include")

    set(${PROJECT_NAME}_MAIN_SRCS
        src/rust/rusterrorformatter.h
        src/rust/rusterrorformatter.cpp
        src/rust/rusteventpublisher.h
        src/rust/rusteventpublisher.cpp
        src/rust/rusteventpublisherimpl.h
        src/rust/main.cpp
    )
endif()

add_library(${PROJECT_NAME} STATIC
    include/hollyphant/main.h
    include/hollyphant/statusitem.h

    src/errorformatter.h
    src/errorformatter.cpp
    src/loggingeventprocessor.h
    src/loggingeventprocessor.cpp
    src/statusitem.cpp

    ${${PROJECT_NAME}_MAIN_SRCS}
)

target_include_directories(${PROJECT_NAME} PUBLIC include)
target_link_libraries(${PROJECT_NAME} Qt5::Core Qt5::Gui Qt5::Qml qmlext ${OPENSSL_LIBRARIES})

if (NOT ${USE_SHIM})
    target_link_libraries(${PROJECT_NAME} hollyphant_ffi ${SQLite3_LIBRARIES})
endif()
