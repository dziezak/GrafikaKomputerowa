#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <iostream>
#include <vector>
#include <cmath> // Dla M_PI
#include <filesystem>
#include "Shader.h"
#include "Object3D.h"
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

#ifndef M_PI
    #define M_PI 3.14159265358979323846
#endif

// Dane wierzchołków
float cubeVerticesColored[] = {
    -0.5f, -0.5f, -0.5f, 1,0,0,  0.5f, -0.5f, -0.5f, 0,1,0,
     0.5f,  0.5f, -0.5f, 0,0,1, -0.5f,  0.5f, -0.5f, 1,1,0,
    -0.5f, -0.5f,  0.5f, 1,0,1,  0.5f, -0.5f,  0.5f, 0,1,1,
     0.5f,  0.5f,  0.5f, 1,1,1, -0.5f,  0.5f,  0.5f, 0,0,0
};

float cubeVerticesTextured[] = {
    -0.5f, -0.5f, -0.5f, 0.0f, 0.0f,  0.5f, -0.5f, -0.5f, 1.0f, 0.0f,
     0.5f,  0.5f, -0.5f, 1.0f, 1.0f, -0.5f,  0.5f, -0.5f, 0.0f, 1.0f,
    -0.5f, -0.5f,  0.5f, 0.0f, 0.0f,  0.5f, -0.5f,  0.5f, 1.0f, 0.0f,
     0.5f,  0.5f,  0.5f, 1.0f, 1.0f, -0.5f,  0.5f,  0.5f, 0.0f, 1.0f
};

unsigned int cubeIndicesForCube[] = {
    0,1,2, 2,3,0, 4,5,6, 6,7,4, 0,4,7, 7,3,0,
    1,5,6, 6,2,1, 3,2,6, 6,7,3, 0,1,5, 5,4,0
};

void generateSphere(float radius, unsigned int sectorCount, unsigned int stackCount,
                    std::vector<float>& vertices, std::vector<unsigned int>& indices) {
    float x, y, z, xy;
    float sectorStep = 2 * M_PI / sectorCount;
    float stackStep = M_PI / stackCount;
    for(unsigned int i = 0; i <= stackCount; ++i) {
        float stackAngle = M_PI / 2 - i * stackStep;
        xy = radius * cosf(stackAngle);
        z = radius * sinf(stackAngle);
        for(unsigned int j = 0; j <= sectorCount; ++j) {
            float sectorAngle = j * sectorStep;
            x = xy * cosf(sectorAngle); y = xy * sinf(sectorAngle);
            vertices.push_back(x); vertices.push_back(y); vertices.push_back(z);
        }
    }
    for(unsigned int i = 0; i < stackCount; ++i) {
        unsigned int k1 = i * (sectorCount + 1);
        unsigned int k2 = k1 + sectorCount + 1;
        for(unsigned int j = 0; j < sectorCount; ++j, ++k1, ++k2) {
            if(i != 0) { indices.push_back(k1); indices.push_back(k2); indices.push_back(k1+1); }
            if(i != stackCount-1) { indices.push_back(k1+1); indices.push_back(k2); indices.push_back(k2+1); }
        }
    }
}

int main() {
    // 1. Inicjalizacja GLFW i okna
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    GLFWwindow* window = glfwCreateWindow(800, 600, "Textured Cube Demo", nullptr, nullptr);
    if (!window) { std::cout << "Failed to create window\n"; glfwTerminate(); return -1; }
    glfwMakeContextCurrent(window);
    
    if(!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)){ std::cout<<"GLAD error\n"; return -1; }
    glEnable(GL_DEPTH_TEST);

    // 2. Kompilacja Shaderów
    Shader colorShader("./shaders/vertex.glsl", "./shaders/fragment.glsl"); 
    Shader textureShader("./shaders/vertex_texture.glsl", "./shaders/fragment_texture.glsl");

    // 3. Tworzenie obiektów 3D
    Object3D coloredCube(cubeVerticesColored, sizeof(cubeVerticesColored)/sizeof(float), 
                         cubeIndicesForCube, 36, &colorShader);

    Object3D texturedCube(cubeVerticesTextured, sizeof(cubeVerticesTextured)/sizeof(float), 
                          cubeIndicesForCube, 36, &textureShader, true);

    std::vector<float> sVert; std::vector<unsigned int> sInd;
    generateSphere(0.5f, 36, 18, sVert, sInd);
    Object3D sphere(sVert.data(), sVert.size(), sInd.data(), sInd.size(), &colorShader);

    // 4. Konfiguracja początkowa pozycji
    coloredCube.setPosition(glm::vec3(0.0f, 0.0f, 0.0f));
    coloredCube.setScale(glm::vec3(0.7f));
    sphere.setPosition(glm::vec3(1.5f, 0.0f, 0.0f));
    sphere.setScale(glm::vec3(0.5f));
    texturedCube.setPosition(glm::vec3(-1.5f, 0.0f, 0.0f));
    texturedCube.setScale(glm::vec3(0.7f));

    // 5. Ładowanie Tekstury
    unsigned int textureWall;
    glGenTextures(1, &textureWall);
    glBindTexture(GL_TEXTURE_2D, textureWall);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);   
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

    int width, height, nrChannels;
    stbi_set_flip_vertically_on_load(true);
    unsigned char *data = stbi_load("./wall.jpg", &width, &height, &nrChannels, 0);
    if (data) {
        GLenum format = (nrChannels == 4) ? GL_RGBA : GL_RGB;
        glTexImage2D(GL_TEXTURE_2D, 0, format, width, height, 0, format, GL_UNSIGNED_BYTE, data);
        glGenerateMipmap(GL_TEXTURE_2D);
    } else {
    std::cout << "Failed to load texture. Reason: " << stbi_failure_reason() << std::endl;
    }
    stbi_image_free(data);

    // 6. Pętla główna
    while(!glfwWindowShouldClose(window)) {
        float time = (float)glfwGetTime();
        glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        // Macierze View i Projection
        glm::mat4 view = glm::translate(glm::mat4(1.0f), glm::vec3(0.0f, 0.0f, -5.0f));
        glm::mat4 projection = glm::perspective(glm::radians(45.0f), 800.0f/600.0f, 0.1f, 100.0f);

        // --- Rysowanie kolorowych ---
        colorShader.use();
        coloredCube.setRotation(glm::vec3(time, time * 0.5f, 0.0f));
        coloredCube.draw(view, projection);
        sphere.setRotation(glm::vec3(time, time * 0.5f, 0.0f));
        sphere.draw(view, projection);

        // --- Rysowanie teksturowanego sześcianu ---
        textureShader.use();
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, textureWall);
        glUniform1i(glGetUniformLocation(textureShader.ID, "texture1"), 0);

        texturedCube.setRotation(glm::vec3(0.0f, time, time * 0.5f));
        texturedCube.draw(view, projection);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}




