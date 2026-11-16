/* SPDX-License-Identifier: MIT */
/* z_xvt 00419f10 */
extern float _DAT_005200ac;

unsigned int __cdecl FUN_00419f10(float *param_1, int param_2, int *param_3, unsigned int *param_4)
{
    int iVar1;
    float *pfVar2;
    float fVar3;
    float fVar4;
    float fVar5;
    float fVar6;
    int bVar7;
    int iVar8;
    int iVar9;
    int iVar10;
    float local_14;
    float local_10;
    float local_c;
    float local_8;
    float local_4;
    float f1;
    float f2;
    unsigned int bits;

    local_4 = *param_1;
    local_14 = param_1[1];
    local_10 = param_1[2];

    __builtin_memcpy(&bits, &local_4, 4);
    if (0x80000000u < bits) {
        local_4 = -local_4;
    }
    __builtin_memcpy(&bits, &local_14, 4);
    if (0x80000000u < bits) {
        local_14 = -local_14;
    }
    __builtin_memcpy(&bits, &local_10, 4);
    if (0x80000000u < bits) {
        local_10 = -local_10;
    }

    if ((local_10 < local_14) || (local_10 < local_4)) {
        if ((local_14 < local_4) || (local_14 < local_10)) {
            iVar9 = 1;
            iVar10 = 2;
        }
        else {
            iVar9 = 0;
            iVar10 = 2;
            param_4[1] = *param_4;
        }
    }
    else {
        iVar9 = 0;
        iVar10 = 1;
        param_4[2] = param_4[1];
        param_4[1] = *param_4;
    }

    __builtin_memcpy(&f1, &param_4[1], 4);
    __builtin_memcpy(&f2, &param_4[2], 4);

    iVar1 = (int)((unsigned int)*param_3 * 3u + (unsigned int)iVar9);
    fVar3 = *(float *)(param_2 + (int)((unsigned int)iVar1 * 4u));
    pfVar2 = (float *)(param_2 + (int)((unsigned int)(int)((unsigned int)*param_3 * 3u + (unsigned int)iVar10) * 4u));
    fVar4 = *pfVar2;
    fVar5 = *(float *)(param_2 + (int)((unsigned int)(int)((unsigned int)param_3[1] * 3u + (unsigned int)iVar9) * 4u));
    fVar6 = *(float *)(param_2 + (int)((unsigned int)(int)((unsigned int)param_3[1] * 3u + (unsigned int)iVar10) * 4u));
    bVar7 = ((f1 - fVar3) * (fVar6 - fVar4) - (f2 - fVar4) * (fVar5 - fVar3)) < _DAT_005200ac;

    fVar3 = *(float *)(param_2 + (int)((unsigned int)(int)((unsigned int)param_3[2] * 3u + (unsigned int)iVar9) * 4u));
    fVar4 = *(float *)(param_2 + (int)((unsigned int)(int)((unsigned int)param_3[2] * 3u + (unsigned int)iVar10) * 4u));
    fVar5 = (f1 - fVar5) * (fVar4 - fVar6) - (f2 - fVar6) * (fVar3 - fVar5);

    if ((fVar5 < _DAT_005200ac) && (!bVar7)) {
        return 0;
    }
    __builtin_memcpy(&bits, &fVar5, 4);
    if ((bits < 0x80000001u) && (bVar7)) {
        return 0;
    }

    local_c = fVar4;
    local_8 = fVar3;
    if (param_3[3] != -1) {
        iVar8 = (int)((unsigned int)param_3[3] * 3u);
        local_8 = *(float *)(param_2 + (int)((unsigned int)(int)((unsigned int)iVar9 + (unsigned int)iVar8) * 4u));
        local_c = *(float *)(param_2 + (int)((unsigned int)(int)((unsigned int)iVar8 + (unsigned int)iVar10) * 4u));
        fVar3 = (f1 - fVar3) * (local_c - fVar4) - (f2 - fVar4) * (local_8 - fVar3);
        if ((fVar3 < _DAT_005200ac) && (!bVar7)) {
            return 0;
        }
        __builtin_memcpy(&bits, &fVar3, 4);
        if ((bits < 0x80000001u) && (bVar7)) {
            return 0;
        }
    }

    fVar3 = (f1 - local_8) * (*pfVar2 - local_c) -
            (*(float *)(param_2 + (int)((unsigned int)iVar1 * 4u)) - local_8) * (f2 - local_c);
    if ((fVar3 < _DAT_005200ac) && (!bVar7)) {
        return 0;
    }
    __builtin_memcpy(&bits, &fVar3, 4);
    if ((bits < 0x80000001u) && (bVar7)) {
        return 0;
    }
    return 1;
}
