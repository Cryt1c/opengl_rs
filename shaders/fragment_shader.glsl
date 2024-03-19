#version 330 core

out vec4 FragColor;

void main() {
    // Normalize screen coordinates to range from 0 to 1
    float x = gl_FragCoord.x / 800.0; // Assuming a window width of 800 pixels
    float y = gl_FragCoord.y / 600.0; // Assuming a window height of 600 pixels
    
    // Use x and y coordinates to generate a color
    FragColor = vec4(x, y, 0.5, 1.0); // Combines x and y for red and green, constant blue, full alpha
}
