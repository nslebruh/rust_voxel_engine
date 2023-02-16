#version 330 core

out vec4 Color;

in vec2 TexCoord;

uniform vec4 ourColor;
uniform sampler2D ourTexture;

//float fog_maxdist = 8.0;
//float fog_mindist = 0.1;
//vec4  fog_colour = vec4(0.4, 0.4, 0.4, 1.0);
//
//float dist = length()

void main()
{
    Color = texture(ourTexture, TexCoord);
}