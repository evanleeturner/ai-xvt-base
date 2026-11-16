/* SPDX-License-Identifier: MIT */
/* z_bop 00407090 */
extern unsigned int DAT_009a1fe8;
extern unsigned short DAT_00a082d8;
extern unsigned int DAT_009e9640;
extern unsigned int DAT_00a00854;
extern unsigned int DAT_009a8e18;
extern unsigned int DAT_009a20b0;
extern unsigned int DAT_009cd26c;
extern unsigned int DAT_009fd438;
extern unsigned int DAT_009a8c00;
extern int _DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009ecc34;
extern unsigned char DAT_0051a44f[];
extern unsigned int DAT_0051a194[];
extern unsigned char s_starshipprotectpln_0051a2fc[];
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d9155[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009fd458[];
extern unsigned char DAT_009e967c[];

unsigned short __cdecl FUN_00459750(int a);
void __cdecl FUN_004269e0(unsigned int a);
short __cdecl FUN_00405860(int a);
long long __cdecl FUN_004ae120(unsigned int a);
long long __cdecl FUN_004ae080(unsigned int a);
void __cdecl FUN_00427bd0(short a, short b, int c);
void __cdecl FUN_004464c0(int a);
void __cdecl FUN_004034e0(unsigned int a, unsigned int b);
void __cdecl FUN_00430610(int a, char b, char c, char d, int e);
void __cdecl FUN_0042e720(unsigned int a, int b);

#define P8(a) (*(unsigned char *)(a))

unsigned int __cdecl FUN_00407090(unsigned int param_1, int param_2)
{
    char *pcVar1;
    unsigned char bVar2;
    int iVar3;
    unsigned int uVar4;
    int iVar5;
    int bVar6;
    short sVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    int iVar10;
    unsigned char *puVar11;
    int iVar12;
    unsigned short *puVar13;
    unsigned short uVar14;
    unsigned int uVar15;
    int iVar16;
    int bVar17;
    long long lVar18;
    long long lVar19;
    unsigned int local_38;
    unsigned int local_28;
    unsigned int local_20;
    unsigned char local_c;
    unsigned short uStack_8 = 0;
    unsigned short local_6;
    int local_sub;
    int local_src;
    unsigned short local_u16;
    short local_s16;
    unsigned int local_u32;

    iVar5 = (int)(param_1 * 0x23);
    __builtin_memcpy(&iVar3, (void *)(DAT_009a1fe8 + 0x1b + iVar5), 4);
    if (iVar3 == -1) {
        local_c = 7;
        uVar14 = FUN_00459750(7);
        uVar15 = (unsigned int)uVar14;
    } else {
        local_c = 6;
        uVar15 = (unsigned int)DAT_00a082d8 + (unsigned int)iVar3 * 0xc;
        uVar9 = uVar15 + 0xc;
        if (DAT_0051a44f[param_2] != 0) {
            uVar15 = uVar15 + 8;
        }
        uVar8 = uVar15 & 0xffff;
        while (uVar8 < uVar9) {
            if (P8((uVar15 & 0xffff) * 0x23 + 3 + DAT_009a1fe8) == 0) {
                iVar3 = (int)((uVar15 & 0xffff) * 0x23);
                __builtin_memcpy(&local_sub, (void *)(DAT_009a1fe8 + 0x1f + iVar3), 4);
                local_u16 = 0;
                __builtin_memcpy((void *)(local_sub + 0x85), &local_u16, 2);
                P8(local_sub + 1) = 0;
                break;
            }
            uVar14 = (unsigned short)(uVar15 + 1);
            uVar15 = (unsigned int)uVar14;
            uVar8 = (unsigned int)uVar14;
        }
        if (uVar9 <= (uVar15 & 0xffff)) {
            uVar15 = (unsigned int)DAT_00a082d8 + 0x60;
            uVar9 = (unsigned int)DAT_00a082d8 + 0x80;
            uVar8 = uVar15 & 0xffff;
            while (uVar8 < uVar9) {
                if (P8((uVar15 & 0xffff) * 0x23 + 3 + DAT_009a1fe8) == 0) {
                    iVar3 = (int)((uVar15 & 0xffff) * 0x23);
                    __builtin_memcpy(&local_sub, (void *)(DAT_009a1fe8 + 0x1f + iVar3), 4);
                    local_u16 = 0;
                    __builtin_memcpy((void *)(local_sub + 0x85), &local_u16, 2);
                    P8(local_sub + 1) = 0;
                    break;
                }
                uVar14 = (unsigned short)(uVar15 + 1);
                uVar15 = (unsigned int)uVar14;
                uVar8 = (unsigned int)uVar14;
            }
        }
        if (uVar9 <= (uVar15 & 0xffff)) {
            return 0xffffffff;
        }
    }
    if ((short)uVar15 == -1) {
        return 0xffffffff;
    }
    iVar16 = (int)((unsigned int)iVar5 + DAT_009a1fe8);
    bVar2 = P8(iVar16 + 3);
    uVar9 = uVar15 & 0xffff;
    iVar3 = (int)(uVar9 * 0x23);
    __builtin_memcpy(&local_sub, (void *)(DAT_009a1fe8 + 0x1f + iVar3), 4);
    P8(local_sub) = 1;
    P8(DAT_009a1fe8 + 2 + iVar3) = local_c;
    P8(DAT_009a1fe8 + 3 + iVar3) = (unsigned char)param_2;
    local_u16 = 1;
    __builtin_memcpy((void *)(local_sub + 0x83), &local_u16, 2);
    local_u16 = (unsigned short)param_1;
    __builtin_memcpy((void *)(local_sub + 0x85), &local_u16, 2);
    P8(local_sub + 0x87) = bVar2;
    FUN_004269e0(((unsigned int)uStack_8 << 16) | (unsigned int)(unsigned short)bVar2);
    __builtin_memcpy(&local_src, (void *)(iVar16 + 0x1f), 4);
    P8(local_sub + 0x88) = P8(local_src + 0x88);
    P8(local_sub + 0x89) = P8(local_src + 0x89);
    __builtin_memcpy(&local_s16, (void *)(iVar16 + 0x12), 2);
    local_s16 = (short)(0xFFFF8000u - (unsigned int)(int)local_s16);
    __builtin_memcpy((void *)(DAT_009a1fe8 + 0x12 + iVar3), &local_s16, 2);
    __builtin_memcpy(&local_u16, (void *)(iVar16 + 0x14), 2);
    __builtin_memcpy((void *)(DAT_009a1fe8 + 0x14 + iVar3), &local_u16, 2);
    __builtin_memcpy(&local_s16, (void *)(iVar16 + 0x10), 2);
    local_s16 = (short)((unsigned int)(int)local_s16 + 0xFFFF8000u);
    __builtin_memcpy((void *)(DAT_009a1fe8 + 0x10 + iVar3), &local_s16, 2);
    local_u16 = (unsigned short)(*(unsigned short *)(s_starshipprotectpln_0051a2fc + param_2 * 2 + 10) >> 1);
    __builtin_memcpy((void *)(local_sub + 0x79), &local_u16, 2);
    __builtin_memcpy(&local_s16, (void *)(local_src + 0x79), 2);
    local_s16 = (short)(local_s16 + *(short *)(s_starshipprotectpln_0051a2fc + param_2 * 2 + 10));
    *(short *)(DAT_009a8e18 + 8 + (uVar9 - DAT_009e9640) * 10) = local_s16;
    __builtin_memcpy(&local_u16, (void *)(local_src + 0x79), 2);
    local_u32 = (unsigned int)local_u16 + DAT_0051a194[param_2];
    __builtin_memcpy((void *)(local_sub + 0x7d), &local_u32, 4);
    puVar11 = (unsigned char *)(local_sub + 0x7d);
    __builtin_memcpy(&local_u32, puVar11, 4);
    if (local_u32 < DAT_0051a194[param_2]) {
        local_u32 = DAT_0051a194[param_2];
        __builtin_memcpy(puVar11, &local_u32, 4);
    }
    sVar7 = FUN_00405860(param_2);
    __builtin_memcpy((void *)(local_sub + 0x81), &sVar7, 2);
    P8(local_sub + 0x92) = 1;
    __builtin_memcpy(&iVar12, (void *)(DAT_009a1fe8 + 0x1f + iVar3), 4);
    P8(iVar12 + 0x8b) = P8(iVar12 + 0x92);
    __builtin_memcpy(&uVar4, (void *)(iVar16 + 4), 4);
    iVar12 = (int)(DAT_009a1fe8 + iVar3);
    __builtin_memcpy((void *)(iVar12 + 4), &uVar4, 4);
    __builtin_memcpy(&local_sub, (void *)(iVar12 + 0x1f), 4);
    __builtin_memcpy((void *)(local_sub + 6), &uVar4, 4);
    __builtin_memcpy(&uVar4, (void *)(iVar16 + 8), 4);
    __builtin_memcpy((void *)(iVar12 + 8), &uVar4, 4);
    __builtin_memcpy(&local_sub, (void *)(iVar12 + 0x1f), 4);
    __builtin_memcpy((void *)(local_sub + 10), &uVar4, 4);
    __builtin_memcpy(&uVar4, (void *)(iVar16 + 0xc), 4);
    __builtin_memcpy((void *)(iVar12 + 0xc), &uVar4, 4);
    __builtin_memcpy(&local_sub, (void *)(iVar12 + 0x1f), 4);
    __builtin_memcpy((void *)(local_sub + 0xe), &uVar4, 4);
    lVar18 = FUN_004ae120((unsigned int)P8(iVar12 + 3));
    lVar19 = FUN_004ae080((unsigned int)bVar2);
    uVar8 = (unsigned int)(unsigned short)((short)lVar18 + (short)lVar19);
    __builtin_memcpy(&local_s16, (void *)(iVar12 + 0x12), 2);
    __builtin_memcpy(&sVar7, (void *)(iVar12 + 0x10), 2);
    FUN_00427bd0(local_s16, sVar7, iVar12);
    __builtin_memcpy(&local_sub, (void *)(iVar12 + 0x1f), 4);
    __builtin_memcpy(&local_s16, (void *)(local_sub + 0x8c), 2);
    lVar18 = (long long)(int)local_s16 * (long long)(int)uVar8;
    DAT_009a20b0 = (unsigned int)lVar18 >> 0xf |
                   (unsigned int)(int)((unsigned long long)lVar18 >> 0x20) << 0x11;
    __builtin_memcpy(&local_sub, (void *)(iVar12 + 0x1f), 4);
    __builtin_memcpy(&local_s16, (void *)(local_sub + 0x8e), 2);
    lVar18 = (long long)(int)local_s16 * (long long)(int)uVar8;
    DAT_009cd26c = (unsigned int)lVar18 >> 0xf |
                   (unsigned int)(int)((unsigned long long)lVar18 >> 0x20) << 0x11;
    __builtin_memcpy(&local_sub, (void *)(iVar12 + 0x1f), 4);
    __builtin_memcpy(&local_s16, (void *)(local_sub + 0x90), 2);
    lVar18 = (long long)(int)local_s16 * (long long)(int)uVar8;
    DAT_009fd438 = (unsigned int)lVar18 >> 0xf |
                   (unsigned int)(int)((unsigned long long)lVar18 >> 0x20) << 0x11;
    FUN_004464c0(iVar12);
    iVar12 = (int)((unsigned int)(unsigned short)((short)uVar15 - (short)DAT_009e9640) * 10);
    __builtin_memcpy(&local_sub, (void *)(DAT_009a1fe8 + 0x1f + iVar3), 4);
    local_u32 = DAT_009a8e18 + iVar12;
    __builtin_memcpy((void *)(local_sub + 0xa5), &local_u32, 4);
    P8(DAT_009a8e18 + 1 + iVar12) = 0;
    *(unsigned short *)(DAT_009a8e18 + 4 + iVar12) = 0xffff;
    *(unsigned short *)(DAT_009a8e18 + 6 + iVar12) = 0;
    *(unsigned short *)(DAT_009a8e18 + 2 + iVar12) = 0xffff;
    P8(DAT_009a8e18 + iVar12) = P8(DAT_009a1fe8 + 0x1b + iVar5);
    uVar15 = (unsigned int)P8(DAT_009a1fe8 + iVar5 + 0x16);
    __builtin_memcpy(&local_sub, (void *)(DAT_009a1fe8 + iVar5 + 0x1f), 4);
    __builtin_memcpy(&iVar16, (void *)(local_sub + 0xa9), 4);
    if ((DAT_009d8c84[uVar15 * 0x566] != 0x15) && (DAT_009d9155[uVar15 * 0x566] != 0x15)) {
        pcVar1 = (char *)(iVar16 + 0x247);
        *pcVar1 = (char)(*pcVar1 - 1);
    }
    local_u16 = 0x1d8;
    __builtin_memcpy((void *)(iVar16 + 0x24a), &local_u16, 2);
    if (param_2 != 0x9b) goto LAB_0040785a;
    local_6 = 0xffff;
    uStack_8 = 0xffff;
    local_20 = 0xffffffff;
    local_38 = 0xffffffff;
    if (DAT_009e9640 < DAT_00a00854) {
        local_28 = DAT_009e9640 * 0x23;
        uVar15 = DAT_009e9640;
        do {
            bVar2 = P8(local_28 + DAT_009a1fe8 + 3);
            __builtin_memcpy(&local_sub, (void *)(local_28 + DAT_009a1fe8 + 0x1f), 4);
            if ((((bVar2 != 0) && (P8(local_sub) == 1)) &&
                 (DAT_0051a44f[bVar2] != 0)) &&
                ((unsigned int)*(unsigned short *)(DAT_009a8e18 + 4 + (uVar15 - DAT_009e9640) * 10) == param_1)) {
                iVar16 = 0;
                if (DAT_009e9640 < DAT_00a00854) {
                    puVar13 = (unsigned short *)(DAT_009a8e18 + 4);
                    iVar10 = (int)(DAT_00a00854 - DAT_009e9640);
                    do {
                        if ((bVar2 == 0x9b) && ((unsigned int)*puVar13 == uVar15)) {
                            iVar16 = (int)((unsigned int)iVar16 + 1);
                        }
                        puVar13 = puVar13 + 5;
                        iVar10 = (int)((unsigned int)iVar10 - 1);
                    } while (iVar10 != 0);
                }
                FUN_004034e0(param_1, uVar15);
                if (iVar16 == 0) {
                    if (DAT_009a8c00 < local_20) {
                        local_20 = DAT_009a8c00;
                        local_6 = (unsigned short)uVar15;
                    }
                } else if (DAT_009a8c00 < local_38) {
                    local_38 = DAT_009a8c00;
                    uStack_8 = (unsigned short)uVar15;
                }
            }
            uVar15 = uVar15 + 1;
            local_28 = local_28 + 0x23;
        } while (uVar15 < DAT_00a00854);
    }
    if (local_6 == 0xffff) {
        uVar14 = (unsigned short)_DAT_00a07cec;
        if ((int)((unsigned int)_DAT_00a07cec & 0xffff) < DAT_009a1fe0) {
            do {
                iVar16 = (int)((unsigned int)uVar14 * 0x23 + DAT_009a1fe8);
                if (P8(iVar16 + 3) != 0) {
                    __builtin_memcpy(&local_sub, (void *)(iVar16 + 0x1f), 4);
                    __builtin_memcpy(&iVar10, (void *)(local_sub + 0xa9), 4);
                    if (P8(iVar10 + 7) == 0) {
                        bVar6 = 0;
                        __builtin_memcpy(&local_sub, (void *)(iVar16 + 0x1b), 4);
                        if (local_sub == -1) {
                            __builtin_memcpy(&local_u16, (void *)(iVar10 + 0x42), 2);
                            if ((unsigned int)local_u16 == param_1) {
LAB_00407797:
                                bVar6 = 1;
                            }
                        } else {
                            __builtin_memcpy(&local_u16,
                                             DAT_009e967c + (unsigned int)local_sub * 0x5bd, 2);
                            uVar15 = (unsigned int)local_u16;
                            bVar17 = 0;
                            if ((unsigned int)DAT_009d8c88[(unsigned int)P8((param_1 & 0xffff) * 0x23 + 0x16 + DAT_009a1fe8) * 0x566] !=
                                uVar15) {
                                bVar17 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)P8((param_1 & 0xffff) * 0x23 + 0x16 + DAT_009a1fe8) * 0x566] +
                                                      uVar15 * 0x1e5] == 0;
                            }
                            if (bVar17) goto LAB_00407797;
                        }
                        if (bVar6) {
                            FUN_004034e0(param_1, (unsigned int)uVar14);
                            if ((DAT_009a8c00 < local_20) && ((int)DAT_009a8c00 < 0x8000)) {
                                local_20 = DAT_009a8c00;
                                local_6 = uVar14;
                            }
                        }
                    }
                }
                uVar14 = (unsigned short)(uVar14 + 1);
            } while ((int)(unsigned int)uVar14 < DAT_009a1fe0);
        }
        if (local_6 != 0xffff) goto LAB_004077e9;
        if (uStack_8 == 0xffff) goto LAB_0040785a;
        *(unsigned short *)(DAT_009a8e18 + 4 + iVar12) = uStack_8;
        P8(DAT_009a8e18 + 1 + iVar12) = 6;
        uVar15 = 0xffff0000u | (unsigned int)uStack_8;
    } else {
LAB_004077e9:
        *(unsigned short *)(DAT_009a8e18 + 4 + iVar12) = local_6;
        P8(DAT_009a8e18 + 1 + iVar12) = 6;
        uVar15 = (unsigned int)local_6;
    }
    __builtin_memcpy(&local_u16, (void *)((uVar15 & 0xffff) * 0x23 + DAT_009a1fe8), 2);
    *(unsigned short *)(DAT_009a8e18 + 6 + iVar12) = local_u16;
LAB_0040785a:
    uVar15 = (unsigned int)*(unsigned short *)(DAT_009a8e18 + 4 + iVar12);
    if (((int)_DAT_00a07cec <= (int)uVar15) && ((int)uVar15 < DAT_009a1fe0)) {
        __builtin_memcpy(&local_sub, (void *)(DAT_009a1fe8 + 0x1f + iVar3), 4);
        puVar13 = (unsigned short *)(local_sub + 0x81);
        __builtin_memcpy(&local_u16, puVar13, 2);
        local_u16 = (unsigned short)(local_u16 >> 1);
        __builtin_memcpy(puVar13, &local_u16, 2);
    }
    uVar14 = *(unsigned short *)(DAT_009a8e18 + 4 + iVar12);
    if (uVar14 != 0xffff) {
        __builtin_memcpy(&local_sub, (void *)((unsigned int)uVar14 * 0x23 + 0x1b + DAT_009a1fe8), 4);
        if (local_sub == DAT_009ecc34) {
            FUN_00430610(0x25, '\0', '\0', '\0', -1);
        }
    }
    __builtin_memcpy(&iVar3, (void *)(DAT_009a1fe8 + 0x1b + iVar5), 4);
    if (iVar3 != -1) {
        FUN_0042e720(uVar9, iVar3);
    }
    return uVar9;
}
