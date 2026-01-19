#pragma once
#include <glm/glm.hpp>

class MirrorCamera {
public:
    glm::vec3 mirrorPos;
    glm::vec3 mirrorNormal;

    MirrorCamera(const glm::vec3& pos, const glm::vec3& normal);


glm::mat4 computeReflectedView(
    const glm::vec3& camPos,
    const glm::vec3& camTarget,
    const glm::vec3& camUp
) const;
    glm::vec3 reflectPoint(const glm::vec3& p) const;
};
