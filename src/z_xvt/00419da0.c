/* SPDX-License-Identifier: MIT */
/* z_xvt 00419da0 */
extern float _DAT_005200ac;
float fabsf(float);

int __cdecl FUN_00419da0(float *param_1, float *param_2, float *param_3, float *param_4, float *param_5)
{
    float local_8;
    float local_4;
    float fVar2;
    int bVar1;

    local_8 = (*param_3 - *param_2) * *param_1 + (param_3[1] - param_2[1]) * param_1[1] +
              (param_3[2] - param_2[2]) * param_1[2];
    local_4 = (param_4[2] - param_2[2]) * param_1[2] +
              (*param_4 - *param_2) * *param_1 + (param_4[1] - param_2[1]) * param_1[1];

    if (((int)local_8 < 0x41200000) && ((unsigned int)local_8 < 0xc1200000)) {
        local_8 = 0.0f;
    }
    if (((int)local_4 < 0x41200000) && ((unsigned int)local_4 < 0xc1200000)) {
        local_4 = 0.0f;
    }

    if (fabsf(local_8) == 0.0f) {
        *param_5 = 0.0f;
        return 1;
    }
    if (fabsf(local_4) == 0.0f) {
        *param_5 = 1.0f;
        return 1;
    }

    if ((0x80000000 < (unsigned int)local_8) && (0 < (int)local_4)) {
        fVar2 = local_8 / (local_4 - local_8);
        *param_5 = local_8;
        bVar1 = fVar2 < _DAT_005200ac;
        *param_5 = fVar2;
        if (bVar1) {
            *param_5 = -*param_5;
        }
        return 1;
    }
    if ((0x80000000 < (unsigned int)local_4) && (0 < (int)local_8)) {
        fVar2 = local_8 / (local_8 - local_4);
        *param_5 = local_8;
        bVar1 = fVar2 < _DAT_005200ac;
        *param_5 = fVar2;
        if (bVar1) {
            *param_5 = -*param_5;
        }
        return 1;
    }
    return 0;
}
