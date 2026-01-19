#include "Mirror.h"
#include <glad/glad.h>

Mirror::Mirror(float width, float height, Shader* shader)
    : Object3D(
        new float[20]{
            -width/2, -height/2, 0,   0, 0,
             width/2, -height/2, 0,   1, 0,
             width/2,  height/2, 0,   1, 1,
            -width/2,  height/2, 0,   0, 1
        },
        20,
        new unsigned int[6]{ 0,1,2, 2,3,0 },
        6,
        shader,
        true
    )
{}


void Mirror::draw(const glm::mat4& view,
                  const glm::mat4& projection,
                  unsigned int reflectionTexture)
{
    shader->use();

    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, reflectionTexture);
    shader->setInt("reflectionTex", 0);

    Object3D::draw(view, projection);
}
