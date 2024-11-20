#version 330 core

in vec2 TexCoord; // Received UV from vertex shader
out vec4 FragColor; // Final fragment color

uniform sampler2D texture0;

void main()
{
    // Use the UV coordinates for the red and green channels, set blue to 0
    FragColor = texture(texture0, TexCoord);
}