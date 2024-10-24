#version 150

in vec2 screen_position;
out vec4 color;

uniform int sim_size;
uniform sampler2D state;



void main() {
	
	const int KERNEL_RADIUS = 15;
	vec3 sum = vec3(0.0);
	for (int x = -KERNEL_RADIUS + 1; x < KERNEL_RADIUS; x++) for (int y = -KERNEL_RADIUS + 1; y < KERNEL_RADIUS; y++) {
		int d2 = x*x + y*y;
		if (d2 >= KERNEL_RADIUS * KERNEL_RADIUS) continue;
		
		float d = sqrt(d2) / KERNEL_RADIUS;
		mat3 w = mat3(
			vec3(exp(4 - 1 / (d * (1 - d))), 0.0, 0.0),
			vec3(0.0, exp(4 - 1 / (d * (1 - d))), 0.0),
			vec3(0.0, 0.0, exp(4 - 1 / (d * (1 - d))))
		);
		
		sum += w * texelFetch(state, (ivec2(gl_FragCoord.xy) + ivec2(x, y)) & (sim_size - 1), 0).rgb;
	}
	
	sum *= vec3(
		0.00368590447337,
		0.00368590447337,
		0.00368590447337
	);
	
	const float dt = 0.00403;
	
	const vec3 center = vec3(
		0.12,
		0.1201,
		0.1202
	);
	
	const vec3 spread = vec3(
		0.015,
		0.015,
		0.015
	);
	
	vec3 a = (sum - center) / spread;
	
	vec3 dvdt = 2.0 * exp(-0.5 * a*a) - 1.0;
	
	color = vec4(clamp(texelFetch(state, ivec2(gl_FragCoord.xy), 0).rgb + dt * dvdt, 0.0, 1.0), 1.0);
	
}

