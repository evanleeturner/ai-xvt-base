/* SPDX-License-Identifier: MIT */
/* z_bop 004d6550 */
extern unsigned int DAT_00aa6cfc;
extern unsigned char DAT_00aa7b13[4];   /* packed: unaligned address */
extern unsigned char DAT_00aa7b1f[4];   /* packed: unaligned address */
extern unsigned char DAT_00aa7b23;

void __cdecl FUN_004d6480(int *param_1, int *param_2);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004d64e0(int *param_1);

/* local_10, local_c, local_8, local_4 are contiguous stack slots in the original */
#define local_10 L.a
#define local_c  L.b
#define local_8  L.c
#define local_4  L.d

void __cdecl FUN_004d6550(int *param_1, int param_2, int param_3, unsigned int param_4)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int *puVar4;
    unsigned char uVar5;
    unsigned int uVar6;
    unsigned int *puVar7;
    unsigned short *puVar8;
    unsigned short *local_14;
    struct { int a; int b; int c; int d; } L;

    if ((*param_1 < param_1[2]) && (param_1[1] < param_1[3])) {
        FUN_004d6480(&local_10, param_1);
        FUN_004d64a0(&local_10, param_2, param_3);
        if ((local_10 < 0x281) && (((-1 < local_8) && (local_c < 0x1e1)) && (-1 < local_4))) {
            FUN_004d64e0(&local_10);
            __builtin_memcpy(&uVar2, DAT_00aa7b13, 4);
            local_4 = (int)((unsigned int)local_4 + 1u);
            uVar1 = ((unsigned int)local_8 - (unsigned int)local_10) + 1u;
            __builtin_memcpy(&uVar6, DAT_00aa7b1f, 4);
            if (uVar6 == 8) {
                puVar4 = (unsigned int *)(uVar2 * (unsigned int)local_c + DAT_00aa6cfc + (unsigned int)local_10);
                if (local_c < local_4) {
                    local_4 = (int)((unsigned int)local_4 - (unsigned int)local_c);
                    do {
                        uVar5 = (unsigned char)param_4;
                        uVar3 = ((unsigned int)uVar5 << 24) | ((unsigned int)uVar5 << 16) |
                                ((unsigned int)uVar5 << 8) | (unsigned int)uVar5;
                        puVar7 = puVar4;
                        for (uVar6 = uVar1 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                            __builtin_memcpy(puVar7, &uVar3, 4);
                            puVar7 = puVar7 + 1;
                        }
                        for (uVar6 = uVar1 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                            *(unsigned char *)puVar7 = uVar5;
                            puVar7 = (unsigned int *)((unsigned int)puVar7 + 1u);
                        }
                        puVar4 = (unsigned int *)((unsigned int)puVar4 + uVar2);
                        local_4 = local_4 + -1;
                    } while (local_4 != 0);
                    return;
                }
            }
            else {
                if (uVar6 != 0x10) {
                    return;
                }
                local_14 = (unsigned short *)(uVar2 * (unsigned int)local_c +
                                              (unsigned int)local_10 * 2u + DAT_00aa6cfc);
                if (DAT_00aa7b23 == 0) {
                    if (local_c < local_4) {
                        local_4 = (int)((unsigned int)local_4 - (unsigned int)local_c);
                        uVar2 = uVar2 & 0xfffffffe;
                        do {
                            uVar6 = uVar1;
                            puVar8 = local_14;
                            if (0 < (int)uVar1) {
                                do {
                                    uVar3 = (unsigned int)*puVar8;
                                    uVar3 = ((((uVar3 & 0xf800) * 0x20 + (uVar3 & 0x7e0) * 8 + (uVar3 & 0x1f)) >> 1) +
                                             ((((param_4 & 0xf800) * 0x20 + (param_4 & 0x7e0) * 8 + (param_4 & 0x1f)) >> 1)));
                                    uVar6 = uVar6 - 1;
                                    *puVar8 = (unsigned short)((((unsigned short)(uVar3 >> 3) & 0x7e0) +
                                                                ((unsigned short)(uVar3 >> 5) & 0xf800)) +
                                                               ((unsigned short)uVar3 & 0x1f));
                                    puVar8 = puVar8 + 1;
                                } while (uVar6 != 0);
                            }
                            local_14 = (unsigned short *)((unsigned int)local_14 + uVar2);
                            local_4 = local_4 + -1;
                        } while (local_4 != 0);
                    }
                }
                else if (local_c < local_4) {
                    local_4 = (int)((unsigned int)local_4 - (unsigned int)local_c);
                    uVar2 = uVar2 & 0xfffffffe;
                    do {
                        uVar6 = uVar1;
                        puVar8 = local_14;
                        if (0 < (int)uVar1) {
                            do {
                                uVar3 = (unsigned int)*puVar8;
                                uVar3 = ((((uVar3 & 0x7c00) * 0x40 + (uVar3 & 0x3e0) * 8 + (uVar3 & 0x1f)) >> 1) +
                                         ((((param_4 & 0x7c00) * 0x40 + (param_4 & 0x3e0) * 8 + (param_4 & 0x1f)) >> 1)));
                                uVar6 = uVar6 - 1;
                                *puVar8 = (unsigned short)((((unsigned short)(uVar3 >> 3) & 0x3e0) +
                                                            ((unsigned short)(uVar3 >> 6) & 0x7c00)) +
                                                           ((unsigned short)uVar3 & 0x1f));
                                puVar8 = puVar8 + 1;
                            } while (uVar6 != 0);
                        }
                        local_14 = (unsigned short *)((unsigned int)local_14 + uVar2);
                        local_4 = local_4 + -1;
                    } while (local_4 != 0);
                    return;
                }
            }
        }
    }
    return;
}

#undef local_10
#undef local_c
#undef local_8
#undef local_4
