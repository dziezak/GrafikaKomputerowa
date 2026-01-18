#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include <iostream>
#include <vector>
#include <cmath>

#include "Shader.h"
#include "Object3D.h"
#include "Camera.h"

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

// ===================== GLOBALNE =====================
bool keys[1024] = { false };
float deltaTime = 0.0f;
float lastFrame = 0.0f;

// ===================== CALLBACKI =====================
void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods)
{
    if (key >= 0 && key < 1024)
    {
        if (action == GLFW_PRESS)
            keys[key] = true;
        else if (action == GLFW_RELEASE)
            keys[key] = false;
    }

    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
        glfwSetWindowShouldClose(window, true);
}

void framebuffer_size_callback(GLFWwindow* window, int width, int height)
{
    glViewport(0, 0, width, height);
}

// ===================== GENEROWANIE SFERY =====================
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

// ===================== GEOMETRIA STATKU =====================
float shipVertices[] = {
    // pos                // normal
    -0.5f, -0.5f, -1.0f,  0.0f, 0.0f, -1.0f,
     0.5f, -0.5f, -1.0f,  0.0f, 0.0f, -1.0f,
     0.5f,  0.5f, -1.0f,  0.0f, 0.0f, -1.0f,
    -0.5f,  0.5f, -1.0f,  0.0f, 0.0f, -1.0f,

    -0.5f, -0.5f,  1.0f,  0.0f, 0.0f, 1.0f,
     0.5f, -0.5f,  1.0f,  0.0f, 0.0f, 1.0f,
     0.5f,  0.5f,  1.0f,  0.0f, 0.0f, 1.0f,
    -0.5f,  0.5f,  1.0f,  0.0f, 0.0f, 1.0f,
};

unsigned int shipIndices[] = {
    0,1,2,  2,3,0,  // tył
    4,5,6,  6,7,4,  // przód
    0,4,7,  7,3,0,  // lewa
    1,5,6,  6,2,1,  // prawa
    3,2,6,  6,7,3,  // góra
    0,1,5,  5,4,0   // dół
};

// ===================== POMOCNICZE =====================
glm::vec3 computeShipForward(const glm::vec3& rotation)
{
    // używamy tylko yaw (rotation.y)
    float yaw = rotation.y;
    glm::vec3 forward;
    forward.x = -sinf(yaw);
    forward.y = 0.0f;
    forward.z = -cosf(yaw);
    return glm::normalize(forward);
}

// ===================== MAIN =====================
int main()
{
    // -------- GLFW --------
    if (!glfwInit())
    {
        std::cout << "Failed to init GLFW\n";
        return -1;
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    GLFWwindow* window = glfwCreateWindow(800, 600, "Solar System", nullptr, nullptr);
    if (!window)
    {
        std::cout << "Failed to create window\n";
        glfwTerminate();
        return -1;
    }

    glfwMakeContextCurrent(window);
    glfwSetKeyCallback(window, key_callback);
    glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);

    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
    {
        std::cout << "Failed to init GLAD\n";
        glfwTerminate();
        return -1;
    }

    glEnable(GL_DEPTH_TEST);

    // -------- SHADER --------
    Shader lightingShader(
        "./shaders/light_vertex.glsl",
        "./shaders/light_fragment2.glsl"
    );
    Shader spotlightShader(
        "./shaders/light_vertex.glsl",
        "./shaders/spotlight_fragment.glsl"
    );

    // -------- KAMERA --------
    int activeCamera = 1;
    bool cKeyPressedLastFrame = false;

    Camera camera1(glm::vec3(0.0f, 2.0f, 10.0f));  // swobodna
    Camera camera2(glm::vec3(0.0f, 4.0f, 15.0f));  // za planetą 1
    Camera camera3(glm::vec3(15.0f, 10.0f, 15.0f)); // statyczna
    Camera cameraShip(glm::vec3(0.0f, 3.0f, -5.0f)); // za statkiem

    glm::vec3 camera2Offset(0.0f, 2.0f, 5.0f);

    // -------- GEOMETRIA SPHERY --------
    std::vector<float> sphereVertices;
    std::vector<unsigned int> sphereIndices;
    generateSphere(1.0f, 48, 24, sphereVertices, sphereIndices);

    // -------- OBIEKTY --------
    Object3D sun(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &lightingShader
    );
    Object3D planet1(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &lightingShader
    );
    Object3D planet2(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &lightingShader
    );
    Object3D planet3(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &lightingShader
    );
    Object3D spaceship(
        shipVertices, sizeof(shipVertices) / sizeof(float),
        shipIndices, sizeof(shipIndices) / sizeof(unsigned int),
        &spotlightShader
    );

    sun.setScale(glm::vec3(1.5f));
    sun.setPosition(glm::vec3(0.0f));

    planet1.setScale(glm::vec3(0.4f));
    planet2.setScale(glm::vec3(0.6f));
    planet3.setScale(glm::vec3(0.3f));

    spaceship.setScale(glm::vec3(0.2f));
    spaceship.setPosition(glm::vec3(0.0f, 0.0f, -5.0f));

    // parametry orbit planet
    float angle1 = 0.0f;
    float angle2 = 0.0f;
    float angle3 = 0.0f;

    float orbitSpeed1 = 0.5f;
    float orbitSpeed2 = 0.3f;
    float orbitSpeed3 = 0.8f;

    float orbitRadius1 = 4.0f;
    float orbitRadius2 = 7.0f;
    float orbitRadius3 = 10.0f;

    // ===================== PĘTLA GŁÓWNA =====================
    while (!glfwWindowShouldClose(window))
    {
        // --- czas ---
        float currentFrame = static_cast<float>(glfwGetTime());
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        // --- input kamery 1 (swobodna) ---
        camera1.ProcessKeyboard(keys, deltaTime);

        // --- czyszczenie ---
        glClearColor(0.02f, 0.02f, 0.06f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        // --- projekcja ---
        int width, height;
        glfwGetFramebufferSize(window, &width, &height);
        glm::mat4 projection = glm::perspective(
            glm::radians(45.0f),
            static_cast<float>(width) / static_cast<float>(height),
            0.1f,
            100.0f
        );

        // -------- RUCH STATKU --------
        float shipSpeed = 5.0f;
        float rotationSpeed = glm::radians(90.0f);

        glm::vec3 shipRot = spaceship.getRotation();
        if (keys[GLFW_KEY_A]) shipRot.y += rotationSpeed * deltaTime;
        if (keys[GLFW_KEY_D]) shipRot.y -= rotationSpeed * deltaTime;
        spaceship.setRotation(shipRot);

        glm::vec3 forward = computeShipForward(shipRot);

        if (keys[GLFW_KEY_W])
            spaceship.setPosition(spaceship.getPosition() + forward * shipSpeed * deltaTime);
        if (keys[GLFW_KEY_S])
            spaceship.setPosition(spaceship.getPosition() - forward * shipSpeed * deltaTime);

        // -------- ORBITY PLANET --------
        angle1 += orbitSpeed1 * deltaTime;
        angle2 += orbitSpeed2 * deltaTime;
        angle3 += orbitSpeed3 * deltaTime;

        planet1.setPosition(glm::vec3(
            cosf(angle1) * orbitRadius1,
            0.0f,
            sinf(angle1) * orbitRadius1
        ));
        planet2.setPosition(glm::vec3(
            cosf(angle2) * orbitRadius2,
            0.0f,
            sinf(angle2) * orbitRadius2
        ));
        planet3.setPosition(glm::vec3(
            cosf(angle3) * orbitRadius3,
            0.0f,
            sinf(angle3) * orbitRadius3
        ));

        // -------- WYBÓR KAMERY --------
        if (keys[GLFW_KEY_C] && !cKeyPressedLastFrame)
        {
            activeCamera = (activeCamera % 4) + 1;
            cKeyPressedLastFrame = true;
        }
        if (!keys[GLFW_KEY_C])
            cKeyPressedLastFrame = false;

        glm::mat4 view;
        glm::vec3 viewPos;

        if (activeCamera == 1)
        {
            view = camera1.GetViewMatrix();
            viewPos = camera1.Position;
        }
        else if (activeCamera == 2)
        {
            camera2.Position = planet1.getPosition() + camera2Offset;
            view = glm::lookAt(
                camera2.Position,
                planet1.getPosition(),
                glm::vec3(0.0f, 1.0f, 0.0f)
            );
            viewPos = camera2.Position;
        }
        else if (activeCamera == 3)
        {
            view = glm::lookAt(
                camera3.Position,
                planet3.getPosition(),
                glm::vec3(0.0f, 1.0f, 0.0f)
            );
            viewPos = camera3.Position;
        }
        else // 4 – kamera za statkiem
        {
            glm::vec3 cameraOffset = -forward * 4.0f + glm::vec3(0.0f, 3.0f, 0.0f);
            cameraShip.Position = spaceship.getPosition() + cameraOffset;
            glm::vec3 camTarget = spaceship.getPosition() + forward * 5.0f;

            view = glm::lookAt(
                cameraShip.Position,
                camTarget,
                glm::vec3(0.0f, 1.0f, 0.0f)
            );
            viewPos = cameraShip.Position;
        }

        // ===================== ŚWIATŁO =====================
        glm::vec3 lightPos(0.0f, 0.0f, 0.0f); // słońce

        // ===== PLANETY + SŁOŃCE (lightingShader) =====
        lightingShader.use();
        lightingShader.setVec3("viewPos", viewPos);
        lightingShader.setVec3("lightPos", lightPos);
        lightingShader.setVec3("lightColor", glm::vec3(1.0f, 0.95f, 0.8f));
        lightingShader.setVec3("ambientLight", glm::vec3(0.15f, 0.15f, 0.2f));
        lightingShader.setVec3("topLightPos", glm::vec3(0.0f, 8.0f, 0.0f));
        lightingShader.setVec3("topLightColor", glm::vec3(0.4f, 0.4f, 0.5f));

        // --- Słońce ---
        lightingShader.setVec3("objectColor", glm::vec3(1.0f, 0.9f, 0.6f));
        lightingShader.setFloat("shininess", 64.0f);
        sun.draw(view, projection);

        // --- Planeta 1 ---
        lightingShader.setVec3("objectColor", glm::vec3(0.4f, 0.6f, 1.0f));
        lightingShader.setFloat("shininess", 32.0f);
        planet1.draw(view, projection);

        // --- Planeta 2 ---
        lightingShader.setVec3("objectColor", glm::vec3(0.8f, 0.4f, 0.4f));
        lightingShader.setFloat("shininess", 16.0f);
        planet2.draw(view, projection);

        // --- Planeta 3 ---
        lightingShader.setVec3("objectColor", glm::vec3(0.4f, 0.8f, 0.5f));
        lightingShader.setFloat("shininess", 8.0f);
        planet3.draw(view, projection);

        // ===== STATEK (spotlightShader) =====
        spotlightShader.use();
        spotlightShader.setMat4("view", view);
        spotlightShader.setMat4("projection", projection);
        spotlightShader.setVec3("objectColor", glm::vec3(0.8f, 0.8f, 0.9f));
        spotlightShader.setVec3("ambientLight", glm::vec3(0.15f, 0.15f, 0.2f));
        spotlightShader.setVec3("topLightPos", glm::vec3(0.0f, 8.0f, 0.0f));
        spotlightShader.setVec3("topLightColor", glm::vec3(0.4f, 0.4f, 0.5f));
        spotlightShader.setVec3("viewPos", viewPos);

        glm::vec3 shipPos = spaceship.getPosition();

        // --- reflektor przód ---
        glm::vec3 frontPos = shipPos + forward * 0.6f;
        spotlightShader.setVec3("spotLightPos", frontPos);
        spotlightShader.setVec3("spotLightDir", forward);
        spotlightShader.setFloat("cutOff", glm::cos(glm::radians(8.0f)));
        spotlightShader.setFloat("outerCutOff", glm::cos(glm::radians(12.0f)));
        spotlightShader.setVec3("spotLightColor", glm::vec3(1.0f, 1.0f, 0.9f));

        // --- reflektor tył ---
        glm::vec3 backPos = shipPos - forward * 0.6f;
        glm::vec3 backDir = -forward;
        spotlightShader.setVec3("backLightPos", backPos);
        spotlightShader.setVec3("backLightDir", backDir);
        spotlightShader.setFloat("backCutOff", glm::cos(glm::radians(15.0f)));
        spotlightShader.setFloat("backOuterCutOff", glm::cos(glm::radians(25.0f)));
        spotlightShader.setVec3("backLightColor", glm::vec3(1.0f, 0.0f, 0.0f));

        // --- rysowanie statku ---
        spaceship.draw(view, projection);

        // --- swap/poll ---
        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}
