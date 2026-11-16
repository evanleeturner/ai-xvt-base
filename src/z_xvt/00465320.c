/* SPDX-License-Identifier: MIT */
/* z_xvt 00465320 */
extern short DAT_006d9980;
extern short DAT_0070acc0;
extern short DAT_00732e8c;
extern short DAT_00732a34;
extern int DAT_0070acc8;
extern int DAT_00525534;

int __cdecl FUN_00478290(int param_1, unsigned int param_2, unsigned int param_3);

void __cdecl FUN_00465320(int param_1, int param_2, int param_3, int param_4, unsigned char param_5)
{
    int iVar1;
    unsigned char *puVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    unsigned int local_4;

    iVar3 = param_1;
    local_4 = (unsigned int)param_3 - (unsigned int)param_1;
    if ((int)local_4 < 0) {
        local_4 = 0u - local_4;
        param_1 = param_3;
        iVar7 = param_2;
        param_2 = param_4;
        param_3 = iVar3;
    }
    else {
        iVar7 = param_4;
        if (local_4 == 0) {
            if (param_1 < DAT_006d9980) {
                return;
            }
            if (DAT_0070acc0 <= param_1) {
                return;
            }
            iVar3 = param_4;
            if (param_4 < param_2) {
                iVar3 = param_2;
                param_2 = param_4;
            }
            if (param_2 < DAT_00732e8c) {
                param_2 = (int)DAT_00732e8c;
            }
            if (DAT_00732a34 <= iVar3) {
                iVar3 = DAT_00732a34 + -1;
            }
            iVar3 = (int)((unsigned int)iVar3 - (unsigned int)param_2);
            if (iVar3 < 1) {
                return;
            }
            puVar2 = (unsigned char *)((unsigned int)DAT_0070acc8 * (unsigned int)param_2 + (unsigned int)param_1 + (unsigned int)DAT_00525534);
            if (iVar3 == 0) {
                return;
            }
            do {
                iVar3 = iVar3 + -1;
                *puVar2 = param_5;
                puVar2 = puVar2 + DAT_0070acc8;
            } while (iVar3 != 0);
            return;
        }
    }
    iVar3 = (int)DAT_0070acc0;
    if ((param_1 < iVar3) && (iVar1 = (int)DAT_006d9980, iVar1 <= param_3)) {
        uVar5 = (unsigned int)iVar7 - (unsigned int)param_2;
        iVar4 = (int)local_4 >> 1;
        if ((int)uVar5 < 0) {
            uVar6 = 0u - uVar5;
            if ((DAT_00732e8c <= param_2) && (iVar3 = (int)DAT_00732a34, iVar7 < iVar3)) {
                if (iVar3 <= param_2) {
                    iVar3 = FUN_00478290((int)((unsigned int)param_2 - (unsigned int)iVar3 + 1u), local_4, uVar6);
                    param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar3);
                    if (DAT_0070acc0 <= param_1) {
                        return;
                    }
                    param_2 = DAT_00732a34 + -1;
                }
                if (param_1 < DAT_006d9980) {
                    iVar3 = FUN_00478290((int)((unsigned int)(int)DAT_006d9980 - (unsigned int)param_1), uVar6, local_4);
                    param_2 = (int)((unsigned int)param_2 - (unsigned int)iVar3);
                    if (param_2 < DAT_00732e8c) {
                        return;
                    }
                    param_1 = (int)DAT_006d9980;
                }
                if (DAT_0070acc0 <= param_3) {
                    param_3 = DAT_0070acc0 + -1;
                }
                if (iVar7 < DAT_00732e8c) {
                    iVar7 = (int)DAT_00732e8c;
                }
                puVar2 = (unsigned char *)((unsigned int)DAT_0070acc8 * (unsigned int)param_2 + (unsigned int)param_1 + (unsigned int)DAT_00525534);
                if ((0u - local_4) == uVar5 || (int)uVar6 < (int)local_4) {
                    iVar1 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                    iVar3 = (int)((unsigned int)param_2 - (unsigned int)iVar7 + 1u);
                    if (iVar1 != 0) {
                        do {
                            iVar1 = iVar1 + -1;
                            *puVar2 = param_5;
                            puVar2 = puVar2 + 1;
                            iVar4 = (int)((unsigned int)iVar4 + uVar5);
                            if (iVar4 < 0) {
                                iVar4 = (int)((unsigned int)iVar4 + local_4);
                                iVar3 = iVar3 + -1;
                                if (iVar3 == 0) {
                                    return;
                                }
                                puVar2 = puVar2 - DAT_0070acc8;
                            }
                        } while (iVar1 != 0);
                        return;
                    }
                }
                else {
                    iVar3 = (int)uVar6 >> 1;
                    iVar1 = (int)((unsigned int)param_3 - (unsigned int)param_1 + 1u);
                    iVar7 = (int)((unsigned int)param_2 - (unsigned int)iVar7);
                    if (iVar7 != 0) {
                        do {
                            iVar7 = iVar7 + -1;
                            *puVar2 = param_5;
                            puVar2 = puVar2 - DAT_0070acc8;
                            iVar3 = (int)((unsigned int)iVar3 - local_4);
                            if (iVar3 < 0) {
                                iVar3 = (int)((unsigned int)iVar3 + uVar6);
                                iVar1 = iVar1 + -1;
                                if (iVar1 == 0) {
                                    return;
                                }
                                puVar2 = puVar2 + 1;
                            }
                        } while (iVar7 != 0);
                        return;
                    }
                }
            }
        }
        else if ((int)uVar5 < 1) {
            if ((DAT_00732e8c <= param_2) && (param_2 < DAT_00732a34)) {
                if (param_1 < iVar1) {
                    param_1 = iVar1;
                }
                if (iVar3 <= param_3) {
                    param_3 = iVar3 + -1;
                }
                iVar3 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                if (0 < iVar3) {
                    puVar2 = (unsigned char *)((unsigned int)DAT_0070acc8 * (unsigned int)param_2 + (unsigned int)param_1 + (unsigned int)DAT_00525534);
                    for (; iVar3 != 0; iVar3 = iVar3 + -1) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + 1;
                    }
                }
            }
        }
        else if ((param_2 < DAT_00732a34) && (iVar3 = (int)DAT_00732e8c, iVar3 <= iVar7)) {
            if (param_2 < iVar3) {
                iVar3 = FUN_00478290((int)((unsigned int)iVar3 - (unsigned int)param_2), local_4, uVar5);
                param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar3);
                if (DAT_0070acc0 <= param_1) {
                    return;
                }
                param_2 = (int)DAT_00732e8c;
            }
            if (param_1 < DAT_006d9980) {
                iVar3 = FUN_00478290((int)((unsigned int)(int)DAT_006d9980 - (unsigned int)param_1), uVar5, local_4);
                param_2 = (int)((unsigned int)param_2 + (unsigned int)iVar3);
                if (DAT_00732a34 <= param_2) {
                    return;
                }
                param_1 = (int)DAT_006d9980;
            }
            if (DAT_0070acc0 <= param_3) {
                param_3 = DAT_0070acc0 + -1;
            }
            if (DAT_00732a34 <= iVar7) {
                iVar7 = DAT_00732a34 + -1;
            }
            puVar2 = (unsigned char *)((unsigned int)DAT_0070acc8 * (unsigned int)param_2 + (unsigned int)param_1 + (unsigned int)DAT_00525534);
            if ((int)local_4 < (int)uVar5) {
                iVar3 = (int)uVar5 >> 1;
                iVar1 = (int)((unsigned int)param_3 - (unsigned int)param_1 + 1u);
                iVar7 = (int)((unsigned int)iVar7 - (unsigned int)param_2);
                if (iVar7 != 0) {
                    do {
                        iVar7 = iVar7 + -1;
                        *puVar2 = param_5;
                        puVar2 = puVar2 + DAT_0070acc8;
                        iVar3 = (int)((unsigned int)iVar3 - local_4);
                        if (iVar3 < 0) {
                            iVar3 = (int)((unsigned int)iVar3 + uVar5);
                            iVar1 = iVar1 + -1;
                            if (iVar1 == 0) {
                                return;
                            }
                            puVar2 = puVar2 + 1;
                        }
                    } while (iVar7 != 0);
                    return;
                }
            }
            else {
                iVar1 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                iVar3 = (int)((unsigned int)iVar7 - (unsigned int)param_2 + 1u);
                if (iVar1 != 0) {
                    do {
                        iVar1 = iVar1 + -1;
                        *puVar2 = param_5;
                        puVar2 = puVar2 + 1;
                        iVar4 = (int)((unsigned int)iVar4 - uVar5);
                        if (iVar4 < 0) {
                            iVar4 = (int)((unsigned int)iVar4 + local_4);
                            iVar3 = iVar3 + -1;
                            if (iVar3 == 0) {
                                return;
                            }
                            puVar2 = puVar2 + DAT_0070acc8;
                        }
                    } while (iVar1 != 0);
                    return;
                }
            }
        }
    }
    return;
}
