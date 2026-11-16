/* SPDX-License-Identifier: MIT */
/* z_xvt 004f4b40 */
extern int            DAT_00757870[];
extern int            DAT_007579d0[];
extern unsigned char  DAT_0081bfb6[4];
extern int            DAT_00757680[];
extern unsigned char  DAT_00757ce0[];
extern unsigned int   DAT_007579f8;
extern unsigned int   DAT_00757a00[];
extern int            DAT_00757a20;
extern int            DAT_0081b850;
extern unsigned int   DAT_0081b854;
extern unsigned int   DAT_0081bcb0;
extern unsigned int   DAT_0081bcd0;
extern unsigned int   DAT_0081b6fc;
extern unsigned char  DAT_0081bc80[];
extern char           DAT_0081b740[];
extern unsigned int   DAT_00758620;
extern unsigned int   DAT_00758624;
extern int            DAT_00758628;
extern int            DAT_0075862c;
extern short          DAT_0075766c;
extern char           DAT_00743935;
extern char           DAT_0085e7c5;
extern const char     s__c_s__c_s_00531334[];
extern const char     s__s__s_00533618[];
extern const char     s__u___c_s__s___c_00533edc[];
extern const char     s_slotsound_0053309c[];

void  FUN_004dcfd0(int *, int, int, int, int);
char *FUN_004dcfa0(unsigned int);
void  FUN_004daac0(int, char *, int *, int, int, unsigned int);
int   FUN_004cdbc0(void);
void  FUN_004da920(int, char *, int *, unsigned int);
void  FUN_004d5180(int *);
void  FUN_004d51b0(int *);
int   _sprintf(char *, const char *, ...);
unsigned char FUN_004db630(int);
void  FUN_004ddc70(int *, int *);
unsigned char FUN_004db530(void);
unsigned char FUN_004db550(void);
int   FUN_004dd7d0(int *, int, int);
void  FUN_004db510(int);
void  FUN_004cb830(int, unsigned int *, int);
void  FUN_004dd030(int *, int, int);
unsigned char FUN_004db5f0(int);
void  FUN_004de190(const char *, int, int, int);
unsigned char FUN_004db610(int);
void  FUN_004db520(void);

#define LB() __builtin_memcpy(&uBfb6, DAT_0081bfb6, 4)

unsigned int FUN_004f4b40(int param_1)
{
    char cVar1;
    unsigned char bVar2;
    unsigned char uVar3;
    char *pcVar4;
    unsigned int *puVar5;
    char *pcVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    unsigned char bVar10;
    int *piVar11;
    int iVar12;
    char *pcVar13;
    int iVar14;
    unsigned int uVar15;
    int iVar16;
    unsigned int uVar17;
    unsigned int uVar18;
    char *local_48;
    int local_3c;
    int local_38;
    int local_34[4];
    unsigned int local_24;
    int local_20[4];
    int local_10[4];
    unsigned int uBfb6;
    int iTmp;
    unsigned int uTmp;

    FUN_004dcfd0(local_34, 0x58, 0x14f, 0xe5, 0x15d);
    piVar11 = local_34;
    uVar18 = 0xffff;
    iVar16 = 1;
    iVar14 = 0;
    pcVar4 = FUN_004dcfa0(0x265);
    FUN_004daac0(0xc, pcVar4, piVar11, iVar14, iVar16, uVar18);
    FUN_004dcfd0(local_34, 0xe8, 0x14f, 0x150, 0x15d);
    piVar11 = local_34;
    uVar18 = 0xffff;
    iVar16 = 1;
    iVar14 = 0;
    pcVar4 = FUN_004dcfa0(0x266);
    FUN_004daac0(0xc, pcVar4, piVar11, iVar14, iVar16, uVar18);
    FUN_004dcfd0(local_34, 0x153, 0x14f, 0x1bb, 0x15d);
    piVar11 = local_34;
    uVar18 = 0xffff;
    iVar16 = 1;
    iVar14 = 0;
    pcVar4 = FUN_004dcfa0(0x29f);
    FUN_004daac0(0xc, pcVar4, piVar11, iVar14, iVar16, uVar18);
    iVar14 = FUN_004cdbc0();
    LB();
    local_24 = (unsigned int)(iVar14 == DAT_00757870[uBfb6 * 8u]);
    LB();
    if (1 < DAT_007579d0[uBfb6]) {
        FUN_004dcfd0(local_34, 0x58, 0x1a6, 0x1bb, 0x1b4);
        if ((local_24 == 0) && (DAT_0081b850 != 2)) {
            uVar15 = 0x268;
            uVar18 = DAT_0081b854;
        }
        else {
            uVar15 = 0x267;
            uVar18 = DAT_0081bcb0;
        }
        piVar11 = local_34;
        pcVar4 = FUN_004dcfa0(uVar15);
        FUN_004da920(0xc, pcVar4, piVar11, uVar18);
    }
    local_48 = (char *)0x0;
    iVar16 = 0;
    FUN_004dcfd0(local_34, 0x153, 0x160, 0x1bb, 0x170);
    iVar14 = DAT_00757a20;
    LB();
    if (0 < DAT_007579d0[uBfb6]) {
        do {
            iVar8 = 0;
            pcVar13 = (char *)(DAT_00757ce0 + 0xe);
            LB();
            iVar12 = (int)((unsigned int)iVar16 + uBfb6 * 8u);
            do {
                __builtin_memcpy(&iTmp, pcVar13, 4);
                if (iTmp == DAT_00757870[iVar12]) break;
                pcVar13 = pcVar13 + 0x2a;
                iVar8 = (int)((unsigned int)iVar8 + 1u);
            } while (pcVar13 < (char *)(DAT_00757ce0 + 0x15e));
            if ((iVar8 != 8) && (DAT_00757680[iVar12] == -1)) {
                iVar9 = (int)((unsigned int)iVar8 * 0x2au);
                __builtin_memcpy(&uVar18, DAT_00757ce0 + iVar9 + 0xe, 4);
                if ((uVar18 != DAT_007579f8) && (DAT_00757870[iVar12] != 0)) {
                    iVar12 = 0;
                    if (0 < iVar14) {
                        puVar5 = DAT_00757a00;
                        do {
                            if (*puVar5 == uVar18) break;
                            puVar5 = puVar5 + 1;
                            iVar12 = (int)((unsigned int)iVar12 + 1u);
                        } while (iVar12 < iVar14);
                    }
                    FUN_004d5180(local_10);
                    FUN_004d51b0(local_34);
                    iVar14 = (int)(DAT_00757ce0 + iVar9);
                    if (iVar12 == DAT_00757a20) {
                        uVar17 = 1;
                        __builtin_memcpy(&iTmp, DAT_00757ce0 + iVar9 + 0x12, 4);
                        pcVar4 = FUN_004dcfa0((unsigned int)iTmp + 0x8eu);
                        _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar4, uVar17);
                        iVar14 = FUN_004cdbc0();
                        __builtin_memcpy(&iTmp, DAT_00757ce0 + iVar9 + 0xe, 4);
                        if ((iVar14 == iTmp) ||
                            (uVar18 = DAT_0081bcd0, DAT_0081b850 == 2)) {
                            uVar18 = *(unsigned int *)(DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2u));
                        }
                    }
                    else {
                        __builtin_memcpy(&iTmp, DAT_00757ce0 + iVar9 + 0x12, 4);
                        pcVar4 = FUN_004dcfa0((unsigned int)iTmp + 0x8eu);
                        _sprintf(DAT_0081b740, s__s__s_00533618, pcVar4, iVar14);
                        uVar18 = DAT_0081b6fc;
                    }
                    FUN_004daac0(0xc, DAT_0081b740, local_34, 0, 1, uVar18);
                    FUN_004d51b0(local_10);
                    local_48 = (char *)((int)local_48 + 1);
                    iVar14 = FUN_004cdbc0();
                    LB();
                    if (((iVar14 == DAT_00757870[uBfb6 * 8u]) || (DAT_0081b850 == 2)) &&
                        ((bVar2 = FUN_004db630(3)) == 0)) {
                        FUN_004ddc70(&local_38, &local_3c);
                        uVar3 = FUN_004db530();
                        if (((uVar3 != 0) ||
                             ((uVar3 = FUN_004db550()) != 0)) &&
                            ((iVar14 = FUN_004dd7d0(local_34, local_38, local_3c)) != 0)) {
                            if (DAT_0081b850 == 2) {
                                __builtin_memcpy(&DAT_007579f8, DAT_00757ce0 + iVar9 + 0xe, 4);
                                FUN_004db510(3);
                            }
                            else {
                                iVar14 = 0;
                                if (0 < DAT_00757a20) {
                                    piVar11 = (int *)DAT_00757a00;
                                    do {
                                        __builtin_memcpy(&iTmp, DAT_00757ce0 + iVar9 + 0xe, 4);
                                        if (*piVar11 == iTmp) break;
                                        piVar11 = piVar11 + 1;
                                        iVar14 = (int)((unsigned int)iVar14 + 1u);
                                    } while (iVar14 < DAT_00757a20);
                                }
                                if (iVar14 == DAT_00757a20) {
                                    __builtin_memcpy(&DAT_00758624, DAT_00757ce0 + iVar9 + 0xe, 4);
                                    DAT_00757a20 = (int)((unsigned int)DAT_00757a20 + 1u);
                                    DAT_00757a00[iVar14] = DAT_00758624;
                                    DAT_00758620 = 0x6b;
                                    DAT_00758628 = FUN_004cdbc0();
                                    FUN_004cb830(0, &DAT_00758620, 0xc);
                                    __builtin_memcpy(&DAT_007579f8, DAT_00757ce0 + iVar9 + 0xe, 4);
                                    iVar14 = 0;
                                    FUN_004db510(3);
                                    uVar18 = DAT_007579f8;
                                    LB();
                                    if (0 < DAT_007579d0[uBfb6]) {
                                        do {
                                            LB();
                                            iVar12 = (int)((unsigned int)iVar14 + uBfb6 * 8u);
                                            if (((unsigned int)DAT_00757870[iVar12] == uVar18) &&
                                                (DAT_00757680[iVar12] != -1)) {
                                                DAT_00758620 = 0x50;
                                                LB();
                                                DAT_00758624 = uBfb6;
                                                DAT_0075862c = -1;
                                                DAT_00758628 = iVar14;
                                                FUN_004cb830(0, &DAT_00758620, 0x14);
                                                uVar18 = DAT_007579f8;
                                            }
                                            iVar14 = (int)((unsigned int)iVar14 + 1u);
                                            LB();
                                        } while (iVar14 < DAT_007579d0[uBfb6]);
                                    }
                                }
                            }
                        }
                    }
                    FUN_004dd030(local_34, 0, 0x11);
                    iVar14 = DAT_00757a20;
                    if (3 < (int)local_48) break;
                }
            }
            iVar16 = (int)((unsigned int)iVar16 + 1u);
            LB();
        } while (iVar16 < DAT_007579d0[uBfb6]);
    }
    iVar16 = 0;
    FUN_004ddc70(&local_38, &local_3c);
    FUN_004dcfd0(local_34, 0x58, 0x160, 0xe5, 0x170);
    iVar14 = 0;
    FUN_004dcfd0(local_20, 0xe8, 0x160, 0x14c, 0x170);
    if (0 < DAT_0075766c) {
        local_48 = &DAT_00743935;
        do {
            LB();
            if ((local_48[-0x51] != '\0') && ((unsigned char)local_48[-0x5d] == uBfb6)) {
                switch (local_48[-0x5e]) {
                case '\0':
                    bVar10 = 2;
                    break;
                case '\x01':
                case '\x04':
                    bVar10 = 3;
                    break;
                case '\x02':
                    bVar10 = 5;
                    break;
                case '\x03':
                    bVar10 = 4;
                    break;
                case '\x05':
                    bVar10 = 6;
                    break;
                default:
                    bVar10 = 1;
                }
                iVar16 = (int)((unsigned int)iVar16 + 1u);
                uVar17 = 1;
                pcVar4 = local_48 + -0xb5;
                pcVar6 = FUN_004dcfa0((unsigned int)((unsigned char)local_48[-99]) + 0x253u);
                _sprintf(DAT_0081b740, s__u___c_s__s___c_00533edc, iVar16, (unsigned int)bVar10, pcVar6, pcVar4,
                         uVar17);
                pcVar4 = local_48;
                if (*local_48 == '\0') {
                    pcVar4 = FUN_004dcfa0(0x264);
                }
                uVar18 = 0xffffffff;
                do {
                    pcVar6 = pcVar4;
                    if (uVar18 == 0) break;
                    uVar18 = uVar18 - 1;
                    pcVar6 = pcVar4 + 1;
                    cVar1 = *pcVar4;
                    pcVar4 = pcVar6;
                } while (cVar1 != '\0');
                uVar18 = ~uVar18;
                iVar12 = -1;
                pcVar4 = DAT_0081b740;
                do {
                    pcVar13 = pcVar4;
                    if (iVar12 == 0) break;
                    iVar12 = iVar12 + -1;
                    pcVar13 = pcVar4 + 1;
                    cVar1 = *pcVar4;
                    pcVar4 = pcVar13;
                } while (cVar1 != '\0');
                pcVar4 = pcVar6 + -uVar18;
                pcVar6 = pcVar13 + -1;
                for (uVar15 = uVar18 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
                    __builtin_memcpy(&uTmp, pcVar4, 4);
                    __builtin_memcpy(pcVar6, &uTmp, 4);
                    pcVar4 = pcVar4 + 4;
                    pcVar6 = pcVar6 + 4;
                }
                for (uVar18 = uVar18 & 3; uVar18 != 0; uVar18 = uVar18 - 1) {
                    *pcVar6 = *pcVar4;
                    pcVar4 = pcVar4 + 1;
                    pcVar6 = pcVar6 + 1;
                }
                iVar12 = -1;
                iVar8 = 0;
                FUN_004daac0(0xc, DAT_0081b740, local_34, 0, 1, 0xffff);
                LB();
                if (0 < DAT_007579d0[uBfb6]) {
                    do {
                        LB();
                        iVar9 = (int)((unsigned int)iVar8 + uBfb6 * 8u);
                        if (DAT_00757680[iVar9] == iVar14) {
                            iVar7 = 0;
                            pcVar13 = (char *)(DAT_00757ce0 + 0xe);
                            do {
                                iVar12 = iVar8;
                                __builtin_memcpy(&iTmp, pcVar13, 4);
                                if (iTmp == DAT_00757870[iVar9]) {
                                    iVar9 = (int)((unsigned int)(int)DAT_00757ce0 + (unsigned int)iVar7 * 0x2au);
                                    uVar17 = 1;
                                    __builtin_memcpy(&iTmp, DAT_00757ce0 + (unsigned int)iVar7 * 0x2au + 0x12, 4);
                                    pcVar4 = FUN_004dcfa0((unsigned int)iTmp + 0x8eu);
                                    _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar4, uVar17, iVar9);
                                    FUN_004d5180(local_10);
                                    FUN_004d51b0(local_20);
                                    iVar9 = FUN_004cdbc0();
                                    __builtin_memcpy(&iTmp, DAT_00757ce0 + (unsigned int)iVar7 * 0x2au + 0xe, 4);
                                    if ((iVar9 == iTmp) ||
                                        (uVar18 = DAT_0081bcd0, DAT_0081b850 == 2)) {
                                        uVar18 = *(unsigned int *)(DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2u));
                                    }
                                    FUN_004daac0(0xc, DAT_0081b740, local_20, 0, 1, uVar18);
                                    FUN_004d51b0(local_10);
                                    break;
                                }
                                pcVar13 = pcVar13 + 0x2a;
                                iVar7 = (int)((unsigned int)iVar7 + 1u);
                            } while (pcVar13 < (char *)(DAT_00757ce0 + 0x15e));
                        }
                        iVar8 = (int)((unsigned int)iVar8 + 1u);
                        LB();
                    } while (iVar8 < DAT_007579d0[uBfb6]);
                }
                LB();
                if (1 < DAT_007579d0[uBfb6]) {
                    if (iVar12 == -1) {
                        bVar2 = FUN_004db630(3);
                        if ((bVar2 != 0) &&
                            (((uVar3 = FUN_004db5f0(3)) != 0 ||
                              ((uVar3 = FUN_004db5f0(3)) != 0)) &&
                             ((iVar12 = FUN_004dd7d0(local_20, local_38, local_3c)) != 0))) {
                            if (DAT_0081b850 == 2) {
                                if (DAT_0085e7c5 != '\0') {
                                    FUN_004de190(s_slotsound_0053309c, 1, 0, 0xff);
                                }
                                iVar12 = 0;
                                LB();
                                if (0 < DAT_007579d0[uBfb6]) {
                                    do {
                                        LB();
                                        if ((unsigned int)DAT_00757870[uBfb6 * 8u + (unsigned int)iVar12] == DAT_007579f8) {
                                            DAT_00757680[uBfb6 * 8u + (unsigned int)iVar12] = iVar14;
                                            break;
                                        }
                                        iVar12 = (int)((unsigned int)iVar12 + 1u);
                                        LB();
                                    } while (iVar12 < DAT_007579d0[uBfb6]);
                                }
                            }
                            else {
                                DAT_00758628 = 0;
                                LB();
                                if (0 < DAT_007579d0[uBfb6]) {
                                    do {
                                        LB();
                                        if ((unsigned int)DAT_00757870[uBfb6 * 8u + (unsigned int)DAT_00758628] == DAT_007579f8) break;
                                        DAT_00758628 = (int)((unsigned int)DAT_00758628 + 1u);
                                        LB();
                                    } while (DAT_00758628 < DAT_007579d0[uBfb6]);
                                }
                                LB();
                                DAT_00758624 = uBfb6;
                                DAT_00758620 = 0x50;
                                DAT_0075862c = iVar14;
                                FUN_004cb830(0, &DAT_00758620, 0x10);
                                DAT_00758620 = 0x6a;
                                DAT_00758624 = DAT_007579f8;
                                FUN_004cb830(0, &DAT_00758620, 8);
                            }
LAB_004f57a9:
                            DAT_007579f8 = 0;
                            FUN_004db520();
                        }
                    }
                    else {
                        bVar2 = FUN_004db630(3);
                        if (bVar2 == 0) {
                            if ((((local_24 != 0) || (DAT_0081b850 == 2)) &&
                                 (((uVar3 = FUN_004db530()) != 0) ||
                                  ((uVar3 = FUN_004db550()) != 0))) &&
                                ((iVar8 = FUN_004dd7d0(local_20, local_38, local_3c)) != 0)) {
                                if (DAT_0081b850 == 2) {
                                    LB();
                                    DAT_007579f8 = (unsigned int)DAT_00757870[(unsigned int)iVar12 + uBfb6 * 8u];
                                    FUN_004db510(3);
                                    LB();
                                    DAT_00757680[(unsigned int)iVar12 + uBfb6 * 8u] = -1;
                                }
                                else {
                                    iVar8 = 0;
                                    if (0 < DAT_00757a20) {
                                        piVar11 = (int *)DAT_00757a00;
                                        do {
                                            LB();
                                            if (*piVar11 == DAT_00757870[(unsigned int)iVar12 + uBfb6 * 8u]) break;
                                            piVar11 = piVar11 + 1;
                                            iVar8 = (int)((unsigned int)iVar8 + 1u);
                                        } while (iVar8 < DAT_00757a20);
                                    }
                                    if (iVar8 == DAT_00757a20) {
                                        DAT_00757a20 = (int)((unsigned int)DAT_00757a20 + 1u);
                                        DAT_00758620 = 0x6b;
                                        LB();
                                        DAT_00758624 = (unsigned int)DAT_00757870[(unsigned int)iVar12 + uBfb6 * 8u];
                                        DAT_00757a00[iVar8] = DAT_00758624;
                                        DAT_00758628 = FUN_004cdbc0();
                                        FUN_004cb830(0, &DAT_00758620, 0xc);
                                        iVar8 = 0;
                                        LB();
                                        DAT_007579f8 = (unsigned int)DAT_00757870[(unsigned int)iVar12 + uBfb6 * 8u];
                                        FUN_004db510(3);
                                        uVar18 = DAT_007579f8;
                                        LB();
                                        if (0 < DAT_007579d0[uBfb6]) {
                                            do {
                                                LB();
                                                iVar12 = (int)((unsigned int)iVar8 + uBfb6 * 8u);
                                                if (((unsigned int)DAT_00757870[iVar12] == uVar18) &&
                                                    (DAT_00757680[iVar12] != -1)) {
                                                    DAT_00758620 = 0x50;
                                                    LB();
                                                    DAT_00758624 = uBfb6;
                                                    DAT_0075862c = -1;
                                                    DAT_00758628 = iVar8;
                                                    FUN_004cb830(0, &DAT_00758620, 0x14);
                                                    uVar18 = DAT_007579f8;
                                                }
                                                iVar8 = (int)((unsigned int)iVar8 + 1u);
                                                LB();
                                            } while (iVar8 < DAT_007579d0[uBfb6]);
                                        }
                                    }
                                }
                            }
                        }
                        else {
                            uVar3 = FUN_004db5f0(3);
                            if (((uVar3 != 0) ||
                                 ((uVar3 = FUN_004db610(3)) != 0)) &&
                                ((iVar8 = FUN_004dd7d0(local_20, local_38, local_3c)) != 0)) {
                                if (DAT_0081b850 == 2) {
                                    if (DAT_0085e7c5 != '\0') {
                                        FUN_004de190(s_slotsound_0053309c, 1, 0, 0xff);
                                    }
                                    iVar7 = 0;
                                    LB();
                                    iVar9 = (int)(uBfb6 * 8u);
                                    iVar8 = DAT_007579d0[uBfb6];
                                    DAT_00757680[(unsigned int)iVar12 + (unsigned int)iVar9] = -1;
                                    if (0 < iVar8) {
                                        do {
                                            if ((unsigned int)DAT_00757870[(unsigned int)iVar9 + (unsigned int)iVar7] == DAT_007579f8) {
                                                DAT_00757680[(unsigned int)iVar9 + (unsigned int)iVar7] = iVar14;
                                                break;
                                            }
                                            iVar7 = (int)((unsigned int)iVar7 + 1u);
                                        } while (iVar7 < iVar8);
                                    }
                                }
                                else {
                                    LB();
                                    DAT_00758624 = uBfb6;
                                    DAT_00758620 = 0x50;
                                    DAT_0075862c = -1;
                                    DAT_00758628 = iVar12;
                                    FUN_004cb830(0, &DAT_00758620, 0x14);
                                    DAT_00758628 = 0;
                                    LB();
                                    if (0 < DAT_007579d0[uBfb6]) {
                                        do {
                                            LB();
                                            if ((unsigned int)DAT_00757870[uBfb6 * 8u + (unsigned int)DAT_00758628] == DAT_007579f8) break;
                                            DAT_00758628 = (int)((unsigned int)DAT_00758628 + 1u);
                                            LB();
                                        } while (DAT_00758628 < DAT_007579d0[uBfb6]);
                                    }
                                    LB();
                                    DAT_00758624 = uBfb6;
                                    DAT_00758620 = 0x50;
                                    DAT_0075862c = iVar14;
                                    FUN_004cb830(0, &DAT_00758620, 0x10);
                                    DAT_00758620 = 0x6a;
                                    DAT_00758624 = DAT_007579f8;
                                    FUN_004cb830(0, &DAT_00758620, 8);
                                }
                                goto LAB_004f57a9;
                            }
                        }
                    }
                }
                FUN_004dd030(local_20, 0, 0x11);
                FUN_004dd030(local_34, 0, 0x11);
                if (3 < iVar16) {
                    return 1;
                }
            }
            iVar14 = (int)((unsigned int)iVar14 + 1u);
            local_48 = local_48 + 0x562;
        } while (iVar14 < DAT_0075766c);
    }
    return 1;
}
