/* SPDX-License-Identifier: MIT */
/* z_bop 00411120 */
int __cdecl FUN_00425ae0(int a, unsigned int b, unsigned int c);

extern short DAT_009a6fe0;
extern short DAT_009d8c00;
extern short DAT_00a0813c;
extern short DAT_00a07ce4;
extern int DAT_009d8c08;
extern int DAT_005233f4;

void __cdecl FUN_00411120(int param_1, int param_2, int param_3, int param_4, unsigned char param_5)
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
            if (param_1 < DAT_009a6fe0) {
                return;
            }
            if (DAT_009d8c00 <= param_1) {
                return;
            }
            iVar3 = param_4;
            if (param_4 < param_2) {
                iVar3 = param_2;
                param_2 = param_4;
            }
            if (param_2 < DAT_00a0813c) {
                param_2 = (int)DAT_00a0813c;
            }
            if (DAT_00a07ce4 <= iVar3) {
                iVar3 = (int)((unsigned int)(int)DAT_00a07ce4 - 1u);
            }
            iVar3 = (int)((unsigned int)iVar3 - (unsigned int)param_2);
            if (iVar3 < 1) {
                return;
            }
            puVar2 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_2 * (unsigned int)DAT_009d8c08 + (unsigned int)DAT_005233f4);
            if (iVar3 == 0) {
                return;
            }
            do {
                iVar3 = (int)((unsigned int)iVar3 - 1u);
                *puVar2 = param_5;
                puVar2 = puVar2 + DAT_009d8c08;
            } while (iVar3 != 0);
            return;
        }
    }
    iVar3 = (int)DAT_009d8c00;
    if ((param_1 < iVar3) && (iVar1 = (int)DAT_009a6fe0, iVar1 <= param_3)) {
        uVar5 = (unsigned int)iVar7 - (unsigned int)param_2;
        iVar4 = (int)local_4 >> 1;
        if ((int)uVar5 < 0) {
            uVar6 = 0u - uVar5;
            if ((DAT_00a0813c <= param_2) && (iVar3 = (int)DAT_00a07ce4, iVar7 < iVar3)) {
                if (iVar3 <= param_2) {
                    iVar3 = FUN_00425ae0((int)(((unsigned int)param_2 - (unsigned int)iVar3) + 1u), local_4, uVar6);
                    param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar3);
                    if (DAT_009d8c00 <= param_1) {
                        return;
                    }
                    param_2 = (int)((unsigned int)(int)DAT_00a07ce4 - 1u);
                }
                if (param_1 < DAT_009a6fe0) {
                    iVar3 = FUN_00425ae0((int)((unsigned int)(int)DAT_009a6fe0 - (unsigned int)param_1), uVar6, local_4);
                    param_2 = (int)((unsigned int)param_2 - (unsigned int)iVar3);
                    if (param_2 < DAT_00a0813c) {
                        return;
                    }
                    param_1 = (int)DAT_009a6fe0;
                }
                if (DAT_009d8c00 <= param_3) {
                    param_3 = (int)((unsigned int)(int)DAT_009d8c00 - 1u);
                }
                if (iVar7 < DAT_00a0813c) {
                    iVar7 = (int)DAT_00a0813c;
                }
                puVar2 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_2 * (unsigned int)DAT_009d8c08 + (unsigned int)DAT_005233f4);
                if ((0u - local_4) == uVar5 || (int)uVar6 < (int)local_4) {
                    iVar1 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                    iVar3 = (int)(((unsigned int)param_2 - (unsigned int)iVar7) + 1u);
                    if (iVar1 != 0) {
                        do {
                            iVar1 = (int)((unsigned int)iVar1 - 1u);
                            *puVar2 = param_5;
                            puVar2 = puVar2 + 1;
                            iVar4 = (int)((unsigned int)iVar4 + uVar5);
                            if (iVar4 < 0) {
                                iVar4 = (int)((unsigned int)iVar4 + local_4);
                                iVar3 = (int)((unsigned int)iVar3 - 1u);
                                if (iVar3 == 0) {
                                    return;
                                }
                                puVar2 = puVar2 - DAT_009d8c08;
                            }
                        } while (iVar1 != 0);
                        return;
                    }
                }
                else {
                    iVar3 = (int)uVar6 >> 1;
                    iVar1 = (int)(((unsigned int)param_3 - (unsigned int)param_1) + 1u);
                    iVar7 = (int)((unsigned int)param_2 - (unsigned int)iVar7);
                    if (iVar7 != 0) {
                        do {
                            iVar7 = (int)((unsigned int)iVar7 - 1u);
                            *puVar2 = param_5;
                            puVar2 = puVar2 - DAT_009d8c08;
                            iVar3 = (int)((unsigned int)iVar3 - local_4);
                            if (iVar3 < 0) {
                                iVar3 = (int)((unsigned int)iVar3 + uVar6);
                                iVar1 = (int)((unsigned int)iVar1 - 1u);
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
            if ((DAT_00a0813c <= param_2) && (param_2 < DAT_00a07ce4)) {
                if (param_1 < iVar1) {
                    param_1 = iVar1;
                }
                if (iVar3 <= param_3) {
                    param_3 = (int)((unsigned int)iVar3 - 1u);
                }
                iVar3 = (int)((unsigned int)param_3 - (unsigned int)param_1);
                if (0 < iVar3) {
                    puVar2 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_2 * (unsigned int)DAT_009d8c08 + (unsigned int)DAT_005233f4);
                    for (; iVar3 != 0; iVar3 = (int)((unsigned int)iVar3 - 1u)) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + 1;
                    }
                }
            }
        }
        else if ((param_2 < DAT_00a07ce4) && (iVar3 = (int)DAT_00a0813c, iVar3 <= iVar7)) {
            if (param_2 < iVar3) {
                iVar3 = FUN_00425ae0((int)((unsigned int)iVar3 - (unsigned int)param_2), local_4, uVar5);
                param_1 = (int)((unsigned int)param_1 + (unsigned int)iVar3);
                if (DAT_009d8c00 <= param_1) {
                    return;
                }
                param_2 = (int)DAT_00a0813c;
            }
            if (param_1 < DAT_009a6fe0) {
                iVar3 = FUN_00425ae0((int)((unsigned int)(int)DAT_009a6fe0 - (unsigned int)param_1), uVar5, local_4);
                param_2 = (int)((unsigned int)param_2 + (unsigned int)iVar3);
                if (DAT_00a07ce4 <= param_2) {
                    return;
                }
                param_1 = (int)DAT_009a6fe0;
            }
            if (DAT_009d8c00 <= param_3) {
                param_3 = (int)((unsigned int)(int)DAT_009d8c00 - 1u);
            }
            if (DAT_00a07ce4 <= iVar7) {
                iVar7 = (int)((unsigned int)(int)DAT_00a07ce4 - 1u);
            }
            puVar2 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_2 * (unsigned int)DAT_009d8c08 + (unsigned int)DAT_005233f4);
            if ((int)local_4 < (int)uVar5) {
                iVar3 = (int)uVar5 >> 1;
                iVar1 = (int)(((unsigned int)param_3 - (unsigned int)param_1) + 1u);
                iVar7 = (int)((unsigned int)iVar7 - (unsigned int)param_2);
                if (iVar7 != 0) {
                    do {
                        iVar7 = (int)((unsigned int)iVar7 - 1u);
                        *puVar2 = param_5;
                        puVar2 = puVar2 + DAT_009d8c08;
                        iVar3 = (int)((unsigned int)iVar3 - local_4);
                        if (iVar3 < 0) {
                            iVar3 = (int)((unsigned int)iVar3 + uVar5);
                            iVar1 = (int)((unsigned int)iVar1 - 1u);
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
                iVar3 = (int)(((unsigned int)iVar7 - (unsigned int)param_2) + 1u);
                if (iVar1 != 0) {
                    do {
                        iVar1 = (int)((unsigned int)iVar1 - 1u);
                        *puVar2 = param_5;
                        puVar2 = puVar2 + 1;
                        iVar4 = (int)((unsigned int)iVar4 - uVar5);
                        if (iVar4 < 0) {
                            iVar4 = (int)((unsigned int)iVar4 + local_4);
                            iVar3 = (int)((unsigned int)iVar3 - 1u);
                            if (iVar3 == 0) {
                                return;
                            }
                            puVar2 = puVar2 + DAT_009d8c08;
                        }
                    } while (iVar1 != 0);
                    return;
                }
            }
        }
    }
    return;
}
