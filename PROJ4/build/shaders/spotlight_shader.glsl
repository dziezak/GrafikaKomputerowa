#version 330 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;

uniform vec3 spotLightPos;
uniform vec3 spotLightDir;
uniform float cutOff;
uniform float outerCutOff;
uniform vec3 spotLightColor;
uniform vec3 objectColor;
uniform vec3 viewPos;
uniform float shininess;

void main()
{
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(spotLightPos - FragPos);
    vec3 viewDir = normalize(viewPos - FragPos);

    // ----- ambient -----
    vec3 ambient = 0.1 * objectColor;

    // ----- diffuse -----
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * spotLightColor;

    // ----- specular -----
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = 0.5 * spec * spotLightColor;

    // ----- uwzględnienie stożka reflektora -----
    float theta = dot(lightDir, normalize(spotLightDir));
    float epsilon = cutOff - outerCutOff;
    float intensity = clamp((theta - outerCutOff)/epsilon, 0.0, 1.0);

    diffuse *= intensity;
    specular *= intensity;

    vec3 result = ambient + diffuse + specular;
    result *= objectColor;

    FragColor = vec4(result, 1.0);
}
