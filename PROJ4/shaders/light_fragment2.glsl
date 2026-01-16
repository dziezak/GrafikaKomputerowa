#version 330 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;

// ===== ŚWIATŁO SŁOŃCA =====
uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform float shininess;

// ===== REFLEKTOR STATKU =====
uniform vec3 spotLightPos;
uniform vec3 spotLightDir;
uniform vec3 spotLightColor;
uniform float cutOff;
uniform float outerCutOff;

void main()
{
    vec3 norm = normalize(Normal);

    // ================= AMBIENT =================
    vec3 ambient = 0.15 * lightColor;

    // ================= DIFFUSE (SŁOŃCE) =================
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // ================= SPECULAR (SŁOŃCE) =================
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = 0.5 * spec * lightColor;

    // ================= REFLEKTOR (STATEK) =================
    vec3 spotDir = normalize(FragPos - spotLightPos);
    float theta = dot(normalize(-spotLightDir), spotDir);

    float epsilon = cutOff - outerCutOff;
    float intensity = clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);

    float spotDiff = max(dot(norm, -spotDir), 0.0);
    vec3 spotDiffuse = spotDiff * spotLightColor * intensity;

    // ================= WYNIK KOŃCOWY =================
    vec3 result =
        ambient +
        diffuse +
        specular +
        spotDiffuse;

    FragColor = vec4(result * objectColor, 1.0);
}
