#pragma once

#define MATERIAL_RET bool
#define MATERIAL_RET_BAD false

#define MATERIAL_ARGS_DEF \
    uint *seed, \
    Ray ray, \
    float3 pos, float3 norm, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    Ray *new_ray, float3 *color

#define MATERIAL_ARGS \
    seed, ray, pos, norm, ibuf, fbuf, new_ray, color

#define MATERIAL_ARGS_B(di, df) \
    seed, ray, pos, norm, ibuf + (di), fbuf + (df), new_ray, color
