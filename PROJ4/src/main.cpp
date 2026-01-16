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
bool keys[1024];
float deltaTime = 0.0f;
float lastFrame = 0.0f;

// ===================== CALLBACK =====================
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
// ===================== SPACESHIP =====================
float shipVertices[] = {
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

// ===================== SPHERE ====================
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

            // position
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

void framebuffer_size_callback(GLFWwindow* window, int width, int height)
{
    glViewport(0, 0, width, height);
}


// ===================== MAIN =====================
int main()
{
    // -------- GLFW --------
    glfwInit();
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
    glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);


    glfwMakeContextCurrent(window);
    glfwSetKeyCallback(window, key_callback);

    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
    {
        std::cout << "Failed to init GLAD\n";
        return -1;
    }

    glEnable(GL_DEPTH_TEST);

    // -------- CAMERA --------
    int activeCamera = 1;
    bool cKeyPressedLastFrame = false;
    Camera camera1(glm::vec3(0.0f, 2.0f, 10.0f));
    Camera camera2(glm::vec3(0.0f, 4.0f, 15.0f));
    Camera camera3(glm::vec3(0.0f, 2.0f, 10.0f));
    Camera cameraShip(glm::vec3(0.0f, 1.0f, -3.0f));
    
    glm::vec3 cameraShipOffset(0.0f, 1.0f, -3.0f);
    glm::vec3 camera2Offset(0.0f, 2.0f, 5.0f);
    glm::vec3 camera3Position(15.0f, 10.0f, 15.0f);


    // -------- SHADER --------
    Shader lightingShader(
        "./shaders/light_vertex.glsl",
        "./shaders/light_fragment2.glsl"
    );
    Shader spotlightShader(
        "./shaders/light_vertex.glsl", 
        "./shaders/spotlight_fragment.glsl"
    );


    // -------- GEOMETRIA SPHERY --------
    std::vector<float> sphereVertices;
    std::vector<unsigned int> sphereIndices;

    generateSphere(1.0f, 48, 24, sphereVertices, sphereIndices);

    // -------- OBIEKTY --------
    Object3D sun(
        sphereVertices.data(), sphereVertices.size(),
        sphereIndices.data(), sphereIndices.size(),
        &lightingShader
    );

    Object3D planet1(
        sphereVertices.data(), sphereVertices.size(),
        sphereIndices.data(), sphereIndices.size(),
        &lightingShader
    );

    Object3D planet2(
        sphereVertices.data(), sphereVertices.size(),
        sphereIndices.data(), sphereIndices.size(),
        &lightingShader
    );

    Object3D planet3(
        sphereVertices.data(), sphereVertices.size(),
        sphereIndices.data(), sphereIndices.size(),
        &lightingShader
    );

    Object3D spaceship(
        shipVertices, sizeof(shipVertices)/sizeof(float),
        shipIndices, sizeof(shipIndices)/sizeof(unsigned int),
        &spotlightShader 
    );

    // -------- TRANSFORMACJE --------
    sun.setScale(glm::vec3(1.5f));

    planet1.setScale(glm::vec3(0.4f));
    planet2.setScale(glm::vec3(0.6f));
    planet3.setScale(glm::vec3(0.3f));



    // ===================== LOOP =====================
    while (!glfwWindowShouldClose(window))
    {
        float currentFrame = glfwGetTime();
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        //camera1.ProcessKeyboard(keys, deltaTime);

        glClearColor(0.02f, 0.02f, 0.06f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        int width, height;
        glfwGetFramebufferSize(window, &width, &height);
        glm::mat4 projection = glm::perspective(
            glm::radians(45.0f),
            static_cast<float>(width) / static_cast<float>(height),
            0.1f,
            100.0f
        );


        // -------- Ruch statku --------

        float shipSpeed = 5.0f; // jednostki na sekundę
        float rotationSpeed = glm::radians(90.0f); // radiany na sekundę

        // obrót w lewo/prawo
        glm::vec3 rot = spaceship.getRotation();
        if (keys[GLFW_KEY_A])
            rot.y += rotationSpeed * deltaTime;
        if (keys[GLFW_KEY_D])
            rot.y -= rotationSpeed * deltaTime;
        spaceship.setRotation(rot);

        glm::vec3 forward;
        float yaw = spaceship.getRotation().y; // rotacja wokół osi Y
        forward.x = sin(yaw);
        forward.y = 0.0f;
        forward.z = cos(yaw);
        glm::vec3 right = glm::cross(forward, glm::vec3(0.0f, 1.0f, 0.0f));

        // poruszanie przód/tył
        if (keys[GLFW_KEY_W])
            spaceship.setPosition(spaceship.getPosition() + forward * shipSpeed * deltaTime);
        if (keys[GLFW_KEY_S])
            spaceship.setPosition(spaceship.getPosition() - forward * shipSpeed * deltaTime);

        // -------- Wybor aktywanej kamery -------
        if (keys[GLFW_KEY_C] && !cKeyPressedLastFrame) {
            activeCamera = (activeCamera % 4) + 1;
            cKeyPressedLastFrame = true;
        }
        if(!keys[GLFW_KEY_C]) {
            cKeyPressedLastFrame = false;
        }

        glm::mat4 view;

        if (activeCamera == 1) {
            view = camera1.GetViewMatrix(); 
        } else if (activeCamera == 2) {
            camera2.Position = planet1.getPosition() + camera2Offset;
            view = glm::lookAt(camera2.Position, planet1.getPosition(), glm::vec3(0.0f, 1.0f, 0.0f));
        } else if (activeCamera == 3) {
            view = glm::lookAt(camera3Position, planet3.getPosition(), glm::vec3(0.0f, 1.0f, 0.0f));
        } else if (activeCamera == 4) {
            cameraShip.Position = spaceship.getPosition() + cameraShipOffset;
            glm::vec3 camTarget = spaceship.getPosition() + forward * 2.0f;
            view = glm::lookAt(spaceship.getPosition() + cameraShipOffset, camTarget, glm::vec3(0.0f, 1.0f, 0.0f));
        }


        // -------- ŚWIATŁO (SŁOŃCE) --------
        glm::vec3 lightPos(0.0f, 0.0f, 0.0f);

        lightingShader.use();
        glUniform3fv(
            glGetUniformLocation(lightingShader.ID, "lightPos"),
            1, &lightPos[0]
        );

        glUniform3fv(
            glGetUniformLocation(lightingShader.ID, "viewPos"),
            1, &camera1.Position[0]
        );

        glUniform3f(
            glGetUniformLocation(lightingShader.ID, "lightColor"),
            1.0f, 0.95f, 0.8f
        );


        // -------- ORBITY --------
        float t = currentFrame;

        planet1.setPosition(glm::vec3(cos(t * 0.8f) * 3.0f, 0.0f, sin(t * 0.8f) * 3.0f));
        planet2.setPosition(glm::vec3(cos(t * 0.5f) * 5.0f, 0.0f, sin(t * 0.5f) * 5.0f));
        planet3.setPosition(glm::vec3(cos(t * 1.2f) * 7.0f, 0.0f, sin(t * 1.2f) * 7.0f));

        planet1.setRotation(glm::vec3(0.0f, t, 0.0f));
        planet2.setRotation(glm::vec3(0.0f, t * 0.7f, 0.0f));
        planet3.setRotation(glm::vec3(0.0f, t * 1.5f, 0.0f));

        // -------- DRAW SPACESHIP ----------
        spotlightShader.use();
        spotlightShader.setVec3("ambientLight", 0.3f, 0.3f, 0.3f); // dodaje trochę światła
        spotlightShader.setVec3("topLightPos", glm::vec3(0.0f, 5.0f, 0.0f));
        spotlightShader.setVec3("topLightColor", glm::vec3(1.0f, 1.0f, 1.0f));

        // Ustawienie macierzy
        spotlightShader.setMat4("view", view);
        spotlightShader.setMat4("projection", projection);

        // Kolor obiektu
        spotlightShader.setVec3("objectColor", 0.8f, 0.8f, 0.9f);

        // Ustawienie pozycji obserwatora (bardzo ważne dla Phonga!)
        spotlightShader.setVec3("viewPos", cameraShip.Position);

        // --------- Dwa reflektory ---------
        // --- obliczenie kierunku statku ---
        glm::vec3 shipPos = spaceship.getPosition();
        glm::vec3 yawPitch = spaceship.getRotation();
        float yaw2 = yawPitch.y;
        float pitch = yawPitch.x;

        glm::vec3 shipForward;
        shipForward.x = cos(pitch) * sin(yaw2);
        shipForward.y = sin(pitch);
        shipForward.z = cos(pitch) * cos(yaw2);
        shipForward = glm::normalize(shipForward);

        // reflektor przed statkiem, nie w środku
        glm::vec3 spotlightPos = shipPos + shipForward * 1.5f;

        // użycie shadera
        spotlightShader.use();
        spotlightShader.setMat4("view", view);
        spotlightShader.setMat4("projection", projection);

        // kolory
        spotlightShader.setVec3("objectColor", 0.8f, 0.8f, 0.9f);
        spotlightShader.setVec3("ambientLight", 0.2f, 0.2f, 0.2f);

        spotlightShader.setVec3("topLightPos", glm::vec3(0.0f, 5.0f, 0.0f));
        spotlightShader.setVec3("topLightColor", glm::vec3(1.0f, 1.0f, 1.0f));

        // reflektor
        spotlightShader.setVec3("spotLightPos", spotlightPos);
        spotlightShader.setVec3("spotLightDir", shipForward);
        spotlightShader.setFloat("cutOff", glm::cos(glm::radians(20.0f)));
        spotlightShader.setFloat("outerCutOff", glm::cos(glm::radians(30.0f)));
        spotlightShader.setVec3("spotLightColor", 1.0f, 1.0f, 0.9f);

        // bardzo ważne dla speculara:
        spotlightShader.setVec3("viewPos", cameraShip.Position);

        // rysowanie statku
        spaceship.draw(view, projection);



        // -------- DRAW --------
        // SŁOŃCE
        glUniform3f(
            glGetUniformLocation(lightingShader.ID, "objectColor"),
            1.0f, 0.9f, 0.6f
        );
        glUniform1f(
            glGetUniformLocation(lightingShader.ID, "shininess"),
            64.0f
        );
        sun.draw(view, projection);

        // PLANETA 1
        glUniform3f(
            glGetUniformLocation(lightingShader.ID, "objectColor"),
            0.4f, 0.6f, 1.0f
        );
        glUniform1f(
            glGetUniformLocation(lightingShader.ID, "shininess"),
            32.0f
        );
        planet1.draw(view, projection);

        // PLANETA 2
        glUniform3f(
            glGetUniformLocation(lightingShader.ID, "objectColor"),
            0.8f, 0.4f, 0.4f
        );
        glUniform1f(
            glGetUniformLocation(lightingShader.ID, "shininess"),
            16.0f
        );
        planet2.draw(view, projection);

        // PLANETA 3
        glUniform3f(
            glGetUniformLocation(lightingShader.ID, "objectColor"),
            0.4f, 0.8f, 0.5f
        );
        glUniform1f(
            glGetUniformLocation(lightingShader.ID, "shininess"),
            8.0f
        );
        planet3.draw(view, projection);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}



