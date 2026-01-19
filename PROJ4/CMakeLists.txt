cmake_minimum_required(VERSION 3.10)
project(PROJ4)
set(CMAKE_CXX_STANDARD 17)

add_executable(PROJ4
    src/main.cpp
    src/Shader.cpp
    src/Object3D.cpp
    src/glad.c
    src/Camera.cpp
    src/Spaceship.cpp
    src/Mirror.cpp
    src/MirrorCamera.cpp
    src/ReflectionBuffer.cpp
    src/MirrorSurface.cpp
)

# Include directories
include_directories(
    include
    src
    C:/dev/GitHub/CPP/Politechnika/GrafikaKomputerowa/vcpkg/installed/x64-mingw-dynamic/include
    C:/dev/GitHub/CPP/Politechnika/GrafikaKomputerowa/vcpkg/buildtrees/glfw3/src/3.4-2448ff4533.clean/include
)

# Linkowanie bibliotek
target_link_libraries(PROJ4
    C:/dev/GitHub/CPP/Politechnika/GrafikaKomputerowa/vcpkg/installed/x64-mingw-dynamic/lib/libglfw3dll.a
    C:/dev/GitHub/CPP/Politechnika/GrafikaKomputerowa/vcpkg/installed/x64-mingw-dynamic/lib/libglm.a
    opengl32
)

