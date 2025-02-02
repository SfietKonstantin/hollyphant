project(harbour-hollyphant CXX)
cmake_minimum_required(VERSION 3.14)

find_package (Qt5 COMPONENTS Core Network Qml Gui Quick REQUIRED)

include(FindPkgConfig)
pkg_search_module(SAILFISH sailfishapp)

set(CMAKE_AUTOMOC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

add_executable(${PROJECT_NAME} main.cpp)

target_link_libraries(${PROJECT_NAME} Qt5::Quick hollyphant-lib)

if ("${SAILFISH_FOUND}")
    message("Building for Sailfish OS")
    add_compile_definitions(${PROJECT_NAME} WITH_SAILFISH)
    target_link_libraries(${PROJECT_NAME} ${SAILFISH_LDFLAGS})
    target_include_directories(${PROJECT_NAME} PRIVATE ${SAILFISH_INCLUDE_DIRS})
else()
    message("Building without Sailfish OS, for fast prototyping on desktop")
endif()

install(TARGETS ${PROJECT_NAME}
    RUNTIME DESTINATION bin
)
install(DIRECTORY qml
    DESTINATION share/harbour-hollyphant
)
install(DIRECTORY translations
    DESTINATION share/harbour-hollyphant
    FILES_MATCHING PATTERN "*.qm"
)
install(FILES harbour-hollyphant.desktop
    DESTINATION share/applications
)
install(FILES icons/86x86/harbour-hollyphant.png
    DESTINATION share/icons/hicolor/86x86/apps
)
install(FILES icons/108x108/harbour-hollyphant.png
    DESTINATION share/icons/hicolor/108x108/apps
)
install(FILES icons/128x128/harbour-hollyphant.png
    DESTINATION share/icons/hicolor/128x128/apps
)
install(FILES icons/172x172/harbour-hollyphant.png
    DESTINATION share/icons/hicolor/172x172/apps
)

# Get the other files reachable from the project tree in Qt Creator
FILE(GLOB TsFiles "translations/*.ts")
add_custom_target(otherfiles SOURCES
    harbour-hollyphant.desktop

    qml/harbour-hollyphant.qml
    qml/MainPage.qml
    qml/Welcome.qml
    qml/accounts/AccountsPage.qml
    qml/accounts/NewAccountBskyDialog.qml
    qml/accounts/NewAccountDialog.qml
    qml/accounts/NewAccountMasAuthDialog.qml
    qml/accounts/NewAccountMasAuthFinalizationPage.qml
    qml/components/ErrorItem.qml
    qml/components/StatusDisplay.qml
    qml/cover/CoverPage.qml

    ${TsFiles}
)

# Tell Qt Creator where the application executable(s) would be located on the
# device.
#
# It is not necessary to list other deployables than executables (runtime
# targets) here. The deployment process of Sailfish OS projects is opaque to
# Qt Creator and the information contained in QtCreatorDeployment.txt is only
# used to locate the executable associated with the active run configuration
# on the device in order to run it.
#
# Search the Qt Creator Manual to learn about the QtCreatorDeployment.txt file
# format.
file(WRITE "${CMAKE_BINARY_DIR}/QtCreatorDeployment.txt"
    "${CMAKE_INSTALL_PREFIX}
${CMAKE_BINARY_DIR}/harbour-hollyphant:bin
")


