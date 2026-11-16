/* SPDX-License-Identifier: MIT */
/* z_xvt 004842b0 */
extern int DAT_006a885e;
extern int DAT_006a884a;
extern int DAT_006a8858;
extern int DAT_006a8874;
extern float DAT_00732f3c;
extern float _DAT_005201dc;
extern float _DAT_005201d4;
extern float _DAT_005201e0;
extern float _DAT_005201e4;
extern float _DAT_006a8a08;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern int DAT_00709ffc;
extern float DAT_006a88b0;

void __cdecl FUN_00414d00(int param_1, unsigned int *param_2, int param_3);
void __cdecl FUN_00414fe0(float *param_1, float *param_2);
void __cdecl FUN_00414070(int *param_1, int param_2, float *param_3, float *param_4, float *param_5);

void __cdecl FUN_004842b0(int *param_1)
{
    float fVar1;
    float fVar2;
    int *piVar3;
    int iVar4;
    int iVar5;
    float *pfVar6;
    float *pfVar7;
    int *piVar8;
    int *piVar9;
    long long lVar10;
    float local_4c;
    float local_44;
    float local_40;
    float local_3c;
    float local_38;
    float local_34;
    float fStack_30;
    float fStack_2c;
    float fStack_28;
    int local_24;
    int local_20;
    int local_1c;
    float fStack_18;
    float fStack_14;
    float fStack_10;
    float fStack_c;
    float fStack_8;
    float fStack_4;

    iVar5 = DAT_006a885e;
    piVar9 = (int *)((unsigned int)param_1[0x2e] * 0x70u + (unsigned int)DAT_006a884a);
    param_1[0x2f] = DAT_006a885e;
    iVar4 = 0;
    pfVar7 = (float *)((unsigned int)DAT_006a8858 + (unsigned int)iVar5 * 0x18u);
    iVar5 = 0;
    param_1[0x32] = 0;
    if (0 < param_1[0x1e]) {
        do {
            iVar5 = (int)((unsigned int)iVar5 + 4u);
            iVar4 = (int)((unsigned int)iVar4 + 1u);
            *(int *)((unsigned int)DAT_006a8874 + 0xfffffffcu + (unsigned int)iVar5) = -1;
        } while (iVar4 < param_1[0x1e]);
    }
    local_20 = 0;
    if (0 < param_1[0x31]) {
        do {
            FUN_00414d00((int)piVar9,
                         (unsigned int *)((unsigned int)param_1[0x26] + (unsigned int)*piVar9 * 0x18u),
                         (int)(param_1 + 2));
            iVar5 = param_1[0x28];
            piVar9[0x12] = 0;
            local_38 = 0.0f;
            local_24 = 0;
            *(float *)&piVar9[0x13] = DAT_00732f3c;
            piVar3 = (int *)((unsigned int)*piVar9 * 0x40u + (unsigned int)iVar5);
            do {
                piVar8 = piVar3;
                iVar5 = *piVar8;
                *(int *)&local_34 = piVar8[0xc];
                if (iVar5 == -1) break;
                iVar4 = *(int *)((unsigned int)DAT_006a8874 + (unsigned int)iVar5 * 4u);
                if (iVar4 == -1) {
                    local_1c = (int)((unsigned int)iVar5 * 0xcu);
                    *(int *)((unsigned int)DAT_006a8874 + (unsigned int)iVar5 * 4u) = param_1[0x32];
                    param_1[0x32] = (int)((unsigned int)param_1[0x32] + 1u);
                    pfVar6 = (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_1c);
                    local_44 = *pfVar6;
                    local_40 = pfVar6[1];
                    local_3c = pfVar6[2];
                    FUN_00414fe0(&local_44, (float *)(param_1 + 5));
                    local_44 = *(float *)&param_1[2] + local_44;
                    local_40 = *(float *)&param_1[3] + local_40;
                    local_3c = *(float *)&param_1[4] + local_3c;
                    if (_DAT_005201dc <= local_3c) {
                        pfVar7[2] = DAT_00732f3c / local_3c;
                        *pfVar7 = pfVar7[2] * local_44;
                        pfVar7[1] = pfVar7[2] * local_40;
                        *pfVar7 = (float)(DAT_007091ae >> 1) + *pfVar7;
                        pfVar7[1] = (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) + pfVar7[1];
                        local_4c = pfVar7[2];
                    } else {
                        pfVar7[2] = local_3c - _DAT_005201dc;
                        *pfVar7 = local_44;
                        pfVar7[1] = local_40;
                        piVar9[3] = -1;
                        local_4c = DAT_00732f3c;
                    }
                    FUN_00414070(param_1, (int)pfVar7,
                                 (float *)((unsigned int)param_1[0x21] + (unsigned int)*(int *)&local_34 * 0xcu),
                                 (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_1c),
                                 (float *)&DAT_006a88b0);
                    pfVar7 = pfVar7 + 6;
                } else {
                    iVar4 = (int)((unsigned int)param_1[0x2f] + (unsigned int)iVar4);
                    if (*(unsigned int *)((unsigned int)DAT_006a8858 + 8u + (unsigned int)iVar4 * 0x18u) < 0x80000001u) {
                        local_4c = *(float *)((unsigned int)DAT_006a8858 + (unsigned int)iVar4 * 0x18u + 8u);
                    } else {
                        piVar9[3] = -1;
                        local_4c = DAT_00732f3c;
                    }
                }
                local_38 = local_38 + local_4c;
                if (*(float *)&piVar9[0x12] < local_4c) {
                    *(float *)&piVar9[0x12] = local_4c;
                }
                if (local_4c < *(float *)&piVar9[0x13]) {
                    *(float *)&piVar9[0x13] = local_4c;
                }
                local_24 = (int)((unsigned int)local_24 + 1u);
                piVar3 = piVar8 + 1;
            } while (local_24 < 4);
            if (param_1[0x20] != 0) {
                iVar5 = piVar8[5];
                pfVar6 = (float *)((unsigned int)param_1[0x1f] + (unsigned int)piVar8[-3] * 0xcu);
                local_44 = *pfVar6;
                local_40 = pfVar6[1];
                local_3c = pfVar6[2];
                FUN_00414fe0(&local_44, (float *)(param_1 + 5));
                local_44 = *(float *)&param_1[2] + local_44;
                local_40 = *(float *)&param_1[3] + local_40;
                local_3c = *(float *)&param_1[4] + local_3c;
                *(float *)&piVar9[10] = (local_44 - *(float *)((unsigned int)param_1[0x20] + 4u + (unsigned int)iVar5 * 8u) * *(float *)&piVar9[7])
                                        - *(float *)((unsigned int)param_1[0x20] + (unsigned int)iVar5 * 8u) * *(float *)&piVar9[4];
                *(float *)&piVar9[0xb] = (local_40 - *(float *)((unsigned int)param_1[0x20] + 4u + (unsigned int)iVar5 * 8u) * *(float *)&piVar9[8])
                                         - *(float *)((unsigned int)param_1[0x20] + (unsigned int)iVar5 * 8u) * *(float *)&piVar9[5];
                *(float *)&piVar9[0xc] = (local_3c - *(float *)((unsigned int)param_1[0x20] + 4u + (unsigned int)iVar5 * 8u) * *(float *)&piVar9[9])
                                         - *(float *)((unsigned int)param_1[0x20] + (unsigned int)iVar5 * 8u) * *(float *)&piVar9[6];
                fStack_18 = *(float *)&piVar9[0xc] * *(float *)&piVar9[8] - *(float *)&piVar9[9] * *(float *)&piVar9[0xb];
                fStack_14 = *(float *)&piVar9[9] * *(float *)&piVar9[10] - *(float *)&piVar9[0xc] * *(float *)&piVar9[7];
                fStack_10 = *(float *)&piVar9[0xb] * *(float *)&piVar9[7] - *(float *)&piVar9[10] * *(float *)&piVar9[8];
                fStack_c = *(float *)&piVar9[6] * *(float *)&piVar9[0xb] - *(float *)&piVar9[0xc] * *(float *)&piVar9[5];
                fStack_8 = *(float *)&piVar9[0xc] * *(float *)&piVar9[4] - *(float *)&piVar9[10] * *(float *)&piVar9[6];
                fStack_4 = *(float *)&piVar9[5] * *(float *)&piVar9[10] - *(float *)&piVar9[0xb] * *(float *)&piVar9[4];
                fStack_30 = *(float *)&piVar9[9] * *(float *)&piVar9[5] - *(float *)&piVar9[6] * *(float *)&piVar9[8];
                fStack_2c = *(float *)&piVar9[6] * *(float *)&piVar9[7] - *(float *)&piVar9[9] * *(float *)&piVar9[4];
                fStack_28 = *(float *)&piVar9[8] * *(float *)&piVar9[4] - *(float *)&piVar9[5] * *(float *)&piVar9[7];
                if (((fStack_30 < 0.0f ? -fStack_30 : fStack_30) == 0.0f) &&
                    ((fStack_2c < 0.0f ? -fStack_2c : fStack_2c) == 0.0f) &&
                    ((fStack_28 < 0.0f ? -fStack_28 : fStack_28) == 0.0f)) {
                    fStack_28 = 1.0f;
                }
                fVar1 = _DAT_005201dc /
                        (*(float *)&piVar9[0xb] * fStack_2c +
                         *(float *)&piVar9[10] * fStack_30 + *(float *)&piVar9[0xc] * fStack_28);
                fVar2 = _DAT_006a8a08 * fVar1;
                *(float *)&piVar9[4] = fStack_18 * fVar2;
                *(float *)&piVar9[5] = fStack_14 * fVar2;
                *(float *)&piVar9[6] = fStack_10 * fVar1;
                *(float *)&piVar9[7] = fStack_c * fVar2;
                *(float *)&piVar9[8] = fStack_8 * fVar2;
                *(float *)&piVar9[9] = fStack_4 * fVar1;
                *(float *)&piVar9[10] = fStack_30 * fVar2;
                *(float *)&piVar9[0xb] = fVar2 * fStack_2c;
                *(float *)&piVar9[0xc] = fVar1 * fStack_28;
                *(float *)&piVar9[6] = *(float *)&piVar9[6] - (float)(DAT_007091ae >> 1) * *(float *)&piVar9[4];
                *(float *)&piVar9[6] = *(float *)&piVar9[6] -
                        (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * *(float *)&piVar9[5];
                *(float *)&piVar9[9] = *(float *)&piVar9[9] - (float)(DAT_007091ae >> 1) * *(float *)&piVar9[7];
                *(float *)&piVar9[9] = *(float *)&piVar9[9] -
                        (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * *(float *)&piVar9[8];
                *(float *)&piVar9[0xc] = *(float *)&piVar9[0xc] - (float)(DAT_007091ae >> 1) * *(float *)&piVar9[10];
                *(float *)&piVar9[0xc] = *(float *)&piVar9[0xc] -
                        (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * *(float *)&piVar9[0xb];
                local_34 = *(float *)&piVar9[8] * *(float *)&piVar9[4] - *(float *)&piVar9[5] * *(float *)&piVar9[7];
                if (local_34 < _DAT_005201d4) {
                    local_34 = -local_34;
                }
                fVar1 = _DAT_005201e4;
                if (*piVar8 == -1) {
                    fVar1 = _DAT_005201e0;
                }
                local_38 = fVar1 / local_38;
                lVar10 = (long long)local_38;
                piVar9[0x1b] = (int)lVar10;
            }
            piVar9 = piVar9 + 0x1c;
            local_20 = (int)((unsigned int)local_20 + 1u);
        } while (local_20 < param_1[0x31]);
    }
    DAT_006a885e = (int)((unsigned int)DAT_006a885e + (unsigned int)param_1[0x32]);
    return;
}
