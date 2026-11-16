/* SPDX-License-Identifier: MIT */
/* z_xvt 00434990 */
extern int            DAT_006d4988;
extern uint           DAT_006db7ac;
extern uint           DAT_006db7b0;
extern uint           DAT_006db7b4;
extern short          DAT_006d9980;
extern short          DAT_00732e8c;
extern unsigned char  DAT_00732e50;
extern int            DAT_0071ecf4;
extern int            DAT_0071eae8;
extern int            DAT_00732a2c;
extern int            DAT_00732a28;
extern int            DAT_00732a24;
extern int            DAT_0072f44c;
extern int            DAT_0072f450;
extern uint           DAT_00732f3c;
extern uint           DAT_006da538;
extern uint           DAT_006db580;
extern unsigned char  DAT_006d4870[];
extern unsigned char  DAT_006d4a4e;
extern unsigned char  DAT_005254c4[];
extern unsigned char  DAT_0071b74a[];
extern unsigned char  DAT_0071bb1b[];
extern unsigned char  DAT_0071bb1f[];
extern unsigned char  DAT_0071bb23[];
extern unsigned char  DAT_0071bb27[];
extern unsigned char  DAT_0070ad47[];
extern void (__cdecl *DAT_006fedd4)(int, int, int, int, uint);
extern void (__cdecl *DAT_006d9984)(int);

extern int   __cdecl FUN_0040ca90(uint, uint);
extern int   __cdecl FUN_0040cad0(uint, uint);
extern int   __cdecl FUN_0042b150(int);
extern void  __cdecl FUN_00421c70(uint, uint);
extern uint  __cdecl FUN_0040ca00(int, int, int);
extern uint  __cdecl FUN_0040ca30(int, int, int);
extern uint  __cdecl FUN_0040ca60(int, int, int);
extern void  __cdecl FUN_0040c970(uint, uint, uint);
extern void  __cdecl FUN_0048eda0(uint);
extern void  __cdecl FUN_0048ee20(char);
extern void  __cdecl FUN_00483410(short, short, int);
extern void  __cdecl FUN_00483540(short, int, int);
/* Ghidra's leading ECX/EDX "params" are register leftovers; callee pops 0 */
extern unsigned long long __cdecl FUN_00413ca0(int, int);
extern uint  __cdecl FUN_004428e0(uint);
extern void  __cdecl FUN_004ac210(ushort, int);
/* Ghidra's leading ECX/EDX "params" are register leftovers; callee pops 0 */
extern unsigned long long __cdecl FUN_004b4060(unsigned char *);
extern void  __cdecl FUN_0048ece0(short, short);
/* Ghidra showed an unresolved `this`; callee pops 0 */
extern void  __cdecl FUN_0048f140(unsigned char *);
extern void  __cdecl FUN_0042a140(uint, uint);
extern void  __cdecl FUN_0040ccc0(ushort, int, int);

void __cdecl FUN_00434990(uint param_1)
{
    char cVar1;
    unsigned char uVar2;
    ushort uVar3;
    char *pcVar4;
    int iVar5;
    int bVar6;
    uint uVar7;
    int iVar8;
    int iVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    uint uVar14;
    uint uVar15;
    uint uVar16;
    short sVar17;
    unsigned long long uVar18;
    short local_10;
    short local_c;
    int tbl;
    int ent;
    int tmp;
    ushort us;
    short ss;
    short ss2;

    uVar15 = DAT_006db7b4;
    uVar7 = DAT_006db7b0;
    uVar14 = DAT_006db7ac;
    iVar5 = (int)(param_1 * 0x23);
    cVar1 = *(char *)(DAT_006d4988 + 2 + iVar5);
    if (((cVar1 != '\r') && (cVar1 != '\v')) && (0 < (int)DAT_006db7b4)) {
        iVar8 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
        iVar9 = FUN_0040cad0(uVar7, uVar15);
        iVar8 = (int)((uint)iVar8 + (uint)(int)DAT_006d9980);
        iVar9 = (int)((uint)iVar9 + (uint)(int)DAT_00732e8c);

        tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
        __builtin_memcpy(&us, DAT_0071b74a + tbl, 2);
        if (us == param_1) {
            uVar16 = 0xffff;
            if ((int)param_1 < DAT_0071eae8) {
                __builtin_memcpy(&ent, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
                __builtin_memcpy(&tmp, (char *)(ent + 0xa7), 4);
                iVar10 = FUN_0042b150(tmp);
                uVar16 = (uint)*(ushort *)(iVar10 + 0x18);
            } else {
                __builtin_memcpy(&iVar10, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
                if (iVar10 != 0) {
                    __builtin_memcpy(&iVar10, (char *)(iVar10 + 0xa7), 4);
                    if (iVar10 != 0) {
                        uVar16 = (uint)*(ushort *)(iVar10 + 4);
                    }
                }
            }
            if (uVar16 != 0xffff) {
                FUN_00421c70(uVar16, (uint)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar5));
                tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
                __builtin_memcpy(&tmp, DAT_0071bb1b + tbl, 4);
                DAT_00732a2c = (int)((uint)DAT_00732a2c - (uint)tmp);
                __builtin_memcpy(&tmp, DAT_0071bb1f + tbl, 4);
                DAT_00732a28 = (int)((uint)DAT_00732a28 - (uint)tmp);
                __builtin_memcpy(&tmp, DAT_0071bb23 + tbl, 4);
                DAT_00732a24 = (int)((uint)DAT_00732a24 - (uint)tmp);
                DAT_006db7ac = FUN_0040ca00(DAT_00732a2c, DAT_00732a28, DAT_00732a24);
                DAT_006db7b0 = FUN_0040ca30(DAT_00732a2c, DAT_00732a28, DAT_00732a24);
                DAT_006db7b4 = FUN_0040ca60(DAT_00732a2c, DAT_00732a28, DAT_00732a24);
                if ((int)DAT_006db7b4 < 1) {
                    FUN_0040c970(uVar14, uVar7, uVar15);
                }
                iVar10 = iVar8;
                iVar13 = iVar9;
                iVar11 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
                iVar11 = (int)((uint)iVar11 + (uint)(int)DAT_00732e8c);
                iVar12 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
                (*DAT_006fedd4)((int)((uint)iVar12 + (uint)(int)DAT_006d9980), iVar11, iVar10, iVar13, 0x36);
            }
        }

        __builtin_memcpy(&iVar10, (char *)(DAT_006d4988 + iVar5 + 0x1f), 4);
        if (iVar10 == 0) {
            uVar2 = DAT_0070ad47[(uint)*(unsigned char *)(DAT_006d4988 + iVar5 + 0x16) * 0x566];
        } else {
            uVar2 = *(unsigned char *)(iVar10 + 0x86);
        }
        switch (uVar2) {
        case 0:
            uVar16 = 0x3f;
            break;
        case 1:
        case 4:
            uVar16 = 0x37;
            break;
        case 2:
            uVar16 = 0x33;
            break;
        default:
            uVar16 = 0x3b;
        }
        FUN_0048eda0(uVar16);
        FUN_0048ee20('\0');

        bVar6 = 0;
        if ((int)param_1 < DAT_0071eae8) {
            bVar6 = 1;
        } else {
            __builtin_memcpy(&iVar10, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            if (iVar10 == 0) {
                bVar6 = 1;
            } else {
                __builtin_memcpy(&tmp, (char *)(iVar10 + 0xa7), 4);
                if (tmp != 0) {
                    bVar6 = 1;
                }
            }
        }
        if (bVar6) {
            tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
            __builtin_memcpy(&tmp, DAT_0071bb23 + tbl, 4);
            DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, (int)(0xffff0000u - (uint)tmp));
            tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
            __builtin_memcpy(&tmp, DAT_0071bb23 + tbl, 4);
            DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, (int)(0xffff0000u - (uint)tmp));
            tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
            __builtin_memcpy(&tmp, DAT_0071bb23 + tbl, 4);
            DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, (int)(0xffff0000u - (uint)tmp));
            if ((int)DAT_006db7b4 < 1) {
                FUN_0040c970(uVar14, uVar7, uVar15);
            }
            iVar10 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
            iVar13 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
            iVar10 = (int)((uint)iVar10 + (uint)(int)DAT_006d9980);
            iVar13 = (int)((uint)iVar13 + (uint)(int)DAT_00732e8c);
            /* UNRESOLVED: upper bytes are a register leftover (CONCAT31 in Ghidra) */
            (*DAT_006fedd4)(iVar10, iVar13, iVar8, iVar9,
                            ((uint)(int)(signed char)((ushort)DAT_006d9980 >> 8) << 8) | (uint)DAT_00732e50);

            iVar11 = DAT_006d4988 + iVar5;
            __builtin_memcpy(&pcVar4, (char *)(iVar11 + 0x1f), 4);
            if ((pcVar4 != (char *)0) && (*pcVar4 == '\0')) {
                if (pcVar4[0x90] != '\0') {
                    __builtin_memcpy(&ss, (char *)(iVar11 + 0x12), 2);
                    __builtin_memcpy(&ss2, (char *)(iVar11 + 0x10), 2);
                    FUN_00483410(ss, ss2, iVar11);
                    __builtin_memcpy(&ss, (char *)(DAT_006d4988 + iVar5 + 0x14), 2);
                    FUN_00483540(ss, 0, DAT_006d4988 + iVar5);
                }
                __builtin_memcpy(&iVar11, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
                __builtin_memcpy(&ss, (char *)(iVar11 + 0x8a), 2);
                iVar12 = (int)ss;
                uVar18 = FUN_00413ca0(0x100, iVar12);
                DAT_0072f44c = (int)((uint)DAT_0072f44c + (uint)(int)uVar18);
                __builtin_memcpy(&tmp, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
                __builtin_memcpy(&ss, (char *)(tmp + 0x8c), 2);
                iVar11 = (int)ss;
                uVar18 = FUN_00413ca0(0x100, iVar11);
                DAT_0072f450 = (int)((uint)DAT_0072f450 + (uint)(int)uVar18);
                __builtin_memcpy(&iVar11, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
                __builtin_memcpy(&us, (char *)(iVar11 + 0x79), 2);
                if (us < 0x400) {
                    __builtin_memcpy(&ss, (char *)(iVar11 + 0x8a), 2);
                    uVar18 = FUN_00413ca0((int)((uint)us << 5), (int)ss);
                    DAT_0072f44c = (int)((uint)DAT_0072f44c + (uint)(int)uVar18);
                    __builtin_memcpy(&iVar11, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
                    __builtin_memcpy(&us, (char *)(iVar11 + 0x79), 2);
                    __builtin_memcpy(&ss, (char *)(iVar11 + 0x8c), 2);
                    uVar18 = FUN_00413ca0((int)((uint)us << 5), (int)ss);
                    iVar11 = (int)uVar18;
                } else {
                    __builtin_memcpy(&ss, (char *)(iVar11 + 0x8a), 2);
                    DAT_0072f44c = (int)((uint)DAT_0072f44c + (uint)(int)ss);
                    __builtin_memcpy(&tmp, (char *)(DAT_006d4988 + 0x1f + iVar5), 4);
                    __builtin_memcpy(&ss, (char *)(tmp + 0x8c), 2);
                    iVar11 = (int)ss;
                }
                uVar16 = DAT_006db7b4;
                uVar7 = DAT_006db7b0;
                uVar14 = DAT_006db7ac;
                DAT_0072f450 = (int)((uint)DAT_0072f450 + (uint)iVar11);
                tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
                __builtin_memcpy(&tmp, DAT_0071bb23 + tbl, 4);
                DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, (int)(0xffff0000u - (uint)tmp));
                tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
                __builtin_memcpy(&tmp, DAT_0071bb23 + tbl, 4);
                DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, (int)(0xffff0000u - (uint)tmp));
                tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
                __builtin_memcpy(&tmp, DAT_0071bb23 + tbl, 4);
                DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, (int)(0xffff0000u - (uint)tmp));
                if ((int)DAT_006db7b4 < 1) {
                    FUN_0040c970(uVar14, uVar7, uVar16);
                }
                uVar14 = (uint)DAT_00732e50;
                iVar11 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
                iVar11 = (int)((uint)iVar11 + (uint)(int)DAT_00732e8c);
                iVar12 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
                (*DAT_006fedd4)((int)((uint)iVar12 + (uint)(int)DAT_006d9980), iVar11, iVar10, iVar13, uVar14);
            }

            FUN_0048eda0(0x40);
            local_10 = (short)iVar8;
            local_c = (short)iVar9;
            uVar14 = FUN_004428e0(param_1);
            uVar15 = (DAT_00732f3c * uVar14) / uVar15;
            if (uVar15 < DAT_006da538 / 0x50) {
                uVar15 = DAT_006da538 / 0x50;
            }
            if (DAT_006da538 >> 1 < uVar15) {
                uVar15 = DAT_006da538 >> 1;
            }
            iVar8 = (int)(uVar15 + 4);
            __builtin_memcpy(&tmp, (char *)(DAT_006d4988 + iVar5 + 0x1f), 4);
            if ((tmp == 0) && (*(char *)(DAT_006d4988 + iVar5 + 2) == '\b')) {
                DAT_006d4870[0] = '\0';
            } else {
                FUN_004ac210((ushort)param_1, 2);
            }
            if (DAT_006d4870[0] != '\0') {
                iVar9 = (int)((((uint)(int)local_c - (uint)DAT_006d4a4e) - (uint)(iVar8 / 2)) - 1u);
                uVar18 = FUN_004b4060(DAT_006d4870);
                FUN_0048ece0((short)(local_10 - ((ushort)uVar18 >> 1)), (short)iVar9);
                FUN_0048f140(DAT_006d4870);
            }
            cVar1 = *(char *)(DAT_006d4988 + 2 + iVar5);
            if ((cVar1 != '\x06') && (cVar1 != '\a')) {
                FUN_0048eda0(0x40);
                tbl = (int)((uint)DAT_0071ecf4 * 0x5bd);
                __builtin_memcpy(&uVar3, DAT_0071bb27 + tbl, 2);
                if (uVar3 != 0xffff) {
                    sVar17 = (short)(local_c + (short)(iVar8 >> 1) + 1);
                    FUN_0042a140((uint)uVar3, param_1);
                    uVar18 = FUN_004b4060(DAT_005254c4);
                    FUN_0048ece0((short)(local_10 - (short)uVar18), sVar17);
                    DAT_006db580 = DAT_006db580 * 0xa1;
                    uVar14 = DAT_006db580 >> 0x10;
                    if (9999 < uVar14) {
                        uVar14 = 9999;
                    }
                    FUN_0040ccc0((ushort)(uVar14 / 100), 2, 1);
                    (*DAT_006d9984)(0x2e);
                    FUN_0040ccc0((ushort)(short)((short)(-(int)(uVar14 / 100)) * 100 + (short)uVar14), 2, 2);
                }
            }
        }
    }
    return;
}
