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
#include "Spaceship.h"
#include "MirrorSurface.h"

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

// ===================== GLOBALNE =====================
bool keys[1024] = { false };
float deltaTime = 0.0f;
float lastFrame = 0.0f;
int SCR_WIDTH  = 800;
int SCR_HEIGHT = 600;
GLuint mirrorDepthBuffer;


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
    SCR_WIDTH = width;
    SCR_HEIGHT = height;
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

            vertices.push_back(x);
            vertices.push_back(y);
            vertices.push_back(z);

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
    float spotYawOffset = 0.0f;
    float spotPitchOffset = 0.0f;
    bool useBlinn = false;
    bool blinnPressedLastFrame = false;
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
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

    glGenRenderbuffers(1, &mirrorDepthBuffer);
    glBindRenderbuffer(GL_RENDERBUFFER, mirrorDepthBuffer);
    glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH_COMPONENT24, SCR_WIDTH, SCR_HEIGHT);
    glBindRenderbuffer(GL_RENDERBUFFER, 0);



    // -------- SHADER --------
    Shader lightingShader(
        "./shaders/light_vertex.glsl",
        "./shaders/light_fragment_camera.glsl"
    );
    Shader spotlightShader(
        "./shaders/light_vertex.glsl",
        "./shaders/spotlight_fragment_camera.glsl"
    );
    Shader fogShader(
        "./shaders/light_vertex.glsl", 
        "./shaders/fog_fragment.glsl"
    );
    Shader mirrorShader(
        "./shaders/mirror_vertex.glsl", 
        "./shaders/mirror_fragment.glsl"
    );


    // -------- KAMERA --------
    int activeCamera = 1;
    bool cKeyPressedLastFrame = false;

    Camera camera1(glm::vec3(0.0f, 4.0f, 20.0f)); 
    Camera camera2(glm::vec3(0.0f, 4.0f, 15.0f));
    Camera camera3(glm::vec3(15.0f, 10.0f, 15.0f));
    Camera cameraShip(glm::vec3(0.0f, 3.0f, -5.0f));

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
    Object3D spaceshipObj(
        shipVertices, sizeof(shipVertices) / sizeof(float),
        shipIndices, sizeof(shipIndices) / sizeof(unsigned int),
        &spotlightShader
    );
    Spaceship spaceship(&spaceshipObj);
    Object3D moon(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &lightingShader
    );
    Object3D planet4(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &lightingShader
    );
    Object3D fogSphere(
        sphereVertices.data(), static_cast<unsigned int>(sphereVertices.size()),
        sphereIndices.data(), static_cast<unsigned int>(sphereIndices.size()),
        &fogShader
    );
    MirrorSurface mirrorSurface(&mirrorShader);


    sun.setScale(glm::vec3(1.3f));
    sun.setPosition(glm::vec3(0.0f));

    planet1.setScale(glm::vec3(0.4f));
    planet2.setScale(glm::vec3(0.6f));
    planet3.setScale(glm::vec3(0.3f));
    planet4.setScale(glm::vec3(0.65f));
    moon.setScale(glm::vec3(0.05f));
    fogSphere.setScale(glm::vec3(30.0f));
    fogSphere.setPosition(glm::vec3(0.0f));


    spaceship.setScale(glm::vec3(0.15f));
    spaceship.setPosition(glm::vec3(0.0f, 0.0f, 5.0f));

    // parametry orbit planet
    float angle1 = 0.0f;
    float angle2 = 0.0f;
    float angle3 = 0.0f;
    float moonAngle = 0.0f;
    float angle4 = 0.0f;

    float orbitSpeed1 = 0.5f;
    float orbitSpeed2 = 0.3f;
    float orbitSpeed3 = 0.8f;
    float orbitSpeed4 = 1.1f;
    float mirrorOrbitSpeed = 0.2f;

    float orbitRadius1 = 4.0f;
    float orbitRadius2 = 7.0f;
    float orbitRadius3 = 10.0f;
    float orbitRadius4 = 13.0f;

    // --- parametry lustra ---
    float mirrorAngle = 0.0f;
    float mirrorOrbitRadius = 8.0f;
    glm::vec3 mirrorPos(0.0f, 0.0f, -20.0f);

    // ===================== PĘTLA GŁÓWNA =====================
    while (!glfwWindowShouldClose(window))
    {
        float currentFrame = static_cast<float>(glfwGetTime());
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        glClearColor(0.02f, 0.02f, 0.06f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        int width, height;
        glfwGetFramebufferSize(window, &width, &height);
        glm::mat4 projection = glm::perspective(
            glm::radians(45.0f),
            static_cast<float>(SCR_WIDTH) / static_cast<float>(SCR_HEIGHT),
            0.1f,
            300.0f
        );

        spaceship.update(keys, deltaTime);
        glm::vec3 forward = spaceship.getForward(); 
        glm::vec3 baseDir = forward;

        glm::vec3 up = glm::vec3(0.0f, 1.0f, 0.0f);
        glm::vec3 right = glm::normalize(glm::cross(baseDir, up));
        up = glm::normalize(glm::cross(right, baseDir));

        glm::vec3 spotDir = glm::normalize(
            baseDir
            + right * (float)sin(spotYawOffset)
            + up    * (float)sin(spotPitchOffset)
        );

        // -------- ORBITY PLANET --------
        angle1 += orbitSpeed1 * deltaTime;
        angle2 += orbitSpeed2 * deltaTime;
        angle3 += orbitSpeed3 * deltaTime;
        angle4 += orbitSpeed4 * deltaTime;
        moonAngle += deltaTime * 3.0f;

        planet1.setPosition(glm::vec3(cosf(angle1) * orbitRadius1,0.0f,sinf(angle1) * orbitRadius1));
        planet2.setPosition(glm::vec3(cosf(angle2) * orbitRadius2,0.0f,sinf(angle2) * orbitRadius2));
        planet3.setPosition(glm::vec3(cosf(angle3) * orbitRadius3,0.0f,sinf(angle3) * orbitRadius3));
        glm::vec3 planet1Pos = planet1.getPosition();
        glm::vec3 moonPos = planet1Pos + glm::vec3(cos(moonAngle) * 0.8f,0.0f,sin(moonAngle) * 0.8f);
        moon.setPosition(moonPos);
        planet4.setPosition(glm::vec3(cosf(angle4) * orbitRadius4,0.0f,sinf(angle4) * orbitRadius4));

        // Aktualizacja pozycji lustra (wewnątrz pętli)
        mirrorAngle += mirrorOrbitSpeed * deltaTime;
        mirrorPos = glm::vec3(sinf(currentFrame * 0.5f) * 5.0f, 2.0f, -8.0f); 
        mirrorSurface.setPosition(mirrorPos);

        // -------- WYBÓR KAMERY --------
        if (keys[GLFW_KEY_C] && !cKeyPressedLastFrame){activeCamera = (activeCamera % 4) + 1;cKeyPressedLastFrame = true;}
        if (!keys[GLFW_KEY_C])cKeyPressedLastFrame = false;

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
        else if (activeCamera == 4)
        {
        glm::vec3 shipPos = spaceship.getPosition();
            glm::vec3 forward = spaceship.getForward();

            glm::vec3 cameraPos =
                shipPos
                - forward * 6.0f
                + glm::vec3(0.0f, 2.5f, 0.0f); 

            glm::vec3 cameraTarget =
                shipPos
                + forward * 10.0f
                + glm::vec3(0.0f, -1.5f, 0.0f); 

            view = glm::lookAt(
                cameraPos,
                cameraTarget,
                glm::vec3(0.0f, 1.0f, 0.0f)
            );

            viewPos = cameraPos;
        }


        float spotRotateSpeed = glm::radians(45.0f);

        if (keys[GLFW_KEY_LEFT])
            spotYawOffset -= spotRotateSpeed * deltaTime;
        if (keys[GLFW_KEY_RIGHT])
            spotYawOffset += spotRotateSpeed * deltaTime;
        if (keys[GLFW_KEY_UP])
            spotPitchOffset += spotRotateSpeed * deltaTime;
        if (keys[GLFW_KEY_DOWN])
            spotPitchOffset -= spotRotateSpeed * deltaTime;

        float maxPitch = glm::radians(60.0f);
        if (spotPitchOffset >  maxPitch) spotPitchOffset =  maxPitch;
        if (spotPitchOffset < -maxPitch) spotPitchOffset = -maxPitch;

        // ===================== NOC/DZIEN button =====================
        static bool isNight = false;
        static bool togglePressed = false;
        if(keys[GLFW_KEY_B] && !togglePressed)
        {
            isNight = !isNight;
            togglePressed = true;
        }
        if(!keys[GLFW_KEY_B])
        {
            togglePressed = false;
        }
        // ===================== MGLA button =====================
        static float fogDensity = 0.01f;
        glm::vec3 fogColor = glm::vec3(0.0f, 0.0f, 0.0f);// better: czarna mgla

        if (keys[GLFW_KEY_M])
            fogDensity += 0.3f * deltaTime;
    
        if (keys[GLFW_KEY_N])
            fogDensity -= 0.3f * deltaTime;

        fogDensity = glm::clamp(fogDensity, 0.0f, 2.0f);

        // ===== Przełączanie Phong / Blinn =====
        if (keys[GLFW_KEY_P] && !blinnPressedLastFrame) {
            useBlinn = !useBlinn;
            blinnPressedLastFrame = true;
        }
        if (!keys[GLFW_KEY_P]) {
            blinnPressedLastFrame = false;
        }

        // ===================== ŚWIATŁO =====================
        glm::vec3 lightPos(0.0f, 0.0f, 0.0f);
        glm::vec3 shipPos = spaceship.getPosition();
        glm::vec3 frontPos = shipPos + forward * 1.2f;
        glm::vec3 backPos  = shipPos - forward * 1.2f;

        // 1) RYSOWANIE MASKI (LUSTRA) DO STENCILA
        glEnable(GL_STENCIL_TEST);
        glStencilOp(GL_KEEP, GL_KEEP, GL_REPLACE);
        glStencilFunc(GL_ALWAYS, 1, 0xFF);
        glStencilMask(0xFF);

        glDepthMask(GL_TRUE); 
        glColorMask(GL_FALSE, GL_FALSE, GL_FALSE, GL_FALSE);
        glClear(GL_STENCIL_BUFFER_BIT); 

        mirrorSurface.draw(view, projection);

        // 2) RYSOWANIE ODBICIA
        glStencilFunc(GL_EQUAL, 1, 0xFF);
        glStencilMask(0x00);
        glColorMask(GL_TRUE, GL_TRUE, GL_TRUE, GL_TRUE);

        glClear(GL_DEPTH_BUFFER_BIT); 

        glm::mat4 reflectedView = view;
        reflectedView = glm::translate(reflectedView, mirrorPos);
        reflectedView = glm::scale(reflectedView, glm::vec3(1, 1, -1));
        reflectedView = glm::translate(reflectedView, -mirrorPos);

        glCullFace(GL_FRONT);

        lightingShader.use();
        lightingShader.setBool("useBlinn", useBlinn);

        glm::vec3 lightPosRefl      = glm::vec3(reflectedView * glm::vec4(lightPos, 1.0));
        glm::vec3 spotLightPosRefl  = glm::vec3(reflectedView * glm::vec4(backPos, 1.0));
        glm::vec3 backLightPosRefl  = glm::vec3(reflectedView * glm::vec4(frontPos, 1.0));
        glm::vec3 spotLightDirRefl  = glm::normalize(glm::mat3(reflectedView) * (-spotDir));
        glm::vec3 backLightDirRefl  = glm::normalize(glm::mat3(reflectedView) * forward);

        lightingShader.setVec3("lightPos", lightPosRefl);
        lightingShader.setVec3("spotLightPos", spotLightPosRefl);
        lightingShader.setVec3("spotLightDir", spotLightDirRefl);
        lightingShader.setVec3("backLightPos", backLightPosRefl);
        lightingShader.setVec3("backLightDir", backLightDirRefl);

        lightingShader.setVec3("objectColor", glm::vec3(0.4f, 0.6f, 1.0f));
        lightingShader.setFloat("shininess", 32.0f);
        planet1.draw(reflectedView, projection);

        lightingShader.setVec3("objectColor", glm::vec3(0.8f, 0.4f, 0.4f));
        lightingShader.setFloat("shininess", 16.0f);
        planet2.draw(reflectedView, projection);

        lightingShader.setVec3("objectColor", glm::vec3(0.4f, 0.8f, 0.5f));
        lightingShader.setFloat("shininess", 8.0f);
        planet3.draw(reflectedView, projection);

        lightingShader.setVec3("objectColor", glm::vec3(0.6f, 0.4f, 0.9f));
        lightingShader.setFloat("shininess", 4.0f);
        planet4.draw(reflectedView, projection);

        if (!isNight)
            lightingShader.setVec3("objectColor", glm::vec3(1.0f, 0.9f, 0.6f));
        else
            lightingShader.setVec3("objectColor", glm::vec3(0.8f, 0.8f, 0.85f));

        lightingShader.setFloat("shininess", 64.0f);
        sun.draw(reflectedView, projection);

        spotlightShader.use();
        spotlightShader.setMat4("view", reflectedView); 
        spotlightShader.setMat4("projection", projection);

        glm::vec3 reflectedViewPos = viewPos;
        reflectedViewPos.z = mirrorPos.z - (viewPos.z - mirrorPos.z);
        spotlightShader.setVec3("viewPos", reflectedViewPos);
        spotlightShader.setVec3("objectColor", glm::vec3(0.8f, 0.8f, 0.9f));
        spotlightShader.setVec3("ambientLight", glm::vec3(0.15f, 0.15f, 0.2f));
        glm::vec3 topLightPosRefl = glm::vec3(reflectedView * glm::vec4(glm::vec3(0, 8, 0), 1.0));
        spotlightShader.setVec3("topLightPos", topLightPosRefl);
        spotlightShader.setVec3("topLightColor", glm::vec3(0.4f, 0.4f, 0.5f));
        spotlightShader.setFloat("fogDensity", fogDensity);
        spotlightShader.setVec3("fogColor", fogColor);

        spotlightShader.setVec3("spotLightPos", spotLightPosRefl);
        spotlightShader.setVec3("spotLightDir", spotLightDirRefl);
        spotlightShader.setVec3("spotLightColor", glm::vec3(1.0f, 1.0f, 0.9f));
        spotlightShader.setFloat("cutOff", glm::cos(glm::radians(8.0f)));
        spotlightShader.setFloat("outerCutOff", glm::cos(glm::radians(12.0f)));

        spotlightShader.setVec3("backLightPos", backLightPosRefl);
        spotlightShader.setVec3("backLightDir", backLightDirRefl);
        spotlightShader.setVec3("backLightColor", glm::vec3(1.0f, 0.0f, 0.0f));
        spotlightShader.setFloat("backCutOff", glm::cos(glm::radians(15.0f)));
        spotlightShader.setFloat("backOuterCutOff", glm::cos(glm::radians(25.0f)));

        glDisable(GL_CULL_FACE); 
        spaceshipObj.draw(reflectedView, projection);
        glEnable(GL_CULL_FACE);

        glCullFace(GL_BACK);
        glDisable(GL_STENCIL_TEST);

        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        mirrorSurface.draw(view, projection);
        glDisable(GL_BLEND);


        // ===== TRANSFORMACJA ŚWIATEŁ DO CAMERA SPACE =====
        glm::vec3 lightPosCam     = glm::vec3(view * glm::vec4(lightPos, 1.0));
        glm::vec3 spotLightPosCam = glm::vec3(view * glm::vec4(backPos, 1.0));
        glm::vec3 backLightPosCam = glm::vec3(view * glm::vec4(frontPos, 1.0));
        glm::vec3 topLightPosCam  = glm::vec3(view * glm::vec4(glm::vec3(0,8,0), 1.0));

        glm::vec3 spotLightDirCam = glm::normalize(glm::mat3(view) * (-spotDir));
        glm::vec3 backLightDirCam = glm::normalize(glm::mat3(view) * forward);

        // ===== PLANETY + SŁOŃCE (lightingShader) =====
        lightingShader.use();

        lightingShader.setBool("useBlinn", useBlinn);
        lightingShader.setVec3("lightPos", lightPosCam);

        if (!isNight)
            lightingShader.setVec3("lightColor", glm::vec3(1.0f, 0.95f, 0.8f));
        else
            lightingShader.setVec3("lightColor", glm::vec3(0.05f, 0.3f, 0.6f));

        lightingShader.setVec3("ambientLight", glm::vec3(0.15f, 0.15f, 0.2f));
        lightingShader.setVec3("topLightPos", topLightPosCam);
        lightingShader.setVec3("topLightColor", glm::vec3(0.4f, 0.4f, 0.5f));

        // ===== Mgła =====
        lightingShader.setVec3("fogColor", fogColor);
        lightingShader.setFloat("fogDensity", fogDensity);

        // ===== REFLEKTOR PRZEDNI =====
        lightingShader.setVec3("spotLightPos", spotLightPosCam);
        lightingShader.setVec3("spotLightDir", spotLightDirCam);
        lightingShader.setVec3("spotLightColor", glm::vec3(1.0f, 1.0f, 0.9f));
        lightingShader.setFloat("cutOff", glm::cos(glm::radians(10.0f)));
        lightingShader.setFloat("outerCutOff", glm::cos(glm::radians(15.0f)));

        // ===== REFLEKTOR TYLNY =====
        lightingShader.setVec3("backLightPos", backLightPosCam);
        lightingShader.setVec3("backLightDir", backLightDirCam);
        lightingShader.setVec3("backLightColor", glm::vec3(1.0f, 0.0f, 0.0f));
        lightingShader.setFloat("backCutOff", glm::cos(glm::radians(5.0f)));
        lightingShader.setFloat("backOuterCutOff", glm::cos(glm::radians(10.0f)));

        // --- Słońce ---
        if (!isNight)
            lightingShader.setVec3("objectColor", glm::vec3(1.0f, 0.9f, 0.6f));
        else
            lightingShader.setVec3("objectColor", glm::vec3(0.8f, 0.8f, 0.85f));

        lightingShader.setFloat("shininess", 64.0f);
        sun.draw(view, projection);

        // --- Planety ---
        lightingShader.setVec3("objectColor", glm::vec3(0.4f, 0.6f, 1.0f));
        lightingShader.setFloat("shininess", 32.0f);
        planet1.draw(view, projection);

        lightingShader.setVec3("objectColor", glm::vec3(0.8f, 0.4f, 0.4f));
        lightingShader.setFloat("shininess", 16.0f);
        planet2.draw(view, projection);

        lightingShader.setVec3("objectColor", glm::vec3(0.4f, 0.8f, 0.5f));
        lightingShader.setFloat("shininess", 8.0f);
        planet3.draw(view, projection);

        lightingShader.setVec3("objectColor", glm::vec3(0.6f, 0.4f, 0.9f));
        lightingShader.setFloat("shininess", 4.0f);
        planet4.draw(view, projection);

        // ===== STATEK (spotlightShader) ====
        spotlightShader.use();
        spotlightShader.setBool("useBlinn", useBlinn);
        spotlightShader.setMat4("view", view);
        spotlightShader.setMat4("projection", projection);
        spotlightShader.setVec3("objectColor", glm::vec3(0.8f, 0.8f, 0.9f));
        spotlightShader.setVec3("ambientLight", glm::vec3(0.15f, 0.15f, 0.2f));
        spotlightShader.setVec3("topLightPos", topLightPosCam);
        spotlightShader.setVec3("topLightColor", glm::vec3(0.4f, 0.4f, 0.5f));
        spotlightShader.setVec3("objectColor", glm::vec3(0.8f, 0.8f, 0.9f));
        spotlightShader.setVec3("ambientLight", glm::vec3(0.15f, 0.15f, 0.2f));

        // --- reflektor przód ---
        spotlightShader.setVec3("spotLightPos", spotLightPosCam);
        spotlightShader.setVec3("spotLightDir", spotLightDirCam);
        spotlightShader.setFloat("cutOff", glm::cos(glm::radians(8.0f)));
        spotlightShader.setFloat("outerCutOff", glm::cos(glm::radians(12.0f)));
        spotlightShader.setVec3("spotLightColor", glm::vec3(1.0f, 1.0f, 0.9f));

        // --- reflektor tył ---
        spotlightShader.setVec3("backLightPos", backLightPosCam);
        spotlightShader.setVec3("backLightDir", backLightDirCam);
        spotlightShader.setFloat("backCutOff", glm::cos(glm::radians(15.0f)));
        spotlightShader.setFloat("backOuterCutOff", glm::cos(glm::radians(25.0f)));
        spotlightShader.setVec3("backLightColor", glm::vec3(1.0f, 0.0f, 0.0f));

        // --- mgła ---
        spotlightShader.setFloat("fogDensity", fogDensity);
        spotlightShader.setVec3("fogColor", fogColor);

        // --- rysowanie statku ---
        moon.draw(view, projection);
        glDisable(GL_CULL_FACE);
        spaceshipObj.draw(view, projection);
        glEnable(GL_CULL_FACE);

        // ===================== MGŁA (fogSphere) =====================
        glDepthMask(GL_FALSE);
        glDisable(GL_CULL_FACE);

        fogShader.use();
        fogShader.setVec3("fogColor", fogColor);
        fogShader.setFloat("fogDensity", fogDensity);
        fogSphere.draw(view, projection);

        glEnable(GL_CULL_FACE);
        glDepthMask(GL_TRUE);



        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}
