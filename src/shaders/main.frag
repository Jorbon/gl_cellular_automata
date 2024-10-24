#version 150

in vec2 screen_position;
out vec4 color;

uniform int sim_size;
uniform sampler2D state;



void main() {
	
	const int KERNEL_RADIUS = 15;
	mat3 sum = mat3(0.0);
	for (int x = -KERNEL_RADIUS + 1; x < KERNEL_RADIUS; x++) for (int y = -KERNEL_RADIUS + 1; y < KERNEL_RADIUS; y++) {
		int d2 = x*x + y*y;
		if (d2 >= KERNEL_RADIUS * KERNEL_RADIUS) continue;
		
		vec3 previous_value = texelFetch(state, (ivec2(gl_FragCoord.xy) + ivec2(x, y)) & (sim_size - 1), 0).rgb;
		
		float d = sqrt(d2) / KERNEL_RADIUS;
		float d_weight_donut = exp(4 - 1 / (d * (1 - d)));
		float d_weight_circle = exp(1 - 1 / (1 - d));
		mat3 w = mat3(
			vec3(d_weight_donut, d_weight_circle * previous_value.g, d_weight_circle),
			vec3(d_weight_circle, d_weight_donut, d_weight_circle * previous_value.b),
			vec3(d_weight_circle * previous_value.r, d_weight_circle, d_weight_donut)
		);
		
		sum += mat3(
			w[0] * previous_value,
			w[1] * previous_value,
			w[2] * previous_value
		);
	}
	
	const float DONUT_SCALE = 0.00368589622711;
	const float CIRCLE_SCALE = 0.00670612723144;
	
	sum = matrixCompMult(sum, mat3(
		vec3(DONUT_SCALE, CIRCLE_SCALE, CIRCLE_SCALE),
		vec3(CIRCLE_SCALE, DONUT_SCALE, CIRCLE_SCALE),
		vec3(CIRCLE_SCALE, CIRCLE_SCALE, DONUT_SCALE)
	));
	
	const float dt = 0.004;
	
	// const float NONE = 1.177410022516;
	
	const float own_center = 0.12;
	
	const mat3 center = mat3(
		vec3(own_center, 1.0, 0.0),
		vec3(0.0, own_center, 1.0),
		vec3(1.0, 0.0, own_center)
	);
	
	const float own_spread = 0.015;
	const float pred_spread = 0.7;
	const float grow_spread = 0.7;
	
	const mat3 spread = mat3(
		vec3(own_spread, grow_spread, pred_spread),
		vec3(pred_spread, own_spread, grow_spread),
		vec3(grow_spread, pred_spread, own_spread)
	);
	
	mat3 a = (sum - center) / spread;
	
	mat3 b = matrixCompMult(a, a);
	
	const float predation = 0.7;
	const float growth = 0.7;
	
	const mat3 dvdt_min = mat3(
		vec3(-1.0, 0.0, -predation),
		vec3(-predation, -1.0, 0.0),
		vec3(0.0, -predation, -1.0)
	);
	const mat3 dvdt_max = mat3(
		vec3(1.0, growth, 0.0),
		vec3(0.0, 1.0, growth),
		vec3(growth, 0.0, 1.0)
	);
	
	vec3 dvdt0 = clamp(2.0 * exp(-0.5 * b[0]) - 1.0, dvdt_min[0], dvdt_max[0]);
	vec3 dvdt1 = clamp(2.0 * exp(-0.5 * b[1]) - 1.0, dvdt_min[1], dvdt_max[1]);
	vec3 dvdt2 = clamp(2.0 * exp(-0.5 * b[2]) - 1.0, dvdt_min[2], dvdt_max[2]);
	
	color = vec4(clamp(texelFetch(state, ivec2(gl_FragCoord.xy), 0).rgb + dt * (dvdt0 + dvdt1 + dvdt2), 0.0, 1.0), 1.0);
	
}

