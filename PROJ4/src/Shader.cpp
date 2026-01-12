#include "Shader.h"

#include <fstream>
#include <sstream>
#include <iostream>
#include <filesystem>

Shader::Shader(const char* vertexPath, const char* fragmentPath)
{
    std::string vertexCode;
    std::string fragmentCode;

    std::ifstream vShaderFile;
    std::ifstream fShaderFile;

    vShaderFile.open(vertexPath);
    fShaderFile.open(fragmentPath);
    std::cout<<"Vertex path: "<<vertexPath<<'\n'<<" Fragment path: "<<'\n';
    std::cout<<std::filesystem::current_path()<<'\n';

    if (!vShaderFile.is_open() || !fShaderFile.is_open())
    {
        std::cerr << "Nie mozna otworzyc plikow shaderow\n";
        return;
    } 
    else {
        std::cout<<"pliki shaderow otwarte\n";
    }

    std::stringstream vStream, fStream;
    vStream << vShaderFile.rdbuf();
    fStream << fShaderFile.rdbuf();

    vShaderFile.close();
    fShaderFile.close();

    vertexCode = vStream.str();
    fragmentCode = fStream.str();

    const char* vCode = vertexCode.c_str();
    const char* fCode = fragmentCode.c_str();

    unsigned int vertex, fragment;
    int success;
    char infoLog[512];

    vertex = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertex, 1, &vCode, nullptr);
    glCompileShader(vertex);

    glGetShaderiv(vertex, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(vertex, 512, nullptr, infoLog);
        std::cerr << "BLAD Vertex Shader:\n" << infoLog << std::endl;
    }

    fragment = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragment, 1, &fCode, nullptr);
    glCompileShader(fragment);

    glGetShaderiv(fragment, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(fragment, 512, nullptr, infoLog);
        std::cerr << "BLAD Fragment Shader:\n" << infoLog << std::endl;
    }

    ID = glCreateProgram();
    glAttachShader(ID, vertex);
    glAttachShader(ID, fragment);
    glLinkProgram(ID);

    glGetProgramiv(ID, GL_LINK_STATUS, &success);
    if (!success)
    {
        glGetProgramInfoLog(ID, 512, nullptr, infoLog);
        std::cerr << "BLAD Program Shader:\n" << infoLog << std::endl;
    }

    glDeleteShader(vertex);
    glDeleteShader(fragment);
}

void Shader::use() const
{
    glUseProgram(ID);
}

void Shader::setMat4(const std::string& name, const glm::mat4& mat) const
{
    glUniformMatrix4fv(
        glGetUniformLocation(ID, name.c_str()),
        1,
        GL_FALSE,
        &mat[0][0]
    );
}
