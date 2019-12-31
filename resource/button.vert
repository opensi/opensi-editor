uniform vec2 u_size;
uniform bool u_hover;
uniform bool u_press;

varying float v_gradient;
varying vec2 v_pos;

void main() {
    gl_Position = gl_ModelViewProjectionMatrix * gl_Vertex;
    gl_TexCoord[0] = gl_TextureMatrix[0] * gl_MultiTexCoord0;
    gl_FrontColor = gl_Color;

    v_gradient = mix(0.0, 0.3, float(u_hover || u_press)) 
                 + mix(gl_TexCoord[0].y, 1.0, float(u_press)) * 0.5;
    v_pos = gl_TexCoord[0].xy * u_size;
}
