/* SPDX-License-Identifier: MIT */
/* z_xvt 004bd330 */
extern int DAT_0064ef50;
extern int DAT_0064ef54;
extern int DAT_0064ef58;
extern int DAT_0064ef5c;
extern unsigned int DAT_0081bcb0;
extern unsigned int DAT_0081bcd0;
extern int DAT_00859c2c;
extern int DAT_0085e6c0;
extern char DAT_0085e7c5;
extern char DAT_0081b740[];
extern char DAT_00859c40[];
extern char DAT_00859c41[];
extern char DAT_00859c55[];
extern char DAT_0085e7a0[];
extern char DAT_0085e7b0[];
extern char DAT_005319c4[];
extern char s__c_s__d__c_s_005319d8[];
extern char s__c_s__s__c_s_005319e8[];
extern char s__c_s___c_s_005319cc[];
extern char s__s__d___c_s_005319b8[];
extern char s__s__s___c_s_005319ac[];
extern char s_configsound_0053138c[];
extern char s_settingsound_0053199c[];

void __cdecl FUN_004ddc70(int *param_1, int *param_2);
void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004dcfa0(unsigned int param_1);
void __cdecl FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4);
int __cdecl _sprintf(char *, const char *, ...);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
unsigned char __cdecl FUN_004df360(int param_1);
unsigned long long __cdecl FUN_004df2c0(int param_3);
void __cdecl FUN_004de190(char *param_1, int param_2, int param_3, int param_4);
unsigned char __cdecl FUN_004df3a0(int param_1);
unsigned char __cdecl FUN_004df380(int param_1);
int __cdecl FUN_004d9010(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7);
void __cdecl FUN_004dd3e0(int *param_1, int param_2, int param_3, int param_4, int param_5);
void __cdecl FUN_004dd070(int *param_1, int param_2, int param_3);
int __cdecl FUN_004dd7d0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004dd570(int *param_1, int param_2, int param_3, short param_4);
unsigned char __cdecl FUN_004db570(void);
unsigned char __cdecl FUN_004db590(void);
unsigned int __cdecl FUN_004bdea0(void);

void __cdecl
FUN_004bd330(unsigned int param_1, unsigned int param_2, int *param_3, unsigned int param_4,
             int *param_5, unsigned int param_6, char *param_7, char *param_8, int *param_9,
             int *param_10, int param_11, unsigned char *param_12, int param_13)
{
    int iVar1;
    unsigned char uVar2;
    char *pcVar3;
    char *pcVar4;
    char *pcVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned char *puVar9;
    unsigned long long uVar10;
    char *_Format;
    int *piVar11;
    int iVar12;
    unsigned int uVar13;
    int iVar14;
    unsigned int uVar15;
    int local_1c[4];
    int iStack_c;
    int local_8;
    int local_4;

    FUN_004ddc70(&local_4, &local_8);
    FUN_004dcfd0(local_1c, 0x54, 0x5a, 0x25c, 0x6a);
    piVar11 = local_1c;
    uVar15 = 0xffff;
    pcVar3 = FUN_004dcfa0(0x192);
    FUN_004da920(0xf, pcVar3, piVar11, uVar15);
    FUN_004dcfd0(local_1c, 0x58, 0x6f, 0x142, 0x7d);
    if (DAT_0064ef58 < 0x10) {
        pcVar4 = (char *)((unsigned int)DAT_0064ef58 + 1u);
        pcVar3 = FUN_004dcfa0(0x194);
        uVar13 = 1;
        pcVar5 = FUN_004dcfa0(0x193);
        _Format = s__c_s__d__c_s_005319d8;
    } else {
        uVar15 = (unsigned int)DAT_0064ef58 + 0x269u;
        pcVar3 = FUN_004dcfa0(0x194);
        uVar13 = 1;
        pcVar4 = FUN_004dcfa0(uVar15);
        pcVar5 = FUN_004dcfa0(0x278);
        _Format = s__c_s__s__c_s_005319e8;
    }
    _sprintf(DAT_0081b740, _Format, 4, pcVar5, pcVar4, uVar13, pcVar3);
    FUN_004daac0(0xc, DAT_0081b740, local_1c, 0, 1, 0xffff);
    FUN_004dd030(local_1c, 0, 0xf);
    if (DAT_0085e6c0 != 0) {
        if (DAT_0064ef5c != 0) {
            _sprintf(DAT_0081b740, s__c_s___c_s_005319cc, 2,
                     (int)((unsigned int)DAT_00859c41 + (unsigned int)DAT_0064ef5c * 0x95u), 1,
                     (int)((unsigned int)DAT_00859c55 + (unsigned int)DAT_0064ef5c * 0x95u));
        } else {
            _sprintf(DAT_0081b740, DAT_005319c4, 2, DAT_00859c41);
        }
        FUN_004daac0(0xc, DAT_0081b740, local_1c, 0, 1, 0xffff);
    }
    FUN_004dd030(local_1c, 0, 0x1e);
    piVar11 = local_1c;
    iVar14 = 1;
    iVar12 = 0;
    uVar15 = DAT_0081bcb0;
    pcVar3 = FUN_004dcfa0(0x27f);
    FUN_004daac0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
    FUN_004dd030(local_1c, 0, 0xf);
    piVar11 = local_1c;
    iVar14 = 1;
    iVar12 = 0;
    uVar15 = DAT_0081bcb0;
    pcVar3 = FUN_004dcfa0(0x280);
    FUN_004daac0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
    FUN_004dd030(local_1c, 0, 0xf);
    piVar11 = local_1c;
    iVar14 = 1;
    iVar12 = 0;
    uVar15 = DAT_0081bcb0;
    pcVar3 = FUN_004dcfa0(0x281);
    FUN_004daac0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
    FUN_004dd030(local_1c, 0, 0xf);
    piVar11 = local_1c;
    iVar14 = 1;
    iVar12 = 0;
    uVar15 = DAT_0081bcb0;
    pcVar3 = FUN_004dcfa0(0x282);
    FUN_004daac0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
    FUN_004dd030(local_1c, 0, 0xf);
    piVar11 = local_1c;
    iVar14 = 1;
    iVar12 = 0;
    uVar15 = DAT_0081bcb0;
    pcVar3 = FUN_004dcfa0(0x283);
    FUN_004daac0(0xc, pcVar3, piVar11, iVar12, iVar14, uVar15);
    uVar2 = FUN_004df360(0);
    iVar12 = (int)uVar2;
    uVar10 = FUN_004df2c0(0);
    iVar14 = (int)uVar10;
    if (((iVar14 != -1) && (iVar14 < 0x10)) || (iVar12 != 0)) {
        if (iVar14 == -1) {
            if (iVar12 != 0) {
                if (((unsigned int)iVar12 - (unsigned int)DAT_0064ef58 != 0xfffffff1u) && (DAT_0085e7c5 != '\0')) {
                    FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
                }
                DAT_0064ef58 = (int)((unsigned int)iVar12 + 0xfu);
                uVar2 = FUN_004df3a0(0);
                iVar12 = (int)((unsigned int)iVar12 - 1u + (unsigned int)uVar2);
                if ((iVar12 < DAT_0064ef54) ||
                    (9 < (int)((unsigned int)iVar12 - (unsigned int)DAT_0064ef54))) {
                    DAT_0064ef54 = iVar12;
                }
            }
        } else {
            if ((iVar14 != DAT_0064ef58) && (DAT_0085e7c5 != '\0')) {
                FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
            }
            DAT_0064ef58 = iVar14;
            if ((iVar14 < DAT_0064ef54) ||
                (9 < (int)((unsigned int)iVar14 - (unsigned int)DAT_0064ef54))) {
                DAT_0064ef54 = iVar14;
            }
        }
        iVar12 = 0;
        if (0 < DAT_0085e6c0) {
            pcVar3 = DAT_00859c40;
            do {
                if (*pcVar3 == DAT_0085e7a0[DAT_0064ef58]) {
                    DAT_0064ef5c = iVar12;
                }
                pcVar3 = pcVar3 + 0x95;
                iVar12 = (int)((unsigned int)iVar12 + 1u);
            } while (iVar12 < DAT_0085e6c0);
        }
    }
    uVar2 = FUN_004df3a0(0);
    iVar12 = (int)uVar2;
    iStack_c = iVar12;
    uVar2 = FUN_004df380(0);
    if ((int)uVar2 != 0) {
        iStack_c = (int)((unsigned int)iVar12 + 4u);
    }
    FUN_004dcfd0(local_1c, 0x58, 0xf6, 0x142, 0x104);
    piVar11 = local_1c;
    uVar15 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(0x27e);
    FUN_004da920(0xc, pcVar3, piVar11, uVar15);
    uVar13 = 0x142;
    if (10 < iStack_c) {
        FUN_004dcfd0(local_1c, 0x139, 0x105, 0x142, 0x19f);
        DAT_0064ef54 = FUN_004d9010(local_1c, DAT_0064ef54, iStack_c, 0, 5, DAT_00859c2c, 1);
        uVar13 = 0x138;
    }
    FUN_004dcfd0(local_1c, 0x58, 0x105, uVar13, 0x19f);
    FUN_004dd3e0(local_1c, 0, 0, 0xffff, 0);
    FUN_004dd070(local_1c, 2, 2);
    FUN_004dd3e0(local_1c, 0, 0, 0, 0);
    local_1c[3] = (int)((unsigned int)local_1c[1] + 0xeu);
    FUN_004dd070(local_1c, 2, 0);
    iVar14 = DAT_0064ef54;
    if (DAT_0064ef54 < (int)((unsigned int)DAT_0064ef54 + 10u)) {
        do {
            if (iStack_c <= iVar14) break;
            iVar8 = 0;
            if (0 < DAT_0085e6c0) {
                pcVar3 = DAT_00859c40;
                do {
                    if (iVar14 < iVar12) {
                        if (DAT_0085e7a0[iVar14] == *pcVar3) break;
                    } else if (DAT_0085e7b0[(int)((unsigned int)iVar14 - (unsigned int)iVar12)] == *pcVar3) break;
                    pcVar3 = pcVar3 + 0x95;
                    iVar8 = (int)((unsigned int)iVar8 + 1u);
                } while (iVar8 < DAT_0085e6c0);
            }
            if (iVar8 < DAT_0085e6c0) {
                iVar6 = FUN_004dd7d0(local_1c, local_4, local_8);
                iVar7 = DAT_0064ef58;
                iVar1 = DAT_0064ef5c;
                if (iVar6 != 0) {
                    FUN_004dd570(local_1c, 0, 0, (short)DAT_0081bcb0);
                    uVar2 = FUN_004db570();
                    if (uVar2 == 0) {
                        uVar2 = FUN_004db590();
                        iVar7 = DAT_0064ef58;
                        iVar1 = DAT_0064ef5c;
                    }
                    if (uVar2 != 0) {
                        iVar1 = iVar8;
                        if (iVar14 < iVar12) {
                            iVar7 = iVar14;
                            if ((iVar14 != DAT_0064ef58) && (DAT_0085e7c5 != '\0')) {
                                FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
                            }
                        } else {
                            if (((unsigned int)iVar14 - (unsigned int)DAT_0064ef58 + (unsigned int)iVar12 != 0x10u) &&
                                (DAT_0085e7c5 != '\0')) {
                                FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
                            }
                            iVar7 = (int)((unsigned int)iVar14 - (unsigned int)iVar12 + 0x10u);
                        }
                    }
                }
                DAT_0064ef5c = iVar1;
                DAT_0064ef58 = iVar7;
                if (iVar14 < iVar12) {
                    uVar15 = 0xffff;
                    if (iVar14 == iVar7) {
                        uVar15 = DAT_0081bcd0 & 0xffff;
                    }
                    pcVar3 = (char *)((unsigned int)iVar14 + 1u);
                    iVar8 = (int)((unsigned int)DAT_00859c41 + (unsigned int)iVar8 * 0x95u);
                    uVar13 = 2;
                    pcVar4 = FUN_004dcfa0(0x193);
                    pcVar5 = s__s__d___c_s_005319b8;
                } else {
                    uVar15 = 0xffff;
                    if ((unsigned int)iVar14 - (unsigned int)iVar7 - (unsigned int)iVar12 == 0xfffffff0u) {
                        uVar15 = DAT_0081bcd0 & 0xffff;
                    }
                    iVar8 = (int)((unsigned int)DAT_00859c41 + (unsigned int)iVar8 * 0x95u);
                    uVar13 = 2;
                    pcVar3 = FUN_004dcfa0((unsigned int)iVar14 - (unsigned int)iVar12 + 0x279u);
                    pcVar4 = FUN_004dcfa0(0x278);
                    pcVar5 = s__s__s___c_s_005319ac;
                }
                _sprintf(DAT_0081b740, pcVar5, pcVar4, pcVar3, uVar13, iVar8);
                FUN_004daac0(0xc, DAT_0081b740, local_1c, 0, 1, uVar15);
            }
            iVar14 = (int)((unsigned int)iVar14 + 1u);
            FUN_004dd030(local_1c, 0, 0xf);
        } while (iVar14 < (int)((unsigned int)DAT_0064ef54 + 10u));
    }
    FUN_004dcfd0(local_1c, 0x14a, 0x6f, 0x24e, 0x7d);
    piVar11 = local_1c;
    uVar15 = DAT_0081bcd0;
    pcVar3 = FUN_004dcfa0(0x27d);
    FUN_004da920(0xc, pcVar3, piVar11, uVar15);
    FUN_004dcfd0(local_1c, 0x24f, 0x7e, 600, 0x19f);
    DAT_0064ef50 = FUN_004d9010(local_1c, DAT_0064ef50, DAT_0085e6c0, 0, 5, DAT_00859c2c, 2);
    FUN_004dcfd0(local_1c, 0x14a, 0x7e, 0x24e, 0x19f);
    FUN_004dd3e0(local_1c, 0, 0, 0xffff, 0);
    FUN_004dd070(local_1c, 2, 2);
    FUN_004dd3e0(local_1c, 0, 0, 0, 0);
    local_1c[3] = (int)((unsigned int)local_1c[1] + 0xeu);
    FUN_004dd070(local_1c, 2, 0);
    if (DAT_0064ef50 < (int)((unsigned int)DAT_0064ef50 + 0x13u)) {
        puVar9 = (unsigned char *)DAT_00859c40 + (int)((unsigned int)DAT_0064ef50 * 0x95u);
        iVar12 = DAT_0064ef50;
        do {
            if (DAT_0085e6c0 <= iVar12) break;
            iVar14 = FUN_004dd7d0(local_1c, local_4, local_8);
            if (iVar14 != 0) {
                FUN_004dd570(local_1c, 0, 0, (short)DAT_0081bcb0);
                uVar2 = FUN_004db570();
                if ((uVar2 != 0) || (uVar2 = FUN_004db590(), uVar2 != 0)) {
                    if ((DAT_0064ef5c != iVar12) && (DAT_0085e7c5 != '\0')) {
                        FUN_004de190(s_settingsound_0053199c, 1, 0, 0xff);
                    }
                    DAT_0064ef5c = iVar12;
                    DAT_0085e7a0[DAT_0064ef58] = (char)*puVar9;
                }
            }
            _sprintf(DAT_0081b740, s__c_s___c_s_005319cc, 2, puVar9 + 1, 1, puVar9 + 0x15);
            uVar15 = DAT_0081bcd0;
            if (iVar12 != DAT_0064ef5c) {
                uVar15 = 0xffff;
            }
            puVar9 = puVar9 + 0x95;
            iVar12 = (int)((unsigned int)iVar12 + 1u);
            FUN_004daac0(0xc, DAT_0081b740, local_1c, 0, 1, uVar15);
            FUN_004dd030(local_1c, 0, 0xf);
        } while (iVar12 < (int)((unsigned int)DAT_0064ef50 + 0x13u));
    }
    iVar12 = 0;
    uVar15 = FUN_004bdea0();
    if (0 < DAT_0085e6c0) {
        pcVar3 = DAT_00859c40;
        iVar14 = DAT_0085e6c0;
        iVar8 = DAT_0064ef58;
        do {
            if (((char)uVar15 != '\0') && (*pcVar3 == (char)uVar15)) {
                if ((DAT_0064ef5c != iVar12) && (DAT_0085e7c5 != '\0')) {
                    FUN_004de190(s_settingsound_0053199c, 1, 0, 0xff);
                    iVar14 = DAT_0085e6c0;
                    iVar8 = DAT_0064ef58;
                }
                DAT_0064ef50 = iVar12;
                DAT_0064ef5c = iVar12;
                DAT_0085e7a0[iVar8] = *pcVar3;
            }
            pcVar3 = pcVar3 + 0x95;
            iVar12 = (int)((unsigned int)iVar12 + 1u);
        } while (iVar12 < iVar14);
    }
    return;
}
