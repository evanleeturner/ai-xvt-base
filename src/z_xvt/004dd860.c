/* SPDX-License-Identifier: MIT */
/* z_xvt 004dd860 */
extern int DAT_00758d49;
extern int DAT_00758d4d;
extern int DAT_00758d51;
extern int DAT_00758d55;
extern int DAT_00758e86;
extern unsigned int *DAT_00758e8a;
extern int DAT_00758e8e;
extern int DAT_00758e92;
extern unsigned int DAT_00758e96;
extern int DAT_00758e9a;
extern char DAT_00758e9e;
extern int DAT_0075964b;
extern int DAT_00759657;
extern int DAT_00758830;

void FUN_004dd030(int *param_1, int param_2, int param_3);
void FUN_004dd000(int *param_1, int *param_2);
void FUN_004dd0b0(int *param_1);
int FUN_004d4d00(void);
void FUN_004d0e60(char *param_1, int param_2, int param_3);
void FUN_004d4d90(void);

void __stdcall FUN_004dd860(void)
{
    int iVar1;
    int iVar2;
    unsigned int *puVar3;
    unsigned int uVar4;
    unsigned short *puVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned int *puVar8;
    unsigned short *puVar9;
    unsigned int *puVar10;
    int iVar11;
    unsigned int *puVar12;
    int local_2c;
    int local_28;
    unsigned short *local_24;
    int local_20[4];
    int local_10[4];
    int x, y, stride, bpp;

    __builtin_memcpy(&iVar2, &DAT_00758e92, 4);
    __builtin_memcpy(&iVar1, &DAT_00758e8e, 4);
    __builtin_memcpy(&x, &DAT_00758d49, 4);
    if ((-1 < x) && (x < 0x280)) {
        __builtin_memcpy(&y, &DAT_00758d4d, 4);
        if ((-1 < y) && (y < 0x1e0)) {
            local_20[0] = 0;
            local_20[2] = (int)((unsigned int)iVar1 - 1u);
            local_20[1] = 0;
            local_20[3] = (int)((unsigned int)iVar2 - 1u);
            FUN_004dd030(local_20, x, y);
            FUN_004dd000(local_10, local_20);
            FUN_004dd0b0(local_20);
            uVar7 = (unsigned int)iVar1 + ((unsigned int)local_20[2] - (unsigned int)local_10[2]);
            iVar2 = (int)(((unsigned int)local_20[3] - (unsigned int)local_10[3]) + (unsigned int)iVar2);
            DAT_00758830 = FUN_004d4d00();
            local_28 = iVar2;
            __builtin_memcpy(&x, &DAT_00758d49, 4);
            __builtin_memcpy(&y, &DAT_00758d4d, 4);
            __builtin_memcpy(&stride, &DAT_0075964b, 4);
            __builtin_memcpy(&bpp, &DAT_00759657, 4);
            if (DAT_00758e9e == '\0') {
                if (bpp == 8) {
                    puVar3 = (unsigned int *)(int)((unsigned int)DAT_00758830 + (unsigned int)y * (unsigned int)stride + (unsigned int)x);
                    __builtin_memcpy(&iVar6, &DAT_00758e86, 4);
                    __builtin_memcpy(&puVar8, &DAT_00758e8a, 4);
                    local_2c = iVar2;
                    if (0 < iVar2) {
                        do {
                            puVar10 = puVar3;
                            puVar12 = puVar8;
                            for (uVar4 = uVar7 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *puVar12 = *puVar10;
                                puVar10 = puVar10 + 1;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar4 = uVar7 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *(unsigned char *)puVar12 = *(unsigned char *)puVar10;
                                puVar10 = (unsigned int *)((unsigned int)puVar10 + 1u);
                                puVar12 = (unsigned int *)((unsigned int)puVar12 + 1u);
                            }
                            iVar11 = 0;
                            if (0 < (int)uVar7) {
                                do {
                                    if (*(char *)((unsigned int)iVar11 + (unsigned int)iVar6) != '\0') {
                                        *(char *)((unsigned int)iVar11 + (unsigned int)puVar3) = *(char *)((unsigned int)iVar11 + (unsigned int)iVar6);
                                    }
                                    iVar11 = (int)((unsigned int)iVar11 + 1u);
                                } while (iVar11 < (int)uVar7);
                            }
                            puVar3 = (unsigned int *)((unsigned int)puVar3 + (unsigned int)stride);
                            local_2c = (int)((unsigned int)local_2c - 1u);
                            iVar6 = (int)((unsigned int)iVar6 + (unsigned int)iVar1);
                            puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)iVar1);
                        } while (local_2c != 0);
                    }
                }
                else if (bpp == 0x10) {
                    local_24 = (unsigned short *)(int)((unsigned int)DAT_00758830 + (unsigned int)y * (unsigned int)stride + (unsigned int)x * 2u);
                    if (0 < iVar2) {
                        __builtin_memcpy(&iVar6, &DAT_00758e86, 4);
                        __builtin_memcpy(&puVar8, &DAT_00758e8a, 4);
                        do {
                            iVar11 = 0;
                            puVar3 = puVar8;
                            puVar5 = local_24;
                            if (0 < (int)uVar7) {
                                do {
                                    *(unsigned short *)puVar3 = *puVar5;
                                    if (*(char *)((unsigned int)iVar11 + (unsigned int)iVar6) == '\x01') {
                                        *puVar5 = 0x1f;
                                    }
                                    else if (*(char *)((unsigned int)iVar11 + (unsigned int)iVar6) == -1) {
                                        *puVar5 = 0xffff;
                                    }
                                    iVar11 = (int)((unsigned int)iVar11 + 1u);
                                    puVar3 = (unsigned int *)((unsigned int)puVar3 + 2u);
                                    puVar5 = puVar5 + 1;
                                } while (iVar11 < (int)uVar7);
                            }
                            iVar6 = (int)((unsigned int)iVar6 + (unsigned int)iVar1);
                            puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)iVar1 * 2u);
                            local_24 = (unsigned short *)((unsigned int)local_24 + ((unsigned int)stride & 0xfffffffeu));
                            local_28 = (int)((unsigned int)local_28 - 1u);
                        } while (local_28 != 0);
                    }
                }
            }
            else {
                if (bpp == 8) {
                    puVar3 = (unsigned int *)(int)((unsigned int)DAT_00758830 + (unsigned int)y * (unsigned int)stride + (unsigned int)x);
                    iVar6 = iVar2;
                    __builtin_memcpy(&puVar8, &DAT_00758e8a, 4);
                    if (0 < iVar2) {
                        do {
                            puVar10 = puVar3;
                            puVar12 = puVar8;
                            for (uVar4 = uVar7 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *puVar12 = *puVar10;
                                puVar10 = puVar10 + 1;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar4 = uVar7 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *(unsigned char *)puVar12 = *(unsigned char *)puVar10;
                                puVar10 = (unsigned int *)((unsigned int)puVar10 + 1u);
                                puVar12 = (unsigned int *)((unsigned int)puVar12 + 1u);
                            }
                            puVar3 = (unsigned int *)((unsigned int)puVar3 + (unsigned int)stride);
                            iVar6 = (int)((unsigned int)iVar6 - 1u);
                            puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)iVar1);
                        } while (iVar6 != 0);
                    }
                }
                else if (bpp == 0x10) {
                    puVar5 = (unsigned short *)(int)((unsigned int)y * (unsigned int)stride + (unsigned int)x * 2u + (unsigned int)DAT_00758830);
                    if (0 < iVar2) {
                        __builtin_memcpy(&puVar8, &DAT_00758e8a, 4);
                        do {
                            uVar4 = uVar7;
                            puVar3 = puVar8;
                            puVar9 = puVar5;
                            if (0 < (int)uVar7) {
                                do {
                                    *(unsigned short *)puVar3 = *puVar9;
                                    uVar4 = uVar4 - 1;
                                    puVar3 = (unsigned int *)((unsigned int)puVar3 + 2u);
                                    puVar9 = puVar9 + 1;
                                } while (uVar4 != 0);
                            }
                            puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)iVar1 * 2u);
                            puVar5 = (unsigned short *)((unsigned int)puVar5 + ((unsigned int)stride & 0xfffffffeu));
                            local_28 = (int)((unsigned int)local_28 - 1u);
                        } while (local_28 != 0);
                    }
                }
                FUN_004d0e60(&DAT_00758e9e, x, y);
            }
            FUN_004d4d90();
            __builtin_memcpy(&x, &DAT_00758d49, 4);
            __builtin_memcpy(&y, &DAT_00758d4d, 4);
            __builtin_memcpy(&DAT_00758d51, &x, 4);
            __builtin_memcpy(&DAT_00758d55, &y, 4);
            __builtin_memcpy(&DAT_00758e96, &uVar7, 4);
            __builtin_memcpy(&DAT_00758e9a, &iVar2, 4);
        }
    }
    return;
}
