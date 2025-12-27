#ifndef SHADER_H
#define SHADER_H

#include <glad/glad.h>
#include <glm/glm.hpp>

#include <string>

class Shader
{
public:
    unsigned int ID;

    // konstruktor – ładuje i kompiluje shadery
    Shader(const char* vertexPath, const char* fragmentPath);

    // użycie programu
    void use() const;

    // uniformy (macierze)
    void setMat4(const std::string& name, const glm::mat4& mat) const;
};

#endif
