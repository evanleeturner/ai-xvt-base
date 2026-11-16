/* SPDX-License-Identifier: MIT */
/* z_xvt 00462b20 */
extern ushort DAT_006fedd2;
extern ushort DAT_006fedd8;
extern byte DAT_0071f2ea;
extern byte DAT_0071f2cd;
extern int *DAT_005f1d20;
extern int DAT_0052552c;
extern uint DAT_00525534;
extern uint DAT_0052550c;

void __cdecl
FUN_00462b20(byte *param_1, ushort param_2, ushort param_3, uint param_4, int param_5, char param_6,
             short param_7)
{
    byte bVar1;
    byte bVar2;
    short sVar3;
    uint uVar4;
    byte *pbVar5;
    byte *pbVar6;
    byte bVar7;
    ushort uVar8;
    byte *pbVar9;
    byte *pbVar10;
    uint uFill;

    DAT_006fedd2 = param_2;
    DAT_006fedd8 = param_3;
    DAT_0071f2ea = (byte)param_4;
    do {
        uVar4 = (uint)DAT_005f1d20[(uint)DAT_006fedd8] + (uint)DAT_006fedd2;
        if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
            uVar4 = uVar4 % DAT_0052550c;
        }
        pbVar5 = (byte *)(uVar4 + DAT_00525534);
        pbVar9 = param_1;
LAB_00462b8f:
        while (1) {
            bVar2 = *pbVar9;
            param_1 = pbVar9 + 1;
            if (0xfa < bVar2) break;
            bVar1 = (byte)(bVar2 >> 2);
            if (param_6 == '\0') {
                bVar1 = (byte)(bVar1 + DAT_0071f2cd);
            }
            uVar8 = (ushort)(bVar2 & 3);
LAB_00462c49:
            uVar8 = (ushort)(uVar8 + 1);
            pbVar9 = param_1;
            if ((uint)bVar1 == param_4) {
                if (param_5 == 0) {
                    pbVar5 = pbVar5 + uVar8;
                } else {
                    pbVar5 = pbVar5 - (uint)uVar8;
                }
            } else {
                bVar2 = bVar1;
                if ((param_6 != '\0') && (bVar2 = DAT_0071f2cd, 0 < param_7)) {
                    bVar2 = (byte)((byte)(bVar1 - (char)param_7) + DAT_0071f2cd);
                }
                if (param_5 == 0) {
                    if (uVar8 != 0) {
                        pbVar10 = pbVar5;
                        uFill = (uint)bVar2 * 0x01010101u;
                        for (uVar4 = (uint)(uVar8 >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                            __builtin_memcpy(pbVar10, &uFill, 4);
                            pbVar10 = pbVar10 + 4;
                        }
                        pbVar5 = pbVar5 + uVar8;
                        for (uVar4 = (uint)(uVar8 & 3); uVar4 != 0; uVar4 = uVar4 - 1) {
                            *pbVar10 = bVar2;
                            pbVar10 = pbVar10 + 1;
                        }
                    }
                } else if (uVar8 != 0) {
                    uVar4 = (uint)uVar8;
                    pbVar6 = pbVar5 - uVar4;
                    pbVar10 = pbVar5;
                    for (; pbVar5 = pbVar6, uVar4 != 0; uVar4 = uVar4 - 1) {
                        *pbVar10 = bVar2;
                        pbVar10 = pbVar10 - 1;
                    }
                }
            }
        }
        if (bVar2 < 0xfc) {
            if (param_6 == '\0') {
                DAT_0071f2cd = *param_1;
                pbVar9 = pbVar9 + 2;
            } else {
                pbVar9 = pbVar9 + 2;
            }
            goto LAB_00462b8f;
        }
        if (bVar2 == 0xfc) {
            bVar2 = *param_1;
            if (param_6 == '\0') {
LAB_00462bf1:
                bVar7 = (byte)(bVar2 + 1);
                bVar1 = bVar2;
            } else {
                bVar7 = DAT_0071f2cd;
                bVar1 = DAT_0071f2cd;
                if (0 < param_7) {
                    bVar2 = (byte)((byte)(bVar2 - (char)param_7) + DAT_0071f2cd);
                    goto LAB_00462bf1;
                }
            }
            pbVar10 = pbVar9 + 3;
            sVar3 = (short)(pbVar9[2] + 1);
            pbVar9 = pbVar10;
            if (sVar3 != 0) {
                do {
                    *pbVar5 = bVar1;
                    if (param_5 == 0) {
                        pbVar5 = pbVar5 + 1;
                    } else {
                        pbVar5 = pbVar5 - 1;
                    }
                    if (0 < (short)(sVar3 - 1)) {
                        *pbVar5 = bVar7;
                        if (param_5 == 0) {
                            pbVar5 = pbVar5 + 1;
                        } else {
                            pbVar5 = pbVar5 - 1;
                        }
                    }
                    sVar3 = (short)(sVar3 - 2);
                } while (0 < sVar3);
            }
            goto LAB_00462b8f;
        }
        if (bVar2 == 0xfd) {
            bVar1 = pbVar9[2];
            uVar8 = (ushort)*param_1;
            param_1 = pbVar9 + 3;
            goto LAB_00462c49;
        }
        if (bVar2 == 0xff) {
            return;
        }
        DAT_006fedd8 = (ushort)(DAT_006fedd8 + 1);
    } while (1);
}
