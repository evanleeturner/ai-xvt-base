/* SPDX-License-Identifier: MIT */
/* z_bop 004ddc90 */
extern int FUN_004d48e0(void);
extern void FUN_004d4970(void);
extern void FUN_004d64a0(int *param_1, int param_2, int param_3);
extern void FUN_004d6480(int *param_1, int *param_2);
extern void FUN_004d64e0(int *param_1);
extern void FUN_004b5570(char *param_1, int param_2, int param_3);

/* Packed / unaligned globals: accessed only through __builtin_memcpy. */
extern int DAT_00aa7209;
extern int DAT_00aa720d;
extern int DAT_00aa7211;
extern int DAT_00aa7215;
extern int DAT_00aa7346;
extern int DAT_00aa734a;
extern int DAT_00aa734e;
extern int DAT_00aa7352;
extern unsigned int DAT_00aa7356;
extern int DAT_00aa735a;
extern char DAT_00aa735e;
extern int DAT_00aa7b13;
extern int DAT_00aa7b1f;
extern int DAT_00aa6cfc;

void FUN_004ddc90(void)
{
    int iVar1;
    int iVar2;
    int iVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned int uVar4;
    unsigned char *puVar3;
    unsigned char *puVar9;
    unsigned char *puVar11;
    unsigned char *puVar12;
    unsigned short *puVar5;
    unsigned short *puVar10;
    unsigned short *local_24;
    int local_2c;
    int local_28;
    int local_20[4];
    int local_10[4];
    int x209;
    int x20d;
    int x6cfc;
    int x7b13;
    int x7b1f;
    int x7346;
    int x734a;
    int tmp;
    char x735e;

    __builtin_memcpy(&iVar2, &DAT_00aa7352, 4);
    __builtin_memcpy(&iVar1, &DAT_00aa734e, 4);
    __builtin_memcpy(&x209, &DAT_00aa7209, 4);
    if ((((-1 < x209) && (x209 < 0x280))) ) {
        __builtin_memcpy(&x20d, &DAT_00aa720d, 4);
        if ((-1 < x20d) && (x20d < 0x1e0)) {
            local_20[0] = 0;
            local_20[2] = (int)((unsigned int)iVar1 + 0xffffffffu);
            local_20[1] = 0;
            local_20[3] = (int)((unsigned int)iVar2 + 0xffffffffu);
            __builtin_memcpy(&x209, &DAT_00aa7209, 4);
            __builtin_memcpy(&x20d, &DAT_00aa720d, 4);
            FUN_004d64a0(local_20, x209, x20d);
            FUN_004d6480(local_10, local_20);
            FUN_004d64e0(local_20);
            uVar8 = (unsigned int)iVar1 + ((unsigned int)local_20[2] - (unsigned int)local_10[2]);
            iVar2 = (int)(((unsigned int)local_20[3] - (unsigned int)local_10[3]) + (unsigned int)iVar2);
            x6cfc = FUN_004d48e0();
            DAT_00aa6cfc = x6cfc;
            local_28 = iVar2;
            __builtin_memcpy(&x735e, &DAT_00aa735e, 1);
            __builtin_memcpy(&x7b1f, &DAT_00aa7b1f, 4);
            __builtin_memcpy(&x7b13, &DAT_00aa7b13, 4);
            __builtin_memcpy(&x209, &DAT_00aa7209, 4);
            __builtin_memcpy(&x20d, &DAT_00aa720d, 4);
            __builtin_memcpy(&x7346, &DAT_00aa7346, 4);
            __builtin_memcpy(&x734a, &DAT_00aa734a, 4);
            if (x735e == '\0') {
                if (x7b1f == 8) {
                    puVar3 = (unsigned char *)(unsigned int)((unsigned int)x6cfc + (unsigned int)x7b13 * (unsigned int)x20d + (unsigned int)x209);
                    iVar7 = x7346;
                    puVar9 = (unsigned char *)(unsigned int)x734a;
                    local_2c = iVar2;
                    if (0 < iVar2) {
                        do {
                            puVar11 = puVar3;
                            puVar12 = puVar9;
                            for (uVar4 = uVar8 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                __builtin_memcpy(puVar12, puVar11, 4);
                                puVar11 = puVar11 + 4;
                                puVar12 = puVar12 + 4;
                            }
                            for (uVar4 = uVar8 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *puVar12 = *puVar11;
                                puVar11 = puVar11 + 1;
                                puVar12 = puVar12 + 1;
                            }
                            iVar6 = 0;
                            if (0 < (int)uVar8) {
                                do {
                                    if (*(char *)(unsigned int)((unsigned int)iVar6 + (unsigned int)iVar7) != '\0') {
                                        *(char *)(puVar3 + iVar6) = *(char *)(unsigned int)((unsigned int)iVar6 + (unsigned int)iVar7);
                                    }
                                    iVar6 = (int)((unsigned int)iVar6 + 1u);
                                } while (iVar6 < (int)uVar8);
                            }
                            puVar3 = puVar3 + x7b13;
                            local_2c = (int)((unsigned int)local_2c - 1u);
                            iVar7 = (int)((unsigned int)iVar7 + (unsigned int)iVar1);
                            puVar9 = puVar9 + iVar1;
                        } while (local_2c != 0);
                    }
                }
                else if ((x7b1f == 0x10) &&
                         (local_24 = (unsigned short *)(unsigned int)((unsigned int)x6cfc + (unsigned int)x7b13 * (unsigned int)x20d + (unsigned int)x209 * 2u),
                          0 < iVar2)) {
                    iVar7 = x7346;
                    puVar9 = (unsigned char *)(unsigned int)x734a;
                    do {
                        iVar6 = 0;
                        puVar3 = puVar9;
                        puVar5 = local_24;
                        if (0 < (int)uVar8) {
                            do {
                                *(unsigned short *)puVar3 = *puVar5;
                                if (*(char *)(unsigned int)((unsigned int)iVar6 + (unsigned int)iVar7) == '\x01') {
                                    *puVar5 = 0x1f;
                                }
                                else if (*(char *)(unsigned int)((unsigned int)iVar6 + (unsigned int)iVar7) == -1) {
                                    *puVar5 = 0xffff;
                                }
                                iVar6 = (int)((unsigned int)iVar6 + 1u);
                                puVar3 = puVar3 + 2;
                                puVar5 = puVar5 + 1;
                            } while (iVar6 < (int)uVar8);
                        }
                        iVar7 = (int)((unsigned int)iVar7 + (unsigned int)iVar1);
                        puVar9 = puVar9 + (unsigned int)iVar1 * 2u;
                        local_24 = (unsigned short *)((unsigned char *)local_24 + ((unsigned int)x7b13 & 0xfffffffeu));
                        local_28 = (int)((unsigned int)local_28 - 1u);
                    } while (local_28 != 0);
                }
            }
            else {
                if (x7b1f == 8) {
                    puVar3 = (unsigned char *)(unsigned int)((unsigned int)x6cfc + (unsigned int)x7b13 * (unsigned int)x20d + (unsigned int)x209);
                    iVar7 = iVar2;
                    puVar9 = (unsigned char *)(unsigned int)x734a;
                    if (0 < iVar2) {
                        do {
                            puVar11 = puVar3;
                            puVar12 = puVar9;
                            for (uVar4 = uVar8 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                __builtin_memcpy(puVar12, puVar11, 4);
                                puVar11 = puVar11 + 4;
                                puVar12 = puVar12 + 4;
                            }
                            for (uVar4 = uVar8 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *puVar12 = *puVar11;
                                puVar11 = puVar11 + 1;
                                puVar12 = puVar12 + 1;
                            }
                            puVar3 = puVar3 + x7b13;
                            iVar7 = (int)((unsigned int)iVar7 - 1u);
                            puVar9 = puVar9 + iVar1;
                        } while (iVar7 != 0);
                    }
                }
                else if ((x7b1f == 0x10) &&
                         (puVar5 = (unsigned short *)(unsigned int)((unsigned int)x7b13 * (unsigned int)x20d + (unsigned int)x209 * 2u + (unsigned int)x6cfc),
                          0 < iVar2)) {
                    puVar9 = (unsigned char *)(unsigned int)x734a;
                    do {
                        uVar4 = uVar8;
                        puVar3 = puVar9;
                        puVar10 = puVar5;
                        if (0 < (int)uVar8) {
                            do {
                                *(unsigned short *)puVar3 = *puVar10;
                                uVar4 = uVar4 - 1;
                                puVar3 = puVar3 + 2;
                                puVar10 = puVar10 + 1;
                            } while (uVar4 != 0);
                        }
                        puVar9 = puVar9 + (unsigned int)iVar1 * 2u;
                        puVar5 = (unsigned short *)((unsigned char *)puVar5 + ((unsigned int)x7b13 & 0xfffffffeu));
                        local_28 = (int)((unsigned int)local_28 - 1u);
                    } while (local_28 != 0);
                }
                __builtin_memcpy(&x209, &DAT_00aa7209, 4);
                __builtin_memcpy(&x20d, &DAT_00aa720d, 4);
                FUN_004b5570(&DAT_00aa735e, x209, x20d);
            }
            FUN_004d4970();
            __builtin_memcpy(&tmp, &DAT_00aa7209, 4);
            __builtin_memcpy(&DAT_00aa7211, &tmp, 4);
            __builtin_memcpy(&tmp, &DAT_00aa720d, 4);
            __builtin_memcpy(&DAT_00aa7215, &tmp, 4);
            __builtin_memcpy(&DAT_00aa7356, &uVar8, 4);
            __builtin_memcpy(&DAT_00aa735a, &iVar2, 4);
        }
    }
    return;
}
