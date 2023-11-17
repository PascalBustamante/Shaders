#version 450

// Input texture coordinates
//layout(location = 0) in vec2 TexCoord;
in vec2 TexCoord;

// Output color
out vec4 FragColor;

// Input texture
uniform sampler2D inputTexture;

void main() {
    // Sample the texture using the provided texture coordinates
    FragColor = texture(inputTexture, TexCoord);
    //FragColor = vec4(TexCoord, 0.0, 1.0);
    //FragColor = vec4(vec3(TexCoord, 0.0), 1.0);

}