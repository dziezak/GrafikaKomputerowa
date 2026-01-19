#version 330 core
in vec3 FragPosCam;
in vec3 NormalCam;
out vec4 FragColor;

// PRZEDNI REFLEKTOR
uniform vec3 spotLightPos;   // camera space
uniform vec3 spotLightDir;   // camera space
uniform float cutOff;
uniform float outerCutOff;
uniform vec3 spotLightColor;

// TYLNY REFLEKTOR
uniform vec3 backLightPos;   // camera space
uniform vec3 backLightDir;   // camera space
uniform float backCutOff;
uniform float backOuterCutOff;
uniform vec3 backLightColor;

uniform vec3 ambientLight;
uniform vec3 topLightPos;    // camera space
uniform vec3 topLightColor;

uniform vec3 objectColor;

// ⭐ PHONG / BLINN
uniform bool useBlinn;

void main()
{
    vec3 norm = normalize(NormalCam);
    vec3 viewDir = normalize(-FragPosCam);

    vec3 result = ambientLight;

    // ===== PRZEDNI REFLEKTOR =====
    vec3 frontDir = normalize(FragPosCam - spotLightPos);
    float frontTheta = dot(normalize(spotLightDir), -frontDir);
    float frontEps = cutOff - outerCutOff;
    float frontIntensity = clamp((frontTheta - outerCutOff) / frontEps, 0.0, 1.0);

    float frontDiff = max(dot(norm, -frontDir), 0.0);
    vec3 frontDiffuse = frontDiff * spotLightColor * frontIntensity;

    float frontSpec;
    if (useBlinn) {
        vec3 halfwayDir = normalize((-frontDir) + viewDir);
        frontSpec = pow(max(dot(norm, halfwayDir), 0.0), 32.0 * 4.0);
    } else {
        vec3 frontReflect = reflect(frontDir, norm);
        frontSpec = pow(max(dot(viewDir, frontReflect), 0.0), 32.0);
    }
    vec3 frontSpecular = frontSpec * spotLightColor * frontIntensity;

    // ===== TYLNY REFLEKTOR =====
    vec3 backDir = normalize(FragPosCam - backLightPos);
    float backTheta = dot(normalize(backLightDir), -backDir);
    float backEps = backCutOff - backOuterCutOff;
    float backIntensity = clamp((backTheta - backOuterCutOff) / backEps, 0.0, 1.0);

    float backDiff = max(dot(norm, -backDir), 0.0);
    vec3 backDiffuse = backDiff * backLightColor * backIntensity;

    float backSpec;
    if (useBlinn) {
        vec3 halfwayDir = normalize((-backDir) + viewDir);
        backSpec = pow(max(dot(norm, halfwayDir), 0.0), 32.0 * 4.0);
    } else {
        vec3 backReflect = reflect(backDir, norm);
        backSpec = pow(max(dot(viewDir, backReflect), 0.0), 32.0);
    }
    vec3 backSpecular = backSpec * backLightColor * backIntensity;

    // ===== ŚWIATŁO Z GÓRY =====
    vec3 topDir = normalize(topLightPos - FragPosCam);
    float topDiff = max(dot(norm, topDir), 0.0);
    vec3 topDiffuse = topDiff * topLightColor;

    // ===== SUMA =====
    result += (frontDiffuse + frontSpecular +
               backDiffuse + backSpecular +
               topDiffuse) * objectColor;

    FragColor = vec4(result, 1.0);
}
