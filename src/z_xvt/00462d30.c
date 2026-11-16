/* SPDX-License-Identifier: MIT */
/* z_xvt 00462d30 */
extern int DAT_00525518;
extern ushort DAT_006fedd2;
extern ushort DAT_006fedd8;
extern byte DAT_0071f2ea;
extern byte DAT_0071f2cd;
extern int *DAT_005f1d20;
extern int DAT_0052552c;
extern uint DAT_00525534;
extern uint DAT_0052550c;

void __cdecl FUN_00415b30(byte *param_1, ushort param_2, ushort param_3, uint param_4, int param_5);

void __cdecl FUN_00462d30(byte *param_1, ushort param_2, ushort param_3, uint param_4, int param_5)
{
    byte bVar1;
    char cVar2;
    uint uVar3;
    char *pcVar4;
    byte *pbVar5;
    char *pcVar6;
    ushort uStack_a;
    byte bStack_4;
    uint uFill;

    if (DAT_00525518 == 2) {
        FUN_00415b30(param_1, param_2, param_3, param_4, param_5);
        return;
    }
    DAT_006fedd2 = param_2;
    DAT_006fedd8 = param_3;
    DAT_0071f2ea = (byte)param_4;
    do {
        uVar3 = (uint)DAT_005f1d20[(uint)DAT_006fedd8] + (uint)DAT_006fedd2;
        if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
            uVar3 = uVar3 % DAT_0052550c;
        }
        pcVar4 = (char *)(uVar3 + DAT_00525534);
LAB_00462dec:
        while (1) {
            pbVar5 = param_1 + 1;
            bVar1 = *param_1;
            if (0xfa < bVar1) break;
            uStack_a = (ushort)(bVar1 & 3);
            bStack_4 = (byte)((bVar1 >> 2) + DAT_0071f2cd);
            param_1 = pbVar5;
LAB_00462f1f:
            uStack_a = (ushort)(uStack_a + 1);
            if (bStack_4 == param_4) {
                if (param_5 == 0) {
                    pcVar4 = pcVar4 + uStack_a;
                } else {
                    pcVar4 = pcVar4 - (uint)uStack_a;
                }
            } else if (param_5 == 0) {
                if (uStack_a != 0) {
                    cVar2 = (char)(bStack_4 + 4);
                    pcVar6 = pcVar4;
                    uFill = (uint)(byte)cVar2 * 0x01010101u;
                    for (uVar3 = (uint)(uStack_a >> 2); uVar3 != 0; uVar3 = uVar3 - 1) {
                        __builtin_memcpy(pcVar6, &uFill, 4);
                        pcVar6 = pcVar6 + 4;
                    }
                    for (uVar3 = uStack_a & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                        *pcVar6 = cVar2;
                        pcVar6 = pcVar6 + 1;
                    }
                    pcVar4 = pcVar4 + uStack_a;
                }
            } else if (uStack_a != 0) {
                pcVar6 = pcVar4;
                for (uVar3 = (uint)uStack_a; uVar3 != 0; uVar3 = uVar3 - 1) {
                    *pcVar6 = (char)(bStack_4 + 4);
                    pcVar6 = pcVar6 - 1;
                }
                pcVar4 = pcVar4 - (uint)uStack_a;
            }
        }
        if (bVar1 < 0xfc) {
            param_1 = param_1 + 2;
            DAT_0071f2cd = *pbVar5;
            goto LAB_00462dec;
        }
        if (bVar1 == 0xfc) {
            bVar1 = *pbVar5;
            pbVar5 = param_1 + 2;
            param_1 = param_1 + 3;
            uStack_a = (ushort)(*pbVar5 + 1);
            if (uStack_a != 0) {
                do {
                    *pcVar4 = (char)(bVar1 + 4);
                    if (param_5 == 0) {
                        pcVar4 = pcVar4 + 1;
                    } else {
                        pcVar4 = pcVar4 - 1;
                    }
                    if (0 < (short)(ushort)(uStack_a - 1)) {
                        *pcVar4 = (char)(bVar1 + 5);
                        if (param_5 == 0) {
                            pcVar4 = pcVar4 + 1;
                        } else {
                            pcVar4 = pcVar4 - 1;
                        }
                    }
                    uStack_a = (ushort)(uStack_a - 2);
                } while (0 < (short)uStack_a);
            }
            goto LAB_00462dec;
        }
        if (bVar1 == 0xfd) {
            uStack_a = (ushort)*pbVar5;
            bStack_4 = param_1[2];
            param_1 = param_1 + 3;
            goto LAB_00462f1f;
        }
        if (bVar1 == 0xff) {
            return;
        }
        DAT_006fedd8 = (ushort)(DAT_006fedd8 + 1);
        param_1 = pbVar5;
    } while (1);
}
