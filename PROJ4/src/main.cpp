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
#include "Camera.h"

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

void generateSphere_orginal(float radius, unsigned int sectorCount, unsigned int stackCount,
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

void generateSphere(float radius, unsigned int sectorCount, unsigned int stackCount,
                    std::vector<float>& vertices, std::vector<unsigned int>& indices) {
    vertices.clear();
    indices.clear();
    float x, y, z, xy;
    float nx, ny, nz, lengthInv = 1.0f / radius;
    float sectorStep = 2 * M_PI / sectorCount;
    float stackStep = M_PI / stackCount;

    for(unsigned int i = 0; i <= stackCount; ++i) {
        float stackAngle = M_PI / 2 - i * stackStep;
        xy = radius * cosf(stackAngle);
        z = radius * sinf(stackAngle);

        for(unsigned int j = 0; j <= sectorCount; ++j) {
            float sectorAngle = j * sectorStep;
            x = xy * cosf(sectorAngle);
            y = xy * sinf(sectorAngle);
            vertices.push_back(x); vertices.push_back(y); vertices.push_back(z);

            nx = x * lengthInv; ny = y * lengthInv; nz = z * lengthInv;
            vertices.push_back(nx); vertices.push_back(ny); vertices.push_back(nz);
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

bool keys[1024];

void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods)
{
    if (key >= 0 && key < 1024)
    {
        if (action == GLFW_PRESS)
            keys[key] = true;
        else if (action == GLFW_RELEASE)
            keys[key] = false;
    }
}
float deltaTime = 0.0f;
float lastFrame = 0.0f;



int main() {
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    GLFWwindow* window = glfwCreateWindow(800, 600, "Textured Cube Demo", nullptr, nullptr);
    if (!window) { std::cout << "Failed to create window\n"; glfwTerminate(); return -1; }
    glfwMakeContextCurrent(window);
    glfwSetKeyCallback(window, key_callback);
    
    if(!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)){ std::cout<<"GLAD error\n"; return -1; }
    glEnable(GL_DEPTH_TEST);

    Camera camera(
        glm::vec3(0.0f, 0.0f, 5.0f)
    ); 

    Shader colorShader("./shaders/vertex.glsl", "./shaders/fragment.glsl"); 
    Shader textureShader("./shaders/vertex_texture.glsl", "./shaders/fragment_texture.glsl");
    Shader lightingShader("./shaders/light_vertex.glsl", "./shaders/light_fragment.glsl");
    Shader lightingShader2("./shaders/light_vertex.glsl", "./shaders/light_fragment2.glsl");

    Object3D coloredCube(cubeVerticesColored, sizeof(cubeVerticesColored)/sizeof(float), 
                         cubeIndicesForCube, 36, &colorShader);

    Object3D texturedCube(cubeVerticesTextured, sizeof(cubeVerticesTextured)/sizeof(float), 
                          cubeIndicesForCube, 36, &textureShader, true);

    std::vector<float> sVert; std::vector<unsigned int> sInd;
    generateSphere_orginal(1.0f, 36, 18, sVert, sInd);
    Object3D sphere(sVert.data(), sVert.size(), sInd.data(), sInd.size(), &colorShader);

    generateSphere(1.0f, 36, 18, sVert, sInd);
    Object3D sphereLighted(sVert.data(), sVert.size(), sInd.data(), sInd.size(), &lightingShader2);

    coloredCube.setPosition(glm::vec3(0.0f, 0.0f, 0.0f));
    coloredCube.setScale(glm::vec3(0.7f));
    sphere.setPosition(glm::vec3(1.5f, 1.0f, 0.0f));
    sphere.setScale(glm::vec3(0.5f));
    sphereLighted.setPosition(glm::vec3(2.0f, -1.0f, 0.0f));
    sphereLighted.setScale(glm::vec3(0.5f));
    texturedCube.setPosition(glm::vec3(-1.5f, 0.0f, 0.0f));
    texturedCube.setScale(glm::vec3(0.7f));

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

while (!glfwWindowShouldClose(window))
{
    float currentFrame = glfwGetTime();
    deltaTime = currentFrame - lastFrame;
    lastFrame = currentFrame;

    // === STEROWANIE KAMERĄ (WASD) ===
    camera.ProcessKeyboard(keys, deltaTime);

    // === ŚWIATŁO ===
    float time = currentFrame;
    float lightX = sin(time) * 3.0f;
    float lightZ = cos(time) * 3.0f;
    glm::vec3 lightPos(lightX, 1.0f, lightZ);

    glClearColor(0.02f, 0.02f, 0.05f, 1.0f); // lepsze pod kosmos 🌌
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // === MACIERZE ===
    glm::mat4 view = camera.GetViewMatrix();
    glm::mat4 projection = glm::perspective(
        glm::radians(45.0f),
        800.0f / 600.0f,
        0.1f,
        100.0f
    );

    // ================= OŚWIETLONA KULA =================
    lightingShader2.use();
    glUniform3fv(glGetUniformLocation(lightingShader2.ID, "lightPos"), 1, &lightPos[0]);
    glUniform3f(glGetUniformLocation(lightingShader2.ID, "lightColor"), 1.0f, 1.0f, 1.0f);
    glUniform3f(glGetUniformLocation(lightingShader2.ID, "objectColor"), 0.5f, 0.8f, 0.2f);

    glUniform3fv(
        glGetUniformLocation(lightingShader2.ID, "viewPos"),
        1,
        &camera.Position[0]
    );

    sphereLighted.setRotation(glm::vec3(0.0f, time, 0.0f));
    sphereLighted.draw(view, projection);

    // ================= OBIEKTY KOLOROWANE =================
    colorShader.use();
    coloredCube.setRotation(glm::vec3(time, time * 0.5f, 0.0f));
    coloredCube.draw(view, projection);

    sphere.setRotation(glm::vec3(time * 0.3f, time * 0.5f, 0.0f));
    sphere.draw(view, projection);

    // ================= OBIEKT TEKSTUROWANY =================
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




