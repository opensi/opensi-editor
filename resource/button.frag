const float u_thickness = 0.1;

uniform vec4 u_bg_color;
uniform vec4 u_border_color;
uniform vec2 u_size;

varying float v_gradient;
varying vec2 v_pos;

float rounded_rectangle(float size) {
    vec2 center = u_size * 0.5;
    vec2 pos = abs(center - v_pos);
    pos.x = max(pos.x - (center.x - center.y), 0.0);
    return smoothstep(0.95, 1.0, length(pos) / (center.y * size));
}

void main() {
    float inner_a = rounded_rectangle(1.0 - u_thickness);
    float outer_a = 1.0 - rounded_rectangle(1.0);

    vec4 bg_color = vec4(u_bg_color.rgb, v_gradient * outer_a);
    vec4 border_color = u_border_color;

    gl_FragColor = mix(bg_color, border_color, min(inner_a, outer_a));
}
