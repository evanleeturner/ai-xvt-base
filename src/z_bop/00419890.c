/* SPDX-License-Identifier: MIT */
/* z_bop 00419890 */
extern unsigned char *DAT_009a1fe8;
extern int DAT_009a1fe0;
extern int DAT_00a07cec;
extern unsigned int DAT_00a07ce8;
extern int DAT_009d1308;
extern unsigned char DAT_009d6940[2];
extern int DAT_00a00854;
extern unsigned char *DAT_009a8e18;
extern unsigned int DAT_009e9640;
extern unsigned char DAT_0051a44f[];

void __cdecl FUN_0041b830(unsigned char *param_1, unsigned short param_2, unsigned short param_3);

void __cdecl FUN_00419890(unsigned char *param_1, unsigned short param_2)
{
    unsigned char *iVar9;
    unsigned char *iVar6;
    unsigned char *puVar1;
    unsigned char *puVar2;
    unsigned char *puVar3;
    unsigned int iVar4;
    int iVar5;
    int iVar7;
    unsigned short uVar10;
    unsigned short uVar12;
    unsigned short uVar3;
    unsigned short uVar8;
    unsigned short uVar11;
    unsigned int uVar7;
    int bVar5;
    int bVar6;
    unsigned char bVar1;
    char cVar2;

    iVar9 = (unsigned char *)((unsigned int)param_2 * 0x23u + (unsigned int)DAT_009a1fe8);
    __builtin_memcpy(&iVar5, iVar9 + 0x1b, 4);

    if (iVar5 == -1) {
        __builtin_memcpy(&puVar1, iVar9 + 0x1f, 4);
        __builtin_memcpy(&uVar12, puVar1 + 0x85, 2);

        switch (iVar9[2]) {
        case 0:
        case 1:
        case 2:
            if (DAT_009d6940[1] == 0) {
                uVar10 = (unsigned short)DAT_00a07cec;
                if ((int)(DAT_00a07cec & 0xffff) < DAT_009a1fe0) {
                    do {
                        iVar4 = (unsigned int)uVar10 * 0x23u;
                        iVar6 = DAT_009a1fe8 + iVar4;
                        if ((DAT_009a1fe8[0x16 + iVar4] != iVar9[0x16]) && (iVar6[3] != 0)) {
                            __builtin_memcpy(&iVar7, iVar6 + 0x1b, 4);
                            __builtin_memcpy(&puVar2, iVar6 + 0x1f, 4);
                            if ((iVar7 != -1) && (puVar2 != 0)) {
                                FUN_0041b830(puVar2 + 0x12, uVar10, param_2);
                            }
                            cVar2 = (char)DAT_009a1fe8[2 + iVar4];
                            if ((cVar2 == 4) || (cVar2 == 3) || (cVar2 == 5)) {
                                __builtin_memcpy(&puVar2, DAT_009a1fe8 + iVar4 + 0x1f, 4);
                                if (puVar2 != 0) {
                                    __builtin_memcpy(&puVar3, puVar2 + 0xa9, 4);
                                    if ((puVar3 != 0) && (puVar3[0x5e] != 0x1e) &&
                                        (*(short *)(puVar3 + 0x68) == -1)) {
                                        FUN_0041b830(puVar2 + 0x12, uVar10, param_2);
                                    }
                                }
                            }
                        }
                        uVar10 = (unsigned short)(uVar10 + 1);
                    } while ((int)(unsigned int)uVar10 < DAT_009a1fe0);
                }
                uVar10 = (unsigned short)DAT_00a07ce8;
                if ((int)(DAT_00a07ce8 & 0xffff) <
                    (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8)) {
                    do {
                        bVar1 = DAT_009a1fe8[(unsigned int)uVar10 * 0x23u + 3];
                        if ((99 < bVar1) && (bVar1 < 0x6a)) {
                            FUN_0041b830(param_1, param_2, uVar10);
                        }
                        uVar10 = (unsigned short)(uVar10 + 1);
                    } while ((int)(unsigned int)uVar10 <
                             (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8));
                    return;
                }
            }
            break;
        case 3:
        case 4:
        case 5:
            __builtin_memcpy(&puVar1, iVar9 + 0x1f, 4);
            __builtin_memcpy(&puVar2, puVar1 + 0xa9, 4);
            if ((puVar2[0x5e] != 0x1e) && (*(short *)(puVar2 + 0x68) == -1) &&
                ((int)(DAT_00a07cec & 0xffff) < DAT_009a1fe0)) {
                uVar10 = (unsigned short)DAT_00a07cec;
                do {
                    if (param_2 != uVar10) {
                        iVar4 = (unsigned int)uVar10 * 0x23u;
                        iVar6 = DAT_009a1fe8 + iVar4;
                        if (iVar6[3] != 0) {
                            __builtin_memcpy(&iVar7, iVar6 + 0x1b, 4);
                            if (iVar7 == -1) {
                                if (iVar6[2] != 0xd) {
                                    FUN_0041b830(param_1, param_2, uVar10);
                                    cVar2 = (char)DAT_009a1fe8[2 + iVar4];
                                    if ((cVar2 == 4) || (cVar2 == 3) || (cVar2 == 5)) {
                                        __builtin_memcpy(&puVar1, DAT_009a1fe8 + iVar4 + 0x1f, 4);
                                        if (puVar1 != 0) {
                                            __builtin_memcpy(&puVar2, puVar1 + 0xa9, 4);
                                            if ((puVar2 != 0) && (puVar2[0x5e] != 0x1e) &&
                                                (*(short *)(puVar2 + 0x68) == -1)) {
                                                goto LAB_00419bf8;
                                            }
                                        }
                                    }
                                }
                            } else {
                                __builtin_memcpy(&puVar1, iVar6 + 0x1f, 4);
                                if (puVar1 != 0) {
LAB_00419bf8:
                                    FUN_0041b830(puVar1 + 0x12, uVar10, param_2);
                                }
                            }
                        }
                    }
                    uVar10 = (unsigned short)(uVar10 + 1);
                    if (DAT_009a1fe0 <= (int)(unsigned int)uVar10) {
                        return;
                    }
                } while (1);
            }
            break;
        case 6:
        case 7:
            uVar10 = (unsigned short)DAT_00a07cec;
            uVar3 = *(unsigned short *)(DAT_009a8e18 + 4 +
                                        ((unsigned int)param_2 - DAT_009e9640) * 10u);
            if ((int)(DAT_00a07cec & 0xffff) < DAT_00a00854) {
                do {
                    iVar6 = (unsigned char *)((unsigned int)uVar10 * 0x23u +
                                              (unsigned int)DAT_009a1fe8);
                    uVar8 = uVar10;
                    bVar6 = 0;
                    if (iVar6[3] != 0) {
                        if ((int)(unsigned int)uVar10 < DAT_009a1fe0) {
                            bVar6 = 1;
                        } else if ((DAT_0051a44f[iVar6[3]] != 0) && (param_2 != uVar8)) {
                            __builtin_memcpy(&puVar1, iVar6 + 0x1f, 4);
                            __builtin_memcpy(&uVar11, puVar1 + 0x85, 2);
                            if (uVar11 != uVar12) {
                                bVar6 = 1;
                            }
                        }
                    }
                    if (bVar6 && (uVar12 != uVar8) && (iVar6[2] != 0xd)) {
                        bVar5 = 0;
                        uVar7 = (unsigned int)uVar3;
                        if (((int)uVar7 < DAT_009a1fe0) && (DAT_00a07cec <= (int)uVar7)) {
                            __builtin_memcpy(&iVar7, DAT_009a1fe8 + uVar7 * 0x23u + 0x1b, 4);
                            if (iVar7 != -1) {
                                bVar5 = 1;
                            }
                        }
                        if (!bVar5) {
                            __builtin_memcpy(&iVar7,
                                             DAT_009a1fe8 + (unsigned int)uVar12 * 0x23u + 0x1b, 4);
                            if ((iVar7 != -1) || (uVar3 == uVar8)) {
                                bVar5 = 1;
                            }
                        }
                        if (bVar5) {
                            FUN_0041b830(param_1, param_2, uVar10);
                        }
                    }
                    uVar10 = (unsigned short)(uVar8 + 1);
                } while ((int)(unsigned int)(unsigned short)(uVar8 + 1) < DAT_00a00854);
            }
            if (DAT_009d6940[1] == 0) {
                uVar10 = (unsigned short)DAT_00a07ce8;
                if ((int)(DAT_00a07ce8 & 0xffff) <
                    (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8)) {
                    do {
                        if (DAT_009a1fe8[(unsigned int)uVar10 * 0x23u + 3] != 0) {
                            FUN_0041b830(param_1, param_2, uVar10);
                        }
                        uVar10 = (unsigned short)(uVar10 + 1);
                    } while ((int)(unsigned int)uVar10 <
                             (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8));
                }
            }
            break;
        }
    } else {
        __builtin_memcpy(&puVar1, iVar9 + 0x1f, 4);
        __builtin_memcpy(&puVar2, puVar1 + 0xa9, 4);
        __builtin_memcpy(&uVar11, puVar2 + 0x1e5, 2);
        uVar10 = (unsigned short)DAT_00a07cec;
        if ((uVar11 != 0) && ((int)(DAT_00a07cec & 0xffff) < DAT_009a1fe0)) {
            do {
                iVar6 = (unsigned char *)((unsigned int)uVar10 * 0x23u +
                                          (unsigned int)DAT_009a1fe8);
                if ((iVar6[3] != 0) && (param_2 != uVar10) && (iVar6[2] != 0xd) &&
                    (iVar9[0x16] != iVar6[0x16])) {
                    FUN_0041b830(param_1, param_2, uVar10);
                }
                uVar10 = (unsigned short)(uVar10 + 1);
            } while ((int)(unsigned int)uVar10 < DAT_009a1fe0);
        }
        if (DAT_009d6940[1] == 0) {
            uVar10 = (unsigned short)DAT_00a07ce8;
            if ((int)(DAT_00a07ce8 & 0xffff) <
                (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8)) {
                do {
                    if (DAT_009a1fe8[(unsigned int)uVar10 * 0x23u + 3] != 0) {
                        FUN_0041b830(param_1, param_2, uVar10);
                    }
                    uVar10 = (unsigned short)(uVar10 + 1);
                } while ((int)(unsigned int)uVar10 <
                         (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8));
                return;
            }
        }
    }
    return;
}
