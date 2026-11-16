/* SPDX-License-Identifier: MIT */
/* z_xvt 004d0bc0 */
extern int DAT_00758830;
extern int DAT_0075964b;   /* unaligned address, read via memcpy */
extern int DAT_00759657;   /* unaligned address, read via memcpy */
extern char DAT_0075965b;

void FUN_004dd000(int *param_1, int *param_2);
void FUN_004dd030(int *param_1, int param_2, int param_3);
uint FUN_004dd0b0(int *param_1);

uint __cdecl FUN_004d0bc0(int *param_1, int *param_2, int param_3, int param_4, uint param_5)
{
    int iVar1;
    int iVar2;
    uint uVar3;
    uint uVar4;
    uint uVar5;
    int iVar6;
    unsigned int iStack_30;
    unsigned int iStack_2c;
    int aStack_24[4];   /* left, top, right, bottom */
    int iStack_14;
    int aStack_10[4];
    int iGlobal_64b;
    int iGlobal_657;
    int iGlobal_830;

    if (param_1 == (int *)0) {
        return 0;
    }
    FUN_004dd000(aStack_24, param_2);
    FUN_004dd030(aStack_24, (int)((unsigned int)param_3 - (unsigned int)*param_2),
                 (int)((unsigned int)param_4 - (unsigned int)param_2[1]));
    FUN_004dd000(aStack_10, aStack_24);
    uVar3 = FUN_004dd0b0(aStack_24);
    if (aStack_24[0] <= aStack_24[2]) {
        if (aStack_24[3] < aStack_24[1]) {
            return uVar3;
        }
        aStack_24[0] = (int)((unsigned int)aStack_24[0] - (unsigned int)aStack_10[0]);
        iStack_30 = (unsigned int)*param_2;
        aStack_24[1] = (int)((unsigned int)aStack_24[1] - (unsigned int)aStack_10[1]);
        aStack_24[2] = (int)((((unsigned int)param_2[2] - (unsigned int)aStack_24[0]) - iStack_30
                              - (unsigned int)aStack_10[2]) + 1u + (unsigned int)aStack_24[2]);
        iVar6 = param_2[1];
        iStack_14 = (int)((((unsigned int)param_2[3] - (unsigned int)aStack_24[1]) - (unsigned int)iVar6
                           - (unsigned int)aStack_10[3]) + 1u + (unsigned int)aStack_24[3]);

        __builtin_memcpy(&iGlobal_657, &DAT_00759657, 4);
        __builtin_memcpy(&iGlobal_64b, &DAT_0075964b, 4);
        iGlobal_830 = DAT_00758830;

        if (iGlobal_657 == 8) {
            iStack_30 = ((unsigned int)iVar6 + (unsigned int)aStack_24[1]) * (unsigned int)*param_1
                        + (unsigned int)param_1[4] + (unsigned int)aStack_24[0] + iStack_30;
            iStack_2c = ((unsigned int)param_4 + (unsigned int)aStack_24[1]) * (unsigned int)iGlobal_64b
                        + (unsigned int)param_3 + (unsigned int)iGlobal_830 + (unsigned int)aStack_24[0];
            if (0 < iStack_14) {
                do {
                    iVar6 = 0;
                    if (0 < aStack_24[2]) {
                        do {
                            if (*(char *)(iStack_30 + (unsigned int)iVar6) != '\0') {
                                *(unsigned char *)(iStack_2c + (unsigned int)iVar6) = (unsigned char)param_5;
                            }
                            iVar6 = iVar6 + 1;
                        } while (iVar6 < aStack_24[2]);
                    }
                    iStack_2c = iStack_2c + (unsigned int)iGlobal_64b;
                    iStack_30 = iStack_30 + (unsigned int)*param_1;
                    iStack_14 = iStack_14 + -1;
                } while (iStack_14 != 0);
            }
        }
        else if (iGlobal_657 == 0x10) {
            iStack_30 = ((unsigned int)iVar6 + (unsigned int)aStack_24[1]) * (unsigned int)*param_1
                        + (unsigned int)param_1[4] + (unsigned int)aStack_24[0] + iStack_30;
            iStack_2c = ((unsigned int)param_4 + (unsigned int)aStack_24[1]) * (unsigned int)iGlobal_64b
                        + ((unsigned int)aStack_24[0] + (unsigned int)param_3) * 2u
                        + (unsigned int)iGlobal_830;
            if (0 < iStack_14) {
                do {
                    iVar6 = 0;
                    if (0 < aStack_24[2]) {
                        do {
                            if (*(unsigned char *)(iStack_30 + (unsigned int)iVar6) != 0) {
                                uVar5 = (uint)param_1[*(unsigned char *)(iStack_30 + (unsigned int)iVar6) + 5] & 0x1f;
                                if (DAT_0075965b == '\0') {
                                    iVar2 = 0xb;
                                    uVar4 = param_5 & 0x7e0;
                                }
                                else {
                                    iVar2 = 10;
                                    uVar4 = param_5 & 0x3e0;
                                }
                                if (DAT_0075965b == '\0') {
                                    iVar1 = 6;
                                }
                                else {
                                    iVar1 = 5;
                                }
                                *(unsigned short *)(iStack_2c + (unsigned int)iVar6 * 2u) =
                                    (unsigned short)(
                                        (unsigned int)((unsigned long long)(uVar5 * (param_5 & 0x1f)) / 0x1f)
                                        + ((unsigned int)((unsigned short)((unsigned int)((int)((param_5 >> iVar2) * uVar5) / 0x1f) << iVar1))
                                           + (unsigned int)((unsigned short)(((uVar4 >> 5) * uVar5) / 0x1f))) * 0x20u);
                            }
                            iVar6 = iVar6 + 1;
                        } while (iVar6 < aStack_24[2]);
                    }
                    iStack_30 = iStack_30 + (unsigned int)*param_1;
                    iStack_2c = iStack_2c + ((unsigned int)iGlobal_64b & 0xfffffffeu);
                    iStack_14 = iStack_14 + -1;
                } while (iStack_14 != 0);
            }
        }
    }
    return uVar3;
}
