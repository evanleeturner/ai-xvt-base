/* SPDX-License-Identifier: MIT */
/* z_bop 00471a10 */
extern float *DAT_0060f1e0;
extern float *DAT_0060f1d0;
extern float *DAT_0060f1c4;
extern int DAT_0099945c;
extern unsigned short DAT_009a1fe4;
extern unsigned short DAT_009d682c;
extern int DAT_009d7680;
extern int DAT_00a081ec;
extern float _DAT_005181c0;
/* DAT_00999462 is a 4-byte global at an unaligned address (0x999462) */
extern unsigned char DAT_00999462[4];

unsigned int __cdecl FUN_00471a10(int param_1, unsigned int *param_2, float *param_3, float *param_4)
{
    float fVar1, fVar2, fVar3;
    int iVar6;
    unsigned int uVar7, uVar8, uVar9;
    float *pfVar10;
    float *pfVar11;
    float fVar4, fVar5;
    int cnt;
    float *pf2;

    pfVar10 = param_4;
    if (0x80000000u < ((unsigned int *)param_4)[2]) {
        pfVar11 = param_3 + 2;
        pfVar10 = param_3;
        param_3 = param_4;
        if (0x80000000u < *(unsigned int *)pfVar11) {
            return 0xffffffffu;
        }
    }
    if (0x80000000u < ((unsigned int *)param_3)[2]) {
        DAT_0060f1e0 = DAT_0060f1d0;
        DAT_0060f1c4 = (float *)((unsigned int)DAT_0099945c +
                       (unsigned int)(*(int *)(param_1 + 0xbc) + *(int *)(param_1 + 200)) * 0x18u);
        DAT_0060f1d0 = DAT_0060f1c4;
        *(int *)(param_1 + 200) = (int)((unsigned int)*(int *)(param_1 + 200) + 1u);
        __builtin_memcpy(&cnt, DAT_00999462, 4);
        cnt = (int)((unsigned int)cnt + 1u);
        __builtin_memcpy(DAT_00999462, &cnt, 4);

        fVar4 = _DAT_005181c0 / pfVar10[2];
        iVar6 = (int)((unsigned int)(DAT_009a1fe4 >> 1) + (unsigned int)DAT_009d7680);
        fVar1 = pfVar10[1];
        fVar2 = param_3[1];
        fVar5 = param_3[2] / ((param_3[2] - fVar4 * (float)DAT_00a081ec) + _DAT_005181c0);
        fVar3 = param_3[1];
        *DAT_0060f1d0 =
            (((*pfVar10 - (float)(DAT_009d682c >> 1)) * fVar4 - *param_3) * fVar5 + *param_3) *
            (float)DAT_00a081ec;
        DAT_0060f1d0[1] =
            (float)DAT_00a081ec * (((fVar1 - (float)iVar6) * fVar4 - fVar2) * fVar5 + fVar3);
        *DAT_0060f1d0 = (float)(DAT_009d682c >> 1) + *DAT_0060f1d0;
        DAT_0060f1d0[1] = (float)(int)((unsigned int)(DAT_009a1fe4 >> 1) + (unsigned int)DAT_009d7680) + DAT_0060f1d0[1];
        DAT_0060f1d0[2] = (float)DAT_00a081ec;
        DAT_0060f1d0[3] = (pfVar10[3] - param_3[3]) * fVar5 + param_3[3];
        param_3 = DAT_0060f1d0;
    }
    if (((unsigned int *)param_3)[1] < 0x80000001u) {
        uVar9 = (unsigned int)(int)(long long)param_3[1];
        if ((float)(int)uVar9 != param_3[1]) {
            uVar9 = uVar9 + 1;
        }
    } else {
        uVar9 = 0;
    }
    if (((unsigned int *)pfVar10)[1] < 0x80000001u) {
        uVar7 = (unsigned int)(int)(long long)pfVar10[1];
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
            uVar7 = (unsigned int)DAT_009a1fe4;
            if ((int)uVar9 < (int)uVar7) {
                if ((int)uVar7 < (int)uVar8) {
                    uVar8 = uVar7;
                }
                pf2 = (float *)param_2;
                param_2[0] = uVar8;
                fVar3 = _DAT_005181c0 / (pfVar11[1] - param_3[1]);
                pf2[4] = (*pfVar11 - *param_3) * fVar3;
                fVar1 = pfVar11[3];
                fVar2 = param_3[3];
                param_2[6] = 0;
                pf2[5] = (fVar1 - fVar2) * fVar3;
                fVar1 = param_3[1];
                pf2[2] = pf2[4] * ((float)(int)uVar9 - fVar1) + *param_3;
                fVar2 = param_3[3];
                param_2[1] = uVar9;
                pf2[3] = ((float)(int)uVar9 - fVar1) * pf2[5] + fVar2;
                return uVar9;
            }
            return 0xffffffffu;
        }
        return 0xffffffffu;
    }
    return 0xffffffffu;
}
