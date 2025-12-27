#ifndef OBJECT3D_H
#define OBJECT3D_H

#include <glad/glad.h>
#include <glm/glm.hpp>
#include "Shader.h"

class Object3D {
public:
    unsigned int VAO, VBO, EBO;
    Shader* shader;

    glm::vec3 position;
    glm::vec3 rotation;
    glm::vec3 scale;

    unsigned int indexCount;

    Object3D(
        float* vertices, unsigned int vertexCount,
        unsigned int* indices, unsigned int indexCount,
        Shader* shader
    );

    ~Object3D();

    void setPosition(const glm::vec3& pos);
    void setRotation(const glm::vec3& rot);
    void setScale(const glm::vec3& scl);

    void draw(const glm::mat4& view, const glm::mat4& projection) const;
};

#endif
