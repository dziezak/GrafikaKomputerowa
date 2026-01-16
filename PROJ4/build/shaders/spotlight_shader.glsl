#version 330 core
in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 spotLightPos;
uniform vec3 spotLightDir;
uniform float cutOff;
uniform float outerCutOff;
uniform vec3 spotLightColor;
uniform vec3 objectColor;
uniform vec3 ambientLight;
uniform vec3 topLightPos;
uniform vec3 topLightColor;
void main()
{
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(spotLightPos - FragPos);

    // kąt stożka
    float theta = dot(lightDir, normalize(spotLightDir));
    float intensity = 0.0;
    if(theta > outerCutOff)
        intensity = clamp((theta - outerCutOff)/(cutOff - outerCutOff), 0.0, 1.0);

    // diffuse
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * spotLightColor * intensity;

    vec3 topDir = normalize(topLightPos - FragPos);
    float topDiff = max(dot(norm, topDir), 0.0);
    vec3 topDiffuse = topDiff * topLightColor;

    vec3 result = ambientLight + (diffuse + topDiffuse) * objectColor;
    FragColor = vec4(result, 1.0);

}
