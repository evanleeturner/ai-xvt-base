/* SPDX-License-Identifier: MIT */
/* z_bop 00442090 */
extern unsigned short DAT_0052747c;
extern unsigned int   DAT_009a8074;
extern int            DAT_009a7bac;
extern int            DAT_00527e90;

void __cdecl FUN_0040b520(void);

void __cdecl FUN_00442090(unsigned char *param_1, unsigned short param_2, short param_3, short param_4)
{
    unsigned char bVar1;
    unsigned char bVar2;
    unsigned char *pbVar3;
    unsigned char *pbVar4;
    unsigned char *pbVar5;
    unsigned short uVar6;
    unsigned short uVar7;
    unsigned short uVar8;
    unsigned short uVar9;
    unsigned char *pbVar10;
    short local_68;
    unsigned char local_63[99];

    pbVar4 = (unsigned char *)((unsigned int)DAT_0052747c + DAT_009a8074);
    if (param_3 != 0) {
        local_68 = param_3;
        do {
            bVar1 = *param_1;
            if (param_4 == 0) {
                uVar9 = 0;
                *pbVar4 = bVar1;
                pbVar3 = param_1 + 1;
                pbVar4 = pbVar4 + 1;
                param_1 = pbVar3;
                if (param_2 != 0) {
                    do {
                        bVar1 = *pbVar3;
                        param_1 = pbVar3 + 1;
                        if (bVar1 == 0) {
                            *pbVar4 = 0;
                            bVar1 = *param_1;
                            pbVar5 = pbVar4 + 1;
                            param_1 = pbVar3 + 2;
                            uVar8 = (unsigned short)(uVar9 + 0xff);
                            if (DAT_009a7bac != 0x140) {
                                if (bVar1 == 0) {
                                    *pbVar5 = 0;
                                    bVar1 = *param_1;
                                    param_1 = pbVar3 + 3;
                                    pbVar5 = pbVar4 + 2;
                                    uVar8 = (unsigned short)(uVar9 + 0x1ff);
                                }
                                else if (bVar1 == 0xff) {
                                    uVar9 = (unsigned short)(uVar9 + 0x1ff);
                                    *pbVar5 = 0;
                                    pbVar4 = pbVar4 + 2;
                                    bVar1 = 0;
                                    goto LAB_00442128;
                                }
                            }
                            uVar9 = uVar8;
                            pbVar4 = pbVar5;
                            bVar1 = (unsigned char)(bVar1 + 1);
                        }
LAB_00442128:
                        *pbVar4 = bVar1;
                        pbVar4 = pbVar4 + 1;
                        uVar9 = (unsigned short)(uVar9 + bVar1);
                        pbVar3 = param_1;
                    } while (uVar9 < param_2);
                }
            }
            else {
                uVar9 = 0;
                param_1 = param_1 + 1;
                pbVar3 = local_63;
                pbVar5 = param_1;
                if (param_2 != 0) {
                    do {
                        bVar2 = *pbVar5;
                        param_1 = pbVar5 + 1;
                        if (bVar2 == 0) {
                            *pbVar3 = 0;
                            bVar2 = *param_1;
                            pbVar10 = pbVar3 + 1;
                            param_1 = pbVar5 + 2;
                            uVar8 = (unsigned short)(uVar9 + 0xff);
                            if (DAT_009a7bac != 0x140) {
                                if (bVar2 == 0) {
                                    *pbVar10 = 0;
                                    bVar2 = *param_1;
                                    param_1 = pbVar5 + 3;
                                    pbVar10 = pbVar3 + 2;
                                    uVar8 = (unsigned short)(uVar9 + 0x1ff);
                                }
                                else if (bVar2 == 0xff) {
                                    uVar9 = (unsigned short)(uVar9 + 0x1ff);
                                    *pbVar10 = 0;
                                    pbVar3 = pbVar3 + 2;
                                    bVar2 = 0;
                                    goto LAB_00442195;
                                }
                            }
                            uVar9 = uVar8;
                            pbVar3 = pbVar10;
                            bVar2 = (unsigned char)(bVar2 + 1);
                        }
LAB_00442195:
                        *pbVar3 = bVar2;
                        pbVar3 = pbVar3 + 1;
                        uVar9 = (unsigned short)(uVar9 + bVar2);
                        pbVar5 = param_1;
                    } while (uVar9 < param_2);
                }
                uVar8 = 0;
                pbVar3 = local_63;
                *pbVar4 = bVar1;
                uVar9 = 0;
                pbVar5 = pbVar3;
                if (param_2 != 0) {
                    do {
                        uVar6 = (unsigned short)*pbVar5;
                        pbVar3 = pbVar5 + 1;
                        uVar7 = uVar8;
                        if (*pbVar5 == 0) {
                            uVar7 = (unsigned short)(uVar8 + 0xff);
                            bVar1 = *pbVar3;
                            uVar6 = (unsigned short)bVar1;
                            *pbVar5 = bVar1;
                            pbVar3 = pbVar5 + 2;
                            pbVar5[1] = 0;
                            if (bVar1 == 0) {
                                uVar7 = (unsigned short)(uVar8 + 0x1ff);
                                uVar6 = (unsigned short)*pbVar3;
                                *pbVar5 = *pbVar3;
                                pbVar3 = pbVar5 + 3;
                                pbVar5[2] = 0;
                            }
                        }
                        uVar9 = (unsigned short)(uVar9 + 1);
                        uVar8 = (unsigned short)(uVar7 + uVar6);
                        pbVar5 = pbVar3;
                    } while (uVar8 < param_2);
                }
                if ((uVar9 & 1) == 0) {
                    *pbVar4 = (unsigned char)(0u - (unsigned int)*pbVar4);
                }
                pbVar3 = pbVar3 + -1;
                pbVar4 = pbVar4 + 1;
                for (; uVar9 != 0; uVar9 = (unsigned short)(uVar9 - 1)) {
                    bVar1 = *pbVar3;
                    pbVar10 = pbVar3 + -1;
                    *pbVar4 = bVar1;
                    pbVar5 = pbVar4 + 1;
                    if (bVar1 == 0) {
                        bVar1 = *pbVar10;
                        pbVar10 = pbVar3 + -2;
                        *pbVar5 = bVar1;
                        pbVar5 = pbVar4 + 2;
                        if (bVar1 == 0) {
                            bVar1 = *pbVar10;
                            pbVar10 = pbVar3 + -3;
                            *pbVar5 = bVar1;
                            pbVar5 = pbVar4 + 3;
                        }
                    }
                    pbVar3 = pbVar10;
                    pbVar4 = pbVar5;
                }
            }
            local_68 = (short)(local_68 - 1);
        } while (local_68 != 0);
    }
    if (DAT_00527e90 != 0) {
        FUN_0040b520();
    }
    return;
}
