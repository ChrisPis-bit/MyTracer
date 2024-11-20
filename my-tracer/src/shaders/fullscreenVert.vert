#version 330 core

layout(location = 0) in vec3 aPos; // Vertex position
//layout(location = 1) in vec2 aTexCoord; // UV coordinates

out vec2 TexCoord; // Pass UV to fragment shader

void main()
{
    gl_Position = vec4(aPos, 1.0); // Transform position to clip space
    TexCoord = gl_Position.xy / 2 + vec2(.5,.5); // Pass UV coordinates
}