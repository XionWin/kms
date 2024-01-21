#version 100
uniform vec2 uViewSize;

attribute vec2 aVertex;
attribute vec2 aCoord;

varying vec4 vColor;
void main()
{
    gl_Position = vec4(2.0 * aVertex.x / uViewSize.x - 1.0, 1.0 - 2.0 * aVertex.y / uViewSize.y, 0.0, 1.0);
    vColor = vec4(aCoord.x, aCoord.y, 0.0, 1.0);
    gl_PointSize = 10.0;
}
