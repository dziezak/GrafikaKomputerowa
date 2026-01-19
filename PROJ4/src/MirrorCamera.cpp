#include "MirrorCamera.h"
#include <glm/gtc/matrix_transform.hpp>

MirrorCamera::MirrorCamera(const glm::vec3& pos, const glm::vec3& normal)
    : mirrorPos(pos), mirrorNormal(glm::normalize(normal))
{
}

glm::mat4 MirrorCamera::computeReflectedView(
    const glm::vec3& camPos,
    const glm::vec3& camTarget,
    const glm::vec3& camUp
) const
{
    glm::vec3 N = glm::normalize(mirrorNormal);

    auto reflectVec = [&](const glm::vec3& v)
    {
        return v - 2.0f * glm::dot(v, N) * N;
    };

    // 1) odbij pozycję i target
    glm::vec3 reflectedPos    = reflectVec(camPos - mirrorPos) + mirrorPos;
    glm::vec3 reflectedTarget = reflectVec(camTarget - mirrorPos) + mirrorPos;

    // 2) odbij wektor up (bardzo ważne!)
    glm::vec3 reflectedUp = reflectVec(camUp);

    return glm::lookAt(
        reflectedPos,
        reflectedTarget,
        reflectedUp
    );
}




glm::vec3 MirrorCamera::reflectPoint(const glm::vec3& p) const
{
    glm::vec3 v = p - mirrorPos;
    return p - 2.0f * glm::dot(v, mirrorNormal) * mirrorNormal;
}

