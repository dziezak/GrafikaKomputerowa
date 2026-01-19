#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "Shader.h"
#include "Mirror.h"

void generateSphere(
    float radius,
    unsigned int sectorCount,
    unsigned int stackCount,
    std::vector<float>& vertices,
    std::vector<unsigned int>& indices)
{
    vertices.clear();
    indices.clear();

    float x, y, z, xy;
    float nx, ny, nz;
    float lengthInv = 1.0f / radius;

    float sectorStep = 2 * M_PI / sectorCount;
    float stackStep  = M_PI / stackCount;

    for (unsigned int i = 0; i <= stackCount; ++i)
    {
        float stackAngle = M_PI / 2 - i * stackStep;
        xy = radius * cosf(stackAngle);
        z  = radius * sinf(stackAngle);

        for (unsigned int j = 0; j <= sectorCount; ++j)
        {
            float sectorAngle = j * sectorStep;

            x = xy * cosf(sectorAngle);
            y = xy * sinf(sectorAngle);

            // pozycja
            vertices.push_back(x);
            vertices.push_back(y);
            vertices.push_back(z);

            // normal
            nx = x * lengthInv;
            ny = y * lengthInv;
            nz = z * lengthInv;

            vertices.push_back(nx);
            vertices.push_back(ny);
            vertices.push_back(nz);
        }
    }

    for (unsigned int i = 0; i < stackCount; ++i)
    {
        unsigned int k1 = i * (sectorCount + 1);
        unsigned int k2 = k1 + sectorCount + 1;

        for (unsigned int j = 0; j < sectorCount; ++j, ++k1, ++k2)
        {
            if (i != 0)
            {
                indices.push_back(k1);
                indices.push_back(k2);
                indices.push_back(k1 + 1);
            }

            if (i != (stackCount - 1))
            {
                indices.push_back(k1 + 1);
                indices.push_back(k2);
                indices.push_back(k2 + 1);
            }
        }
    }
}

int main()
{
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    GLFWwindow* window = glfwCreateWindow(800, 600, "Mirror Test", NULL, NULL);
    glfwMakeContextCurrent(window);

    gladLoadGLLoader((GLADloadproc)glfwGetProcAddress);

    glEnable(GL_DEPTH_TEST);

    Shader lightingShader("./shaders/light_vertex.glsl","./shaders/light_fragment_camera.glsl");
    Shader mirrorShader("./shaders/mirror_vertex.glsl", "./shaders/mirror_fragment.glsl");

    Mirror mirrorObj(2.0f, 2.0f, &mirrorShader);
    std::vector<float> sphereVertices;
    std::vector<unsigned int> sphereIndices;
    generateSphere(1.0f, 48, 24, sphereVertices, sphereIndices);


    mirrorObj.setPosition(glm::vec3(0, 0, -3));
    mirrorObj.setRotation(glm::vec3(0, 0, 0));
    mirrorObj.setScale(glm::vec3(1, 1, 1));
    Object3D sun(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &lightingShader
    );
    sun.setScale(glm::vec3(1.3f));
    sun.setPosition(glm::vec3(0.0f));

    while (!glfwWindowShouldClose(window))
    {
        glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        glm::mat4 view = glm::lookAt(
            glm::vec3(0, 0, 0),   
            glm::vec3(0, 0, -1), 
            glm::vec3(0, 1, 0)
        );

        glm::mat4 projection = glm::perspective(
            glm::radians(45.0f),
            800.0f / 600.0f,
            0.1f,
            100.0f
        );

        glDisable(GL_CULL_FACE); 

        mirrorObj.draw(view, projection, 0);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}
