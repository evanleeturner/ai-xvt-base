/* SPDX-License-Identifier: MIT */
/* z_xvt 004eaed0 */
extern unsigned int DAT_0081bcd0;
extern unsigned short DAT_0081b6fc;
extern unsigned char DAT_0081bfba[4];   /* unaligned int at 0x81bfba */
extern unsigned char DAT_0081bfb6[4];   /* unaligned int at 0x81bfb6 */
extern unsigned char DAT_00757cee[];    /* unaligned records, stride 0x2a; name at -0xe */
extern int DAT_00743860[8];
extern int DAT_007579d0[];
extern int DAT_00757870[];
extern int DAT_00757680[];
extern int DAT_00757a28;
extern int DAT_0081b850;
extern unsigned char DAT_0081bc80[];
extern char DAT_0081b740[];
extern char DAT_005321c4[];
extern char s__c_s__c_s_00531334[];
extern char s__s__s_00533618[];

extern int _sprintf(char *, const char *, ...);
int FUN_004cde00(void);
char *FUN_004dcfa0(int);
void FUN_004da7c0(int, char *, int, int, unsigned int);
int FUN_004cdbc0(void);
void FUN_004dcfd0(unsigned int *, int, int, int, int);
void FUN_004d5180(unsigned int *);
void FUN_004d51b0(unsigned int *);
unsigned int FUN_004ecce0(int);
int FUN_004ec8d0(int);
char FUN_004eca20(int);
char FUN_004ecba0(int);
void FUN_004daac0(int, char *, unsigned int *, int, int, unsigned int);

void __cdecl FUN_004eaed0(int param_1)
{
    char cVar1;
    int iVar2;
    char *pcVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned char *piVar7;
    int iVar8;
    unsigned short uVar9;
    int bVar10;
    int iVar12;
    int iVar13;
    int iVar14;
    int iVal0;
    int iVal1;
    int iBfba;
    int iBfb6;
    int *local_54;
    unsigned char *local_50;
    int *local_4c;
    int local_44;
    int *local_40;
    int *local_3c;
    int *local_38;
    int local_34;
    unsigned int local_30[4];
    unsigned int local_20[4];
    unsigned int local_10[4];
    char *pName;
    int *pScan;

    iVar2 = FUN_004cde00();
    iVar2 = (int)(0u - (unsigned int)(((int)((unsigned int)iVar2 * 0xfu)) >> 1));
    iVar8 = (int)((unsigned int)iVar2 + 0x155u);
    iVar12 = 0x58;
    iVar13 = iVar8;
    uVar6 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(0xae);
    FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar6);
    iVar12 = 0xe4;
    iVar13 = iVar8;
    uVar6 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(0x232);
    FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar6);
    iVar12 = 0x152;
    iVar13 = iVar8;
    uVar6 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(0x233);
    FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar6);
    iVar12 = 0x17a;
    iVar13 = iVar8;
    uVar6 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(0x234);
    FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar6);
    iVar13 = 0x193;
    uVar6 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(0x235);
    iVar2 = (int)((unsigned int)iVar2 + 0x164u);
    FUN_004da7c0(0xc, pcVar3, iVar13, iVar8, uVar6);
    FUN_004cde00();
    __builtin_memcpy(&iBfba, DAT_0081bfba, 4);
    if ((iBfba == 0) || (iBfba == 3) || (iBfba == 4)) {
        /* loop 1 */
        iVar13 = 0;
        piVar7 = DAT_00757cee;
        local_54 = DAT_00743860;
        do {
            __builtin_memcpy(&iVal0, piVar7, 4);
            iVar8 = iVal0;
            if ((iVar8 != 0) && ((iVar12 = FUN_004cdbc0()) == iVar8)) {
                __builtin_memcpy(&iVal1, piVar7 + 4, 4);
                uVar9 = DAT_0081b6fc;
                if (*local_54 == 0) {
                    uVar9 = (unsigned short)DAT_0081bcd0;
                }
                uVar5 = (unsigned int)uVar9;
                FUN_004dcfd0(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
                FUN_004d5180(local_20);
                FUN_004d51b0(local_30);
                pName = (char *)(piVar7 - 0xe);
                uVar6 = uVar5;
                if (*local_54 == 0) {
                    pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                    _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, 1, pName);
                    iVar8 = FUN_004cdbc0();
                    if ((iVar8 == iVal0) || (DAT_0081b850 == 2)) {
                        uVar6 = *(unsigned int *)(DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
                    }
                } else {
                    pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                    _sprintf(DAT_0081b740, s__s__s_00533618, pcVar3, pName);
                }
                FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar2, uVar6);
                FUN_004d51b0(local_20);
                iVar12 = 0xe4;
                iVar8 = iVar2;
                uVar6 = uVar5;
                uVar4 = FUN_004ecce0(iVar13);
                pcVar3 = FUN_004dcfa0((int)(uVar4 + 0x15u));
                FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar6);
                iVar14 = 0x152;
                iVar8 = iVar2;
                uVar6 = uVar5;
                iVar12 = FUN_004ec8d0(iVar13);
                pcVar3 = FUN_004dcfa0((int)((unsigned int)iVar12 + 0x236u));
                FUN_004da7c0(0xc, pcVar3, iVar14, iVar8, uVar6);
                iVar12 = 0x17a;
                iVar8 = iVar2;
                uVar6 = uVar5;
                cVar1 = FUN_004eca20(iVar13);
                pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x240u));
                FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar6);
                iVar12 = 0x193;
                iVar8 = iVar2;
                cVar1 = FUN_004ecba0(iVar13);
                iVar2 = (int)((unsigned int)iVar2 + 0xfu);
                pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x245u));
                FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar5);
            }
            piVar7 = piVar7 + 0x2a;
            local_54 = local_54 + 1;
            iVar13 = (int)((unsigned int)iVar13 + 1u);
        } while (local_54 < DAT_00743860 + 8);

        /* loop 2 */
        iVar13 = 0;
        piVar7 = DAT_00757cee;
        local_4c = DAT_00743860;
        do {
            __builtin_memcpy(&iVal0, piVar7, 4);
            iVar8 = iVal0;
            if ((iVar8 != 0) && ((iVar12 = FUN_004cdbc0()) != iVar8)) {
                __builtin_memcpy(&iBfb6, DAT_0081bfb6, 4);
                iVar12 = 0;
                iVar8 = DAT_007579d0[iBfb6];
                if (0 < iVar8) {
                    do {
                        if (DAT_00757870[(int)((unsigned int)iVar12 + (unsigned int)iBfb6 * 8u)] == iVal0) break;
                        iVar12 = (int)((unsigned int)iVar12 + 1u);
                    } while (iVar12 < iVar8);
                }
                if (iVar8 != iVar12) {
                    __builtin_memcpy(&iVal1, piVar7 + 4, 4);
                    uVar9 = DAT_0081b6fc;
                    if (*local_4c == 0) {
                        uVar9 = (unsigned short)DAT_0081bcd0;
                    }
                    uVar5 = (unsigned int)uVar9;
                    FUN_004dcfd0(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
                    FUN_004d5180(local_20);
                    FUN_004d51b0(local_30);
                    pName = (char *)(piVar7 - 0xe);
                    uVar6 = uVar5;
                    if (*local_4c == 0) {
                        pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                        _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, 1, pName);
                        iVar8 = FUN_004cdbc0();
                        if ((iVar8 == iVal0) || (DAT_0081b850 == 2)) {
                            uVar6 = *(unsigned int *)(DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
                        }
                    } else {
                        pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                        _sprintf(DAT_0081b740, s__s__s_00533618, pcVar3, pName);
                    }
                    FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar2, uVar6);
                    FUN_004d51b0(local_20);
                    iVar12 = 0xe4;
                    iVar8 = iVar2;
                    uVar6 = uVar5;
                    uVar4 = FUN_004ecce0(iVar13);
                    pcVar3 = FUN_004dcfa0((int)(uVar4 + 0x15u));
                    FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar6);
                    iVar14 = 0x152;
                    iVar8 = iVar2;
                    uVar6 = uVar5;
                    iVar12 = FUN_004ec8d0(iVar13);
                    pcVar3 = FUN_004dcfa0((int)((unsigned int)iVar12 + 0x236u));
                    FUN_004da7c0(0xc, pcVar3, iVar14, iVar8, uVar6);
                    iVar12 = 0x17a;
                    iVar8 = iVar2;
                    uVar6 = uVar5;
                    cVar1 = FUN_004eca20(iVar13);
                    pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x240u));
                    FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar6);
                    iVar12 = 0x193;
                    iVar8 = iVar2;
                    cVar1 = FUN_004ecba0(iVar13);
                    pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x245u));
                    iVar2 = (int)((unsigned int)iVar2 + 0xfu);
                    FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar5);
                }
            }
            piVar7 = piVar7 + 0x2a;
            local_4c = local_4c + 1;
            iVar13 = (int)((unsigned int)iVar13 + 1u);
        } while (local_4c < DAT_00743860 + 8);

        /* loop 3 */
        local_54 = DAT_00743860;
        piVar7 = DAT_00757cee;
        do {
            __builtin_memcpy(&iVal0, piVar7, 4);
            iVar13 = iVal0;
            if ((iVar13 != 0) && ((iVar8 = FUN_004cdbc0()) != iVar13)) {
                __builtin_memcpy(&iBfb6, DAT_0081bfb6, 4);
                iVar8 = 0;
                iVar13 = DAT_007579d0[iBfb6];
                bVar10 = (iVar13 == 0);
                if (0 < iVar13) {
                    do {
                        if (DAT_00757870[(int)((unsigned int)iVar8 + (unsigned int)iBfb6 * 8u)] == iVal0) break;
                        iVar8 = (int)((unsigned int)iVar8 + 1u);
                    } while (iVar8 < iVar13);
                    bVar10 = (iVar13 == iVar8);
                }
                if (bVar10) {
                    __builtin_memcpy(&iVal1, piVar7 + 4, 4);
                    uVar9 = DAT_0081b6fc;
                    if (*local_54 == 0) {
                        uVar9 = (unsigned short)DAT_0081bcd0;
                    }
                    uVar5 = (unsigned int)uVar9;
                    FUN_004dcfd0(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
                    FUN_004d5180(local_20);
                    FUN_004d51b0(local_30);
                    pName = (char *)(piVar7 - 0xe);
                    uVar6 = uVar5;
                    if (*local_54 == 0) {
                        pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                        _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, 1, pName);
                        iVar13 = FUN_004cdbc0();
                        if ((iVar13 == iVal0) || (DAT_0081b850 == 2)) {
                            uVar6 = *(unsigned int *)(DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
                        }
                    } else {
                        pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                        _sprintf(DAT_0081b740, s__s__s_00533618, pcVar3, pName);
                    }
                    FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar2, uVar6);
                    FUN_004d51b0(local_20);
                    FUN_004dcfd0(local_10, 0xe4, iVar2, 0x193, (int)((unsigned int)iVar2 + 0xeu));
                    iVar8 = 0;
                    iVar13 = 1;
                    pcVar3 = FUN_004dcfa0(0x249);
                    iVar2 = (int)((unsigned int)iVar2 + 0xfu);
                    FUN_004daac0(0xc, pcVar3, local_10, iVar13, iVar8, uVar5);
                }
            }
            piVar7 = piVar7 + 0x2a;
            local_54 = local_54 + 1;
            if ((int *)0x74387f < local_54) {
                return;
            }
        } while (1);
    }

    /* else branch: loop 1 */
    iVar13 = 0;
    piVar7 = DAT_00757cee;
    local_54 = DAT_00743860;
    do {
        __builtin_memcpy(&iVal0, piVar7, 4);
        iVar8 = iVal0;
        if ((iVar8 != 0) && ((iVar12 = FUN_004cdbc0()) == iVar8)) {
            __builtin_memcpy(&iVal1, piVar7 + 4, 4);
            uVar9 = DAT_0081b6fc;
            if (*local_54 == 0) {
                uVar9 = (unsigned short)DAT_0081bcd0;
            }
            uVar5 = (unsigned int)uVar9;
            FUN_004dcfd0(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
            FUN_004d5180(local_20);
            FUN_004d51b0(local_30);
            pName = (char *)(piVar7 - 0xe);
            uVar6 = uVar5;
            if (*local_54 == 0) {
                pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, 1, pName);
                iVar8 = FUN_004cdbc0();
                if ((iVar8 == iVal0) || (DAT_0081b850 == 2)) {
                    uVar6 = *(unsigned int *)(DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
                }
            } else {
                pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                _sprintf(DAT_0081b740, s__s__s_00533618, pcVar3, pName);
            }
            FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar2, uVar6);
            FUN_004d51b0(local_20);
            iVar12 = 0xe4;
            iVar8 = iVar2;
            uVar6 = uVar5;
            uVar4 = FUN_004ecce0(iVar13);
            pcVar3 = FUN_004dcfa0((int)(uVar4 + 0x15u));
            FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar6);
            iVar14 = 0x152;
            iVar8 = iVar2;
            uVar6 = uVar5;
            iVar12 = FUN_004ec8d0(iVar13);
            pcVar3 = FUN_004dcfa0((int)((unsigned int)iVar12 + 0x236u));
            FUN_004da7c0(0xc, pcVar3, iVar14, iVar8, uVar6);
            iVar12 = 0x17a;
            iVar8 = iVar2;
            uVar6 = uVar5;
            cVar1 = FUN_004eca20(iVar13);
            pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x240u));
            FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar6);
            iVar12 = 0x193;
            iVar8 = iVar2;
            cVar1 = FUN_004ecba0(iVar13);
            iVar2 = (int)((unsigned int)iVar2 + 0xfu);
            pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x245u));
            FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar5);
        }
        piVar7 = piVar7 + 0x2a;
        local_54 = local_54 + 1;
        iVar13 = (int)((unsigned int)iVar13 + 1u);
    } while (local_54 < DAT_00743860 + 8);

    /* else branch: loop 2 */
    local_44 = 0;
    local_50 = DAT_00757cee;
    local_40 = DAT_00743860;
    do {
        __builtin_memcpy(&iVal0, local_50, 4);
        iVar8 = iVal0;
        iVar13 = iVar2;
        if ((iVar8 != 0) && ((iVar12 = FUN_004cdbc0()) != iVar8)) {
            __builtin_memcpy(&iVal1, local_50 + 4, 4);
            uVar9 = DAT_0081b6fc;
            if (*local_40 == 0) {
                uVar9 = (unsigned short)DAT_0081bcd0;
            }
            uVar6 = (unsigned int)uVar9;
            iVar13 = 0;
            bVar10 = 0;
            local_34 = 0;
            if (0 < DAT_00757a28) {
                local_3c = DAT_00757870;
                local_38 = DAT_007579d0;
                do {
                    iVar12 = 0;
                    iVar8 = *local_38;
                    if (0 < iVar8) {
                        pScan = local_3c;
                        do {
                            if (*pScan == iVal0) {
                                if (DAT_00757680[(int)((unsigned int)iVar13 + (unsigned int)iVar12)] == -1) {
                                    bVar10 = 1;
                                }
                                break;
                            }
                            pScan = pScan + 1;
                            iVar12 = (int)((unsigned int)iVar12 + 1u);
                        } while (iVar12 < iVar8);
                        if (iVar12 < iVar8) break;
                    }
                    iVar13 = (int)((unsigned int)iVar13 + 8u);
                    local_3c = local_3c + 8;
                    local_38 = local_38 + 1;
                    local_34 = (int)((unsigned int)local_34 + 1u);
                } while (local_34 < DAT_00757a28);
            }
            iVar13 = (int)((unsigned int)iVar2 + 0xfu);
            FUN_004dcfd0(local_30, 0x58, iVar2, 0xe2, iVar13);
            FUN_004d5180(local_20);
            FUN_004d51b0(local_30);
            uVar5 = uVar6;
            pName = (char *)(local_50 - 0xe);
            if (*local_40 == 0) {
                pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar3, 1, pName);
                iVar8 = FUN_004cdbc0();
                if ((iVar8 == iVal0) || (DAT_0081b850 == 2)) {
                    uVar5 = *(unsigned int *)(DAT_0081bc80 + (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
                }
            } else {
                pcVar3 = FUN_004dcfa0((int)((unsigned int)iVal1 + 0x8eu));
                _sprintf(DAT_0081b740, s__s__s_00533618, pcVar3, pName);
            }
            FUN_004da7c0(0xc, DAT_0081b740, 0x58, iVar2, uVar5);
            FUN_004d51b0(local_20);
            if (bVar10) {
                FUN_004da7c0(0xc, DAT_005321c4, 0xe4, iVar2, uVar6);
                FUN_004da7c0(0xc, DAT_005321c4, 0x152, iVar2, uVar6);
                FUN_004da7c0(0xc, DAT_005321c4, 0x17a, iVar2, uVar6);
                iVar8 = 0x193;
                pcVar3 = DAT_005321c4;
            } else {
                iVar12 = 0xe4;
                iVar8 = iVar2;
                uVar5 = uVar6;
                uVar4 = FUN_004ecce0(local_44);
                pcVar3 = FUN_004dcfa0((int)(uVar4 + 0x15u));
                FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar5);
                iVar14 = 0x152;
                iVar8 = iVar2;
                uVar5 = uVar6;
                iVar12 = FUN_004ec8d0(local_44);
                pcVar3 = FUN_004dcfa0((int)((unsigned int)iVar12 + 0x236u));
                FUN_004da7c0(0xc, pcVar3, iVar14, iVar8, uVar5);
                iVar12 = 0x17a;
                iVar8 = iVar2;
                uVar5 = uVar6;
                cVar1 = FUN_004eca20(local_44);
                pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x240u));
                FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar5);
                iVar8 = 0x193;
                cVar1 = FUN_004ecba0(local_44);
                pcVar3 = FUN_004dcfa0((int)((unsigned int)(unsigned char)cVar1 + 0x245u));
            }
            FUN_004da7c0(0xc, pcVar3, iVar8, iVar2, uVar6);
        }
        local_50 = local_50 + 0x2a;
        local_40 = local_40 + 1;
        local_44 = (int)((unsigned int)local_44 + 1u);
        iVar2 = iVar13;
        if ((int *)0x74387f < local_40) {
            return;
        }
    } while (1);
}
