#include "MirrorSurface.h"

MirrorSurface::MirrorSurface(Shader* shader)
    : Object3D(
        new float[24]{
            -5.0f, -5.0f, 0.0f,   0.0f, 0.0f, 1.0f,
             5.0f, -5.0f, 0.0f,   0.0f, 0.0f, 1.0f,
            -5.0f,  5.0f, 0.0f,   0.0f, 0.0f, 1.0f,
             5.0f,  5.0f, 0.0f,   0.0f, 0.0f, 1.0f
        },
        24, 
        new unsigned int[6]{ 0,1,2, 1,3,2 },
        6,
        shader,
        false 
    )
{
}
