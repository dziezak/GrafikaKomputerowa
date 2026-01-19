#pragma once
#include <glad/glad.h>

class ReflectionBuffer {
public:
    unsigned int FBO;
    unsigned int texture;
    unsigned int RBO;

    ReflectionBuffer(int width, int height);
    void bind();
    void unbind();
    void resize(int width, int height);
};
