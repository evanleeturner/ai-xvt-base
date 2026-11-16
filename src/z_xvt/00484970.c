/* SPDX-License-Identifier: MIT */
/* z_xvt 00484970 */
typedef unsigned int undefined4;

extern int DAT_006a884a;      /* packed, at unaligned address */
extern int DAT_006a885e;      /* packed, at unaligned address */
extern int DAT_006a8858;
extern int DAT_006a8874;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern int DAT_00709ffc;
extern unsigned int DAT_00732f3c;
extern float DAT_006a88b0;
extern float _DAT_005201e8;
extern float _DAT_005201dc;
extern float _DAT_005201d4;

void __cdecl FUN_00414d00(int param_1, undefined4 *param_2, int param_3);
void __cdecl FUN_00414fe0(float *param_1, float *param_2);
void __cdecl FUN_00414070(int *param_1, int param_2, float *param_3, float *param_4, float *param_5);

void __cdecl FUN_00484970(int *param_1)
{
    int iVar1;
    unsigned long long uVar2;
    float fVar3;
    float fVar4;
    int *piVar5;
    int iVar6;
    int iVar7;
    float *pfVar8;
    float *pfVar9;
    int *piVar10;
    int *piVar11;
    float *pf11;
    float *pf1;
    int lVar12;
    int lVar13;
    float local_4c;
    float local_44;
    float local_40;
    float local_3c;
    int local_38;
    float local_38f;
    float fStack_34;
    float fStack_30;
    float fStack_2c;
    int local_28;
    int local_24;
    float local_20;
    float fStack_1c;
    float fStack_18;
    int local_14;
    float local_10;
    float fStack_c;
    float fStack_8;
    float fStack_4;
    int g_884a;
    int g_885e;

    __builtin_memcpy(&g_885e, &DAT_006a885e, 4);
    __builtin_memcpy(&g_884a, &DAT_006a884a, 4);
    iVar1 = g_885e;
    iVar7 = 0;
    __builtin_memcpy(&local_20, &DAT_00732f3c, 4);
    fStack_1c = 0.0f;
    uVar2 = (unsigned long long)(unsigned int)DAT_00732f3c;
    piVar11 = (int *)((unsigned int)param_1[0x2e] * 0x70u + (unsigned int)g_884a);
    pf1 = (float *)param_1;
    param_1[0x2f] = g_885e;
    iVar6 = DAT_006a8858;
    param_1[0x32] = 0;
    pfVar9 = (float *)((unsigned int)iVar6 + (unsigned int)iVar1 * 0x18u);
    iVar6 = 0;
    local_10 = (((float)uVar2) / pf1[4]) * _DAT_005201e8;
    if (0 < param_1[0x1e]) {
        do {
            iVar7 = (int)((unsigned int)iVar7 + 4u);
            iVar6 = iVar6 + 1;
            *(int *)((unsigned int)DAT_006a8874 - 4u + (unsigned int)iVar7) = -1;
        } while (iVar6 < param_1[0x1e]);
    }
    local_24 = 0;
    if (0 < param_1[0x31]) {
        do {
            pf11 = (float *)piVar11;
            FUN_00414d00((int)piVar11,
                         (undefined4 *)((unsigned int)param_1[0x26] + (unsigned int)*piVar11 * 0x18u),
                         (int)(param_1 + 2));
            iVar6 = param_1[0x28];
            piVar11[0x12] = 0;
            local_28 = 0;
            pf11[0x13] = (float)(unsigned long long)(unsigned int)DAT_00732f3c;
            piVar5 = (int *)((unsigned int)*piVar11 * 0x40u + (unsigned int)iVar6);
            do {
                piVar10 = piVar5;
                iVar6 = *piVar10;
                local_38 = piVar10[0xc];
                if (iVar6 == -1) break;
                iVar1 = *(int *)((unsigned int)DAT_006a8874 + (unsigned int)iVar6 * 4u);
                if (iVar1 == -1) {
                    *(int *)((unsigned int)DAT_006a8874 + (unsigned int)iVar6 * 4u) = param_1[0x32];
                    param_1[0x32] = param_1[0x32] + 1;
                    local_14 = (int)((unsigned int)iVar6 * 0xcu);
                    pfVar8 = (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_14);
                    local_44 = *pfVar8;
                    local_40 = pfVar8[1];
                    local_3c = pfVar8[2];
                    FUN_00414fe0(&local_44, (float *)(param_1 + 5));
                    local_44 = pf1[2] + local_44;
                    local_40 = pf1[3] + local_40;
                    local_3c = pf1[4] + local_3c + _DAT_005201e8;
                    pfVar9[2] = local_10 / local_3c;
                    *pfVar9 = pfVar9[2] * local_44;
                    pfVar9[1] = pfVar9[2] * local_40;
                    *pfVar9 = (float)(DAT_007091ae >> 1) + *pfVar9;
                    pfVar9[1] = (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) + pfVar9[1];
                    local_4c = pfVar9[2];
                    FUN_00414070(param_1, (int)pfVar9,
                                 (float *)((unsigned int)param_1[0x21] + (unsigned int)local_38 * 0xcu),
                                 (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_14),
                                 (float *)&DAT_006a88b0);
                    pfVar9 = pfVar9 + 6;
                }
                else {
                    local_4c = *(float *)((unsigned int)DAT_006a8858 + 8u +
                                          (unsigned int)(param_1[0x2f] + iVar1) * 0x18u);
                }
                if (pf11[0x12] < local_4c) {
                    pf11[0x12] = local_4c;
                }
                if (local_4c < pf11[0x13]) {
                    pf11[0x13] = local_4c;
                }
                local_28 = local_28 + 1;
                piVar5 = piVar10 + 1;
            } while (local_28 < 4);
            if (param_1[0x20] != 0) {
                iVar1 = piVar10[5];
                local_44 = *(float *)((unsigned int)param_1[0x1f] + (unsigned int)piVar10[-3] * 0xcu);
                iVar6 = (int)((unsigned int)param_1[0x1f] + (unsigned int)piVar10[-3] * 0xcu);
                local_40 = *(float *)(iVar6 + 4);
                local_3c = *(float *)(iVar6 + 8);
                FUN_00414fe0(&local_44, (float *)(param_1 + 5));
                local_44 = pf1[2] + local_44;
                local_40 = pf1[3] + local_40;
                local_3c = pf1[4] + local_3c + _DAT_005201e8;
                pfVar8 = (float *)((unsigned int)param_1[0x20] + (unsigned int)iVar1 * 8u);
                pf11[10] = (local_44 - pfVar8[1] * pf11[7]) - *pfVar8 * pf11[4];
                pf11[0xb] = (local_40 -
                             *(float *)((unsigned int)param_1[0x20] + 4u + (unsigned int)iVar1 * 8u) * pf11[8]) -
                            *(float *)((unsigned int)param_1[0x20] + (unsigned int)iVar1 * 8u) * pf11[5];
                pf11[0xc] = (local_3c -
                             *(float *)((unsigned int)param_1[0x20] + 4u + (unsigned int)iVar1 * 8u) * pf11[9]) -
                            *(float *)((unsigned int)param_1[0x20] + (unsigned int)iVar1 * 8u) * pf11[6];
                local_20 = pf11[0xc] * pf11[8] - pf11[9] * pf11[0xb];
                fStack_1c = pf11[9] * pf11[10] - pf11[0xc] * pf11[7];
                fStack_18 = pf11[0xb] * pf11[7] - pf11[10] * pf11[8];
                fStack_c = pf11[6] * pf11[0xb] - pf11[0xc] * pf11[5];
                fStack_8 = pf11[0xc] * pf11[4] - pf11[10] * pf11[6];
                fStack_4 = pf11[5] * pf11[10] - pf11[0xb] * pf11[4];
                fStack_34 = pf11[9] * pf11[5] - pf11[6] * pf11[8];
                fStack_30 = pf11[6] * pf11[7] - pf11[9] * pf11[4];
                fStack_2c = pf11[8] * pf11[4] - pf11[5] * pf11[7];
                if (((fStack_34 < 0.0f ? -fStack_34 : fStack_34) == 0.0f) &&
                    ((fStack_30 < 0.0f ? -fStack_30 : fStack_30) == 0.0f) &&
                    ((fStack_2c < 0.0f ? -fStack_2c : fStack_2c) == 0.0f)) {
                    fStack_2c = 1.0f;
                }
                fVar3 = _DAT_005201dc /
                        (pf11[0xb] * fStack_30 + pf11[10] * fStack_34 + pf11[0xc] * fStack_2c);
                fVar4 = fVar3 / local_10;
                pf11[4] = local_20 * fVar4;
                pf11[5] = fStack_1c * fVar4;
                pf11[6] = fStack_18 * fVar3;
                pf11[7] = fStack_c * fVar4;
                pf11[8] = fStack_8 * fVar4;
                pf11[9] = fStack_4 * fVar3;
                pf11[10] = fStack_34 * fVar4;
                pf11[0xb] = fVar4 * fStack_30;
                pf11[0xc] = fVar3 * fStack_2c;
                pf11[6] = pf11[6] - (float)(DAT_007091ae >> 1) * pf11[4];
                pf11[6] = pf11[6] -
                          (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * pf11[5];
                pf11[9] = pf11[9] - (float)(DAT_007091ae >> 1) * pf11[7];
                pf11[9] = pf11[9] -
                          (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * pf11[8];
                pf11[0xc] = pf11[0xc] - (float)(DAT_007091ae >> 1) * pf11[10];
                pf11[0xc] = pf11[0xc] -
                            (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * pf11[0xb];
                local_38f = pf11[8] * pf11[4] * local_3c * local_3c;
                if (local_38f < _DAT_005201d4) {
                    local_38f = -local_38f;
                }
                lVar12 = (int)local_38f;          /* __ftol */
                piVar11[0x1b] = lVar12;
                local_38f = pf11[5] * pf11[7] * local_3c * local_3c;
                if (local_38f < _DAT_005201d4) {
                    local_38f = -local_38f;
                }
                lVar13 = (int)local_38f;          /* __ftol */
                piVar11[0x1b] = (int)((unsigned int)lVar13 + (unsigned int)lVar12);
            }
            piVar11 = piVar11 + 0x1c;
            local_24 = local_24 + 1;
        } while (local_24 < param_1[0x31]);
    }
    __builtin_memcpy(&g_885e, &DAT_006a885e, 4);
    g_885e = (int)((unsigned int)g_885e + (unsigned int)param_1[0x32]);
    __builtin_memcpy(&DAT_006a885e, &g_885e, 4);
    return;
}
