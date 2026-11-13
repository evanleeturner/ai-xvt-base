/* SPDX-License-Identifier: MIT */
/* z_xvt 00465eb0 */
extern byte DAT_00732e8f;
extern int DAT_006db7b4;
extern int DAT_00732f3c;
extern int DAT_00525518;
extern byte DAT_00703ae0;
extern byte DAT_007091b0;
extern byte DAT_006db5c0;

void __cdecl FUN_004a6a80(void);
void __cdecl FUN_004774e0(int, int, int, int, float);
void __stdcall FUN_004a6f90(void);

void __cdecl FUN_00465eb0(byte *param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    int iVar1;
    float fVar3;
    int iVar4;
    int iVar5;
    byte *pbVar6;
    byte *pbVar7;
    float uVar2;

    if (DAT_00732e8f == 0) {
        FUN_004a6a80();
    }
    fVar3 = (float)DAT_006db7b4;
    uVar2 = (float)DAT_00732f3c;
    if (DAT_00525518 == 2) {
        for (; param_4 < param_6; param_4 = param_4 + 1) {
            if (param_3 < param_5) {
                iVar1 = param_3 - param_5;
                iVar4 = param_3;
                iVar5 = param_3;
                pbVar6 = param_1;
                if (param_5 <= param_3) goto LAB_00465f30;
LAB_00465f1f:
                do {
                    if (param_1[1] != 0x80) {
                        param_1 = param_1 + 2;
                        iVar4 = iVar4 + 1;
                        if (iVar4 < param_5) goto LAB_00465f1f;
                    }
                    iVar1 = iVar4 - param_5;
                    iVar5 = iVar4;
                    pbVar6 = param_1;
LAB_00465f30:
                    param_1 = pbVar6;
                    if (iVar5 == param_5) break;
                    iVar4 = iVar5;
                    pbVar7 = pbVar6;
                    if ((iVar5 < param_5) != (iVar1 < 0)) {
                        do {
                            param_1 = pbVar7;
                            if (pbVar7[1] != 0x80) break;
                            param_1 = pbVar7 + 2;
                            iVar4 = iVar4 + 1;
                            pbVar7[1] = (&DAT_00703ae0)[*pbVar7];
                            *pbVar7 = (&DAT_007091b0)[*pbVar7];
                            pbVar7 = param_1;
                        } while (iVar4 < param_5);
                    }
                    FUN_004774e0((int)pbVar6, iVar5, iVar4, param_4, uVar2 / fVar3);
                } while (iVar4 < param_5);
            }
            param_1 = param_1 + param_2;
        }
    }
    else {
        for (; param_4 < param_6; param_4 = param_4 + 1) {
            if (param_3 < param_5) {
                iVar1 = param_3 - param_5;
                iVar4 = param_3;
                iVar5 = param_3;
                pbVar6 = param_1;
                if (param_5 <= param_3) goto LAB_00465fb3;
LAB_00465fa5:
                do {
                    if (0x3f < *param_1) {
                        param_1 = param_1 + 1;
                        iVar4 = iVar4 + 1;
                        if (iVar4 < param_5) goto LAB_00465fa5;
                    }
                    iVar1 = iVar4 - param_5;
                    iVar5 = iVar4;
                    pbVar6 = param_1;
LAB_00465fb3:
                    param_1 = pbVar6;
                    if (iVar5 == param_5) break;
                    iVar4 = iVar5;
                    pbVar7 = pbVar6;
                    if ((iVar5 < param_5) != (iVar1 < 0)) {
                        do {
                            param_1 = pbVar7;
                            if (0x3f < *pbVar7) break;
                            param_1 = pbVar7 + 1;
                            iVar4 = iVar4 + 1;
                            *pbVar7 = (&DAT_006db5c0)[*pbVar7];
                            pbVar7 = param_1;
                        } while (iVar4 < param_5);
                    }
                    FUN_004774e0((int)pbVar6, iVar5, iVar4, param_4, uVar2 / fVar3);
                } while (iVar4 < param_5);
            }
            param_1 = param_1 + param_2;
        }
    }
    if (DAT_00732e8f == 0) {
        FUN_004a6f90();
    }
    return;
}
