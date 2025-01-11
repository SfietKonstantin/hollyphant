project(harbour-hollyphant)
cmake_minimum_required(VERSION 3.14)

add_subdirectory(cmake/corrosion)
add_subdirectory(3rdparty)
add_subdirectory(lib)
add_subdirectory(bin)

add_custom_target(distfiles
    SOURCES
    rpm/harbour-hollyphant.changes.in
    rpm/harbour-hollyphant.changes.run.in
    rpm/harbour-hollyphant.spec
    rpm/harbour-hollyphant.yaml
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

