#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 FragPosCam;
out vec3 NormalCam;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    // pozycja w przestrzeni kamery
    vec4 posCam = view * model * vec4(aPos, 1.0);
    FragPosCam = posCam.xyz;

    // normalne w przestrzeni kamery
    mat3 normalMatrix = mat3(transpose(inverse(view * model)));
    NormalCam = normalize(normalMatrix * aNormal);

    gl_Position = projection * posCam;
}
