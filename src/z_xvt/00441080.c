/* SPDX-License-Identifier: MIT */
/* z_xvt 00441080 */
/* Unresolved: FUN_0049f010's return is a bool/byte with stale upper EAX bits in Ghidra's
   view; the original tests AX (low 16 bits), so it is prototyped as unsigned short.
   Unresolved: unaff_EBP and unaff_EDI are passed to FUN_004a3830 as the decompiler shows
   them (caller registers); real types and sources are unknown, initialised to 0 here. */

extern unsigned int _DAT_00732a3c;
extern unsigned int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern unsigned int DAT_00732a38;
extern unsigned int DAT_00703c88;
extern unsigned int DAT_006db580;

extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071bb1b[];
extern unsigned char DAT_0071bb1f[];
extern unsigned char DAT_0071bb23[];
extern unsigned char DAT_0070ad48[];
extern char DAT_0070aee5[];
extern char DAT_0070aee9[];
extern unsigned char DAT_006d4b36[];
extern unsigned char DAT_006daa00[];
extern unsigned char DAT_006daa01[];
extern unsigned char DAT_006daa02[];
extern unsigned char DAT_006daa04[];
extern unsigned char DAT_006daa05[];
extern unsigned char DAT_006daa06[];
extern unsigned char DAT_006daa0b[];
extern unsigned char DAT_006daa0f[];
extern unsigned char DAT_006daa10[];
extern unsigned char DAT_006daa11[];

unsigned short __cdecl FUN_0049f010(unsigned int a, unsigned short b, unsigned short c);
void __cdecl FUN_00421c70(unsigned int a, int b);
void __cdecl FUN_00418cd0(int a, int b, int c);
void __cdecl FUN_0042a140(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_004a3830(unsigned int a, int b, int c, int d, unsigned int e, int f);

unsigned int __cdecl FUN_00441080(int param_1, int param_2)
{
    char cVar1;
    int iVar2;
    _Bool bVar3;
    unsigned int uVar5;
    int iVar6;
    unsigned int uVar8;
    int iVar9;
    char *pcVar10;
    unsigned int unaff_EBP = 0;
    unsigned int uVar11;
    unsigned int uVar12;
    int unaff_EDI = 0;
    char *pcVar13;
    short local_20;
    short sStack_1e;
    short sStack_1c;
    unsigned int local_14;
    char *local_c;
    int local_8;
    unsigned int local_4;
    unsigned short us_tmp;
    unsigned int ui_tmp;
    int i_tmp;

    local_4 = 0xffffffff;
    local_14 = 0xffffffff;
    iVar2 = (int)((unsigned int)param_2 * 0x5bdu);
    __builtin_memcpy(&us_tmp, DAT_0071b73c + iVar2, 2);
    uVar5 = (unsigned int)us_tmp;
    sStack_1e = -1;
    local_20 = -1;
    if (_DAT_00732a3c < DAT_006d4980) {
        local_8 = (int)(_DAT_00732a3c * 0x23u);
        uVar8 = _DAT_00732a3c;
        do {
            sStack_1c = (short)uVar8;
            iVar9 = (int)((unsigned int)DAT_006d4988 + (unsigned int)local_8);
            __builtin_memcpy(&ui_tmp, DAT_0071b730 + iVar2, 4);
            if ((*(char *)(iVar9 + 3) != '\0') && (ui_tmp != uVar8) &&
                (*(char *)(iVar9 + 2) != '\r')) {
                if ((sStack_1c == -1) || ((int)DAT_006d4980 <= (int)(uVar8 & 0xffff))) {
                    bVar3 = 0;
                } else {
                    iVar6 = (int)((uVar8 & 0xffff) * 0x23u + (unsigned int)DAT_006d4988);
                    __builtin_memcpy(&i_tmp, (char *)(iVar6 + 0x1b), 4);
                    if (i_tmp == -1) {
                        bVar3 = 0;
                    } else {
                        __builtin_memcpy(&i_tmp, (char *)(iVar6 + 0x1f), 4);
                        __builtin_memcpy(&iVar6, (char *)(i_tmp + 0xa7), 4);
                        if (iVar6 == 0 || (*(unsigned char *)(iVar6 + 0x1e0) & 1) == 0) {
                            bVar3 = 0;
                        } else if (*(char *)(iVar6 + 0x237) == '\0') {
                            bVar3 = 0;
                        } else if (*(char *)(iVar6 + 0x233) != '\x03') {
                            bVar3 = 0;
                        } else {
                            __builtin_memcpy(&us_tmp, (char *)(iVar6 + 0x238), 2);
                            if (us_tmp == 0) {
                                bVar3 = 0;
                            } else {
                                bVar3 = 1;
                            }
                        }
                    }
                }
                if (!bVar3) {
                    iVar6 = (int)((unsigned int)*(unsigned char *)(iVar9 + 0x16) * 0x566u);
                    if ((unsigned int)DAT_0070ad48[iVar6] != uVar5) {
                        uVar11 = 0;
                        pcVar13 = DAT_0070aee5 + iVar6;
                        bVar3 = 0;
                        pcVar10 = DAT_0070aee9 + uVar5 + iVar6;
                        do {
                            if ((*pcVar10 != '\0') && (*pcVar13 == (char)param_1) &&
                                (-1 < pcVar13[3]) &&
                                (DAT_006d4b36[uVar11 + uVar5 * 8 +
                                              (unsigned int)*(unsigned char *)(iVar9 + 0x16) * 0x126] ==
                                 4)) {
                                bVar3 = 1;
                            }
                            pcVar13 = pcVar13 + 0x4e;
                            pcVar10 = pcVar10 + 0x4e;
                            uVar11 = uVar11 + 1;
                        } while (uVar11 < 8);
                        iVar9 = (int)((uVar5 * 7u + (unsigned int)param_1) * 0x2au);
                        if ((DAT_006daa00[iVar9] != '\n') && (DAT_006daa00[iVar9] != '\0')) {
                            if (FUN_0049f010(uVar8, (unsigned short)DAT_006daa01[iVar9],
                                             (unsigned short)DAT_006daa02[iVar9]) != 0) {
                                bVar3 = 1;
                            }
                        }
                        if ((DAT_006daa04[iVar9] != '\n') && (DAT_006daa04[iVar9] != '\0')) {
                            if (FUN_0049f010(uVar8, (unsigned short)DAT_006daa05[iVar9],
                                             (unsigned short)DAT_006daa06[iVar9]) != 0) {
                                bVar3 = 1;
                            }
                        }
                        if ((DAT_006daa0b[iVar9] != '\n') && (DAT_006daa0b[iVar9] != '\0')) {
                            if (FUN_0049f010(uVar8, (unsigned short)DAT_006daa01[iVar9],
                                             (unsigned short)DAT_006daa02[iVar9]) != 0) {
                                bVar3 = 1;
                            }
                        }
                        if ((DAT_006daa0f[iVar9] != '\n') && (DAT_006daa0f[iVar9] != '\0')) {
                            if (FUN_0049f010(uVar8, (unsigned short)DAT_006daa10[iVar9],
                                             (unsigned short)DAT_006daa11[iVar9]) != 0) {
                                bVar3 = 1;
                            }
                        }
                        if (bVar3) {
                            __builtin_memcpy(&i_tmp, (char *)((unsigned int)DAT_006d4988 + 0x1fu +
                                                              (unsigned int)local_8), 4);
                            __builtin_memcpy(&i_tmp, (char *)(i_tmp + 0xa7), 4);
                            cVar1 = *(char *)(i_tmp + 7);
                            if (cVar1 == '\0' || cVar1 == '\x06') {
                                __builtin_memcpy(&ui_tmp, DAT_0071b730 + iVar2, 4);
                                if (ui_tmp == 0xffffffff) {
                                    int a, b, c;
                                    FUN_00421c70(uVar8, 0);
                                    __builtin_memcpy(&a, DAT_0071bb1b + iVar2, 4);
                                    __builtin_memcpy(&b, DAT_0071bb1f + iVar2, 4);
                                    __builtin_memcpy(&c, DAT_0071bb23 + iVar2, 4);
                                    FUN_00418cd0((int)((unsigned int)DAT_00732a2c - (unsigned int)a),
                                                 (int)((unsigned int)DAT_00732a28 - (unsigned int)b),
                                                 (int)((unsigned int)DAT_00732a24 - (unsigned int)c));
                                } else {
                                    FUN_0042a140(ui_tmp, uVar8);
                                }
                                uVar11 = FUN_004a3830(uVar8, param_2, 0, 1, unaff_EBP, unaff_EDI);
                                if (uVar11 == 0) {
                                    if (DAT_006db580 < local_14) {
                                        local_14 = DAT_006db580;
                                        local_20 = sStack_1c;
                                    }
                                } else if (DAT_006db580 < local_4) {
                                    local_4 = DAT_006db580;
                                    sStack_1e = sStack_1c;
                                }
                            }
                        }
                    }
                }
            }
            local_8 = (int)((unsigned int)local_8 + 0x23u);
            uVar8 = uVar8 + 1;
        } while (uVar8 < DAT_006d4980);
    }
    sStack_1c = (short)DAT_00732a38;
    if (DAT_00732a38 < DAT_00703c88 + DAT_00732a38) {
        local_14 = DAT_00732a38 * 0x23u;
        uVar8 = DAT_00732a38;
        do {
            sStack_1c = (short)uVar8;
            if (*(char *)(local_14 + (unsigned int)DAT_006d4988 + 3) != '\0') {
                uVar11 = (unsigned int)*(unsigned char *)(local_14 + (unsigned int)DAT_006d4988 + 0x16);
                iVar9 = (int)(uVar11 * 0x566u);
                __builtin_memcpy(&us_tmp, DAT_0071b73c + iVar2, 2);
                if ((unsigned short)DAT_0070ad48[iVar9] != us_tmp) {
                    uVar12 = 0;
                    pcVar13 = DAT_0070aee5 + iVar9;
                    local_c = DAT_0070aee9 + uVar5 + iVar9;
                    do {
                        if ((*local_c != '\0') && (*pcVar13 == (char)param_1) &&
                            (-1 < pcVar13[3]) &&
                            (DAT_006d4b36[uVar12 + uVar5 * 8 + uVar11 * 0x126u] == 4)) {
                            __builtin_memcpy(&ui_tmp, DAT_0071b730 + iVar2, 4);
                            if (ui_tmp == 0xffffffff) {
                                int a, b, c;
                                FUN_00421c70(uVar8, 0);
                                __builtin_memcpy(&a, DAT_0071bb1b + iVar2, 4);
                                __builtin_memcpy(&b, DAT_0071bb1f + iVar2, 4);
                                __builtin_memcpy(&c, DAT_0071bb23 + iVar2, 4);
                                FUN_00418cd0((int)((unsigned int)DAT_00732a2c - (unsigned int)a),
                                             (int)((unsigned int)DAT_00732a28 - (unsigned int)b),
                                             (int)((unsigned int)DAT_00732a24 - (unsigned int)c));
                            } else {
                                FUN_0042a140(ui_tmp, uVar8);
                            }
                            if (DAT_006db580 < local_4) {
                                local_4 = DAT_006db580;
                                sStack_1e = sStack_1c;
                            }
                        }
                        pcVar13 = pcVar13 + 0x4e;
                        local_c = local_c + 0x4e;
                        uVar12 = uVar12 + 1;
                    } while (uVar12 < 8);
                }
            }
            local_14 = local_14 + 0x23u;
            uVar8 = uVar8 + 1;
            sStack_1c = (short)uVar8;
        } while (uVar8 < DAT_00703c88 + DAT_00732a38);
    }
    if (sStack_1e == -1) {
        return 0xffff0000u | (unsigned int)(unsigned short)local_20;
    }
    return ((unsigned int)(unsigned short)sStack_1c << 16) | (unsigned int)(unsigned short)sStack_1e;
}
