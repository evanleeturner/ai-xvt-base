/* SPDX-License-Identifier: MIT */
/* z_xvt 0045cff0 */
extern unsigned int DAT_00732f3c;
extern float _DAT_005201b0;
extern int DAT_006a884a;   /* packed: address 0x6a884a not 4-aligned */
extern int DAT_006a885e;   /* packed: address 0x6a885e not 4-aligned */
extern int DAT_006a8858;
extern int DAT_006a8874;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern unsigned int DAT_00709ffc;
extern float DAT_006a88b0;

void __cdecl FUN_00414fe0(float *param_1, float *param_2);
void __cdecl FUN_00414070(int *param_1, int param_2, float *param_3, float *param_4, float *param_5);

void __cdecl FUN_0045cff0(int *param_1, int param_2)
{
    unsigned long long uVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    float *pfVar5;
    float *pfVar6;
    int *piVar7;
    int *piVar8;
    float *pfVar9;
    float *pfParam1;
    float local_2c;
    float local_24;
    float fStack_20;
    float local_1c;
    int local_18;
    int local_14;
    int local_10;
    int local_c;
    float local_8;
    int local_4;
    int tmp_884a;
    int tmp_885e;

    pfParam1 = (float *)param_1;

    __builtin_memcpy(&tmp_885e, &DAT_006a885e, 4);
    __builtin_memcpy(&tmp_884a, &DAT_006a884a, 4);

    iVar2 = tmp_885e;
    iVar4 = 0;
    uVar1 = (unsigned long long)DAT_00732f3c;
    piVar8 = (int *)((unsigned int)param_1[0x2e] * 0x70u + (unsigned int)tmp_884a);
    pfVar9 = (float *)piVar8;
    param_1[0x2f] = tmp_885e;
    iVar3 = DAT_006a8858;
    param_1[0x32] = 0;
    pfVar6 = (float *)((unsigned int)iVar3 + (unsigned int)iVar2 * 0x18u);
    iVar3 = 0;
    local_8 = ((float)uVar1 / pfParam1[4]) * _DAT_005201b0;
    if (0 < param_1[0x1e]) {
        do {
            iVar4 = (int)((unsigned int)iVar4 + 4u);
            iVar3 = iVar3 + 1;
            *(int *)((unsigned int)DAT_006a8874 + (unsigned int)-4 + (unsigned int)iVar4) = -1;
        } while (iVar3 < param_1[0x1e]);
    }
    local_14 = 0;
    if (0 < param_1[0x31]) {
        do {
            iVar3 = param_1[0x28];
            pfVar9[0x12] = 0.0f;
            piVar7 = (int *)((unsigned int)*piVar8 * 0x40u + (unsigned int)iVar3);
            local_18 = 0;
            pfVar9[0x13] = (float)DAT_00732f3c;
            do {
                local_10 = *piVar7;
                local_c = piVar7[8];
                local_4 = piVar7[0xc];
                piVar7 = piVar7 + 1;
                if (local_10 == -1) break;
                iVar3 = *(int *)((unsigned int)DAT_006a8874 + (unsigned int)local_10 * 4u);
                if (iVar3 == -1) {
                    *(int *)((unsigned int)DAT_006a8874 + (unsigned int)local_10 * 4u) = param_1[0x32];
                    local_10 = (int)((unsigned int)local_10 * 0xcu);
                    pfVar5 = (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_10);
                    param_1[0x32] = param_1[0x32] + 1;
                    local_24 = *pfVar5;
                    fStack_20 = pfVar5[1];
                    local_1c = pfVar5[2];
                    FUN_00414fe0(&local_24, (float *)(param_1 + 5));
                    local_24 = pfParam1[2] + local_24;
                    fStack_20 = pfParam1[3] + fStack_20;
                    local_1c = pfParam1[4] + local_1c + _DAT_005201b0;
                    pfVar6[2] = local_8 / local_1c;
                    *pfVar6 = pfVar6[2] * local_24;
                    pfVar6[1] = pfVar6[2] * fStack_20;
                    *pfVar6 = (float)(DAT_007091ae >> 1) + *pfVar6;
                    pfVar6[1] = (float)(int)((unsigned int)(DAT_006d4984 >> 1) + DAT_00709ffc) + pfVar6[1];
                    local_2c = pfVar6[2];
                    FUN_00414070(param_1, (int)pfVar6,
                                 (float *)((unsigned int)param_1[0x21] + (unsigned int)local_4 * 0xcu),
                                 (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_10),
                                 &DAT_006a88b0);
                    pfVar6[4] = *(float *)((unsigned int)param_1[0x20] + (unsigned int)local_c * 8u);
                    pfVar6[5] = *(float *)((unsigned int)param_1[0x20] + 4u + (unsigned int)local_c * 8u);
                    pfVar6 = pfVar6 + 6;
                }
                else {
                    local_2c = *(float *)((unsigned int)DAT_006a8858 + 8u +
                                          ((unsigned int)param_1[0x2f] + (unsigned int)iVar3) * 0x18u);
                }
                if (pfVar9[0x12] < local_2c) {
                    pfVar9[0x12] = local_2c;
                }
                if (local_2c < pfVar9[0x13]) {
                    pfVar9[0x13] = local_2c;
                }
                local_18 = local_18 + 1;
            } while (local_18 < 4);
            piVar8 = piVar8 + 0x1c;
            pfVar9 = pfVar9 + 0x1c;
            local_14 = local_14 + 1;
        } while (local_14 < param_1[0x31]);
    }
    __builtin_memcpy(&tmp_885e, &DAT_006a885e, 4);
    tmp_885e = (int)((unsigned int)tmp_885e + (unsigned int)param_1[0x32]);
    __builtin_memcpy(&DAT_006a885e, &tmp_885e, 4);
    return;
}
