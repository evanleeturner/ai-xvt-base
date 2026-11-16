/* SPDX-License-Identifier: MIT */
/* z_xvt 004b2f40 */
extern unsigned short DAT_00527e74;
extern unsigned int DAT_006da9f4;
extern int DAT_006da538;
extern int DAT_0052ae18;

void FUN_0045f5b0(void);

void __cdecl FUN_004b2f40(unsigned char *param_1, unsigned short param_2, short param_3, short param_4)
{
    unsigned char bVar1;
    unsigned char bVar2;
    unsigned char *pbVar3;
    unsigned char *pbVar4;
    unsigned short uVar5;
    unsigned short uVar6;
    unsigned char *pbVar7;
    unsigned short uVar8;
    unsigned short uVar9;
    unsigned char *pbVar10;
    short local_68;
    unsigned char local_63[99];

    pbVar3 = (unsigned char *)((unsigned int)DAT_00527e74 + DAT_006da9f4);
    if (param_3 != 0) {
        local_68 = param_3;
        do {
            bVar1 = *param_1;
            if (param_4 == 0) {
                uVar5 = 0;
                *pbVar3 = bVar1;
                pbVar10 = param_1 + 1;
                pbVar3 = pbVar3 + 1;
                param_1 = pbVar10;
                if (param_2 != 0) {
                    do {
                        bVar1 = *pbVar10;
                        param_1 = pbVar10 + 1;
                        if (bVar1 == 0) {
                            *pbVar3 = 0;
                            bVar1 = *param_1;
                            pbVar4 = pbVar3 + 1;
                            param_1 = pbVar10 + 2;
                            uVar9 = (unsigned short)(uVar5 + 0xff);
                            if (DAT_006da538 != 0x140) {
                                if (bVar1 == 0) {
                                    *pbVar4 = 0;
                                    bVar1 = *param_1;
                                    param_1 = pbVar10 + 3;
                                    pbVar4 = pbVar3 + 2;
                                    uVar9 = (unsigned short)(uVar5 + 0x1ff);
                                }
                                else if (bVar1 == 0xff) {
                                    uVar5 = (unsigned short)(uVar5 + 0x1ff);
                                    *pbVar4 = 0;
                                    pbVar3 = pbVar3 + 2;
                                    bVar1 = 0;
                                    goto LAB_004b2fd8;
                                }
                            }
                            uVar5 = uVar9;
                            pbVar3 = pbVar4;
                            bVar1 = (unsigned char)(bVar1 + 1);
                        }
LAB_004b2fd8:
                        *pbVar3 = bVar1;
                        pbVar3 = pbVar3 + 1;
                        uVar5 = (unsigned short)(uVar5 + bVar1);
                        pbVar10 = param_1;
                    } while (uVar5 < param_2);
                }
            }
            else {
                uVar5 = 0;
                param_1 = param_1 + 1;
                pbVar10 = local_63;
                pbVar4 = param_1;
                if (param_2 != 0) {
                    do {
                        bVar2 = *pbVar4;
                        param_1 = pbVar4 + 1;
                        if (bVar2 == 0) {
                            *pbVar10 = 0;
                            bVar2 = *param_1;
                            pbVar7 = pbVar10 + 1;
                            param_1 = pbVar4 + 2;
                            uVar9 = (unsigned short)(uVar5 + 0xff);
                            if (DAT_006da538 != 0x140) {
                                if (bVar2 == 0) {
                                    *pbVar7 = 0;
                                    bVar2 = *param_1;
                                    param_1 = pbVar4 + 3;
                                    pbVar7 = pbVar10 + 2;
                                    uVar9 = (unsigned short)(uVar5 + 0x1ff);
                                }
                                else if (bVar2 == 0xff) {
                                    uVar5 = (unsigned short)(uVar5 + 0x1ff);
                                    *pbVar7 = 0;
                                    pbVar10 = pbVar10 + 2;
                                    bVar2 = 0;
                                    goto LAB_004b3045;
                                }
                            }
                            uVar5 = uVar9;
                            pbVar10 = pbVar7;
                            bVar2 = (unsigned char)(bVar2 + 1);
                        }
LAB_004b3045:
                        *pbVar10 = bVar2;
                        pbVar10 = pbVar10 + 1;
                        uVar5 = (unsigned short)(uVar5 + bVar2);
                        pbVar4 = param_1;
                    } while (uVar5 < param_2);
                }
                uVar9 = 0;
                pbVar10 = local_63;
                *pbVar3 = bVar1;
                uVar5 = 0;
                pbVar4 = pbVar10;
                if (param_2 != 0) {
                    do {
                        uVar6 = (unsigned short)*pbVar4;
                        pbVar10 = pbVar4 + 1;
                        uVar8 = uVar9;
                        if (*pbVar4 == 0) {
                            uVar8 = (unsigned short)(uVar9 + 0xff);
                            bVar1 = *pbVar10;
                            uVar6 = (unsigned short)bVar1;
                            *pbVar4 = bVar1;
                            pbVar10 = pbVar4 + 2;
                            pbVar4[1] = 0;
                            if (bVar1 == 0) {
                                uVar8 = (unsigned short)(uVar9 + 0x1ff);
                                uVar6 = (unsigned short)*pbVar10;
                                *pbVar4 = *pbVar10;
                                pbVar10 = pbVar4 + 3;
                                pbVar4[2] = 0;
                            }
                        }
                        uVar5 = (unsigned short)(uVar5 + 1);
                        uVar9 = (unsigned short)(uVar8 + uVar6);
                        pbVar4 = pbVar10;
                    } while (uVar9 < param_2);
                }
                if ((uVar5 & 1) == 0) {
                    *pbVar3 = (unsigned char)(0u - (unsigned int)*pbVar3);
                }
                pbVar10 = pbVar10 + -1;
                pbVar3 = pbVar3 + 1;
                for (; uVar5 != 0; uVar5 = (unsigned short)(uVar5 - 1)) {
                    bVar1 = *pbVar10;
                    pbVar7 = pbVar10 + -1;
                    *pbVar3 = bVar1;
                    pbVar4 = pbVar3 + 1;
                    if (bVar1 == 0) {
                        bVar1 = *pbVar7;
                        pbVar7 = pbVar10 + -2;
                        *pbVar4 = bVar1;
                        pbVar4 = pbVar3 + 2;
                        if (bVar1 == 0) {
                            bVar1 = *pbVar7;
                            pbVar7 = pbVar10 + -3;
                            *pbVar4 = bVar1;
                            pbVar4 = pbVar3 + 3;
                        }
                    }
                    pbVar10 = pbVar7;
                    pbVar3 = pbVar4;
                }
            }
            local_68 = (short)(local_68 + -1);
        } while (local_68 != 0);
    }
    if (DAT_0052ae18 != 0) {
        FUN_0045f5b0();
    }
    return;
}
