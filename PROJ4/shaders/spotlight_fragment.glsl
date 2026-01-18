#version 330 core
in vec3 FragPos;
in vec3 Normal;
out vec4 FragColor;

// PRZEDNI REFLEKTOR
uniform vec3 spotLightPos;
uniform vec3 spotLightDir;
uniform float cutOff;
uniform float outerCutOff;
uniform vec3 spotLightColor;

// TYLNY REFLEKTOR
uniform vec3 backLightPos;
uniform vec3 backLightDir;
uniform float backCutOff;
uniform float backOuterCutOff;
uniform vec3 backLightColor;

uniform vec3 ambientLight;
uniform vec3 topLightPos;
uniform vec3 topLightColor;

uniform vec3 objectColor;
uniform vec3 viewPos;

void main()
{
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    // ===== Ambient =====
    vec3 result = ambientLight;

    // ===== PRZEDNI REFLEKTOR =====
    vec3 frontDir = normalize(FragPos - spotLightPos);          // kierunek z fragmentu do światła
    float frontTheta = dot(normalize(spotLightDir), -frontDir); // dot z osią reflektora
    float frontEps = cutOff - outerCutOff;
    float frontIntensity = clamp((frontTheta - outerCutOff) / frontEps, 0.0, 1.0);

    float frontDiff = max(dot(norm, -frontDir), 0.0);
    vec3 frontDiffuse = frontDiff * spotLightColor * frontIntensity;

    vec3 frontReflect = reflect(frontDir, norm);
    float frontSpec = pow(max(dot(viewDir, frontReflect), 0.0), 32.0);
    vec3 frontSpecular = frontSpec * spotLightColor * frontIntensity;

    // ===== TYLNY REFLEKTOR =====
    vec3 backDir = normalize(FragPos - backLightPos);            // kierunek z fragmentu do tylnego światła
    float backTheta = dot(normalize(backLightDir), -backDir);    // dot z osią reflektora
    float backEps = backCutOff - backOuterCutOff;
    float backIntensity = clamp((backTheta - backOuterCutOff) / backEps, 0.0, 1.0);

    float backDiff = max(dot(norm, -backDir), 0.0);
    vec3 backDiffuse = backDiff * backLightColor * backIntensity;

    vec3 backReflect = reflect(backDir, norm);
    float backSpec = pow(max(dot(viewDir, backReflect), 0.0), 32.0);
    vec3 backSpecular = backSpec * backLightColor * backIntensity;

    // ===== Światło z góry =====
    vec3 topDir = normalize(topLightPos - FragPos);
    float topDiff = max(dot(norm, topDir), 0.0);
    vec3 topDiffuse = topDiff * topLightColor;

    // ===== SUMA =====
    result += (frontDiffuse + frontSpecular +
               backDiffuse + backSpecular +
               topDiffuse) * objectColor;

    FragColor = vec4(result, 1.0);
}
