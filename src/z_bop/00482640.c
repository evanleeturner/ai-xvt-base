/* SPDX-License-Identifier: MIT */
/* z_bop 00482640 */
typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned int u32;
typedef unsigned long long u64;

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

extern u16 DAT_009a73a0;
extern u8 DAT_009e9670[];
extern u32 DAT_009a1ff8;
extern int DAT_009a1fe0;
extern u8 *DAT_009a1fe8;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern u8 DAT_0051c659[];
extern u8 DAT_00521f1c[];

void FUN_00403540(u32 a, u32 b);
void FUN_00458e20(u32 a, u32 b);
int FUN_004ae640(u32 a, u32 b);
int FUN_004ae6e0(u32 a, u32 b);
int FUN_004ae5a0(u32 a, u32 b);
void FUN_004037b0(u8 *a, int b, int c, int d);
u32 FUN_0041f410(int a, int b, int c);
void FUN_00427bd0(short a, short b, u8 *c);
void FUN_00427d30(short a, int b, u8 *c);

u64 __fastcall FUN_00482640(u32 param_1, u32 param_2, u16 param_3, short param_4, int param_5)
{
    u8 *puVar1;
    u8 *puVar2;
    u8 *puVar3;
    u8 *puVar4;
    int iVar6, iVar7, iVar8, iVar9, iVar11;
    u32 uVar10, uVar13, uVar15, uVar16;
    u64 uVar2;
    long long lVar3, lVar4, lVar5;
    short sVar14, sStack_a, sStack_8;
    signed char sStack_6;
    u16 uVar12;
    u16 uVar17;
    u8 bVar18;
    short sVar19, sVar20, sVar21;
    int iVar22;
    int iVar23, iVar24, iVar25;
    short sVar26, sVar27, sVar28;
    int iVar29;
    u32 uVar30;

    DAT_009a73a0 = 0xffff;
    iVar11 = (int)((u32)param_5 * 0x5bdu);
    puVar1 = DAT_009e9670 + iVar11;
    __builtin_memcpy(&uVar10, puVar1, 4);
    if (uVar10 == 0xffffffff) {
        return (((u64)param_2 << 32) | 0xffffffffULL) & 0xffffffffffff0000ULL;
    }
    uVar16 = (u32)param_3;
    FUN_00403540(uVar16, uVar10);
    if (DAT_009a1ff8 < 0xa0000) {
        FUN_00458e20(uVar16, 0);
        __builtin_memcpy(&uVar17, puVar1 + 0x1a, 2);
        if (uVar17 == param_3) {
            puVar2 = DAT_009a1fe8 + uVar16 * 0x23;
            if (puVar2[3] != 0 && (int)uVar16 < DAT_009a1fe0) {
                __builtin_memcpy(&puVar3, puVar2 + 0x1f, 4);
                __builtin_memcpy(&iVar22, puVar3 + 0xa9, 4);
                if (iVar22 != 0) {
                    __builtin_memcpy(&uVar17, puVar1 + 0x29, 2);
                    uVar10 = (u32)uVar17;
                    uVar15 = (u32)puVar2[3];
                    iVar7 = FUN_004ae640(uVar15, uVar10);
                    iVar7 = (int)(0u - (u32)iVar7);
                    iVar8 = FUN_004ae6e0(uVar15, uVar10);
                    iVar9 = FUN_004ae5a0(uVar15, uVar10);
                    FUN_004037b0(puVar2, iVar9, iVar8, iVar7);
                    DAT_00a07cdc = (int)((u32)DAT_00a07cdc + (u32)DAT_009d8a58);
                    DAT_00a07cd8 = (int)((u32)DAT_00a07cd8 + (u32)DAT_009d8a54);
                    DAT_00a07cd4 = (int)((u32)DAT_00a07cd4 + (u32)DAT_009d8b60);
                    __builtin_memcpy(&iVar22, puVar1, 4);
                    puVar2 = DAT_009a1fe8 + (u32)iVar22 * 0x23;
                    __builtin_memcpy(&iVar23, puVar2 + 4, 4);
                    __builtin_memcpy(&iVar24, puVar2 + 8, 4);
                    __builtin_memcpy(&iVar25, puVar2 + 0xc, 4);
                    DAT_009a1ff8 = FUN_0041f410((int)((u32)DAT_00a07cdc - (u32)iVar23),
                                                (int)((u32)DAT_00a07cd8 - (u32)iVar24),
                                                (int)((u32)DAT_00a07cd4 - (u32)iVar25));
                }
            }
        }
        __builtin_memcpy(&iVar22, puVar1, 4);
        puVar2 = DAT_009a1fe8 + (u32)iVar22 * 0x23;
        __builtin_memcpy(&iVar23, puVar2 + 4, 4);
        __builtin_memcpy(&iVar24, puVar2 + 8, 4);
        __builtin_memcpy(&iVar25, puVar2 + 0xc, 4);
        sVar14 = (short)((int)((u32)DAT_00a07cdc - (u32)iVar23) >> 4);
        sStack_6 = 4;
        sStack_a = (short)((int)((u32)DAT_00a07cd8 - (u32)iVar24) >> 4);
        sStack_8 = (short)((int)((u32)DAT_00a07cd4 - (u32)iVar25) >> 4);
    } else {
        FUN_00458e20(uVar16, 0);
        __builtin_memcpy(&iVar22, puVar1, 4);
        puVar2 = DAT_009a1fe8 + (u32)iVar22 * 0x23;
        __builtin_memcpy(&iVar23, puVar2 + 4, 4);
        __builtin_memcpy(&iVar24, puVar2 + 8, 4);
        __builtin_memcpy(&iVar25, puVar2 + 0xc, 4);
        sVar14 = (short)(((u32)DAT_00a07cdc - (u32)iVar23) >> 8);
        sStack_6 = 8;
        sStack_a = (short)(((u32)DAT_00a07cd8 - (u32)iVar24) >> 8);
        sStack_8 = (short)(((u32)DAT_00a07cd4 - (u32)iVar25) >> 8);
    }
    __builtin_memcpy(&iVar22, puVar1, 4);
    puVar2 = DAT_009a1fe8 + (u32)iVar22 * 0x23;
    __builtin_memcpy(&puVar3, puVar2 + 0x1f, 4);
    if (puVar3[0x92] != 0) {
        __builtin_memcpy(&sVar26, puVar2 + 0x12, 2);
        __builtin_memcpy(&sVar27, puVar2 + 0x10, 2);
        FUN_00427bd0(sVar26, sVar27, puVar2);
        __builtin_memcpy(&iVar22, puVar1, 4);
        puVar2 = DAT_009a1fe8 + (u32)iVar22 * 0x23;
        __builtin_memcpy(&sVar28, puVar2 + 0x14, 2);
        FUN_00427d30(sVar28, 0, puVar2);
    }
    iVar6 = (int)sVar14;
    __builtin_memcpy(&iVar22, puVar1, 4);
    puVar2 = DAT_009a1fe8 + (u32)iVar22 * 0x23;
    __builtin_memcpy(&puVar3, puVar2 + 0x1f, 4);

    __builtin_memcpy(&sVar19, puVar3 + 0x93, 2);
    lVar3 = (long long)iVar6 * (long long)(int)sVar19;
    iVar7 = (int)sStack_a;
    __builtin_memcpy(&sVar20, puVar3 + 0x95, 2);
    lVar4 = (long long)iVar7 * (long long)(int)sVar20;
    iVar8 = (int)sStack_8;
    __builtin_memcpy(&sVar21, puVar3 + 0x97, 2);
    lVar5 = (long long)iVar8 * (long long)(int)sVar21;
    uVar10 = (u32)((u64)lVar5 >> 15);
    uVar15 = (u32)((u64)lVar4 >> 15) + (u32)((u64)lVar3 >> 15) + uVar10;
    uVar12 = (u16)(uVar10 >> 16);
    iVar11 = (int)(u32)puVar3;
    if ((int)uVar15 < 1) {
        return ((u64)(u32)iVar11 << 32) | ((u64)uVar12 << 16);
    }
    if (0x20000 < (int)uVar15) {
        return ((u64)(u32)iVar11 << 32) | ((u64)uVar12 << 16);
    }
    if (param_4 != 0 && (int)uVar15 < 0x2000) {
        sStack_6 = (signed char)(sStack_6 + 1);
    }
    __builtin_memcpy(&sVar19, puVar3 + 0x99, 2);
    lVar3 = (long long)iVar6 * (long long)(int)sVar19;
    __builtin_memcpy(&sVar20, puVar3 + 0x9b, 2);
    lVar4 = (long long)iVar7 * (long long)(int)sVar20;
    __builtin_memcpy(&sVar21, puVar3 + 0x9d, 2);
    lVar5 = (long long)iVar8 * (long long)(int)sVar21;
    uVar10 = (u32)((u64)lVar3 >> 15) + (u32)((u64)lVar4 >> 15) + (u32)((u64)lVar5 >> 15);
    if ((int)uVar10 < 0) {
        uVar10 = 0u - uVar10;
    }
    uVar13 = (uVar10 >> 0x18) + (u32)(0xffffff7fu < uVar10 * 0x100u);
    if (uVar13 < uVar15) {
        uVar2 = ((u64)uVar13 << 32) | (u32)(uVar10 * 0x100u + 0x80u);
        uVar30 = (u32)(uVar2 >> 32);
        if (uVar15 == 0 || uVar30 >= uVar15) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        iVar11 = (int)(u32)(uVar2 / uVar15);
        uVar13 = (u32)(uVar2 % (u64)uVar15);
    } else {
        iVar11 = 0x7fffff00;
    }
    if (0xa0 < iVar11) {
        return (((u64)uVar13 << 32) | (u32)iVar11) & 0xffffffffffff0000ULL;
    }
    __builtin_memcpy(&sVar19, puVar3 + 0x9f, 2);
    lVar3 = (long long)iVar6 * (long long)(int)sVar19;
    __builtin_memcpy(&sVar20, puVar3 + 0xa1, 2);
    lVar4 = (long long)iVar7 * (long long)(int)sVar20;
    __builtin_memcpy(&sVar21, puVar3 + 0xa3, 2);
    lVar5 = (long long)iVar8 * (long long)(int)sVar21;
    uVar10 = (u32)((u64)lVar4 >> 15) + (u32)((u64)lVar3 >> 15) + (u32)((u64)lVar5 >> 15);
    if ((int)uVar10 < 0) {
        uVar10 = 0u - uVar10;
    }
    uVar13 = (uVar10 >> 0x18) + (u32)(0xffffff7fu < uVar10 * 0x100u);
    if (uVar13 < uVar15) {
        uVar2 = ((u64)uVar13 << 32) | (u32)(uVar10 * 0x100u + 0x80u);
        uVar30 = (u32)(uVar2 >> 32);
        if (uVar15 == 0 || uVar30 >= uVar15) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        iVar6 = (int)(u32)(uVar2 / uVar15);
        uVar13 = (u32)(uVar2 % (u64)uVar15);
    } else {
        iVar6 = 0x7fffff00;
    }
    if (100 < iVar6) {
        return (((u64)uVar13 << 32) | (u32)iVar6) & 0xffffffffffff0000ULL;
    }
    puVar2 = DAT_009a1fe8 + uVar16 * 0x23;
    __builtin_memcpy(&puVar3, puVar2 + 0x1f, 4);
    if (puVar3 == 0) {
        goto use_default;
    }
    __builtin_memcpy(&puVar4, puVar3 + 0xa9, 4);
    if (puVar4 == 0) {
use_default:
        __builtin_memcpy(&iVar7, DAT_00521f1c + (u32)puVar2[3] * 0x1a, 4);
    } else {
        iVar7 = (int)((u32)puVar4[4] * 0x101u);
        __builtin_memcpy(&sVar26, DAT_0051c659 + iVar7 + 4, 2);
        __builtin_memcpy(&sVar27, DAT_0051c659 + iVar7 + 6, 2);
        __builtin_memcpy(&sVar28, DAT_0051c659 + iVar7 + 2, 2);
        iVar29 = (int)(short)(sVar26 + sVar27 + sVar28) / 3;
        bVar18 = DAT_0051c659[iVar7];
        iVar7 = (int)((u32)iVar29 << (bVar18 & 0x1f));
    }
    uVar10 = (u32)(iVar7 >> sStack_6) * 0x100u;
    uVar16 = ((u32)(iVar7 >> sStack_6) >> 0x18) + (u32)(0xffffff7fu < uVar10);
    if (uVar16 < uVar15) {
        uVar2 = ((u64)uVar16 << 32) | (u32)(uVar10 + 0x80u);
        uVar30 = (u32)(uVar2 >> 32);
        if (uVar15 == 0 || uVar30 >= uVar15) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        iVar7 = (int)(u32)(uVar2 / uVar15);
        uVar16 = (u32)(uVar2 % (u64)uVar15);
    } else {
        iVar7 = 0x7fffff00;
    }
    if (iVar7 < 1) {
        iVar7 = 1;
    }
    if (param_4 == 0) {
        iVar7 = (int)((u32)iVar7 * 3u);
        if (iVar7 < 10) {
            iVar7 = 9;
        }
    }
    DAT_009a73a0 = (u16)((short)iVar11 + (short)((u32)((u32)iVar6 * 0xe8bau) >> 0x10));
    uVar12 = (u16)((u32)iVar7 >> 0x10);
    if (iVar11 < iVar7 && ((int)((u32)iVar6 * 0xe8bau) >> 0x10) < iVar7) {
        return ((u64)uVar16 << 32) | ((u32)uVar12 << 16) | 1u;
    }
    return ((u64)uVar16 << 32) | ((u64)uVar12 << 16);
}
