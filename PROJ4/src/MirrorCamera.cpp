#include "MirrorCamera.h"
#include <glm/gtc/matrix_transform.hpp>

MirrorCamera::MirrorCamera(const glm::vec3& pos, const glm::vec3& normal)
    : mirrorPos(pos), mirrorNormal(glm::normalize(normal))
{
}

glm::mat4 MirrorCamera::computeReflectedView(const glm::vec3& camPos,
                                             const glm::vec3& camTarget) const
{
    // odbicie pozycji
    glm::vec3 camToMirror = camPos - mirrorPos;
    glm::vec3 reflectedPos =
        camPos - 2.0f * glm::dot(camToMirror, mirrorNormal) * mirrorNormal;

    // odbicie kierunku patrzenia
    glm::vec3 dir = camTarget - camPos;
    glm::vec3 reflectedDir =
        dir - 2.0f * glm::dot(dir, mirrorNormal) * mirrorNormal;

    glm::vec3 reflectedTarget = reflectedPos + reflectedDir;

    return glm::lookAt(reflectedPos, reflectedTarget, glm::vec3(0,1,0));
}
