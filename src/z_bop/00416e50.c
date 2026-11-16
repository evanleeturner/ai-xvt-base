/* SPDX-License-Identifier: MIT */
/* z_bop 00416e50 */
extern unsigned int *DAT_00550b88;
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

#define ENT_ADDR(off) ((void *)((unsigned int)DAT_009a1fe8 + (unsigned int)(off) + (unsigned int)iVar8))
#define OFS(p, off) ((void *)((unsigned int)(p) + (unsigned int)(off)))

void __stdcall FUN_00416e50(void)
{
    int *piVar1;
    int iVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int *puVar6;
    unsigned int *puVar7;
    int iVar8;
    int iVar9;
    unsigned int *puVar10;

    iVar9 = 0;
    puVar7 = DAT_00550b88;
    if (0 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8)) {
        iVar8 = 0;
        puVar6 = DAT_00550b88;
        do {
            if ((iVar9 < DAT_009a8e24) || (puVar7 = puVar6, DAT_009cd274 <= iVar9)) {
                puVar7 = (unsigned int *)((unsigned int)puVar6 + 1u);
                *(unsigned char *)ENT_ADDR(3) = *(unsigned char *)puVar6;
                puVar10 = (unsigned int *)ENT_ADDR(0);
                if (*(char *)OFS(puVar10, 3) == '\0') {
                    for (iVar3 = 7; iVar3 != 0; iVar3 = iVar3 + -1) {
                        __builtin_memset(puVar10, 0, 4);
                        puVar10 = puVar10 + 1;
                    }
                    __builtin_memset(puVar10, 0, 2);
                    *(unsigned char *)OFS(puVar10, 2) = 0;
                    iVar2 = -1;
                    __builtin_memcpy(ENT_ADDR(0x1b), &iVar2, 4);
                    __builtin_memcpy(&puVar6, ENT_ADDR(0x1f), 4);
                    if (puVar6 != (unsigned int *)0x0) {
                        for (iVar3 = 0x22; iVar3 != 0; iVar3 = iVar3 + -1) {
                            __builtin_memset(puVar6, 0, 4);
                            puVar6 = puVar6 + 1;
                        }
                        __builtin_memset(puVar6, 0, 2);
                        *(unsigned char *)OFS(puVar6, 2) = 0;
                        __builtin_memcpy(&puVar6, ENT_ADDR(0x1f), 4);
                        *(unsigned char *)OFS(puVar6, 0x88) = 0xff;
                        __builtin_memcpy(&puVar6, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&puVar6, OFS(puVar6, 0xa9), 4);
                        if (puVar6 != (unsigned int *)0x0) {
                            for (iVar3 = 0x104; iVar3 != 0; iVar3 = iVar3 + -1) {
                                __builtin_memset(puVar6, 0, 4);
                                puVar6 = puVar6 + 1;
                            }
                            __builtin_memset(puVar6, 0, 2);
                        }
                        __builtin_memcpy(&puVar6, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&puVar6, OFS(puVar6, 0xa5), 4);
                        if (puVar6 != (unsigned int *)0x0) {
                            __builtin_memset(puVar6, 0, 4);
                            __builtin_memset(puVar6 + 1, 0, 4);
                            __builtin_memset(puVar6 + 2, 0, 2);
                        }
                        __builtin_memcpy(&puVar6, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&puVar6, OFS(puVar6, 0xad), 4);
                        if (puVar6 != (unsigned int *)0x0) {
                            for (iVar3 = 0x13; iVar3 != 0; iVar3 = iVar3 + -1) {
                                __builtin_memset(puVar6, 0, 4);
                                puVar6 = puVar6 + 1;
                            }
                        }
                    }
                }
                else {
                    for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
                        __builtin_memcpy(puVar10, puVar7, 4);
                        puVar7 = puVar7 + 1;
                        puVar10 = puVar10 + 1;
                    }
                    __builtin_memcpy(puVar10, puVar7, 2);
                    *(unsigned char *)OFS(puVar10, 2) = *(unsigned char *)OFS(puVar7, 2);
                    puVar7 = puVar6 + 9;
                    __builtin_memcpy(&iVar3, ENT_ADDR(0x1f), 4);
                    if (iVar3 != 0) {
                        iVar2 = (int)((unsigned int)iVar3 - 1u);
                        __builtin_memcpy(ENT_ADDR(0x1f), &iVar2, 4);
                        __builtin_memcpy(&iVar2, ENT_ADDR(0x1f), 4);
                        iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009d6820);
                        __builtin_memcpy(ENT_ADDR(0x1f), &iVar2, 4);
                    }
                    __builtin_memcpy(&puVar10, ENT_ADDR(0x1f), 4);
                    if (puVar10 != (unsigned int *)0x0) {
                        for (iVar3 = 0x2c; iVar3 != 0; iVar3 = iVar3 + -1) {
                            __builtin_memcpy(puVar10, puVar7, 4);
                            puVar7 = puVar7 + 1;
                            puVar10 = puVar10 + 1;
                        }
                        *(unsigned char *)puVar10 = *(unsigned char *)puVar7;
                        puVar10 = (unsigned int *)OFS(puVar6, 0xd5);

                        __builtin_memcpy(&iVar3, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&iVar2, OFS(iVar3, 0xa9), 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 - 1u);
                            __builtin_memcpy(OFS(iVar3, 0xa9), &iVar2, 4);
                            __builtin_memcpy(&iVar3, ENT_ADDR(0x1f), 4);
                            piVar1 = (int *)OFS(iVar3, 0xa9);
                            __builtin_memcpy(&iVar2, piVar1, 4);
                            iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_00a07bd0);
                            __builtin_memcpy(piVar1, &iVar2, 4);
                        }
                        __builtin_memcpy(&iVar3, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&iVar2, OFS(iVar3, 0xa5), 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 - 1u);
                            __builtin_memcpy(OFS(iVar3, 0xa5), &iVar2, 4);
                            __builtin_memcpy(&iVar3, ENT_ADDR(0x1f), 4);
                            piVar1 = (int *)OFS(iVar3, 0xa5);
                            __builtin_memcpy(&iVar2, piVar1, 4);
                            iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009a8e18);
                            __builtin_memcpy(piVar1, &iVar2, 4);
                        }
                        __builtin_memcpy(&iVar3, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&iVar2, OFS(iVar3, 0xad), 4);
                        if (iVar2 != 0) {
                            iVar2 = (int)((unsigned int)iVar2 - 1u);
                            __builtin_memcpy(OFS(iVar3, 0xad), &iVar2, 4);
                            __builtin_memcpy(&iVar3, ENT_ADDR(0x1f), 4);
                            piVar1 = (int *)OFS(iVar3, 0xad);
                            __builtin_memcpy(&iVar2, piVar1, 4);
                            iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_009a8da0);
                            __builtin_memcpy(piVar1, &iVar2, 4);
                        }
                        __builtin_memcpy(&puVar7, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&puVar7, OFS(puVar7, 0xa9), 4);
                        if (puVar7 != (unsigned int *)0x0) {
                            for (iVar3 = 0x118; iVar3 != 0; iVar3 = iVar3 + -1) {
                                __builtin_memcpy(puVar7, puVar10, 4);
                                puVar10 = puVar10 + 1;
                                puVar7 = puVar7 + 1;
                            }
                            __builtin_memcpy(puVar7, puVar10, 2);
                            puVar10 = (unsigned int *)OFS(puVar6, 0x537);
                            iVar3 = 0;
                            do {
                                __builtin_memcpy(&puVar7, ENT_ADDR(0x1f), 4);
                                __builtin_memcpy(&puVar7, OFS(puVar7, 0xa9), 4);
                                piVar1 = (int *)((unsigned int)puVar7 + 0x41eu + (unsigned int)iVar3);
                                __builtin_memcpy(&iVar2, piVar1, 4);
                                if (iVar2 != 0) {
                                    iVar2 = (int)((unsigned int)iVar2 - 1u + (unsigned int)DAT_009a1fe8);
                                    __builtin_memcpy(piVar1, &iVar2, 4);
                                }
                                iVar3 = (int)((unsigned int)iVar3 + 4u);
                            } while (iVar3 < 0x40);
                            __builtin_memcpy(&puVar7, ENT_ADDR(0x1f), 4);
                            __builtin_memcpy(&puVar7, OFS(puVar7, 0xa9), 4);
                            piVar1 = (int *)OFS(puVar7, 0x45e);
                            __builtin_memcpy(&iVar2, piVar1, 4);
                            if (iVar2 != 0) {
                                iVar2 = (int)((unsigned int)iVar2 - 1u + (unsigned int)DAT_009a1fe8);
                                __builtin_memcpy(piVar1, &iVar2, 4);
                            }
                        }
                        __builtin_memcpy(&puVar6, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&puVar6, OFS(puVar6, 0xa5), 4);
                        puVar7 = puVar10;
                        if (puVar6 != (unsigned int *)0x0) {
                            puVar7 = (unsigned int *)OFS(puVar10, 10);
                            __builtin_memcpy(puVar6, puVar10, 4);
                            __builtin_memcpy(puVar6 + 1, puVar10 + 1, 4);
                            __builtin_memcpy(puVar6 + 2, puVar10 + 2, 2);
                        }
                        __builtin_memcpy(&puVar6, ENT_ADDR(0x1f), 4);
                        __builtin_memcpy(&puVar6, OFS(puVar6, 0xad), 4);
                        if (puVar6 != (unsigned int *)0x0) {
                            puVar10 = puVar7;
                            for (iVar3 = 0x13; iVar3 != 0; iVar3 = iVar3 + -1) {
                                __builtin_memcpy(puVar6, puVar10, 4);
                                puVar10 = puVar10 + 1;
                                puVar6 = puVar6 + 1;
                            }
                            puVar7 = puVar7 + 0x13;
                        }
                    }
                }
            }
            iVar8 = (int)((unsigned int)iVar8 + 0x23u);
            iVar9 = (int)((unsigned int)iVar9 + 1u);
            puVar6 = puVar7;
        } while (iVar9 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8));
    }
    __builtin_memcpy(&DAT_009ed228, puVar7, 4);
    __builtin_memcpy(&_DAT_009ed22c, puVar7 + 1, 4);
    __builtin_memcpy(&DAT_009d8b70, puVar7 + 2, 4);
    __builtin_memcpy(&DAT_009d8b74, puVar7 + 3, 4);
    puVar6 = puVar7 + 4;
    puVar10 = (unsigned int *)&DAT_009a2000;
    for (iVar9 = 0x28; iVar9 != 0; iVar9 = iVar9 + -1) {
        __builtin_memcpy(puVar10, puVar6, 4);
        puVar6 = puVar6 + 1;
        puVar10 = puVar10 + 1;
    }
    __builtin_memcpy(puVar10, puVar6, 2);
    iVar9 = (int)DAT_009a2000;
    puVar6 = (unsigned int *)OFS(puVar7, 0xb2);
    puVar10 = &DAT_009a20c0;
    for (uVar4 = ((unsigned int)iVar9 * 0x126u) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        __builtin_memcpy(puVar10, puVar6, 4);
        puVar6 = puVar6 + 1;
        puVar10 = puVar10 + 1;
    }
    for (uVar4 = ((unsigned int)iVar9 * 0x126u) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *(unsigned char *)puVar10 = *(unsigned char *)puVar6;
        puVar6 = (unsigned int *)((unsigned int)puVar6 + 1u);
        puVar10 = (unsigned int *)((unsigned int)puVar10 + 1u);
    }
    puVar7 = (unsigned int *)((unsigned int)puVar7 + 0xb2u + (unsigned int)(int)DAT_009a2000 * 0x126u);
    uVar4 = (unsigned int)(int)DAT_009a2000 * 0x566u;
    puVar6 = puVar7;
    puVar10 = &DAT_009d8c30;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(puVar10, puVar6, 4);
        puVar6 = puVar6 + 1;
        puVar10 = puVar10 + 1;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *(unsigned char *)puVar10 = *(unsigned char *)puVar6;
        puVar6 = (unsigned int *)((unsigned int)puVar6 + 1u);
        puVar10 = (unsigned int *)((unsigned int)puVar10 + 1u);
    }
    puVar7 = (unsigned int *)((unsigned int)puVar7 + (unsigned int)(int)DAT_009a2000 * 0x566u);
    puVar6 = puVar7;
    puVar10 = &DAT_009d6940;
    for (iVar9 = 0x34c; iVar9 != 0; iVar9 = iVar9 + -1) {
        __builtin_memcpy(puVar10, puVar6, 4);
        puVar6 = puVar6 + 1;
        puVar10 = puVar10 + 1;
    }
    puVar6 = puVar7 + 0x34c;
    puVar10 = &DAT_009ec5e0;
    for (iVar9 = 5; iVar9 != 0; iVar9 = iVar9 + -1) {
        __builtin_memcpy(puVar10, puVar6, 4);
        puVar6 = puVar6 + 1;
        puVar10 = puVar10 + 1;
    }
    __builtin_memcpy(puVar10, puVar6, 2);
    __builtin_memcpy(&DAT_00a08294, OFS(puVar7, 0xd46), 2);
    __builtin_memcpy(&DAT_00a080f8, puVar7 + 0x352, 4);
    DAT_009d8c20 = *(unsigned char *)(puVar7 + 0x353);
    __builtin_memcpy(&DAT_009eca28, OFS(puVar7, 0xd4d), 4);
    __builtin_memcpy(&DAT_009a7ba4, OFS(puVar7, 0xd51), 4);
    __builtin_memcpy(&DAT_009fd430, OFS(puVar7, 0xd55), 4);
    __builtin_memcpy(&DAT_00a0814c, OFS(puVar7, 0xd59), 4);
    __builtin_memcpy(&DAT_009a7b50, OFS(puVar7, 0xd5d), 4);
    __builtin_memcpy(&DAT_009d767c, OFS(puVar7, 0xd61), 4);
    __builtin_memcpy(&_DAT_00a07cec, OFS(puVar7, 0xd65), 4);
    __builtin_memcpy(&DAT_009a1fe0, OFS(puVar7, 0xd69), 4);
    __builtin_memcpy(&DAT_009d1144, OFS(puVar7, 0xd6d), 4);
    __builtin_memcpy(&DAT_009fe7dc, OFS(puVar7, 0xd71), 4);
    __builtin_memcpy(&DAT_009e9640, OFS(puVar7, 0xd75), 4);
    __builtin_memcpy(&DAT_00a00854, OFS(puVar7, 0xd79), 4);
    __builtin_memcpy(&DAT_009a7b58, OFS(puVar7, 0xd7d), 4);
    __builtin_memcpy(&DAT_009d1140, OFS(puVar7, 0xd81), 4);
    __builtin_memcpy(&DAT_009d1310, OFS(puVar7, 0xd85), 4);
    __builtin_memcpy(&DAT_009ec47c, OFS(puVar7, 0xd89), 4);
    __builtin_memcpy(&DAT_009a8e24, OFS(puVar7, 0xd8d), 4);
    __builtin_memcpy(&DAT_009cd274, OFS(puVar7, 0xd91), 4);
    __builtin_memcpy(&DAT_00a07ce8, OFS(puVar7, 0xd95), 4);
    __builtin_memcpy(&DAT_009d1308, OFS(puVar7, 0xd99), 4);
    puVar6 = (unsigned int *)OFS(puVar7, 0xd9d);
    puVar10 = &DAT_009d1320;
    for (iVar9 = 0x1540; iVar9 != 0; iVar9 = iVar9 + -1) {
        __builtin_memcpy(puVar10, puVar6, 4);
        puVar6 = puVar6 + 1;
        puVar10 = puVar10 + 1;
    }
    __builtin_memcpy(&DAT_009a8068, OFS(puVar7, 0x629d), 4);
    __builtin_memcpy(&DAT_009a73a4, OFS(puVar7, 0x62a1), 4);
    puVar6 = (unsigned int *)OFS(puVar7, 0x62a5);
    puVar10 = &DAT_009a7a40;
    for (iVar9 = 0x40; iVar9 != 0; iVar9 = iVar9 + -1) {
        __builtin_memcpy(puVar10, puVar6, 4);
        puVar6 = puVar6 + 1;
        puVar10 = puVar10 + 1;
    }
    __builtin_memcpy(&DAT_009d113c, OFS(puVar7, 0x63a5), 2);
    __builtin_memcpy(&DAT_009a8d40, OFS(puVar7, 0x63a7), 2);
    __builtin_memcpy(&DAT_00523424, OFS(puVar7, 0x63a9), 4);
    puVar7 = (unsigned int *)OFS(puVar7, 0x63ad);
    puVar6 = &DAT_009e9670;
    for (iVar9 = 0xb7a; iVar9 != 0; iVar9 = iVar9 + -1) {
        __builtin_memcpy(puVar6, puVar7, 4);
        puVar7 = puVar7 + 1;
        puVar6 = puVar6 + 1;
    }
    return;
}
