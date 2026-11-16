/* SPDX-License-Identifier: MIT */
/* z_bop 00472c90 */
/* Prototypes */
unsigned int *FUN_00479de0(int param_1, char *param_2);
void FUN_00471e00(unsigned int *param_1);
void FUN_004210a0(float *param_1, float *param_2);
void FUN_00420fe0(float *param_1, float *param_2);
void FUN_004211d0(float *param_1, float *param_2);
long double FUN_00421040(float *param_1, float *param_2);
long double FUN_00421060(float *param_1, int param_2);
long double FUN_00421080(float *param_1, int param_2);
void FUN_00421300(float *param_1, float *param_2);
void __cdecl FUN_00472c90(int param_1, unsigned int *param_2, unsigned int *param_3);

/* Globals (types inferred from usage) */
extern int DAT_005270b0;
extern unsigned int DAT_0060f1d8;
extern int *DAT_0060f1ec;
extern int DAT_0060f1c0;
extern int DAT_0060f1e8;
extern int _DAT_0060f1f0;
extern int DAT_0060f204;
extern float *_DAT_0060f208;
extern int _DAT_005233a0;
extern int DAT_005233a4;
extern float DAT_005233a8;
extern int DAT_009a8e34;
extern int DAT_005271d4;
extern float _DAT_005181c0;
extern float _DAT_005181dc;

#define PF(i) (*(float *)&param_3[(i)])

void __cdecl FUN_00472c90(int param_1, unsigned int *param_2, unsigned int *param_3)
{
    float *pfVar1;
    int iVar2;
    float fVar3;
    unsigned int *puVar4;
    int iVar5;
    int *piVar6;
    float *pfVar7;
    int iVar8;
    float *pfVar9;
    int iVar10;
    int iVar11;
    long double fVar12;
    float local_12c;
    int local_128;
    float local_120[4];
    float local_110[16];
    unsigned int local_d0[52];

    if (param_2 == (unsigned int *)0) {
        return;
    }
    do {
        iVar10 = 0;
        local_128 = 0;
        while (param_2[1] == 7) {
            if (DAT_005270b0 == 0) {
                param_2 = FUN_00479de0(param_1, (char *)param_2[5]);
            }
            else if (*(char *)param_2[5] == '\0') {
                param_2 = (unsigned int *)*param_2;
            }
            else {
                puVar4 = FUN_00479de0(param_1, (char *)param_2[5]);
                *param_2 = (unsigned int)puVar4;
                *(unsigned char *)param_2[5] = 0;
                param_2 = (unsigned int *)*param_2;
            }
            if (param_2 == (unsigned int *)0) {
                return;
            }
        }
        pfVar9 = (float *)param_2[5];
        if (pfVar9 == (float *)0) {
            switch (param_2[1]) {
            case 10:
                iVar11 = (int)param_2[4];
                if ((iVar11 == 8) || (iVar11 == 7)) {
                    param_3[0x22] = DAT_0060f1d8;
                }
                else if ((iVar11 == 6) || (iVar11 == 5)) {
                    param_3[0x27] = DAT_0060f1d8;
                }
                else {
                    param_3[0x1d] = DAT_0060f1d8;
                }
                break;
            case 0x14:
                param_3[0x29] = *param_2;
                DAT_0060f1ec = (int *)param_2[5];
                param_3[0x2a] = (unsigned int)DAT_0060f1ec;
                param_3[0x2b] = param_3[0x2a];
                iVar11 = (int)(param_3[0x2a] + 0x18);
                param_3[0x2b] = (unsigned int)iVar11;
                piVar6 = (int *)(param_3 + 0x2c);
                if (DAT_0060f1ec[1] == 0) {
                    *piVar6 = *DAT_0060f1ec;
                }
                else {
                    *piVar6 = iVar11;
                    iVar2 = (int)param_3[0x2a];
                    iVar5 = (int)((unsigned int)*(int *)(iVar2 + 0x14) * (unsigned int)*(int *)(iVar2 + 0x10));
                    if (*(int *)(iVar2 + 8) == iVar5) {
                        iVar5 = *(int *)(iVar2 + 0xc);
                    }
                    *piVar6 = (int)((unsigned int)iVar5 + (unsigned int)iVar11);
                }
                iVar11 = *piVar6;
                *piVar6 = (int)((unsigned int)iVar11 + 0x1000u);
                param_3[0x2d] = (unsigned int)iVar11 + 0x1000u;
                *piVar6 = (int)((unsigned int)*piVar6 - 0x1000u);
                break;
            case 0x18:
                local_128 = (int)((unsigned int)_DAT_005233a0 + 1u);
                iVar11 = (int)param_2[2];
                if (iVar11 < local_128) goto LAB_004733f4;
            }
        }
        else {
            switch (param_2[1]) {
            case 1:
            case 0xf:
            case 0x10:
            case 0x11:
                pfVar7 = pfVar9 + 1;
                param_3[0x23] = param_2[4];
                param_3[0x24] = *(unsigned int *)pfVar9;
                param_3[0x28] = (unsigned int)pfVar7;
                pfVar7 = pfVar7 + (int)((unsigned int)param_2[4] * 16u);
                param_3[0x25] = (unsigned int)pfVar7;
                iVar11 = (int)param_2[4];
                param_3[0x26] = (unsigned int)(pfVar7 + (int)((unsigned int)iVar11 * 3u));
                piVar6 = DAT_0060f1ec;
                iVar2 = (int)param_2[4];
                if (param_3[0x2a] == 0) {
                    param_3[0x2a] = (unsigned int)DAT_0060f1ec;
                    param_3[0x2b] = (unsigned int)piVar6;
                    piVar6 = piVar6 + 6;
                    param_3[0x2b] = (unsigned int)piVar6;
                    if (DAT_0060f1ec[1] == 0) {
                        piVar6 = (int *)*DAT_0060f1ec;
LAB_00472ddd:
                        param_3[0x2c] = (unsigned int)piVar6;
                    }
                    else {
                        param_3[0x2c] = (unsigned int)piVar6;
                        iVar5 = (int)param_3[0x2a];
                        iVar8 = (int)((unsigned int)*(int *)(iVar5 + 0x14) * (unsigned int)*(int *)(iVar5 + 0x10));
                        if (*(int *)(iVar5 + 8) != iVar8) {
                            piVar6 = (int *)((unsigned int)(int)piVar6 + (unsigned int)iVar8);
                            goto LAB_00472ddd;
                        }
                        param_3[0x2c] = (unsigned int)*(int *)(iVar5 + 0xc) + (unsigned int)(int)piVar6;
                    }
                    iVar5 = (int)param_3[0x2c];
                    param_3[0x2c] = (unsigned int)iVar5 + 0x1000u;
                    param_3[0x2d] = (unsigned int)iVar5 + 0x1000u;
                    param_3[0x2c] = param_3[0x2c] - 0x1000u;
                }
                if (param_3[0x21] == 0) {
                    param_3[0x21] = (unsigned int)(pfVar7 + (int)((unsigned int)iVar11 * 3u)
                                                          + (int)((unsigned int)iVar2 * 6u));
                    FUN_00471e00(param_3);
                    param_3[0x21] = 0;
                }
                else {
                    FUN_00471e00(param_3);
                }
                break;
            case 2:
                FUN_004210a0((float *)(param_3 + 5), pfVar9 + 3);
                FUN_00420fe0((float *)(param_3 + 2), pfVar9 + 3);
                PF(2) = *pfVar9 + PF(2);
                PF(3) = pfVar9[1] + PF(3);
                PF(4) = PF(4) + pfVar9[2];
                FUN_004211d0((float *)(param_3 + 0x11), pfVar9 + 3);
                fVar12 = FUN_00421040(pfVar9, (float *)(param_3 + 0x11));
                PF(0xe) = (float)((long double)PF(0xe) - fVar12);
                fVar12 = FUN_00421060(pfVar9, (int)(param_3 + 0x11));
                PF(0xf) = (float)((long double)PF(0xf) - fVar12);
                fVar12 = FUN_00421080(pfVar9, (int)(param_3 + 0x11));
                PF(0x10) = (float)((long double)PF(0x10) - fVar12);
                break;
            case 3:
                param_3[0x1e] = param_2[4];
                param_3[0x1f] = (unsigned int)pfVar9;
                break;
            case 4:
                PF(2) = PF(2) + *pfVar9;
                PF(3) = PF(3) + pfVar9[1];
                PF(4) = PF(4) + pfVar9[2];
                fVar12 = FUN_00421040(pfVar9, (float *)(param_3 + 0x11));
                PF(0xe) = (float)((long double)PF(0xe) - fVar12);
                fVar12 = FUN_00421060(pfVar9, (int)(param_3 + 0x11));
                PF(0xf) = (float)((long double)PF(0xf) - fVar12);
                fVar12 = FUN_00421080(pfVar9, (int)(param_3 + 0x11));
                PF(0x10) = (float)((long double)PF(0x10) - fVar12);
                break;
            case 5:
                FUN_004210a0((float *)(param_3 + 5), pfVar9);
                FUN_00420fe0((float *)(param_3 + 2), pfVar9);
                FUN_004211d0((float *)(param_3 + 0x11), pfVar9);
                break;
            case 6:
                pfVar7 = pfVar9 + 1;
                pfVar1 = pfVar9 + 2;
                PF(5) = PF(5) * *pfVar9;
                PF(6) = PF(6) * *pfVar7;
                PF(7) = PF(7) * *pfVar1;
                PF(8) = PF(8) * *pfVar9;
                PF(9) = PF(9) * *pfVar7;
                PF(10) = PF(10) * *pfVar1;
                PF(0xb) = PF(0xb) * *pfVar9;
                PF(0xc) = PF(0xc) * *pfVar7;
                PF(0xd) = PF(0xd) * *pfVar1;
                PF(2) = PF(2) * *pfVar9;
                PF(3) = PF(3) * *pfVar7;
                PF(4) = PF(4) * *pfVar1;
                fVar3 = _DAT_005181c0 / *pfVar9;
                PF(0x11) = PF(0x11) * fVar3;
                PF(0x12) = PF(0x12) * fVar3;
                PF(0x13) = fVar3 * PF(0x13);
                fVar3 = _DAT_005181c0 / *pfVar7;
                PF(0x14) = PF(0x14) * fVar3;
                PF(0x15) = PF(0x15) * fVar3;
                PF(0x16) = fVar3 * PF(0x16);
                fVar3 = _DAT_005181c0 / *pfVar1;
                PF(0x17) = PF(0x17) * fVar3;
                PF(0x18) = PF(0x18) * fVar3;
                PF(0x19) = fVar3 * PF(0x19);
                break;
            case 10:
                iVar11 = (int)param_2[4];
                if ((iVar11 == 8) || (iVar11 == 7)) {
                    param_3[0x22] = DAT_0060f1d8;
                }
                else if ((iVar11 == 6) || (iVar11 == 5)) {
                    param_3[0x27] = DAT_0060f1d8;
                }
                else {
                    param_3[0x1d] = DAT_0060f1d8;
                }
                break;
            case 0xb:
                _DAT_0060f208 = pfVar9;
                param_3[0x21] = (unsigned int)pfVar9;
                break;
            case 0xd:
                param_3[0x20] = (unsigned int)pfVar9;
                break;
            case 0x13:
                param_3[0x1a] = *(unsigned int *)pfVar9;
                param_3[0x1b] = *(unsigned int *)(pfVar9 + 1);
                param_3[0x1c] = *(unsigned int *)(pfVar9 + 2);
                break;
            case 0x14:
                param_3[0x29] = *param_2;
                DAT_0060f1ec = (int *)param_2[5];
                param_3[0x2a] = (unsigned int)DAT_0060f1ec;
                param_3[0x2b] = param_3[0x2a];
                iVar11 = (int)(param_3[0x2a] + 0x18);
                param_3[0x2b] = (unsigned int)iVar11;
                piVar6 = (int *)(param_3 + 0x2c);
                if (DAT_0060f1ec[1] == 0) {
                    *piVar6 = *DAT_0060f1ec;
                }
                else {
                    *piVar6 = iVar11;
                    iVar2 = (int)param_3[0x2a];
                    iVar5 = (int)((unsigned int)*(int *)(iVar2 + 0x14) * (unsigned int)*(int *)(iVar2 + 0x10));
                    if (*(int *)(iVar2 + 8) == iVar5) {
                        iVar5 = *(int *)(iVar2 + 0xc);
                    }
                    *piVar6 = (int)((unsigned int)iVar5 + (unsigned int)iVar11);
                }
                iVar11 = *piVar6;
                *piVar6 = (int)((unsigned int)iVar11 + 0x1000u);
                param_3[0x2d] = (unsigned int)iVar11 + 0x1000u;
                *piVar6 = (int)((unsigned int)*piVar6 - 0x1000u);
                break;
            case 0x15:
                if ((DAT_009a8e34 < 1) || (DAT_005233a4 != 0)) {
                    if (DAT_005233a4 == 0) {
                        iVar10 = 1;
                    }
                    else {
                        iVar10 = DAT_005233a4;
                        if ((int)param_2[2] < DAT_005233a4) {
                            iVar10 = -1;
                        }
                    }
                }
                else {
                    local_12c = 1.0f;
                    if (0 < (int)DAT_005233a8) {
                        local_12c = _DAT_005181c0 / ((float)DAT_009a8e34 * DAT_005233a8);
                    }
                    iVar10 = 1;
                    iVar11 = (int)param_2[2];
                    if (0 < iVar11) {
                        do {
                            if (*pfVar9 <= local_12c) break;
                            pfVar9 = pfVar9 + 1;
                            iVar10 = (int)((unsigned int)iVar10 + 1u);
                        } while (iVar10 <= iVar11);
                        if (iVar10 <= iVar11) break;
                    }
                    iVar10 = -1;
                }
                break;
            case 0x17:
                if ((param_3[1] & 0x7fffffffu) != 0) {
                    PF(0xe) = PF(0xe) - *pfVar9;
                    PF(0xf) = PF(0xf) - pfVar9[1];
                    PF(0x10) = PF(0x10) - pfVar9[2];
                    fVar12 = FUN_00421040(pfVar9, (float *)(param_3 + 5));
                    PF(2) = (float)(fVar12 + (long double)PF(2));
                    fVar12 = FUN_00421060(pfVar9, (int)(param_3 + 5));
                    PF(3) = (float)(fVar12 + (long double)PF(3));
                    fVar12 = FUN_00421080(pfVar9, (int)(param_3 + 5));
                    PF(4) = (float)(fVar12 + (long double)PF(4));
                    local_120[0] = pfVar9[3] * _DAT_005181dc;
                    local_120[1] = pfVar9[4] * _DAT_005181dc;
                    local_120[2] = pfVar9[5] * _DAT_005181dc;
                    *(unsigned int *)&local_120[3] = param_3[1];
                    FUN_00421300(local_110, local_120);
                    FUN_004210a0((float *)(param_3 + 0x11), local_110);
                    FUN_00420fe0((float *)(param_3 + 0xe), local_110);
                    FUN_004211d0((float *)(param_3 + 5), local_110);
                    PF(0xe) = PF(0xe) + *pfVar9;
                    PF(0xf) = PF(0xf) + pfVar9[1];
                    PF(0x10) = PF(0x10) + pfVar9[2];
                    fVar12 = FUN_00421040(pfVar9, (float *)(param_3 + 5));
                    PF(2) = (float)((long double)PF(2) - fVar12);
                    fVar12 = FUN_00421060(pfVar9, (int)(param_3 + 5));
                    PF(3) = (float)((long double)PF(3) - fVar12);
                    fVar12 = FUN_00421080(pfVar9, (int)(param_3 + 5));
                    PF(4) = (float)((long double)PF(4) - fVar12);
                }
                break;
            case 0x18:
                iVar11 = (int)param_2[2];
                local_128 = (int)((unsigned int)_DAT_005233a0 + 1u);
                if (iVar11 < (int)((unsigned int)_DAT_005233a0 + 1u)) {
LAB_004733f4:
                    local_128 = iVar11;
                }
            }
        }
        if (param_2[2] == 0) {
            return;
        }
        if (local_128 == 0) {
            if (iVar10 == 0) {
                puVar4 = local_d0;
                for (iVar10 = 0x34; iVar10 != 0; iVar10 = (int)((unsigned int)iVar10 - 1u)) {
                    *puVar4 = *param_3;
                    param_3 = param_3 + 1;
                    puVar4 = puVar4 + 1;
                }
                iVar11 = 0;
                DAT_0060f1c0 = 0;
                DAT_0060f1e8 = 0;
                iVar10 = 0;
                _DAT_0060f208 = (float *)0;
                _DAT_0060f1f0 = 0;
                DAT_0060f1d8 = 0;
                DAT_0060f204 = 0;
                if ((int)param_2[2] < 1) {
                    DAT_0060f1c0 = 0;
                    DAT_0060f1d8 = 0;
                    DAT_0060f1e8 = 0;
                    _DAT_0060f1f0 = 0;
                    DAT_0060f204 = 0;
                    _DAT_0060f208 = (float *)0;
                    return;
                }
                do {
                    DAT_005271d4 = (int)((unsigned int)DAT_005271d4 + 1u);
                    iVar10 = (int)((unsigned int)iVar10 + 1u);
                    iVar11 = (int)((unsigned int)iVar11 + 4u);
                    FUN_00472c90(param_1,
                                 *(unsigned int **)((unsigned int)param_2[3] - 4u + (unsigned int)iVar11),
                                 local_d0);
                } while (iVar10 < (int)param_2[2]);
                return;
            }
            if (iVar10 == -1) {
                return;
            }
            param_2 = *(unsigned int **)((unsigned int)param_2[3] - 4u + (unsigned int)iVar10 * 4u);
        }
        else {
            param_2 = *(unsigned int **)((unsigned int)param_2[3] - 4u + (unsigned int)local_128 * 4u);
        }
        DAT_005271d4 = (int)((unsigned int)DAT_005271d4 + 1u);
        if (param_2 == (unsigned int *)0) {
            return;
        }
    } while (1);
}
