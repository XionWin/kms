#version 100
uniform vec2 uViewSize;

attribute vec2 aVertex;
attribute vec2 aCoord;

varying vec2 vCoord;
void main()
{
    gl_Position = vec4(2.0 * aVertex.x / uViewSize.x - 1.0, 1.0 - 2.0 * aVertex.y / uViewSize.y, 0.0, 1.0);
    vCoord = aCoord;
    gl_PointSize = 10.0;
}
