/* SPDX-License-Identifier: MIT */
/* z_bop 0044b140 */
extern short DAT_009a6fe0;
extern short DAT_009d8c00;
extern short DAT_00a0813c;
extern short DAT_00a07ce4;
extern int DAT_005233f4;
extern int DAT_005233d8;
extern unsigned short DAT_00a00530[];

int __cdecl FUN_0040e1a0(void);
int __cdecl FUN_00425ae0(int, int, int);

void __cdecl FUN_0044b140(int param_1, int param_2, int param_3, int param_4, unsigned char param_5)
{
    unsigned short uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned short *puVar7;
    int iVar8;
    unsigned int local_8;

    iVar5 = param_4;
    iVar6 = param_1;
    iVar4 = DAT_005233f4;
    uVar1 = DAT_00a00530[param_5];
    local_8 = (unsigned int)param_3 - (unsigned int)param_1;
    if ((int)local_8 < 0) {
        param_1 = param_3;
        local_8 = 0U - local_8;
        param_4 = param_2;
        param_2 = iVar5;
        param_3 = iVar6;
    }
    else if (local_8 == 0) {
        if (param_1 < DAT_009a6fe0) {
            return;
        }
        if (DAT_009d8c00 <= param_1) {
            return;
        }
        if (param_4 < param_2) {
            param_4 = param_2;
            param_2 = iVar5;
        }
        if (param_2 < DAT_00a0813c) {
            param_2 = (int)DAT_00a0813c;
        }
        if (DAT_00a07ce4 <= param_4) {
            param_4 = DAT_00a07ce4 - 1;
        }
        param_4 = (int)((unsigned int)param_4 - (unsigned int)param_2);
        if (param_4 < 1) {
            return;
        }
        iVar6 = FUN_0040e1a0();
        puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar6 * (unsigned int)param_2 + (unsigned int)param_1 * (unsigned int)DAT_005233d8);
        if (param_4 == 0) {
            return;
        }
        do {
            param_4 = param_4 - 1;
            *puVar7 = uVar1;
            iVar4 = FUN_0040e1a0();
            puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)iVar4);
        } while (param_4 != 0);
        return;
    }
    iVar6 = (int)DAT_009d8c00;
    if ((param_1 < iVar6) && (iVar5 = (int)DAT_009a6fe0, iVar5 <= param_3)) {
        uVar2 = (unsigned int)param_4 - (unsigned int)param_2;
        iVar8 = (int)local_8 >> 1;
        if ((int)uVar2 < 0) {
            uVar3 = 0U - uVar2;
            if ((DAT_00a0813c <= param_2) && (iVar4 = (int)DAT_00a07ce4, param_4 < iVar4)) {
                if (iVar4 <= param_2) {
                    iVar4 = FUN_00425ae0((int)((unsigned int)(param_2 - iVar4) + 1U), (int)local_8, (int)uVar3);
                    param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar4);
                    if (DAT_009d8c00 <= param_1) {
                        return;
                    }
                    param_2 = DAT_00a07ce4 - 1;
                }
                if (param_1 < DAT_009a6fe0) {
                    iVar4 = FUN_00425ae0(DAT_009a6fe0 - param_1, (int)uVar3, (int)local_8);
                    param_2 = (int)((unsigned int)param_2 - (unsigned int)iVar4);
                    if (param_2 < DAT_00a0813c) {
                        return;
                    }
                    param_1 = (int)DAT_009a6fe0;
                }
                iVar4 = DAT_005233f4;
                if (DAT_009d8c00 <= param_3) {
                    param_3 = DAT_009d8c00 - 1;
                }
                if (param_4 < DAT_00a0813c) {
                    param_4 = (int)DAT_00a0813c;
                }
                iVar6 = FUN_0040e1a0();
                puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar6 * (unsigned int)param_2 + (unsigned int)param_1 * (unsigned int)DAT_005233d8);
                if ((int)local_8 < (int)uVar3) {
                    iVar6 = (int)uVar3 >> 1;
                    iVar5 = (param_3 - param_1) + 1;
                    iVar4 = param_2 - param_4;
                    if (iVar4 != 0) {
                        do {
                            iVar4 = iVar4 - 1;
                            *puVar7 = uVar1;
                            iVar8 = FUN_0040e1a0();
                            puVar7 = (unsigned short *)((unsigned int)puVar7 - (unsigned int)iVar8);
                            iVar6 = (int)((unsigned int)iVar6 - local_8);
                            if (iVar6 < 0) {
                                iVar6 = (int)((unsigned int)iVar6 + uVar3);
                                iVar5 = iVar5 - 1;
                                if (iVar5 == 0) {
                                    return;
                                }
                                puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_005233d8);
                            }
                        } while (iVar4 != 0);
                        return;
                    }
                }
                else {
                    iVar6 = param_3 - param_1;
                    iVar4 = (param_2 - param_4) + 1;
                    if (iVar6 != 0) {
                        do {
                            iVar6 = iVar6 - 1;
                            *puVar7 = uVar1;
                            puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_005233d8);
                            iVar8 = (int)((unsigned int)iVar8 + uVar2);
                            if (iVar8 < 0) {
                                iVar8 = (int)((unsigned int)iVar8 + local_8);
                                iVar4 = iVar4 - 1;
                                if (iVar4 == 0) {
                                    return;
                                }
                                iVar5 = FUN_0040e1a0();
                                puVar7 = (unsigned short *)((unsigned int)puVar7 - (unsigned int)iVar5);
                            }
                        } while (iVar6 != 0);
                        return;
                    }
                }
            }
        }
        else if ((int)uVar2 < 1) {
            if ((DAT_00a0813c <= param_2) && (param_2 < DAT_00a07ce4)) {
                if (param_1 < iVar5) {
                    param_1 = iVar5;
                }
                if (iVar6 <= param_3) {
                    param_3 = iVar6 - 1;
                }
                iVar6 = param_3 - param_1;
                if (0 < iVar6) {
                    iVar5 = FUN_0040e1a0();
                    puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar5 * (unsigned int)param_2 + (unsigned int)param_1 * (unsigned int)DAT_005233d8);
                    for (; iVar6 != 0; iVar6 = iVar6 - 1) {
                        *puVar7 = uVar1;
                        puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_005233d8);
                    }
                }
            }
        }
        else if ((param_2 < DAT_00a07ce4) && (iVar4 = (int)DAT_00a0813c, iVar4 <= param_4)) {
            if (param_2 < iVar4) {
                iVar4 = FUN_00425ae0(iVar4 - param_2, (int)local_8, (int)uVar2);
                param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar4);
                if (DAT_009d8c00 <= param_1) {
                    return;
                }
                param_2 = (int)DAT_00a0813c;
            }
            if (param_1 < DAT_009a6fe0) {
                iVar4 = FUN_00425ae0(DAT_009a6fe0 - param_1, (int)uVar2, (int)local_8);
                param_2 = (int)((unsigned int)param_2 + (unsigned int)iVar4);
                if (DAT_00a07ce4 <= param_2) {
                    return;
                }
                param_1 = (int)DAT_009a6fe0;
            }
            iVar4 = DAT_005233f4;
            if (DAT_009d8c00 <= param_3) {
                param_3 = DAT_009d8c00 - 1;
            }
            if (DAT_00a07ce4 <= param_4) {
                param_4 = DAT_00a07ce4 - 1;
            }
            iVar6 = FUN_0040e1a0();
            puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar6 * (unsigned int)param_2 + (unsigned int)param_1 * (unsigned int)DAT_005233d8);
            if ((int)local_8 < (int)uVar2) {
                iVar4 = (int)uVar2 >> 1;
                iVar6 = (param_3 - param_1) + 1;
                param_4 = param_4 - param_2;
                if (param_4 != 0) {
                    do {
                        param_4 = param_4 - 1;
                        *puVar7 = uVar1;
                        iVar5 = FUN_0040e1a0();
                        puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)iVar5);
                        iVar4 = (int)((unsigned int)iVar4 - local_8);
                        if (iVar4 < 0) {
                            iVar4 = (int)((unsigned int)iVar4 + uVar2);
                            iVar6 = iVar6 - 1;
                            if (iVar6 == 0) {
                                return;
                            }
                            puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_005233d8);
                        }
                    } while (param_4 != 0);
                    return;
                }
            }
            else {
                iVar4 = param_3 - param_1;
                param_4 = (param_4 - param_2) + 1;
                if (iVar4 != 0) {
                    do {
                        iVar4 = iVar4 - 1;
                        *puVar7 = uVar1;
                        puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_005233d8);
                        iVar8 = (int)((unsigned int)iVar8 - uVar2);
                        if (iVar8 < 0) {
                            iVar8 = (int)((unsigned int)iVar8 + local_8);
                            param_4 = param_4 - 1;
                            if (param_4 == 0) {
                                return;
                            }
                            iVar6 = FUN_0040e1a0();
                            puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)iVar6);
                        }
                    } while (iVar4 != 0);
                    return;
                }
            }
        }
    }
    return;
}
