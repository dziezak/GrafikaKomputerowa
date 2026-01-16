#version 330 core
in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 spotLightPos;
uniform vec3 spotLightDir;
uniform float cutOff;
uniform float outerCutOff;
uniform vec3 spotLightColor;

uniform vec3 ambientLight;
uniform vec3 topLightPos;
uniform vec3 topLightColor;

uniform vec3 objectColor;
uniform vec3 viewPos;   // <-- DODANE

void main()
{
    vec3 norm = normalize(Normal);

    // kierunek od światła do fragmentu
    vec3 lightDir = normalize(FragPos - spotLightPos);

    // kąt między osią reflektora a kierunkiem do fragmentu
    float theta = dot(normalize(spotLightDir), lightDir);

    float epsilon = cutOff - outerCutOff;
    float intensity = clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);

    // diffuse od reflektora
    float diff = max(dot(norm, -lightDir), 0.0);
    vec3 diffuse = diff * spotLightColor * intensity;

    // światło z góry
    vec3 topDir = normalize(topLightPos - FragPos);
    float topDiff = max(dot(norm, topDir), 0.0);
    vec3 topDiffuse = topDiff * topLightColor;

    // specular (Phong) od reflektora
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
    vec3 specular = spec * spotLightColor * intensity;

    vec3 result = ambientLight + (diffuse + topDiffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
}
