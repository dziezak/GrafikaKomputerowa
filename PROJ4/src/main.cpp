#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <iostream>
#include <vector>
#include "Shader.h"
#include "Object3D.h"
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

// --- dane sześcianów i sfery ---
float cubeVerticesColored[] = {
    -0.5f, -0.5f, -0.5f, 1,0,0,
     0.5f, -0.5f, -0.5f, 0,1,0,
     0.5f,  0.5f, -0.5f, 0,0,1,
    -0.5f,  0.5f, -0.5f, 1,1,0,
    -0.5f, -0.5f,  0.5f, 1,0,1,
     0.5f, -0.5f,  0.5f, 0,1,1,
     0.5f,  0.5f,  0.5f, 1,1,1,
    -0.5f,  0.5f,  0.5f, 0,0,0
};

float cubeVerticesTextured[] = {
    // x, y, z, u, v
    -0.5f, -0.5f, -0.5f, 0.0f, 0.0f,
     0.5f, -0.5f, -0.5f, 1.0f, 0.0f,
     0.5f,  0.5f, -0.5f, 1.0f, 1.0f,
    -0.5f,  0.5f, -0.5f, 0.0f, 1.0f,
    -0.5f, -0.5f,  0.5f, 0.0f, 0.0f,
     0.5f, -0.5f,  0.5f, 1.0f, 0.0f,
     0.5f,  0.5f,  0.5f, 1.0f, 1.0f,
    -0.5f,  0.5f,  0.5f, 0.0f, 1.0f
};

unsigned int cubeIndicesForCube[] = {
    0,1,2, 2,3,0,
    4,5,6, 6,7,4,
    0,4,7, 7,3,0,
    1,5,6, 6,2,1,
    3,2,6, 6,7,3,
    0,1,5, 5,4,0
};

// --- funkcja do generowania sfery ---
void generateSphere(float radius, unsigned int sectorCount, unsigned int stackCount,
                    std::vector<float>& vertices, std::vector<unsigned int>& indices)
{
    float x, y, z, xy;
    float sectorStep = 2 * M_PI / sectorCount;
    float stackStep = M_PI / stackCount;
    for(unsigned int i = 0; i <= stackCount; ++i)
    {
        float stackAngle = M_PI / 2 - i * stackStep;
        xy = radius * cosf(stackAngle);
        z = radius * sinf(stackAngle);
        for(unsigned int j = 0; j <= sectorCount; ++j)
        {
            float sectorAngle = j * sectorStep;
            x = xy * cosf(sectorAngle);
            y = xy * sinf(sectorAngle);
            vertices.push_back(x);
            vertices.push_back(y);
            vertices.push_back(z);
        }
    }
    for(unsigned int i = 0; i < stackCount; ++i)
    {
        unsigned int k1 = i * (sectorCount + 1);
        unsigned int k2 = k1 + sectorCount + 1;
        for(unsigned int j = 0; j < sectorCount; ++j, ++k1, ++k2)
        {
            if(i != 0) { indices.push_back(k1); indices.push_back(k2); indices.push_back(k1+1); }
            if(i != stackCount-1) { indices.push_back(k1+1); indices.push_back(k2); indices.push_back(k2+1); }
        }
    }
}

int main()
{
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR,3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR,3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    GLFWwindow* window = glfwCreateWindow(800,600,"Textured Cube Demo", nullptr,nullptr);
    glfwMakeContextCurrent(window);
    if(!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)){ std::cout<<"GLAD error\n"; return -1; }
    glEnable(GL_DEPTH_TEST);

    // --- SHADERY ---
    Shader colorShader("shaders/vertex.glsl","shaders/fragment.glsl");       // do kolorowych
    Shader textureShader("shaders/vertex_texture.glsl","shaders/fragment_texture.glsl"); // do tekstury

    // --- OBIEKTY ---
    Object3D coloredCube(cubeVerticesColored, sizeof(cubeVerticesColored)/sizeof(float), cubeIndicesForCube, sizeof(cubeIndicesForCube)/sizeof(unsigned int), &colorShader);

    // sfera
    std::vector<float> sphereVertices; std::vector<unsigned int> sphereIndices;
    generateSphere(0.5f,36,18,sphereVertices,sphereIndices);
    Object3D sphere(sphereVertices.data(), sphereVertices.size(), sphereIndices.data(), sphereIndices.size(), &colorShader);

    // sześcian teksturowany
    Object3D texturedCube(cubeVerticesTextured, sizeof(cubeVerticesTextured)/sizeof(float), cubeIndicesForCube, sizeof(cubeIndicesForCube)/sizeof(unsigned int), &textureShader);

	// --- TEKSTURA ---
	unsigned int texture;
	glGenTextures(1, &texture);
	glBindTexture(GL_TEXTURE_2D, texture);

	// ustawienia parametrów tekstury
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

	// wczytanie obrazu
	int width, height, nrChannels;
	unsigned char* data = stbi_load("src/wall.jpg", &width, &height, &nrChannels, 0);
	if (data) {
		glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, data);
		glGenerateMipmap(GL_TEXTURE_2D);
	} else {
		std::cout << "Failed to load texture\n";
	}
	stbi_image_free(data);

	// ustawienie uniformu tekstury przy użyciu klasy Shader
	textureShader.use();
	glActiveTexture(GL_TEXTURE0);
	glBindTexture(GL_TEXTURE_2D, texture);
	glUniform1i(glGetUniformLocation(textureShader.ID, "texture1"), 0);


    texturedCube.setPosition(glm::vec3(-2.0f,0.0f,0.0f));
	
    texturedCube.setScale(glm::vec3(0.7f));
    coloredCube.setPosition(glm::vec3(0.0f,0.0f,0.0f));
    coloredCube.setScale(glm::vec3(0.7f));
    sphere.setPosition(glm::vec3(1.5f,0.0f,0.0f));
    sphere.setScale(glm::vec3(0.5f));

	while(!glfwWindowShouldClose(window))
	{
		float time = glfwGetTime();
		glClearColor(0.1f,0.1f,0.1f,1.0f);
		glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);

		glm::mat4 view = glm::translate(glm::mat4(1.0f), glm::vec3(0.0f,0.0f,-5.0f));
		glm::mat4 projection = glm::perspective(glm::radians(45.0f), 800.0f/600.0f, 0.1f, 100.0f);

		// rotacje
		coloredCube.setRotation(glm::vec3(time,time*0.5f,0.0f));
		sphere.setRotation(glm::vec3(time,time*0.5f,0.0f));
		texturedCube.setRotation(glm::vec3(time,time,0.0f));

		// rysowanie kolorowych obiektów
		colorShader.use();
		coloredCube.draw(view,projection);
		sphere.draw(view,projection);

		// rysowanie teksturowanego sześcianu
		textureShader.use();
		glActiveTexture(GL_TEXTURE0);
		glBindTexture(GL_TEXTURE_2D, texture);
		glUniform1i(glGetUniformLocation(textureShader.ID, "texture1"), 0);
		texturedCube.draw(view,projection);

		glfwSwapBuffers(window);
		glfwPollEvents();
	}


    glfwTerminate();
    return 0;
}





