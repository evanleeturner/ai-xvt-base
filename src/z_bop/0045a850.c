/* SPDX-License-Identifier: MIT */
/* z_bop 0045a850 */
/* Prototypes (return types unresolved; Ghidra showed none for most) */
void __cdecl FUN_0041bba0(unsigned int a);
void __cdecl FUN_0041bb00(unsigned short a);
void __cdecl FUN_0045acd0(int a);
unsigned int __cdecl FUN_00425b70(unsigned int a, unsigned short b);
void __cdecl FUN_00402cb0(unsigned short a);
void __cdecl FUN_00402970(unsigned int a);
unsigned int __cdecl FUN_00404450(unsigned short a);
int __cdecl FUN_0046b610(char *a);

extern int _DAT_00a07cec;
extern int DAT_009a1fe0;
extern unsigned char *DAT_009a1fe8;
extern unsigned char *DAT_00a08104;
extern int DAT_00a07ce8;
extern int DAT_009d1308;
extern unsigned short DAT_009a73c2;

extern unsigned char DAT_009d9192[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9681[];
extern unsigned char DAT_009e9684[];
extern unsigned char DAT_009e9696[];
extern unsigned char DAT_009e96bb[];
extern unsigned char DAT_009e96bd[];
extern unsigned char DAT_009e96bf[];
extern unsigned char DAT_009e96c1[];
extern unsigned char DAT_009e96c3[];
extern unsigned char DAT_009e969d[];
extern unsigned char DAT_009e968a[];
extern unsigned char DAT_009a7a40[];
extern unsigned char DAT_00524120[];
extern unsigned char DAT_009d8cd2[];
extern unsigned char DAT_009d8cd3[];
extern unsigned char DAT_009d1320[];
extern unsigned char DAT_0051c583[];
extern unsigned short DAT_00527810[];
extern char DAT_0051a1d0[];
extern char s_stationaryldrpln_0051a1bc[];
extern char s_stationaryflwpln_0051a1a8[];
extern char s_disabledpln_0051a368[];
extern char s_escortldr1pln_0051a1e4[];

unsigned int __cdecl FUN_0045a850(int param_1, int param_2, int param_3)
{
    unsigned char *pbVar1;
    char cVar2;
    unsigned char bVar3;
    unsigned char *iVar4;
    unsigned int iVar5;
    unsigned short uVar6;
    int iVar7;
    unsigned int uVar9;
    unsigned int uVar10;
    unsigned int uVar11;
    unsigned short uVar12;
    int iVar14;
    char *pcVar15;
    char *pcVar16;
    char *pcVar17;
    int bVar18;
    unsigned char *tmp;
    unsigned char *tmp2;
    int tmpi;
    unsigned int tmpu;
    unsigned short tmps;

    if (param_2 == 1) {
        uVar12 = 0;
        if (_DAT_00a07cec < DAT_009a1fe0) {
            pcVar16 = (char *)(DAT_009a1fe8 + (unsigned int)_DAT_00a07cec * 0x23u + 3u);
            iVar7 = (int)((unsigned int)DAT_009a1fe0 - (unsigned int)_DAT_00a07cec);
            do {
                if (*pcVar16 != '\0') {
                    __builtin_memcpy(&tmpi, DAT_009d9192 + (unsigned int)(unsigned char)pcVar16[0x13] * 0x566u, 4);
                    if (tmpi == param_1) {
                        __builtin_memcpy(&tmp, pcVar16 + 0x1c, 4);
                        __builtin_memcpy(&tmp2, tmp + 0xa9, 4);
                        cVar2 = (char)tmp2[7];
                        if (cVar2 != '\x03' && cVar2 != '\x05' && cVar2 != '\x04') {
                            uVar12 = (unsigned short)(uVar12 + 1);
                        }
                    }
                }
                pcVar16 = pcVar16 + 0x23;
                iVar7 = (int)((unsigned int)iVar7 - 1u);
            } while (iVar7 != 0);
        }
        if (uVar12 < 2) {
            return 0;
        }
    }
    iVar7 = (int)((unsigned int)param_1 * 0x5bdu);
    __builtin_memcpy(&uVar10, DAT_009e9670 + iVar7, 4);
    if (uVar10 == 0xffffffff) {
        return 0;
    }
    iVar5 = uVar10 * 0x23u;
    tmpu = 0xffffffff;
    __builtin_memcpy(DAT_009a1fe8 + 0x1b + iVar5, &tmpu, 4);
    __builtin_memcpy(&tmp, DAT_009a1fe8 + 0x1f + iVar5, 4);
    tmp[0x92] = 1;
    __builtin_memcpy(&tmp, DAT_009a1fe8 + 0x1f + iVar5, 4);
    tmp[0x8b] = 1;
    FUN_0041bba0(uVar10);
    uVar12 = (unsigned short)uVar10;
    FUN_0041bb00(uVar12);
    FUN_0045acd0(param_1);
    iVar14 = 0;
    __builtin_memcpy(&tmp, DAT_009a1fe8 + 0x1f + iVar5, 4);
    __builtin_memcpy(&iVar4, tmp + 0xa9, 4);
    do {
        iVar4[0x220 + iVar14] = 0;
        iVar4[0x222 + iVar14] = 0;
        iVar4[0x224 + iVar14] = 0;
        tmps = 0;
        __builtin_memcpy(iVar4 + 0x226 + iVar14 * 2, &tmps, 2);
        tmpu = 0;
        __builtin_memcpy(iVar4 + 0x22a + iVar14 * 4, &tmpu, 4);
        iVar14 = iVar14 + 1;
    } while (iVar14 < 2);
    iVar14 = 0;
    do {
        pbVar1 = iVar4 + 0x235 + iVar14;
        iVar14 = iVar14 + 1;
        iVar4[0x234 + iVar14] = (unsigned char)((*pbVar1 & 0x81) | 1);
        tmps = 0;
        __builtin_memcpy(iVar4 + 0x235 + iVar14 * 2, &tmps, 2);
    } while (iVar14 < 2);
    tmps = 0;
    __builtin_memcpy(iVar4 + 0x23b, &tmps, 2);
    {
        unsigned int a, b;
        __builtin_memcpy(&a, iVar4 + 0x211, 4);
        __builtin_memcpy(&b, iVar4 + 0x215, 4);
        a = a + b;
        __builtin_memcpy(iVar4 + 0x211, &a, 4);
    }
    tmpu = 0;
    __builtin_memcpy(iVar4 + 0x215, &tmpu, 4);
    iVar4[0x21a] = 0;
    tmpu = 0xffffffff;
    __builtin_memcpy(DAT_009e9670 + iVar7, &tmpu, 4);
    DAT_009e9681[iVar7] = 0;
    DAT_009e9684[iVar7] = 0;
    DAT_009e9696[iVar7] = 0;
    tmp = DAT_00a08104;
    tmps = 0;
    __builtin_memcpy(DAT_009e96bb + iVar7, &tmps, 2);
    __builtin_memcpy(DAT_009e96bd + iVar7, &tmps, 2);
    __builtin_memcpy(DAT_009e96bf + iVar7, &tmps, 2);
    __builtin_memcpy(DAT_009e96c1 + iVar7, &tmps, 2);
    __builtin_memcpy(DAT_009e96c3 + iVar7, &tmps, 2);
    tmps = 0xffff;
    __builtin_memcpy(DAT_009e969d + iVar7, &tmps, 2);
    tmp[0x2a] = 0;
    if (param_3 == 0) {
        iVar7 = FUN_0046b610(DAT_0051a1d0);
        DAT_00a08104[0x34] = (unsigned char)iVar7;
        DAT_00a08104[0x35] = DAT_00a08104[0x34];
        DAT_00a08104 = iVar4;
        FUN_00402cb0(uVar12);
        FUN_00402970(uVar10);
        iVar4[0xa4] = 0;
        iVar4[0x9c] = 0;
        iVar4[0xab] = 0;
        iVar4[0x96] = 0;
        iVar4[0x97] = 0;
        return 1;
    }
    bVar3 = DAT_009a7a40[(unsigned int)DAT_00524120[DAT_009d8cd2[(unsigned int)DAT_009a1fe8[0x16 + iVar5] * 0x566u]]];
    DAT_00a08104[0x35] = bVar3;
    DAT_00a08104[0x34] = bVar3;
    bVar18 = (bVar3 == 0);
    pcVar16 = (char *)(DAT_009d1320 + (unsigned int)bVar3 * 0x55u);
    iVar14 = 8;
    pcVar15 = pcVar16;
    pcVar17 = DAT_0051a1d0;
    do {
        if (iVar14 == 0) break;
        iVar14 = iVar14 - 1;
        bVar18 = (*pcVar15 == *pcVar17);
        pcVar15 = pcVar15 + 1;
        pcVar17 = pcVar17 + 1;
    } while (bVar18);
    if (!bVar18) {
        iVar14 = 0x11;
        bVar18 = 0;
        pcVar15 = pcVar16;
        pcVar17 = s_stationaryldrpln_0051a1bc;
        do {
            if (iVar14 == 0) break;
            iVar14 = iVar14 - 1;
            bVar18 = (*pcVar15 == *pcVar17);
            pcVar15 = pcVar15 + 1;
            pcVar17 = pcVar17 + 1;
        } while (bVar18);
        if (!bVar18) {
            iVar14 = 0x11;
            bVar18 = 0;
            pcVar15 = pcVar16;
            pcVar17 = s_stationaryflwpln_0051a1a8;
            do {
                if (iVar14 == 0) break;
                iVar14 = iVar14 - 1;
                bVar18 = (*pcVar15 == *pcVar17);
                pcVar15 = pcVar15 + 1;
                pcVar17 = pcVar17 + 1;
            } while (bVar18);
            if (!bVar18) {
                iVar14 = 0xc;
                bVar18 = 0;
                pcVar15 = pcVar16;
                pcVar17 = s_disabledpln_0051a368;
                do {
                    if (iVar14 == 0) break;
                    iVar14 = iVar14 - 1;
                    bVar18 = (*pcVar15 == *pcVar17);
                    pcVar15 = pcVar15 + 1;
                    pcVar17 = pcVar17 + 1;
                } while (bVar18);
                if (!bVar18) {
                    iVar14 = 0xe;
                    bVar18 = 0;
                    pcVar15 = s_escortldr1pln_0051a1e4;
                    do {
                        if (iVar14 == 0) break;
                        iVar14 = iVar14 - 1;
                        bVar18 = (*pcVar16 == *pcVar15);
                        pcVar16 = pcVar16 + 1;
                        pcVar15 = pcVar15 + 1;
                    } while (bVar18);
                    if (bVar18) {
                        uVar6 = 0x8000;
                    } else {
                        uVar6 = DAT_00527810[(unsigned int)DAT_009d8cd3[(unsigned int)DAT_009a1fe8[0x16 + iVar5] * 0x566u]];
                    }
                    goto LAB_0045ab79;
                }
            }
        }
    }
    uVar6 = 0;
LAB_0045ab79:
    __builtin_memcpy(iVar4 + 0xbd, &uVar6, 2);
    __builtin_memcpy(&tmps, DAT_0051c583 + (unsigned int)iVar4[4] * 0x101u, 2);
    uVar9 = FUN_00425b70((unsigned int)tmps, uVar6);
    __builtin_memcpy(&tmp, DAT_009a1fe8 + 0x1f + iVar5, 4);
    tmps = (unsigned short)uVar9;
    __builtin_memcpy(tmp + 0x79, &tmps, 2);
    __builtin_memcpy(&tmp, DAT_009a1fe8 + 0x1f + iVar5, 4);
    tmps = 0;
    __builtin_memcpy(tmp + 0x7b, &tmps, 2);
    DAT_00a08104 = iVar4;
    FUN_00402cb0(uVar12);
    FUN_00402970(uVar10);
    __builtin_memcpy(&tmps, DAT_009e968a + iVar7, 2);
    if (tmps != 0xffff) {
        uVar10 = (unsigned int)tmps;
        if ((int)uVar10 < DAT_009a1fe0 ||
            (DAT_00a07ce8 <= (int)uVar10 &&
             (int)uVar10 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8))) {
            bVar18 = 0;
            uVar10 = 0;
            do {
                DAT_009a73c2 = (unsigned short)uVar10;
                __builtin_memcpy(&tmps, DAT_009e968a + iVar7, 2);
                uVar11 = FUN_00404450(tmps);
                if ((short)uVar11 != 0) {
                    bVar18 = 1;
                }
                uVar10 = uVar10 + 1;
            } while (uVar10 < 3);
            if (bVar18) {
                __builtin_memcpy(&tmps, DAT_009e968a + iVar7, 2);
                __builtin_memcpy(iVar4 + 0x55, &tmps, 2);
            }
        }
    }
    return 1;
}
