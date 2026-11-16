/* SPDX-License-Identifier: MIT */
/* z_bop 004167f0 */
extern unsigned char *DAT_00550b88;
extern unsigned char *DAT_00550bd8;
extern int DAT_009d1308;
extern int DAT_00a07ce8;
extern int DAT_009a8e24;
extern int DAT_009cd274;
extern int DAT_009a1fe8;
extern int DAT_009d6820;
extern int DAT_00a07bd0;
extern int DAT_009a8e18;
extern int DAT_009a8da0;

extern unsigned int DAT_009ed228;
extern unsigned int _DAT_009ed22c;
extern unsigned int DAT_009d8b70;
extern unsigned int DAT_009d8b74;

extern short DAT_009a2000;
extern unsigned int DAT_009a20c0;
extern unsigned int DAT_009d8c30;
extern unsigned int DAT_009d6940;
extern unsigned int DAT_009ec5e0;

extern short DAT_00a08294;
extern unsigned int DAT_00a080f8;
extern unsigned char DAT_009d8c20;
extern unsigned int DAT_009eca28;
extern unsigned int DAT_009a7ba4;
extern unsigned int DAT_009fd430;
extern unsigned int DAT_00a0814c;
extern unsigned int DAT_009a7b50;
extern unsigned int DAT_009d767c;
extern unsigned int _DAT_00a07cec;
extern unsigned int DAT_009a1fe0;
extern unsigned int DAT_009d1144;
extern unsigned int DAT_009fe7dc;
extern unsigned int DAT_009e9640;
extern unsigned int DAT_00a00854;
extern unsigned int DAT_009a7b58;
extern unsigned int DAT_009d1140;
extern unsigned int DAT_009d1310;
extern unsigned int DAT_009ec47c;
extern unsigned int DAT_009d1320;
extern unsigned int DAT_009a8068;
extern unsigned int DAT_009a73a4;
extern unsigned int DAT_009a7a40;
extern short DAT_009d113c;
extern short DAT_009a8d40;
extern unsigned int DAT_00523424;
extern unsigned int DAT_009e9670;

void __stdcall FUN_004167f0(void)
{
    int iVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned char *puVar7;
    unsigned char *puVar8;
    unsigned char *puVar9;
    int iVar10;
    unsigned char *puVar12;
    unsigned char *puVar13;
    unsigned char *puVar14;

    iVar10 = 0;
    puVar8 = DAT_00550b88;
    if (0 < DAT_009d1308 + DAT_00a07ce8) {
        iVar3 = 0;
        puVar7 = DAT_00550b88;
        do {
            if ((iVar10 < DAT_009a8e24) || (puVar8 = puVar7, DAT_009cd274 <= iVar10)) {
                puVar12 = (unsigned char *)(unsigned int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar3);
                puVar8 = puVar7 + 1;
                *puVar7 = puVar12[3];
                if (puVar12[3] != 0) {
                    __builtin_memcpy(&iVar2, puVar12 + 0x1f, 4);
                    if (iVar2 != 0) {
                        iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_009d6820);
                        __builtin_memcpy(puVar12 + 0x1f, &iVar2, 4);
                        iVar2 = (int)((unsigned int)iVar2 + 1u);
                        __builtin_memcpy(puVar12 + 0x1f, &iVar2, 4);
                    }
                    puVar9 = puVar12;
                    for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
                        __builtin_memcpy(puVar8, puVar9, 4);
                        puVar9 = puVar9 + 4;
                        puVar8 = puVar8 + 4;
                    }
                    __builtin_memcpy(puVar8, puVar9, 2);
                    puVar8[2] = puVar9[2];
                    puVar8 = puVar7 + 0x24;
                    __builtin_memcpy(&iVar4, puVar12 + 0x1f, 4);
                    if (iVar4 != 0) {
                        iVar2 = (int)((unsigned int)iVar4 + 0xFFFFFFFFu);
                        __builtin_memcpy(puVar12 + 0x1f, &iVar2, 4);
                        iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009d6820);
                        __builtin_memcpy(puVar12 + 0x1f, &iVar2, 4);
                    }
                    __builtin_memcpy(&iVar4, puVar12 + 0x1f, 4);
                    puVar13 = (unsigned char *)(unsigned int)iVar4;
                    if (iVar4 != 0) {
                        __builtin_memcpy(&iVar2, puVar13 + 0xa9, 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_00a07bd0);
                            __builtin_memcpy(puVar13 + 0xa9, &iVar2, 4);
                            iVar2 = (int)((unsigned int)iVar2 + 1u);
                            __builtin_memcpy(puVar13 + 0xa9, &iVar2, 4);
                        }
                        __builtin_memcpy(&iVar2, puVar13 + 0xa5, 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_009a8e18);
                            __builtin_memcpy(puVar13 + 0xa5, &iVar2, 4);
                            iVar2 = (int)((unsigned int)iVar2 + 1u);
                            __builtin_memcpy(puVar13 + 0xa5, &iVar2, 4);
                        }
                        __builtin_memcpy(&iVar2, puVar13 + 0xad, 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_009a8da0);
                            __builtin_memcpy(puVar13 + 0xad, &iVar2, 4);
                            iVar2 = (int)((unsigned int)iVar2 + 1u);
                            __builtin_memcpy(puVar13 + 0xad, &iVar2, 4);
                        }
                        puVar9 = puVar13;
                        for (iVar4 = 0x2c; iVar4 != 0; iVar4 = iVar4 + -1) {
                            __builtin_memcpy(puVar8, puVar9, 4);
                            puVar9 = puVar9 + 4;
                            puVar8 = puVar8 + 4;
                        }
                        *puVar8 = *puVar9;
                        __builtin_memcpy(&iVar2, puVar13 + 0xa9, 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 + 0xFFFFFFFFu);
                            __builtin_memcpy(puVar13 + 0xa9, &iVar2, 4);
                            iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_00a07bd0);
                            __builtin_memcpy(puVar13 + 0xa9, &iVar2, 4);
                        }
                        __builtin_memcpy(&iVar2, puVar13 + 0xa5, 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 + 0xFFFFFFFFu);
                            __builtin_memcpy(puVar13 + 0xa5, &iVar2, 4);
                            iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009a8e18);
                            __builtin_memcpy(puVar13 + 0xa5, &iVar2, 4);
                        }
                        __builtin_memcpy(&iVar2, puVar13 + 0xad, 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 + 0xFFFFFFFFu);
                            __builtin_memcpy(puVar13 + 0xad, &iVar2, 4);
                            iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009a8da0);
                            __builtin_memcpy(puVar13 + 0xad, &iVar2, 4);
                        }
                        puVar9 = puVar7 + 0xd5;
                        __builtin_memcpy(&iVar4, puVar13 + 0xa9, 4);
                        puVar14 = (unsigned char *)(unsigned int)iVar4;
                        if (iVar4 != 0) {
                            iVar4 = 0;
                            do {
                                __builtin_memcpy(&iVar2, puVar14 + 0x41e + iVar4, 4);
                                if (iVar2 != 0) {
                                    iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_009a1fe8);
                                    __builtin_memcpy(puVar14 + 0x41e + iVar4, &iVar2, 4);
                                    iVar2 = (int)((unsigned int)iVar2 + 1u);
                                    __builtin_memcpy(puVar14 + 0x41e + iVar4, &iVar2, 4);
                                }
                                iVar4 = iVar4 + 4;
                            } while (iVar4 < 0x40);
                            __builtin_memcpy(&iVar2, puVar14 + 0x45e, 4);
                            if (iVar2 != 0) {
                                iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_009a1fe8);
                                __builtin_memcpy(puVar14 + 0x45e, &iVar2, 4);
                                iVar2 = (int)((unsigned int)iVar2 + 1u);
                                __builtin_memcpy(puVar14 + 0x45e, &iVar2, 4);
                            }
                            puVar9 = puVar7 + 0x537;
                            puVar8 = puVar14;
                            puVar7 = puVar7 + 0xd5;
                            for (iVar4 = 0x118; iVar4 != 0; iVar4 = iVar4 + -1) {
                                __builtin_memcpy(puVar7, puVar8, 4);
                                puVar8 = puVar8 + 4;
                                puVar7 = puVar7 + 4;
                            }
                            __builtin_memcpy(puVar7, puVar8, 2);
                            iVar4 = 0;
                            do {
                                __builtin_memcpy(&iVar2, puVar14 + 0x41e + iVar4, 4);
                                if (iVar2 != 0) {
                                    iVar2 = (int)((unsigned int)iVar2 + 0xFFFFFFFFu);
                                    __builtin_memcpy(puVar14 + 0x41e + iVar4, &iVar2, 4);
                                    iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009a1fe8);
                                    __builtin_memcpy(puVar14 + 0x41e + iVar4, &iVar2, 4);
                                }
                                iVar4 = iVar4 + 4;
                            } while (iVar4 < 0x40);
                            __builtin_memcpy(&iVar2, puVar14 + 0x45e, 4);
                            if (iVar2 != 0) {
                                iVar2 = (int)((unsigned int)iVar2 + 0xFFFFFFFFu);
                                __builtin_memcpy(puVar14 + 0x45e, &iVar2, 4);
                                iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009a1fe8);
                                __builtin_memcpy(puVar14 + 0x45e, &iVar2, 4);
                            }
                        }
                        __builtin_memcpy(&iVar4, puVar13 + 0xa5, 4);
                        puVar7 = (unsigned char *)(unsigned int)iVar4;
                        puVar8 = puVar9;
                        if (puVar7 != (unsigned char *)0) {
                            __builtin_memcpy(puVar9, puVar7, 4);
                            puVar8 = puVar9 + 10;
                            __builtin_memcpy(puVar9 + 4, puVar7 + 4, 4);
                            __builtin_memcpy(puVar9 + 8, puVar7 + 8, 2);
                        }
                        __builtin_memcpy(&iVar4, puVar13 + 0xad, 4);
                        puVar7 = (unsigned char *)(unsigned int)iVar4;
                        if (puVar7 != (unsigned char *)0) {
                            puVar9 = puVar8;
                            for (iVar4 = 0x13; iVar4 != 0; iVar4 = iVar4 + -1) {
                                __builtin_memcpy(puVar9, puVar7, 4);
                                puVar7 = puVar7 + 4;
                                puVar9 = puVar9 + 4;
                            }
                            puVar8 = puVar8 + 0x4c;
                        }
                    }
                }
            }
            iVar3 = iVar3 + 0x23;
            iVar10 = iVar10 + 1;
            puVar7 = puVar8;
        } while (iVar10 < DAT_009d1308 + DAT_00a07ce8);
    }

    __builtin_memcpy(puVar8, &DAT_009ed228, 4);
    __builtin_memcpy(puVar8 + 4, &_DAT_009ed22c, 4);
    __builtin_memcpy(puVar8 + 8, &DAT_009d8b70, 4);
    __builtin_memcpy(puVar8 + 12, &DAT_009d8b74, 4);
    puVar7 = (unsigned char *)&DAT_009a2000;
    puVar9 = puVar8 + 16;
    for (iVar10 = 0x28; iVar10 != 0; iVar10 = iVar10 + -1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    __builtin_memcpy(puVar9, puVar7, 2);
    iVar10 = (int)DAT_009a2000;
    puVar7 = (unsigned char *)&DAT_009a20c0;
    puVar9 = puVar8 + 0xb2;
    for (uVar5 = (unsigned int)iVar10 * 0x126u >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    for (uVar5 = (unsigned int)iVar10 * 0x126u & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar9 = *puVar7;
        puVar7 = puVar7 + 1;
        puVar9 = puVar9 + 1;
    }
    puVar8 = puVar8 + 0xb2 + (unsigned int)(int)DAT_009a2000 * 0x126u;
    uVar5 = (unsigned int)(int)DAT_009a2000 * 0x566u;
    puVar7 = (unsigned char *)&DAT_009d8c30;
    puVar9 = puVar8;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar9 = *puVar7;
        puVar7 = puVar7 + 1;
        puVar9 = puVar9 + 1;
    }
    puVar8 = puVar8 + (unsigned int)(int)DAT_009a2000 * 0x566u;
    puVar7 = (unsigned char *)&DAT_009d6940;
    puVar9 = puVar8;
    for (iVar10 = 0x34c; iVar10 != 0; iVar10 = iVar10 + -1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    puVar7 = (unsigned char *)&DAT_009ec5e0;
    puVar9 = puVar8 + 0xd30;
    for (iVar10 = 5; iVar10 != 0; iVar10 = iVar10 + -1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    __builtin_memcpy(puVar9, puVar7, 2);
    __builtin_memcpy(puVar8 + 0xd46, &DAT_00a08294, 2);
    __builtin_memcpy(puVar8 + 0xd48, &DAT_00a080f8, 4);
    puVar8[0xd4c] = DAT_009d8c20;
    __builtin_memcpy(puVar8 + 0xd4d, &DAT_009eca28, 4);
    __builtin_memcpy(puVar8 + 0xd51, &DAT_009a7ba4, 4);
    __builtin_memcpy(puVar8 + 0xd55, &DAT_009fd430, 4);
    __builtin_memcpy(puVar8 + 0xd59, &DAT_00a0814c, 4);
    __builtin_memcpy(puVar8 + 0xd5d, &DAT_009a7b50, 4);
    __builtin_memcpy(puVar8 + 0xd61, &DAT_009d767c, 4);
    __builtin_memcpy(puVar8 + 0xd65, &_DAT_00a07cec, 4);
    __builtin_memcpy(puVar8 + 0xd69, &DAT_009a1fe0, 4);
    __builtin_memcpy(puVar8 + 0xd6d, &DAT_009d1144, 4);
    __builtin_memcpy(puVar8 + 0xd71, &DAT_009fe7dc, 4);
    __builtin_memcpy(puVar8 + 0xd75, &DAT_009e9640, 4);
    __builtin_memcpy(puVar8 + 0xd79, &DAT_00a00854, 4);
    __builtin_memcpy(puVar8 + 0xd7d, &DAT_009a7b58, 4);
    __builtin_memcpy(puVar8 + 0xd81, &DAT_009d1140, 4);
    __builtin_memcpy(puVar8 + 0xd85, &DAT_009d1310, 4);
    __builtin_memcpy(puVar8 + 0xd89, &DAT_009ec47c, 4);
    __builtin_memcpy(puVar8 + 0xd8d, &DAT_009a8e24, 4);
    __builtin_memcpy(puVar8 + 0xd91, &DAT_009cd274, 4);
    __builtin_memcpy(puVar8 + 0xd95, &DAT_00a07ce8, 4);
    __builtin_memcpy(puVar8 + 0xd99, &DAT_009d1308, 4);
    puVar7 = (unsigned char *)&DAT_009d1320;
    puVar9 = puVar8 + 0xd9d;
    for (iVar10 = 0x1540; iVar10 != 0; iVar10 = iVar10 + -1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    __builtin_memcpy(puVar8 + 0x629d, &DAT_009a8068, 4);
    __builtin_memcpy(puVar8 + 0x62a1, &DAT_009a73a4, 4);
    puVar7 = (unsigned char *)&DAT_009a7a40;
    puVar9 = puVar8 + 0x62a5;
    for (iVar10 = 0x40; iVar10 != 0; iVar10 = iVar10 + -1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    __builtin_memcpy(puVar8 + 0x63a5, &DAT_009d113c, 2);
    __builtin_memcpy(puVar8 + 0x63a7, &DAT_009a8d40, 2);
    __builtin_memcpy(puVar8 + 0x63a9, &DAT_00523424, 4);
    puVar7 = (unsigned char *)&DAT_009e9670;
    puVar9 = puVar8 + 0x63ad;
    for (iVar10 = 0xb7a; iVar10 != 0; iVar10 = iVar10 + -1) {
        __builtin_memcpy(puVar9, puVar7, 4);
        puVar7 = puVar7 + 4;
        puVar9 = puVar9 + 4;
    }
    DAT_00550bd8 = (unsigned char *)((unsigned int)puVar8 +
                                     (0x9195u - (unsigned int)DAT_00550b88));
    return;
}
