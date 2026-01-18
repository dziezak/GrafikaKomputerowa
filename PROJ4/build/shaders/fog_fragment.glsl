#version 330 core
in vec3 FragPos;
out vec4 FragColor;

uniform vec3 fogColor;
uniform float fogDensity;

void main()
{
    float alpha = fogDensity;  // im większe fogDensity, tym mniej przezroczyste
    FragColor = vec4(fogColor, alpha);
}
