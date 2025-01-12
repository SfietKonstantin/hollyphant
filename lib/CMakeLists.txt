cmake_minimum_required(VERSION 3.14)
project(hollyphant-lib CXX)
option(USE_SHIM BOOLEAN CACHE)

include(FindPkgConfig)

find_package (Qt5 COMPONENTS Core Gui Qml REQUIRED)
find_package(OpenSSL REQUIRED)
pkg_search_module(SAILFISH sailfishapp)

set(CMAKE_AUTOMOC ON)

if (${USE_SHIM})
    set(${PROJECT_NAME}_MAIN_SRCS
        src/shim/main.cpp
    )
else()
    # Use Rust
    corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES hollyphant-ffi FLAGS -j1)

    if ("${SAILFISH_FOUND}")
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
        corrosion_set_env_vars(hollyphant_ffi
            "CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}"
        )
    endif()
    corrosion_link_libraries(hollyphant_ffi ${PROJECT_NAME})

    include_directories(src "${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/include")

    set(${PROJECT_NAME}_MAIN_SRCS
        src/rust/rusteventpublisher.h
        src/rust/rusteventpublisher.cpp
        src/rust/rusteventpublisherimpl.h
        src/rust/main.cpp
    )
endif()

add_library(${PROJECT_NAME} STATIC
    include/hollyphant/main.h
    include/hollyphant/hollyphant.h

    src/hollyphant.cpp
    ${${PROJECT_NAME}_MAIN_SRCS}
)

target_include_directories(${PROJECT_NAME} PUBLIC include)
target_link_libraries(${PROJECT_NAME} Qt5::Core Qt5::Gui Qt5::Qml qmlext)

if (NOT ${USE_SHIM})
    target_link_libraries(${PROJECT_NAME} hollyphant_ffi)
endif()
