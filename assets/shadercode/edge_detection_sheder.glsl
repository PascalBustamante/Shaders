#version 450

// Input texture
layout(binding = 0) uniform sampler2D inputTexture;

// Output image
layout(binding = 1) uniform writeonly image2D outputImage;

layout(local_size_x = 16, local_size_y = 16) in;

void main() {
   // Compute the pixel coordinates
   ivec2 pixelCoords = ivec2(gl_GlobalInvocationID.xy);

   // Sample the input texture
   vec4 color = texelFetch(inputTexture, pixelCoords, 0);

   // Compute the edge detection
   float edge = color.r + color.g + color.b > 1.5 ? 1.0 : 0.0;

   // Write the result to the output image
   imageStore(outputImage, pixelCoords, vec4(vec3(edge), 1.0));
}