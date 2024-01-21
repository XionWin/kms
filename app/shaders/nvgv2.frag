#version 100
precision mediump float;

varying vec4 vColor;
void main()
{
    gl_FragColor = vColor; //vec4(1.0, 1.0, 1.0, 1.0);
}