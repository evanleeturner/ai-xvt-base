/* SPDX-License-Identifier: MIT */
/* z_xvt 00417540 */
extern short DAT_006d9980;
extern short DAT_0070acc0;
extern short DAT_00732e8c;
extern short DAT_00732a34;
extern int DAT_00525534;
extern int DAT_00525518;
extern unsigned short DAT_007325e0[];

int FUN_00462000(void);
int FUN_00478290(int a, int b, int c);

void __cdecl FUN_00417540(int param_1, int param_2, int param_3, int param_4, unsigned char param_5)
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
    iVar4 = DAT_00525534;
    uVar1 = DAT_007325e0[param_5];
    local_8 = (unsigned int)param_3 - (unsigned int)param_1;
    if ((int)local_8 < 0) {
        param_1 = param_3;
        local_8 = 0u - local_8;
        param_4 = param_2;
        param_2 = iVar5;
        param_3 = iVar6;
    }
    else if (local_8 == 0) {
        if (param_1 < DAT_006d9980) {
            return;
        }
        if (DAT_0070acc0 <= param_1) {
            return;
        }
        if (param_4 < param_2) {
            param_4 = param_2;
            param_2 = iVar5;
        }
        if (param_2 < DAT_00732e8c) {
            param_2 = (int)DAT_00732e8c;
        }
        if (DAT_00732a34 <= param_4) {
            param_4 = DAT_00732a34 - 1;
        }
        param_4 = (int)((unsigned int)param_4 - (unsigned int)param_2);
        if (param_4 < 1) {
            return;
        }
        iVar6 = FUN_00462000();
        puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar6 * (unsigned int)param_2
                                    + (unsigned int)DAT_00525518 * (unsigned int)param_1);
        if (param_4 == 0) {
            return;
        }
        do {
            param_4 = param_4 - 1;
            *puVar7 = uVar1;
            iVar4 = FUN_00462000();
            puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)iVar4);
        } while (param_4 != 0);
        return;
    }
    iVar6 = (int)DAT_0070acc0;
    if ((param_1 < iVar6) && (iVar5 = (int)DAT_006d9980, iVar5 <= param_3)) {
        uVar2 = (unsigned int)param_4 - (unsigned int)param_2;
        iVar8 = (int)local_8 >> 1;
        if ((int)uVar2 < 0) {
            uVar3 = 0u - uVar2;
            if ((DAT_00732e8c <= param_2) && (iVar4 = (int)DAT_00732a34, param_4 < iVar4)) {
                if (iVar4 <= param_2) {
                    iVar4 = FUN_00478290((int)(((unsigned int)param_2 - (unsigned int)iVar4) + 1u),
                                         (int)local_8, (int)uVar3);
                    param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar4);
                    if (DAT_0070acc0 <= param_1) {
                        return;
                    }
                    param_2 = DAT_00732a34 - 1;
                }
                if (param_1 < DAT_006d9980) {
                    iVar4 = FUN_00478290((int)((unsigned int)(int)DAT_006d9980 - (unsigned int)param_1),
                                         (int)uVar3, (int)local_8);
                    param_2 = (int)((unsigned int)param_2 - (unsigned int)iVar4);
                    if (param_2 < DAT_00732e8c) {
                        return;
                    }
                    param_1 = (int)DAT_006d9980;
                }
                iVar4 = DAT_00525534;
                if (DAT_0070acc0 <= param_3) {
                    param_3 = DAT_0070acc0 - 1;
                }
                if (param_4 < DAT_00732e8c) {
                    param_4 = (int)DAT_00732e8c;
                }
                iVar6 = FUN_00462000();
                puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar6 * (unsigned int)param_2
                                            + (unsigned int)DAT_00525518 * (unsigned int)param_1);
                if ((int)local_8 < (int)uVar3) {
                    iVar6 = (int)uVar3 >> 1;
                    iVar5 = (int)(((unsigned int)param_3 - (unsigned int)param_1) + 1u);
                    iVar4 = (int)((unsigned int)param_2 - (unsigned int)param_4);
                    if (iVar4 != 0) {
                        do {
                            iVar4 = iVar4 - 1;
                            *puVar7 = uVar1;
                            iVar8 = FUN_00462000();
                            puVar7 = (unsigned short *)((unsigned int)puVar7 - (unsigned int)iVar8);
                            iVar6 = (int)((unsigned int)iVar6 - local_8);
                            if (iVar6 < 0) {
                                iVar6 = (int)((unsigned int)iVar6 + uVar3);
                                iVar5 = iVar5 - 1;
                                if (iVar5 == 0) {
                                    return;
                                }
                                puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_00525518);
                            }
                        } while (iVar4 != 0);
                        return;
                    }
                }
                else {
                    iVar6 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                    iVar4 = (int)(((unsigned int)param_2 - (unsigned int)param_4) + 1u);
                    if (iVar6 != 0) {
                        do {
                            iVar6 = iVar6 - 1;
                            *puVar7 = uVar1;
                            puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_00525518);
                            iVar8 = (int)((unsigned int)iVar8 + uVar2);
                            if (iVar8 < 0) {
                                iVar8 = (int)((unsigned int)iVar8 + local_8);
                                iVar4 = iVar4 - 1;
                                if (iVar4 == 0) {
                                    return;
                                }
                                iVar5 = FUN_00462000();
                                puVar7 = (unsigned short *)((unsigned int)puVar7 - (unsigned int)iVar5);
                            }
                        } while (iVar6 != 0);
                        return;
                    }
                }
            }
        }
        else if ((int)uVar2 < 1) {
            if ((DAT_00732e8c <= param_2) && (param_2 < DAT_00732a34)) {
                if (param_1 < iVar5) {
                    param_1 = iVar5;
                }
                if (iVar6 <= param_3) {
                    param_3 = iVar6 - 1;
                }
                iVar6 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                if (0 < iVar6) {
                    iVar5 = FUN_00462000();
                    puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar5 * (unsigned int)param_2
                                                + (unsigned int)DAT_00525518 * (unsigned int)param_1);
                    for (; iVar6 != 0; iVar6 = iVar6 - 1) {
                        *puVar7 = uVar1;
                        puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_00525518);
                    }
                }
            }
        }
        else if ((param_2 < DAT_00732a34) && (iVar4 = (int)DAT_00732e8c, iVar4 <= param_4)) {
            if (param_2 < iVar4) {
                iVar4 = FUN_00478290((int)((unsigned int)iVar4 - (unsigned int)param_2),
                                     (int)local_8, (int)uVar2);
                param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar4);
                if (DAT_0070acc0 <= param_1) {
                    return;
                }
                param_2 = (int)DAT_00732e8c;
            }
            if (param_1 < DAT_006d9980) {
                iVar4 = FUN_00478290((int)((unsigned int)(int)DAT_006d9980 - (unsigned int)param_1),
                                     (int)uVar2, (int)local_8);
                param_2 = (int)((unsigned int)param_2 + (unsigned int)iVar4);
                if (DAT_00732a34 <= param_2) {
                    return;
                }
                param_1 = (int)DAT_006d9980;
            }
            iVar4 = DAT_00525534;
            if (DAT_0070acc0 <= param_3) {
                param_3 = DAT_0070acc0 - 1;
            }
            if (DAT_00732a34 <= param_4) {
                param_4 = DAT_00732a34 - 1;
            }
            iVar6 = FUN_00462000();
            puVar7 = (unsigned short *)((unsigned int)iVar4 + (unsigned int)iVar6 * (unsigned int)param_2
                                        + (unsigned int)DAT_00525518 * (unsigned int)param_1);
            if ((int)local_8 < (int)uVar2) {
                iVar4 = (int)uVar2 >> 1;
                iVar6 = (int)(((unsigned int)param_3 - (unsigned int)param_1) + 1u);
                param_4 = (int)((unsigned int)param_4 - (unsigned int)param_2);
                if (param_4 != 0) {
                    do {
                        param_4 = param_4 - 1;
                        *puVar7 = uVar1;
                        iVar5 = FUN_00462000();
                        puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)iVar5);
                        iVar4 = (int)((unsigned int)iVar4 - local_8);
                        if (iVar4 < 0) {
                            iVar4 = (int)((unsigned int)iVar4 + uVar2);
                            iVar6 = iVar6 - 1;
                            if (iVar6 == 0) {
                                return;
                            }
                            puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_00525518);
                        }
                    } while (param_4 != 0);
                    return;
                }
            }
            else {
                iVar4 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                param_4 = (int)(((unsigned int)param_4 - (unsigned int)param_2) + 1u);
                if (iVar4 != 0) {
                    do {
                        iVar4 = iVar4 - 1;
                        *puVar7 = uVar1;
                        puVar7 = (unsigned short *)((unsigned int)puVar7 + (unsigned int)DAT_00525518);
                        iVar8 = (int)((unsigned int)iVar8 - uVar2);
                        if (iVar8 < 0) {
                            iVar8 = (int)((unsigned int)iVar8 + local_8);
                            param_4 = param_4 - 1;
                            if (param_4 == 0) {
                                return;
                            }
                            iVar6 = FUN_00462000();
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
