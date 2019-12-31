uniform vec4 u_select_color;
varying float v_gradient;

void main() {
    vec4 bg_color = vec4(u_select_color.rgb, v_gradient);

    gl_FragColor = bg_color;
}
