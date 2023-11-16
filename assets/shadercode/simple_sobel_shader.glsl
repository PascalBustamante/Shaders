#version 450

// Input texture coordinates
layout(location = 0) in vec2 TexCoord;

// Output color
out vec4 FragColor;

// Input texture
uniform sampler2D inputTexture;

// Function to perform convolution with a given kernel
vec3 convolution(sampler2D texture, vec2 texCoord, mat3 kernel) {
    vec3 sum = vec3(0.0);
    for (int i = -1; i <= 1; ++i) {
        for (int j = -1; j <= 1; ++j) {
            // Sample neighboring pixel and apply the convolution kernel
            sum += texture2D(inputTexture, texCoord + vec2(i, j)).rgb * kernel[i + 1][j + 1];
        }
    }
    return sum;
}

void main() {
    // Define Sobel operators for both x and y directions
    mat3 sobelX = mat3(-1, 0, 1, -2, 0, 2, -1, 0, 1);
    mat3 sobelY = mat3(-1, -2, -1, 0, 0, 0, 1, 2, 1);

    // Apply convolution for both x and y directions
    vec3 gradientX = convolution(inputTexture, TexCoord, sobelX);
    vec3 gradientY = convolution(inputTexture, TexCoord, sobelY);

    // Combine the x and y gradients to compute the overall gradient magnitude
    vec3 gradient = sqrt(gradientX * gradientX + gradientY * gradientY);

    // Output the result
    FragColor = vec4(gradient, 1.0);
}
