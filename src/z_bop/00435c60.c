/* SPDX-License-Identifier: MIT */
/* z_bop 00435c60 */
extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a69[];
extern unsigned char DAT_009e9a6b[];
extern unsigned char DAT_009e9a6d[];
extern unsigned char DAT_009e9a6f[];
extern unsigned char DAT_009e9a73[];
extern unsigned char DAT_009e9a75[];
extern unsigned char DAT_009e9a86[];
extern int DAT_009a1fe8;
extern short DAT_009a8070;
extern short DAT_00a080f0;
extern unsigned int DAT_00a004b0;
extern unsigned int DAT_009fe7f4;
extern unsigned int DAT_00a00490;
extern unsigned int DAT_00a07cd4;
extern unsigned int DAT_00a07cd8;
extern unsigned int DAT_00a07cdc;
extern int DAT_00523410;

void __cdecl FUN_00427940(short a, short b, short c, int d, short e, short f, int g);
void __cdecl FUN_0046a8d0(int a, int b, int c);
void __cdecl FUN_00458e20(unsigned int a, int b);

void __cdecl FUN_00435c60(int param_1)
{
    int iVar3;
    unsigned short uVar1;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar10;
    int iVar11;
    short sVar9, sVar12, sVar13, sVar14, sVar15;
    long long lVar2;
    short s6f, s6b, s6d, s73, s75, ss1;
    short zero16;
    short c4000;
    unsigned short us;
    unsigned int u5b, u5f, u63;
    int i86;
    int t4, t8, tc;
    unsigned char *tp;

    zero16 = 0;
    c4000 = 0x4000;
    iVar3 = (int)((unsigned int)param_1 * 0x5bdU);
    __builtin_memcpy(&uVar1, &DAT_009e9a67[iVar3], 2);

    if (DAT_009e9683[iVar3] < 2) {
        if (uVar1 == 0xffff) {
            __builtin_memcpy(&s75, &DAT_009e9a75[iVar3], 2);
            __builtin_memcpy(&s73, &DAT_009e9a73[iVar3], 2);
            if (s75 != 0 || s73 != 0) {
                __builtin_memcpy(&s6f, &DAT_009e9a6f[iVar3], 2);
                __builtin_memcpy(&s6b, &DAT_009e9a6b[iVar3], 2);
                __builtin_memcpy(&s6d, &DAT_009e9a6d[iVar3], 2);
                __builtin_memcpy(&s73, &DAT_009e9a73[iVar3], 2);
                __builtin_memcpy(&s75, &DAT_009e9a75[iVar3], 2);
                FUN_00427940(s6f, s6b, s6d, 0, s73, s75, 0);
                FUN_0046a8d0((int)DAT_00a004b0, (int)DAT_009fe7f4, (int)DAT_00a00490);
                sVar9 = DAT_00a080f0;
                __builtin_memcpy(&DAT_009e9a75[iVar3], &zero16, 2);
                ss1 = DAT_009a8070;
                __builtin_memcpy(&DAT_009e9a73[iVar3], &zero16, 2);
                __builtin_memcpy(&DAT_009e9a6b[iVar3], &sVar9, 2);
                __builtin_memcpy(&DAT_009e9a6d[iVar3], &ss1, 2);
            }
            __builtin_memcpy(&sVar9, &DAT_009e9a75[iVar3], 2);
            __builtin_memcpy(&sVar15, &DAT_009e9a73[iVar3], 2);
            __builtin_memcpy(&sVar14, &DAT_009e9a6d[iVar3], 2);
            __builtin_memcpy(&sVar13, &DAT_009e9a6b[iVar3], 2);
            __builtin_memcpy(&sVar12, &DAT_009e9a6f[iVar3], 2);
        } else {
            tp = (unsigned char *)((unsigned int)uVar1 * 0x23U + 0x14U + (unsigned int)DAT_009a1fe8);
            __builtin_memcpy(&ss1, tp, 2);
            __builtin_memcpy(&DAT_009e9a6f[iVar3], &ss1, 2);
            __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
            tp = (unsigned char *)((unsigned int)us * 0x23U + 0x12U + (unsigned int)DAT_009a1fe8);
            __builtin_memcpy(&ss1, tp, 2);
            __builtin_memcpy(&DAT_009e9a6b[iVar3], &ss1, 2);
            __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
            tp = (unsigned char *)((unsigned int)us * 0x23U + 0x10U + (unsigned int)DAT_009a1fe8);
            __builtin_memcpy(&ss1, tp, 2);
            __builtin_memcpy(&DAT_009e9a6d[iVar3], &ss1, 2);
            __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
            FUN_00458e20((unsigned int)us, 0);
            u5b = DAT_00a07cdc;
            u5f = DAT_00a07cd8;
            u63 = DAT_00a07cd4;
            __builtin_memcpy(&DAT_009e9a5b[iVar3], &u5b, 4);
            __builtin_memcpy(&DAT_009e9a5f[iVar3], &u5f, 4);
            __builtin_memcpy(&DAT_009e9a63[iVar3], &u63, 4);
            __builtin_memcpy(&sVar9, &DAT_009e9a75[iVar3], 2);
            __builtin_memcpy(&sVar15, &DAT_009e9a73[iVar3], 2);
            __builtin_memcpy(&sVar14, &DAT_009e9a6d[iVar3], 2);
            __builtin_memcpy(&sVar13, &DAT_009e9a6b[iVar3], 2);
            __builtin_memcpy(&sVar12, &DAT_009e9a6f[iVar3], 2);
        }
        FUN_00427940(sVar12, sVar13, sVar14, 0, sVar15, sVar9, 0);
        uVar4 = DAT_009fe7f4;
        __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
        if ((short)us != -1) {
            __builtin_memcpy(&i86, &DAT_009e9a86[iVar3], 4);
            lVar2 = (long long)i86 * (long long)(int)DAT_00a004b0;
            __builtin_memcpy(&u5b, &DAT_009e9a5b[iVar3], 4);
            u5b = u5b - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
            __builtin_memcpy(&DAT_009e9a5b[iVar3], &u5b, 4);
            uVar5 = DAT_00a00490;
            __builtin_memcpy(&i86, &DAT_009e9a86[iVar3], 4);
            lVar2 = (long long)i86 * (long long)(int)uVar4;
            __builtin_memcpy(&u5f, &DAT_009e9a5f[iVar3], 4);
            u5f = u5f - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
            __builtin_memcpy(&DAT_009e9a5f[iVar3], &u5f, 4);
            __builtin_memcpy(&i86, &DAT_009e9a86[iVar3], 4);
            lVar2 = (long long)i86 * (long long)(int)uVar5;
            __builtin_memcpy(&u63, &DAT_009e9a63[iVar3], 4);
            u63 = u63 - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
            __builtin_memcpy(&DAT_009e9a63[iVar3], &u63, 4);
        }
        __builtin_memcpy(&us, &DAT_009e9a69[iVar3], 2);
        if (us == 0xffff) {
            return;
        }
        iVar11 = (int)((unsigned int)us * 0x23U + (unsigned int)DAT_009a1fe8);
        __builtin_memcpy(&t4, (unsigned char *)(iVar11 + 4), 4);
        __builtin_memcpy(&u5b, &DAT_009e9a5b[iVar3], 4);
        __builtin_memcpy(&t8, (unsigned char *)(iVar11 + 8), 4);
        __builtin_memcpy(&u5f, &DAT_009e9a5f[iVar3], 4);
        __builtin_memcpy(&tc, (unsigned char *)(iVar11 + 0xc), 4);
        __builtin_memcpy(&u63, &DAT_009e9a63[iVar3], 4);
        FUN_0046a8d0((int)((unsigned int)t4 - u5b),
                     (int)((unsigned int)t8 - u5f),
                     (int)((unsigned int)tc - u63));
        sVar9 = DAT_00a080f0;
        ss1 = DAT_009a8070;
        __builtin_memcpy(&DAT_009e9a6f[iVar3], &zero16, 2);
        __builtin_memcpy(&DAT_009e9a6b[iVar3], &sVar9, 2);
        __builtin_memcpy(&DAT_009e9a6d[iVar3], &ss1, 2);
        __builtin_memcpy(&s6f, &DAT_009e9a6f[iVar3], 2);
        __builtin_memcpy(&s6b, &DAT_009e9a6b[iVar3], 2);
        FUN_00427940(s6f, s6b, ss1, 0, 0, 0, 0);
        return;
    }

    if (uVar1 == 0xffff) {
        __builtin_memcpy(&DAT_009e9a6f[iVar3], &zero16, 2);
        __builtin_memcpy(&DAT_009e9a6b[iVar3], &c4000, 2);
        __builtin_memcpy(&DAT_009e9a6d[iVar3], &zero16, 2);
        __builtin_memcpy(&DAT_009e9a75[iVar3], &zero16, 2);
        sVar9 = (short)((int)((((unsigned int)DAT_009e9683[iVar3] & 0x7fU) << 0xe)) / -0x7f);
    } else {
        tp = (unsigned char *)((unsigned int)uVar1 * 0x23U + 0x14U + (unsigned int)DAT_009a1fe8);
        __builtin_memcpy(&ss1, tp, 2);
        __builtin_memcpy(&DAT_009e9a6f[iVar3], &ss1, 2);
        __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
        tp = (unsigned char *)((unsigned int)us * 0x23U + 0x12U + (unsigned int)DAT_009a1fe8);
        __builtin_memcpy(&ss1, tp, 2);
        __builtin_memcpy(&DAT_009e9a6b[iVar3], &ss1, 2);
        __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
        tp = (unsigned char *)((unsigned int)us * 0x23U + 0x10U + (unsigned int)DAT_009a1fe8);
        __builtin_memcpy(&ss1, tp, 2);
        __builtin_memcpy(&DAT_009e9a6d[iVar3], &ss1, 2);
        __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
        FUN_00458e20((unsigned int)us, 0);
        u5b = DAT_00a07cdc;
        u5f = DAT_00a07cd8;
        u63 = DAT_00a07cd4;
        iVar11 = DAT_00523410;
        __builtin_memcpy(&DAT_009e9a5b[iVar3], &u5b, 4);
        __builtin_memcpy(&DAT_009e9a5f[iVar3], &u5f, 4);
        __builtin_memcpy(&DAT_009e9a63[iVar3], &u63, 4);
        if (iVar11 != 0) goto LAB_00435ee2;

        __builtin_memcpy(&sVar9, &DAT_009e9a75[iVar3], 2);
        iVar11 = -(int)(sVar9 / 2);
        if (iVar11 < 1) {
            if (iVar11 < -0x1000) {
                iVar11 = -0x1000;
            }
            if (-0x10 < iVar11) {
                iVar11 = -0x10;
            }
            iVar10 = -(int)sVar9;
            if (iVar11 < iVar10) goto LAB_00435daa;
        } else {
            if (sVar9 / 2 != -0x1000 && 0xfff < iVar11) {
                iVar11 = 0x1000;
            }
            if (iVar11 < 0x10) {
                iVar11 = 0x10;
            }
            iVar10 = -(int)sVar9;
            if (iVar10 < iVar11) {
LAB_00435daa:
                iVar11 = iVar10;
            }
        }
        ss1 = (short)(sVar9 + (short)iVar11);
        __builtin_memcpy(&DAT_009e9a75[iVar3], &ss1, 2);
        if ((DAT_009e9683[iVar3] & 0x80) == 0) {
            __builtin_memcpy(&sVar9, &DAT_009e9a73[iVar3], 2);
            iVar11 = -(int)(sVar9 / 2);
            if (iVar11 < 1) {
                if (iVar11 < -0x1000) {
                    iVar11 = -0x1000;
                }
                if (-0x10 < iVar11) {
                    iVar11 = -0x10;
                }
                iVar10 = -(int)sVar9;
                if (iVar11 < iVar10) goto LAB_00435e8d;
            } else {
                if (sVar9 / 2 != -0x1000 && 0xfff < iVar11) {
                    iVar11 = 0x1000;
                }
                if (iVar11 < 0x10) {
                    iVar11 = 0x10;
                }
                iVar10 = -(int)sVar9;
                if (iVar10 < iVar11) {
LAB_00435e8d:
                    iVar11 = iVar10;
                }
            }
            ss1 = (short)(sVar9 + (short)iVar11);
            __builtin_memcpy(&DAT_009e9a73[iVar3], &ss1, 2);
            goto LAB_00435ee2;
        }
        __builtin_memcpy(&DAT_009e9a6f[iVar3], &zero16, 2);
        __builtin_memcpy(&DAT_009e9a6b[iVar3], &c4000, 2);
        __builtin_memcpy(&DAT_009e9a6d[iVar3], &zero16, 2);
        __builtin_memcpy(&s73, &DAT_009e9a73[iVar3], 2);
        iVar10 = -0x4000 - (int)s73;
        iVar11 = iVar10 / 2;
        if (iVar11 < 1) {
            if (iVar11 < -0x1000) {
                iVar11 = -0x1000;
            }
            if (-0x10 < iVar11) {
                iVar11 = -0x10;
            }
            if (iVar11 < iVar10) goto LAB_00435e29;
        } else {
            if (0x1000 < iVar11) {
                iVar11 = 0x1000;
            }
            if (iVar11 < 0x10) {
                iVar11 = 0x10;
            }
            if (iVar10 < iVar11) {
LAB_00435e29:
                iVar11 = iVar10;
            }
        }
        __builtin_memcpy(&s73, &DAT_009e9a73[iVar3], 2);
        sVar9 = (short)((short)iVar11 + s73);
    }
    __builtin_memcpy(&DAT_009e9a73[iVar3], &sVar9, 2);

LAB_00435ee2:
    __builtin_memcpy(&s6f, &DAT_009e9a6f[iVar3], 2);
    __builtin_memcpy(&s6b, &DAT_009e9a6b[iVar3], 2);
    __builtin_memcpy(&s6d, &DAT_009e9a6d[iVar3], 2);
    __builtin_memcpy(&s73, &DAT_009e9a73[iVar3], 2);
    __builtin_memcpy(&s75, &DAT_009e9a75[iVar3], 2);
    FUN_00427940(s6f, s6b, s6d, 0, s73, s75, 0);
    uVar4 = DAT_009fe7f4;
    __builtin_memcpy(&us, &DAT_009e9a67[iVar3], 2);
    if ((short)us == -1) {
        return;
    }
    __builtin_memcpy(&i86, &DAT_009e9a86[iVar3], 4);
    lVar2 = (long long)i86 * (long long)(int)DAT_00a004b0;
    __builtin_memcpy(&u5b, &DAT_009e9a5b[iVar3], 4);
    u5b = u5b - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
    __builtin_memcpy(&DAT_009e9a5b[iVar3], &u5b, 4);
    uVar5 = DAT_00a00490;
    __builtin_memcpy(&i86, &DAT_009e9a86[iVar3], 4);
    lVar2 = (long long)i86 * (long long)(int)uVar4;
    __builtin_memcpy(&u5f, &DAT_009e9a5f[iVar3], 4);
    u5f = u5f - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
    __builtin_memcpy(&DAT_009e9a5f[iVar3], &u5f, 4);
    __builtin_memcpy(&i86, &DAT_009e9a86[iVar3], 4);
    lVar2 = (long long)i86 * (long long)(int)uVar5;
    __builtin_memcpy(&u63, &DAT_009e9a63[iVar3], 4);
    u63 = u63 - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
    __builtin_memcpy(&DAT_009e9a63[iVar3], &u63, 4);
    return;
}
