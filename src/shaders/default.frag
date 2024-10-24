#version 150

in vec2 screen_position;
out vec4 color;

uniform sampler2D screen_texture;

void main() {
	color = texture(screen_texture, screen_position.xy);
}
