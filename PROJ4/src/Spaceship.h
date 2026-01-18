#ifndef SPACESHIP_H
#define SPACESHIP_H

#include <glm/glm.hpp>
#include "Object3D.h"

class Spaceship {
public:
    Object3D* object;

    float moveSpeed;
    float rotationSpeed;

    Spaceship(Object3D* obj);

    void update(bool* keys, float deltaTime);

    glm::vec3 getForward() const;
    glm::vec3 getPosition() const;
    glm::vec3 getRotation() const;

    void setPosition(const glm::vec3& pos);
    void setRotation(const glm::vec3& rot);
    void setScale(const glm::vec3& s);
};

#endif
