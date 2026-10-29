cmake_minimum_required(VERSION 3.22)

if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_SOURCE_DIR)
  project(WowGameplayBridge LANGUAGES CXX)
endif()

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.6.1
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH "${CMAKE_CURRENT_SOURCE_DIR}/gameplay/Cargo.toml"
)

corrosion_add_cxxbridge(wow_gameplay_bridge
    CRATE wow_gameplay
    FILES lib.rs
)

if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_SOURCE_DIR)
    include(CTest)

    if(BUILD_TESTING)
        add_executable(wow_cooldown_bridge_test
            "${CMAKE_CURRENT_SOURCE_DIR}/../tests/gameplay/cooldown_bridge_test.cpp"
        )
        target_compile_features(wow_cooldown_bridge_test PRIVATE cxx_std_17)
        target_link_libraries(wow_cooldown_bridge_test PRIVATE wow_gameplay_bridge)

        add_test(
            NAME gameplay.cooldown_bridge
            COMMAND wow_cooldown_bridge_test
        )
    endif()
endif()
