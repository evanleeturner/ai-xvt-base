/* SPDX-License-Identifier: MIT */
/* z_xvt 004859c0 */
extern float _DAT_005201dc;
extern float *DAT_00602698;
extern float *DAT_00602688;
extern float *DAT_0060267c;
extern int DAT_006a8858;
extern unsigned short DAT_006d4984;
extern unsigned short DAT_007091ae;
extern int DAT_00709ffc;
extern int DAT_00732f3c;
/* DAT_006a885e is at an unaligned address (0x6a885e); accessed via memcpy */
extern unsigned char DAT_006a885e[4];

uint __cdecl FUN_004859c0(int param_1, uint *param_2, float *param_3, float *param_4)
{
    float fVar1;
    float fVar2;
    float fVar3;
    float fVar4;
    float fVar5;
    int iVar6;
    uint uVar7;
    uint uVar8;
    uint uVar9;
    float *pfVar10;
    float *pfVar11;
    uint uBits;
    uint uTmp;

    pfVar10 = param_4;
    __builtin_memcpy(&uBits, &param_4[2], 4);
    if (0x80000000 < uBits) {
        pfVar11 = param_3 + 2;
        pfVar10 = param_3;
        param_3 = param_4;
        __builtin_memcpy(&uBits, pfVar11, 4);
        if (0x80000000 < uBits) {
            return 0xffffffff;
        }
    }
    __builtin_memcpy(&uBits, &param_3[2], 4);
    if (0x80000000 < uBits) {
        DAT_00602698 = DAT_00602688;
        DAT_0060267c = (float *)((uint)DAT_006a8858 +
                                 ((uint)*(int *)(param_1 + 0xbc) + (uint)*(int *)(param_1 + 200)) * 0x18u);
        DAT_00602688 = DAT_0060267c;
        *(int *)(param_1 + 200) = (int)((uint)*(int *)(param_1 + 200) + 1u);
        __builtin_memcpy(&uTmp, DAT_006a885e, 4);
        uTmp = uTmp + 1u;
        __builtin_memcpy(DAT_006a885e, &uTmp, 4);
        fVar4 = _DAT_005201dc / pfVar10[2];
        iVar6 = (int)((uint)(DAT_006d4984 >> 1) + (uint)DAT_00709ffc);
        fVar1 = pfVar10[1];
        fVar2 = param_3[1];
        fVar5 = param_3[2] / ((param_3[2] - (float)DAT_00732f3c * fVar4) + _DAT_005201dc);
        fVar3 = param_3[1];
        *DAT_00602688 =
            (float)DAT_00732f3c *
            (((*pfVar10 - (float)(DAT_007091ae >> 1)) * fVar4 - *param_3) * fVar5 + *param_3);
        DAT_00602688[1] =
            (float)DAT_00732f3c * (((fVar1 - (float)iVar6) * fVar4 - fVar2) * fVar5 + fVar3);
        *DAT_00602688 = (float)(DAT_007091ae >> 1) + *DAT_00602688;
        DAT_00602688[1] = (float)(int)((uint)(DAT_006d4984 >> 1) + (uint)DAT_00709ffc) + DAT_00602688[1];
        DAT_00602688[2] = (float)DAT_00732f3c;
        DAT_00602688[3] = (pfVar10[3] - param_3[3]) * fVar5 + param_3[3];
        param_3 = DAT_00602688;
    }
    __builtin_memcpy(&uBits, &param_3[1], 4);
    if (uBits < 0x80000001) {
        /* compiler emits __ftol (0x005019c8) for this float->int64 truncation */
        uVar9 = (uint)(long long)param_3[1];
        if ((float)(int)uVar9 != param_3[1]) {
            uVar9 = uVar9 + 1;
        }
    } else {
        uVar9 = 0;
    }
    __builtin_memcpy(&uBits, &pfVar10[1], 4);
    if (uBits < 0x80000001) {
        uVar7 = (uint)(long long)pfVar10[1];
        if ((float)(int)uVar7 != pfVar10[1]) {
            uVar7 = uVar7 + 1;
        }
    } else {
        uVar7 = 0;
    }
    if (uVar7 != uVar9) {
        uVar8 = uVar7;
        pfVar11 = pfVar10;
        if ((int)uVar7 < (int)uVar9) {
            uVar8 = uVar9;
            uVar9 = uVar7;
            pfVar11 = param_3;
            param_3 = pfVar10;
        }
        if (0 < (int)uVar8) {
            uVar7 = (uint)DAT_006d4984;
            if ((int)uVar9 < (int)uVar7) {
                if ((int)uVar7 < (int)uVar8) {
                    uVar8 = uVar7;
                }
                *param_2 = uVar8;
                fVar3 = _DAT_005201dc / (pfVar11[1] - param_3[1]);
                param_2[4] = (uint)((*pfVar11 - *param_3) * fVar3);
                fVar1 = pfVar11[3];
                fVar2 = param_3[3];
                param_2[6] = 0;
                param_2[5] = (uint)((fVar1 - fVar2) * fVar3);
                fVar1 = param_3[1];
                param_2[2] = (uint)((float)param_2[4] * ((float)(int)uVar9 - fVar1) + *param_3);
                fVar2 = param_3[3];
                param_2[1] = uVar9;
                param_2[3] = (uint)(((float)(int)uVar9 - fVar1) * (float)param_2[5] + fVar2);
                return uVar9;
            }
            return 0xffffffff;
        }
        return 0xffffffff;
    }
    return 0xffffffff;
}
