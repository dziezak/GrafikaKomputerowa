#include "Spaceship.h"
#include <GLFW/glfw3.h>
#include <glm/gtc/matrix_transform.hpp>

Spaceship::Spaceship(Object3D* obj)
    : object(obj),
      moveSpeed(5.0f),
      rotationSpeed(glm::radians(90.0f))
{
}

void Spaceship::update(bool* keys, float deltaTime)
{
    glm::vec3 rot = object->getRotation();

    // --- obrót ---
    if (keys[GLFW_KEY_A]) rot.y += rotationSpeed * deltaTime;
    if (keys[GLFW_KEY_D]) rot.y -= rotationSpeed * deltaTime;

    object->setRotation(rot);

    // --- kierunek ruchu ---
    glm::vec3 forward = getForward();

    // --- ruch ---
    glm::vec3 pos = object->getPosition();
    if (keys[GLFW_KEY_W]) pos += forward * moveSpeed * deltaTime;
    if (keys[GLFW_KEY_S]) pos -= forward * moveSpeed * deltaTime;

    object->setPosition(pos);
}

glm::vec3 Spaceship::getForward() const
{
    float yaw = object->getRotation().y;

    glm::vec3 forward;
    forward.x = -sinf(yaw);
    forward.y = 0.0f;
    forward.z = -cosf(yaw);

    return glm::normalize(forward);
}

glm::vec3 Spaceship::getPosition() const
{
    return object->getPosition();
}

glm::vec3 Spaceship::getRotation() const
{
    return object->getRotation();
}

void Spaceship::setPosition(const glm::vec3& pos)
{
    object->setPosition(pos);
}

void Spaceship::setRotation(const glm::vec3& rot)
{
    object->setRotation(rot);
}

void Spaceship::setScale(const glm::vec3& s)
{
    object->setScale(s);
}
