#version 150

in vec2 position;
out vec2 screen_position;

void main() {
	screen_position = (position + 1.0) * 0.5;
	gl_Position = vec4(position, 0.0, 1.0);
}
