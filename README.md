# Chris' Path Tracer
Welcome to the github page of my simple path tracer. After following a course on advanced graphics during my masters, I decided to use what I learned to put together my own path tracer.
This is also my first time using rust, so the quality of the code might not be the best, but this project allows me to improve my skills in understanding the language. 
The project uses GLFW to generate the window, OpenGL to render the image, and currently the path tracing is done from the CPU (in parallel threads using rayon). Every frame, it traces new rays and accumulates it onto the previously rendered pixels.
![image](https://github.com/user-attachments/assets/ac4bf908-e621-4eea-acb3-033fa143c486)

## Current Features
- Path tracing
- Rendering on CPU
- Multiple primitives in scene (Spheres and Planes)
  - Each primitive can be added as a light to the scene
- Indirect light bounces
- Next event estimation
- Russian roulette

## To-Do
- Different Materials
  - Glossiness
  - Metallic
  - Dielectrics
  - Textures for albedo, bump map, etc.
- Camera movement
- Anti-aliasing
- Depth of field
- WGPU for rendering (Simply want to learn it)
- GUI for adjusting tracing settings
- Import and render models
  - With implementation of BVH acceleration structure
- Rendering on GPU (Compute shader with OpenCL)
- Wavefront path tracing
- Light importance sampling
- Non-homogeneous volumes
