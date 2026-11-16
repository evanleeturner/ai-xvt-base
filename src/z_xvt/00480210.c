/* SPDX-License-Identifier: MIT */
/* z_xvt 00480210 */
extern int DAT_006d4988;
extern int DAT_006d4980;
extern char DAT_006d4a02;
extern int DAT_00703c88;
extern char DAT_007092db;
extern unsigned short DAT_0071e6be;
extern int DAT_0071ecf4;
extern int DAT_00732a38;
extern short DAT_00732a3c;
extern unsigned char DAT_0070ad51[];
extern unsigned char DAT_0052b07e[];
extern unsigned char DAT_0052686c[];
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b744[];

void __cdecl FUN_0049f9c0(unsigned short a, unsigned short b, int c);
unsigned int __cdecl FUN_004a7aa0(unsigned int a);
void __cdecl FUN_0040a980(int a, int b);
void __cdecl FUN_0040abf0(int a);
void __cdecl FUN_00423d00(int a);
void __cdecl FUN_00421580(int a);
void __cdecl FUN_0041e890(unsigned short a);
int __cdecl FUN_00423200(int a, unsigned int b, unsigned int c, int d);
void __cdecl FUN_004441e0(int a);
void __cdecl FUN_00444330(int a);
void __cdecl FUN_004a41d0(int a);
void __cdecl FUN_004a1d70(int a, int b);
void __cdecl FUN_0040a1a0(unsigned int a, unsigned int b, int c);
void __cdecl FUN_00445ba0(int a, int b);
void __cdecl FUN_00445c50(int a, int b);

void __cdecl FUN_00480210(int param_1, int param_2, int param_3)
{
    short sVar1;
    char cVar2;
    unsigned char bVar3;
    int iVar4;
    int bVar5;
    unsigned int uVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    unsigned short uVar10;
    int iVar11;
    unsigned short uVar12;
    int iVar13;
    unsigned short uVar14;
    int iVar15;
    int iTmp;
    int iTmp2;
    unsigned int uTmp;
    unsigned short sTmp;

    iVar4 = (int)((unsigned int)param_1 * 0x5bd);
    __builtin_memcpy(&iVar11, DAT_0071b730 + iVar4, 4);
    iVar9 = (int)((unsigned int)iVar11 * 0x23);
    iVar7 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar9);
    __builtin_memcpy(&iTmp, (unsigned char *)(DAT_006d4988 + 0x1f + iVar9), 4);
    __builtin_memcpy(&iVar8, (unsigned char *)(iTmp + 0xa7), 4);
    if (*(char *)(iVar8 + 7) != '\0') {
        DAT_0071b744[iVar4] = 0;
    }
    __builtin_memcpy(&uTmp, DAT_0071b730 + 0x15 + iVar4, 4);
    uVar6 = (unsigned int)DAT_0071e6be + uTmp;
    __builtin_memcpy(DAT_0071b730 + 0x15 + iVar4, &uVar6, 4);
    if (DAT_0071b744[iVar4] != 1) {
        if (DAT_0071b744[iVar4] != 2) {
            return;
        }
        if (0x49b < uVar6) {
            bVar3 = *(unsigned char *)(iVar7 + 0x16);
            FUN_0049f9c0((unsigned short)iVar11, (unsigned short)bVar3, 0x11);
            if (((DAT_006d4a02 != 2) && (DAT_007092db == 1)) &&
                (DAT_0070ad51[(unsigned int)bVar3 * 0x566] != 'c')) {
                uVar6 = FUN_004a7aa0(((((unsigned int)DAT_006d4988 >> 8) << 8) |
                                      (unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar9)) & 0xffff00ff);
                __builtin_memcpy(&uTmp, DAT_0071b730 + 0x6d + iVar4, 4);
                uTmp = uTmp + (unsigned int)DAT_0052b07e[(uVar6 & 0xffff) * 0xef] * 0x28;
                __builtin_memcpy(DAT_0071b730 + 0x6d + iVar4, &uTmp, 4);
            }
            __builtin_memcpy(&iTmp, DAT_0071b730 + iVar4, 4);
            if (iTmp != -1) {
                FUN_0040a980(0, param_1);
                FUN_0040abf0(0);
            }
            *(unsigned char *)(DAT_006d4988 + 3 + iVar9) = 0;
            FUN_00423d00(param_1);
            FUN_00421580(iVar8);
            DAT_0071b744[iVar4] = 0;
            FUN_0041e890((unsigned short)bVar3);
            iVar7 = FUN_00423200(param_1, 0xffffffff, 0, 0);
            if (iVar7 != 0) {
                FUN_004441e0(param_1);
                FUN_00444330(param_1);
                return;
            }
            if (DAT_0071ecf4 == param_1) {
                FUN_004a41d0(0x119);
            }
            return;
        }
        bVar5 = 0;
        __builtin_memcpy(&iVar7, DAT_0071b730 + iVar4, 4);
        for (sVar1 = DAT_00732a3c; sVar1 < DAT_006d4980; sVar1 = (short)(sVar1 + 1)) {
            iVar11 = (int)((unsigned int)(int)sVar1 * 0x23 + (unsigned int)DAT_006d4988);
            if (*(char *)(iVar11 + 3) != '\0') {
                iVar8 = (int)((unsigned int)iVar7 * 0x23 + (unsigned int)DAT_006d4988);
                iVar9 = (int)((unsigned int)*(int *)(iVar11 + 4) - (unsigned int)*(int *)(iVar8 + 4));
                iVar13 = (int)((unsigned int)*(int *)(iVar11 + 8) - (unsigned int)*(int *)(iVar8 + 8));
                iVar15 = (int)((unsigned int)*(int *)(iVar11 + 0xc) - (unsigned int)*(int *)(iVar8 + 0xc));
                if (iVar9 < 0) {
                    iVar9 = (int)(0u - (unsigned int)iVar9);
                }
                if (iVar15 < 0) {
                    iVar15 = (int)(0u - (unsigned int)iVar15);
                }
                if ((((int)((unsigned int)iVar9 -
                            (unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar11 + 3) * 0x16)) <
                      (int)(unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar8 + 3) * 0x16)) &&
                     ((int)((unsigned int)iVar15 -
                            (unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar11 + 3) * 0x16)) <
                      (int)(unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar8 + 3) * 0x16))) &&
                    ((0 < iVar13 && (iVar13 < 0x40000)))) {
                    bVar5 = 1;
                    break;
                }
            }
        }
        if (!bVar5) {
            for (sVar1 = (short)DAT_00732a38; (int)sVar1 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38);
                 sVar1 = (short)(sVar1 + 1)) {
                iVar11 = (int)((unsigned int)(int)sVar1 * 0x23 + (unsigned int)DAT_006d4988);
                if (*(char *)(iVar11 + 3) != '\0') {
                    iVar8 = (int)((unsigned int)iVar7 * 0x23 + (unsigned int)DAT_006d4988);
                    iVar9 = (int)((unsigned int)*(int *)(iVar11 + 4) - (unsigned int)*(int *)(iVar8 + 4));
                    iVar13 = (int)((unsigned int)*(int *)(iVar11 + 8) - (unsigned int)*(int *)(iVar8 + 8));
                    iVar15 = (int)((unsigned int)*(int *)(iVar11 + 0xc) - (unsigned int)*(int *)(iVar8 + 0xc));
                    if (iVar9 < 0) {
                        iVar9 = (int)(0u - (unsigned int)iVar9);
                    }
                    if (iVar15 < 0) {
                        iVar15 = (int)(0u - (unsigned int)iVar15);
                    }
                    if ((((int)((unsigned int)iVar9 -
                                (unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar11 + 3) * 0x16)) <
                          (int)(unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar8 + 3) * 0x16)) &&
                         ((int)((unsigned int)iVar15 -
                                (unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar11 + 3) * 0x16)) <
                          (int)(unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar8 + 3) * 0x16))) &&
                        ((0 < iVar13 && (iVar13 < 0x40000)))) {
                        bVar5 = 1;
                        break;
                    }
                }
            }
            if (!bVar5) {
                uVar12 = (unsigned short)(uVar6 / 0xec);
                if (uVar12 == 0) {
                    iVar11 = 10;
                } else if (uVar12 == 1) {
                    iVar11 = 0x19;
                } else if (uVar12 == 2) {
                    iVar11 = 0x32;
                } else if (uVar12 == 3) {
                    iVar11 = 100;
                } else {
                    iVar11 = 1000;
                }
                FUN_00445ba0(iVar7, iVar11);
                *(unsigned int *)((unsigned int)iVar7 * 0x23 + 8 + (unsigned int)DAT_006d4988) =
                    *(unsigned int *)((unsigned int)iVar7 * 0x23 + 8 + (unsigned int)DAT_006d4988) +
                    (unsigned int)uVar12 * (unsigned int)DAT_0071e6be * 0xe0;
                return;
            }
        }
        FUN_004a1d70(0x6f, param_1);
        __builtin_memcpy(&iTmp, DAT_0071b730 + iVar4, 4);
        if ((iTmp == -1) ||
            ((((cVar2 = *(char *)((int)((unsigned int)iTmp * 0x23) + 3 + DAT_006d4988),
                cVar2 != 1 && (cVar2 != 2)) && (cVar2 != 3)) &&
              ((cVar2 != 0xe && (cVar2 != 4)))))) {
            bVar5 = 0;
        } else {
            bVar5 = 1;
        }
        if (bVar5) {
            uVar6 = 0x54;
        } else {
            uVar6 = 0x24;
        }
        FUN_0040a1a0(uVar6, 0xffffffff, param_1);
        DAT_0071b744[iVar4] = 0;
        __builtin_memcpy(&iTmp, (unsigned char *)((int)((unsigned int)iVar7 * 0x23) + 0x1f + DAT_006d4988), 4);
        sTmp = 10;
        __builtin_memcpy((unsigned char *)(iTmp + 0x79), &sTmp, 2);
        return;
    }
    uVar12 = *(unsigned short *)(iVar7 + 0x14);
    uVar14 = *(unsigned short *)(iVar7 + 0x10);
    uVar10 = *(unsigned short *)(iVar7 + 0x12);
    if (((uVar12 == 0) && (uVar10 == 0x4000)) && (uVar14 == 0)) {
        uVar12 = 0;
        uVar14 = 0;
        if (0x49b < uVar6) {
            DAT_0071b744[iVar4] = 2;
            FUN_004a1d70(0x6b, param_1);
            uTmp = 0;
            __builtin_memcpy(DAT_0071b730 + 0x15 + iVar4, &uTmp, 4);
            FUN_0040a1a0(0x51, 0xffffffff, param_1);
        }
        goto LAB_0048038f;
    }
    if (uVar12 < 0x8000) {
        uVar12 = (unsigned short)((unsigned int)uVar12 + (unsigned int)DAT_0071e6be * (unsigned int)-0x20);
        if (0x7fff < uVar12) {
LAB_00480336:
            uVar12 = 0;
        }
    } else {
        uVar12 = (unsigned short)((unsigned int)uVar12 + (unsigned int)DAT_0071e6be * 0x20u);
        if (uVar12 < 0x8000) goto LAB_00480336;
    }
    if (uVar14 < 0x8000) {
        uVar14 = (unsigned short)((unsigned int)uVar14 + (unsigned int)DAT_0071e6be * (unsigned int)-0x10);
        if (0x7fff < uVar14) {
LAB_00480356:
            uVar14 = 0;
        }
    } else {
        uVar14 = (unsigned short)((unsigned int)uVar14 + (unsigned int)DAT_0071e6be * 0x10u);
        if (uVar14 < 0x8000) goto LAB_00480356;
    }
    if ((uVar10 < 0xc000) && (0x4000 < uVar10)) {
        uVar10 = (unsigned short)((unsigned int)uVar10 + (unsigned int)DAT_0071e6be * (unsigned int)-0x10);
        if (0x3fff < uVar10) goto LAB_0048038f;
    } else if ((uVar10 == 0x4000) ||
               ((uVar10 = (unsigned short)((unsigned int)uVar10 + (unsigned int)DAT_0071e6be * 0x10u),
                 uVar10 < 0x4001) || (0xbfff < uVar10)))
        goto LAB_0048038f;
    uVar10 = 0x4000;
LAB_0048038f:
    *(unsigned short *)(DAT_006d4988 + 0x14 + iVar9) = uVar12;
    *(unsigned short *)(DAT_006d4988 + 0x10 + iVar9) = uVar14;
    *(unsigned short *)(DAT_006d4988 + 0x12 + iVar9) = uVar10;
    __builtin_memcpy(&iTmp, (unsigned char *)(DAT_006d4988 + 0x1f + iVar9), 4);
    *(unsigned char *)(iTmp + 0x90) = 1;
    __builtin_memcpy(&iVar7, (unsigned char *)(DAT_006d4988 + 0x1f + iVar9), 4);
    *(unsigned char *)(iVar7 + 0x89) = *(unsigned char *)(iVar7 + 0x90);
    __builtin_memcpy(&iTmp, (unsigned char *)(DAT_006d4988 + 0x1f + iVar9), 4);
    __builtin_memcpy(&iTmp2, (unsigned char *)(iTmp + 0xa7), 4);
    __builtin_memcpy((unsigned char *)(iTmp2 + 0xd), &uVar10, 2);
    FUN_00445c50(iVar11, 200);
    return;
}
