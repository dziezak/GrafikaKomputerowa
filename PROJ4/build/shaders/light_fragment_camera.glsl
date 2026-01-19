#version 330 core
in vec3 FragPosCam;
in vec3 NormalCam;
out vec4 FragColor;

// ================= SŁOŃCE =================
uniform vec3 lightPos;      // już w camera space
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform float shininess;

// ================= REFLEKTORY =================
uniform vec3 spotLightPos;  // camera space
uniform vec3 spotLightDir;  // camera space
uniform vec3 spotLightColor;
uniform float cutOff;
uniform float outerCutOff;

uniform vec3 backLightPos;  // camera space
uniform vec3 backLightDir;  // camera space
uniform vec3 backLightColor;
uniform float backCutOff;
uniform float backOuterCutOff;

// ================= DODATKOWE ŚWIATŁO =================
uniform vec3 ambientLight;
uniform vec3 topLightPos;   // camera space
uniform vec3 topLightColor;

// ================= Mgła =================
uniform float fogDensity;
uniform vec3 fogColor;

void main()
{
    vec3 norm = normalize(NormalCam);

    // kamera w układzie kamery jest w (0,0,0)
    vec3 viewDir = normalize(-FragPosCam);

    // ===== AMBIENT =====
    vec3 ambient = ambientLight;

    // ===== ŚWIATŁO SŁOŃCA =====
    vec3 sunDir = normalize(lightPos - FragPosCam);
    float diff = max(dot(norm, sunDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 reflectDir = reflect(-sunDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = spec * lightColor;

    // ===== REFLEKTOR PRZEDNI =====
    vec3 spotDir = normalize(FragPosCam - spotLightPos);
    float theta = dot(normalize(-spotLightDir), spotDir);
    float epsilon = cutOff - outerCutOff;
    float intensity = clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);

    float spotDiff = max(dot(norm, -spotDir), 0.0);
    vec3 spotDiffuse = spotDiff * spotLightColor * intensity;
    vec3 spotSpecular = pow(max(dot(viewDir, reflect(spotDir, norm)), 0.0), shininess) * spotLightColor * intensity;

    // ===== REFLEKTOR TYLNY =====
    vec3 backDir = normalize(FragPosCam - backLightPos);
    float backTheta = dot(normalize(-backLightDir), backDir);
    float backEps = backCutOff - backOuterCutOff;
    float backIntensity = clamp((backTheta - backOuterCutOff) / backEps, 0.0, 1.0);

    float backDiff = max(dot(norm, -backDir), 0.0);
    vec3 backDiffuse = backDiff * backLightColor * backIntensity;
    vec3 backSpecular = pow(max(dot(viewDir, reflect(backDir, norm)), 0.0), shininess) * backLightColor * backIntensity;

    // ===== DODATKOWE ŚWIATŁO Z GÓRY =====
    vec3 topDir = normalize(topLightPos - FragPosCam);
    float topDiff = max(dot(norm, topDir), 0.0);
    vec3 topDiffuse = topDiff * topLightColor;

    // ============================================================
    // ROZDZIELENIE ŚWIATEŁ
    // ============================================================

    vec3 globalLight =
        ambient +
        diffuse +
        specular +
        topDiffuse;

    vec3 spotLight =
        spotDiffuse +
        spotSpecular +
        backDiffuse +
        backSpecular;

    vec3 baseColor = globalLight * objectColor;
    vec3 spotColor = spotLight * objectColor;

    // ============================================================
    // MGŁA TYLKO NA GLOBALNE ŚWIATŁO
    // ============================================================

    float depth = gl_FragCoord.z / gl_FragCoord.w;
    float fogFactor = clamp(exp(-depth * fogDensity), 0.0, 1.0);

    vec3 foggedBase = mix(fogColor, baseColor, fogFactor);

    // ============================================================
    // FINALNY KOLOR
    // ============================================================

    vec3 finalColor = foggedBase + spotColor;

    FragColor = vec4(finalColor, 1.0);
}
