file(GLOB_RECURSE PRIVATE_SOURCES CONFIGURE_DEPENDS "*.cpp")

add_library(Magic STATIC
    ${PRIVATE_SOURCES}
)

target_include_directories(Magic
    PUBLIC
        ${CMAKE_SOURCE_DIR}
)

if (WOWDECOMP_ENABLE_RUST)
    target_link_libraries(Magic PRIVATE wow_gameplay_bridge)
    target_compile_definitions(Magic PRIVATE WOWDECOMP_ENABLE_RUST=1)
endif ()
