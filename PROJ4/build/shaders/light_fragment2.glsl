#version 330 core
in vec3 FragPos;
in vec3 Normal;
out vec4 FragColor;

// ================= SŁOŃCE =================
uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform vec3 viewPos;
uniform float shininess;

// ================= REFLEKTORY =================
uniform vec3 spotLightPos;
uniform vec3 spotLightDir;
uniform vec3 spotLightColor;
uniform float cutOff;
uniform float outerCutOff;

uniform vec3 backLightPos;
uniform vec3 backLightDir;
uniform vec3 backLightColor;
uniform float backCutOff;
uniform float backOuterCutOff;

// ================= DODATKOWE ŚWIATŁO =================
uniform vec3 ambientLight; // np. 0.2,0.2,0.2
uniform vec3 topLightPos;
uniform vec3 topLightColor;

void main()
{
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    // ===== AMBIENT =====
    vec3 ambient = ambientLight;

    // ===== ŚWIATŁO SŁOŃCA =====
    vec3 sunDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, sunDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 reflectDir = reflect(-sunDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = spec * lightColor;

    // ===== REFLEKTOR PRZEDNI =====
    vec3 spotDir = normalize(FragPos - spotLightPos);
    float theta = dot(normalize(-spotLightDir), spotDir);
    float epsilon = cutOff - outerCutOff;
    float intensity = clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);

    float spotDiff = max(dot(norm, -spotDir), 0.0);
    vec3 spotDiffuse = spotDiff * spotLightColor * intensity;
    vec3 spotSpecular = pow(max(dot(viewDir, reflect(spotDir, norm)), 0.0), shininess) * spotLightColor * intensity;

    // ===== REFLEKTOR TYLNY =====
    vec3 backDir = normalize(FragPos - backLightPos);
    float backTheta = dot(normalize(-backLightDir), backDir);
    float backEps = backCutOff - backOuterCutOff;
    float backIntensity = clamp((backTheta - backOuterCutOff) / backEps, 0.0, 1.0);

    float backDiff = max(dot(norm, -backDir), 0.0);
    vec3 backDiffuse = backDiff * backLightColor * backIntensity;
    vec3 backSpecular = pow(max(dot(viewDir, reflect(backDir, norm)), 0.0), shininess) * backLightColor * backIntensity;

    // ===== DODATKOWE ŚWIATŁO Z GÓRY =====
    vec3 topDir = normalize(topLightPos - FragPos);
    float topDiff = max(dot(norm, topDir), 0.0);
    vec3 topDiffuse = topDiff * topLightColor;

    // ===== WYNIK KOŃCOWY =====
    vec3 result = ambient + (diffuse + specular +
                             spotDiffuse + spotSpecular +
                             backDiffuse + backSpecular +
                             topDiffuse) * objectColor;

    FragColor = vec4(result, 1.0);
}
