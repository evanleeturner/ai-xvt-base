/* SPDX-License-Identifier: MIT */
/* z_xvt 004d05d0 */
extern int DAT_00758830;
extern int DAT_0075964b;   /* packed: address not 4-aligned, read via memcpy */
extern int DAT_00759657;   /* packed: address not 4-aligned, read via memcpy */
extern char DAT_0075965b;

void FUN_004dd000(int *param_1, int *param_2);
unsigned int FUN_004dd0b0(int *param_1);

unsigned int __cdecl FUN_004d05d0(int *param_1, int param_2, int param_3)
{
    unsigned short uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned short *puVar4;
    int iVar5;
    unsigned int iVar6;
    unsigned int iVar7;
    unsigned int iVar8;
    unsigned int iStack_2c;
    unsigned short *puStack_28;
    int iStack_24;
    int aiStack_20[4];
    int aiStack_10[4];
    int l_pitch;
    int l_bpp;

    if (param_1 == (int *)0) {
        return 0;
    }
    aiStack_20[0] = param_2;
    aiStack_20[1] = param_3;
    aiStack_20[2] = (int)((unsigned int)*param_1 + (unsigned int)param_2 - 1u);
    aiStack_20[3] = (int)((unsigned int)param_1[1] + (unsigned int)param_3 - 1u);
    FUN_004dd000(aiStack_10, aiStack_20);
    uVar2 = FUN_004dd0b0(aiStack_20);
    if ((aiStack_20[0] <= aiStack_20[2]) && (aiStack_20[1] <= aiStack_20[3])) {
        aiStack_20[0] = (int)((unsigned int)aiStack_20[0] - (unsigned int)aiStack_10[0]);
        aiStack_20[1] = (int)((unsigned int)aiStack_20[1] - (unsigned int)aiStack_10[1]);
        iVar8 = (unsigned int)*param_1;
        iVar5 = (int)((((unsigned int)aiStack_20[2] - (unsigned int)aiStack_20[0]) - (unsigned int)aiStack_10[2]) + iVar8);
        iStack_24 = (int)((((unsigned int)param_1[1] - (unsigned int)aiStack_20[1]) - (unsigned int)aiStack_10[3]) + (unsigned int)aiStack_20[3]);
        __builtin_memcpy(&l_bpp, &DAT_00759657, 4);
        __builtin_memcpy(&l_pitch, &DAT_0075964b, 4);
        if (l_bpp == 8) {
            iVar8 = (unsigned int)aiStack_20[1] * iVar8 + (unsigned int)param_1[4] + (unsigned int)aiStack_20[0];
            iVar6 = (unsigned int)param_2 + ((unsigned int)aiStack_20[1] + (unsigned int)param_3) * (unsigned int)l_pitch
                    + (unsigned int)DAT_00758830 + (unsigned int)aiStack_20[0];
            if (0 < iStack_24) {
                do {
                    iVar7 = 0;
                    if (0 < iVar5) {
                        do {
                            if (*(char *)(iVar8 + iVar7) != '\0') {
                                *(char *)(iVar6 + iVar7) = *(char *)(iVar8 + iVar7);
                            }
                            iVar7 = iVar7 + 1;
                        } while ((int)iVar7 < iVar5);
                    }
                    iVar6 = iVar6 + (unsigned int)l_pitch;
                    iVar8 = iVar8 + (unsigned int)*param_1;
                    iStack_24 = iStack_24 - 1;
                } while (iStack_24 != 0);
            }
        }
        else if (l_bpp == 0x10) {
            iStack_2c = (unsigned int)aiStack_20[1] * iVar8 + (unsigned int)param_1[4] + (unsigned int)aiStack_20[0];
            puStack_28 = (unsigned short *)(((unsigned int)aiStack_20[1] + (unsigned int)param_3) * (unsigned int)l_pitch
                         + ((unsigned int)aiStack_20[0] + (unsigned int)param_2) * 2u + (unsigned int)DAT_00758830);
            if (DAT_0075965b == '\0') {
                if (0 < iStack_24) {
                    do {
                        iVar8 = 0;
                        puVar4 = puStack_28;
                        if (0 < iVar5) {
                            do {
                                if (*(unsigned char *)(iStack_2c + iVar8) != 0) {
                                    uVar3 = (unsigned int)param_1[*(unsigned char *)(iStack_2c + iVar8) + 5];
                                    uVar1 = *puVar4;
                                    uVar3 = ((((unsigned int)(uVar1 & 0x1f) + (((unsigned int)(uVar1 & 0x7e0) + (unsigned int)(uVar1 & 0xf800) * 4u) * 8u)) >> 1)
                                            + ((((uVar3 & 0xf800u) * 0x20u + (uVar3 & 0x7e0u) * 8u + (uVar3 & 0x1fu))) >> 1));
                                    *puVar4 = (unsigned short)(((unsigned short)(uVar3 >> 5) & 0xf800)
                                              + ((unsigned short)(uVar3 >> 3) & 0x7e0)
                                              + ((unsigned short)uVar3 & 0x1f));
                                }
                                iVar8 = iVar8 + 1;
                                puVar4 = puVar4 + 1;
                            } while ((int)iVar8 < iVar5);
                        }
                        iStack_2c = iStack_2c + (unsigned int)*param_1;
                        puStack_28 = (unsigned short *)((unsigned int)puStack_28 + ((unsigned int)l_pitch & 0xfffffffeu));
                        iStack_24 = iStack_24 - 1;
                    } while (iStack_24 != 0);
                }
            }
            else if (0 < iStack_24) {
                do {
                    iVar8 = 0;
                    puVar4 = puStack_28;
                    if (0 < iVar5) {
                        do {
                            if (*(unsigned char *)(iStack_2c + iVar8) != 0) {
                                uVar1 = *puVar4;
                                uVar3 = (unsigned int)param_1[*(unsigned char *)(iStack_2c + iVar8) + 5];
                                uVar3 = ((((uVar3 & 0x7c00u) * 0x40u + (uVar3 & 0x3e0u) * 8u + (uVar3 & 0x1fu))) >> 1)
                                        + ((((unsigned int)(uVar1 & 0x1f) + (((unsigned int)(uVar1 & 0x3e0) + (unsigned int)(uVar1 & 0x7c00) * 8u) * 8u))) >> 1);
                                *puVar4 = (unsigned short)(((unsigned short)(uVar3 >> 3) & 0x3e0)
                                          + ((unsigned short)(uVar3 >> 6) & 0x7c00)
                                          + ((unsigned short)uVar3 & 0x1f));
                            }
                            iVar8 = iVar8 + 1;
                            puVar4 = puVar4 + 1;
                        } while ((int)iVar8 < iVar5);
                    }
                    iStack_2c = iStack_2c + (unsigned int)*param_1;
                    puStack_28 = (unsigned short *)((unsigned int)puStack_28 + ((unsigned int)l_pitch & 0xfffffffeu));
                    iStack_24 = iStack_24 - 1;
                } while (iStack_24 != 0);
            }
        }
    }
    return uVar2;
}
