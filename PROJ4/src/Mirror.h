#pragma once
#include "Object3D.h"

class Mirror : public Object3D {
public:
    Mirror(float width, float height, Shader* shader);

    void draw(const glm::mat4& view,
              const glm::mat4& projection,
              unsigned int reflectionTexture);
};
