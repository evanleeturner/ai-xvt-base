/* SPDX-License-Identifier: MIT */
/* z_xvt 00444630 */
extern int DAT_00525550;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int _DAT_00732a3c;
extern int DAT_0071ecf4;
extern unsigned char DAT_007092c0;
extern short _DAT_00732950;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b740[];
extern unsigned char DAT_0071e528[];
extern unsigned char DAT_0070b252[];
extern int DAT_0070aaf0[];
extern int DAT_0070ab10[];

void  FUN_0041e890(unsigned short a);
int   FUN_00423200(int a, int b, int c, int d);
void  FUN_004441e0(int a);
int  *FUN_00432cc0(int a);
void  FUN_004a3490(int a, int *b);
void  FUN_004a1d70(int a, int b);
void  FUN_004a41d0(int a);
void  FUN_00423890(int a, int b, int c);
void  FUN_00438970(void);
int   FUN_00432ca0(void);
void  FUN_0042c670(int a);
int   FUN_00431d30(void);
void  FUN_0042c580(void);

void __stdcall FUN_00444630(void)
{
    char cVar1;
    int iVar2;
    int *piVar3;
    unsigned char *pcVar4;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned char *piVar9;
    unsigned char *rec;
    unsigned int tmp32;
    unsigned int tmp32b;
    unsigned short us;
    int *piVarA;
    int val;

    iVar8 = 0;
    piVar9 = DAT_0071b730;
    do {
        if (piVar9[0x10] != 0) {
            if (piVar9[0x11] == 0) {
                if ((piVar9[0x13] != 0) && (piVar9[0x10] != 2) && (DAT_00525550 == 0)) {
                    iVar2 = -1;
                    for (iVar6 = (int)((unsigned int)DAT_006d4980 - (unsigned int)_DAT_00732a3c); iVar6 != 0; iVar6 = iVar6 - 1) {
                        iVar2 = iVar2 + 1;
                        if (DAT_006d4980 <= iVar2) {
                            iVar2 = _DAT_00732a3c;
                        }
                        iVar7 = (int)((unsigned int)iVar2 * 0x23u + (unsigned int)DAT_006d4988);
                        rec = (unsigned char *)iVar7;
                        if (rec[3] != 0) {
                            __builtin_memcpy(&val, DAT_0070b252 + (unsigned int)rec[0x16] * 0x566, 4);
                            if (val == iVar8) {
                                __builtin_memcpy(&tmp32, rec + 0x1f, 4);
                                __builtin_memcpy(&tmp32b, (unsigned char *)(tmp32 + 0xa7), 4);
                                cVar1 = *(char *)(tmp32b + 7);
                                if (cVar1 != 3 && cVar1 != 5 && cVar1 != 4) break;
                            }
                        }
                    }
                    if (iVar6 == 0) {
                        __builtin_memcpy(&us, piVar9 + 0xe, 2);
                        FUN_0041e890(us);
                        iVar2 = -1;
                        for (iVar6 = (int)((unsigned int)DAT_006d4980 - (unsigned int)_DAT_00732a3c); iVar6 != 0; iVar6 = iVar6 - 1) {
                            iVar2 = iVar2 + 1;
                            if (DAT_006d4980 <= iVar2) {
                                iVar2 = _DAT_00732a3c;
                            }
                            iVar7 = (int)((unsigned int)iVar2 * 0x23u + (unsigned int)DAT_006d4988);
                            rec = (unsigned char *)iVar7;
                            if (rec[3] != 0) {
                                __builtin_memcpy(&val, DAT_0070b252 + (unsigned int)rec[0x16] * 0x566, 4);
                                if (val == iVar8) {
                                    __builtin_memcpy(&tmp32, rec + 0x1f, 4);
                                    __builtin_memcpy(&tmp32b, (unsigned char *)(tmp32 + 0xa7), 4);
                                    cVar1 = *(char *)(tmp32b + 7);
                                    if (cVar1 != 3 && cVar1 != 5 && cVar1 != 4) break;
                                }
                            }
                        }
                        if (iVar6 == 0) {
                            FUN_004441e0(iVar8);
                            if (iVar8 != DAT_0071ecf4) {
                                piVar3 = FUN_00432cc0(iVar8);
                                FUN_004a3490(0, piVar3);
                                FUN_004a1d70(0x173, DAT_0071ecf4);
                                iVar2 = 0;
                                pcVar4 = DAT_0071b740;
                                do {
                                    if (*pcVar4 == 1) {
                                        iVar2 = iVar2 + 1;
                                    }
                                    pcVar4 = pcVar4 + 0x5bd;
                                } while (pcVar4 < DAT_0071e528);
                                if (DAT_0071b740[(int)((unsigned int)DAT_0071ecf4 * 0x5bdu)] == 1) {
                                    if (iVar2 != 1) goto LAB_00444929;
                                    iVar2 = 0x175;
                                    goto LAB_0044493b;
                                }
                            }
                        }
                    }
                }
            }
            else if (DAT_00525550 == 0) {
                int first;
                __builtin_memcpy(&first, piVar9, 4);
                if (first != -1) {
                    int second;
                    rec = (unsigned char *)(int)((unsigned int)first * 0x23u + (unsigned int)DAT_006d4988);
                    __builtin_memcpy(&second, piVar9 + 4, 4);
                    __builtin_memcpy(&us, rec, 2);
                    if (rec[3] == 0 || (unsigned int)second != (unsigned int)us) {
                        __builtin_memcpy(&us, piVar9 + 0xe, 2);
                        FUN_0041e890(us);
                        iVar2 = FUN_00423200(iVar8, (int)0xffffffff, 0, 0);
                        if (iVar2 == 0) {
                            if (iVar8 == DAT_0071ecf4) {
                                FUN_004a41d0(0x118);
                            }
                        }
                        else {
                            FUN_004441e0(iVar8);
                            if (iVar8 != DAT_0071ecf4) {
                                piVar3 = FUN_00432cc0(iVar8);
                                FUN_004a3490(0, piVar3);
                                FUN_004a1d70(0x173, DAT_0071ecf4);
                                iVar2 = 0;
                                pcVar4 = DAT_0071b740;
                                do {
                                    if (*pcVar4 == 1) {
                                        iVar2 = iVar2 + 1;
                                    }
                                    pcVar4 = pcVar4 + 0x5bd;
                                } while (pcVar4 < DAT_0071e528);
                                if (DAT_0071b740[(int)((unsigned int)DAT_0071ecf4 * 0x5bdu)] == 1) {
                                    if (iVar2 == 1) {
                                        iVar2 = 0x175;
                                    }
                                    else {
LAB_00444929:
                                        _DAT_00732950 = (short)iVar2;
                                        iVar2 = 0x174;
                                    }
LAB_0044493b:
                                    FUN_004a1d70(iVar2, DAT_0071ecf4);
                                }
                            }
                        }
                    }
                }
            }
        }
        piVar9 = piVar9 + 0x5bd;
        iVar8 = iVar8 + 1;
        if ((unsigned char *)0x71e517 < piVar9) {
            if (DAT_00525550 == 0) {
                iVar8 = 0;
                pcVar4 = DAT_0071b740;
                piVarA = DAT_0070aaf0;
                do {
                    if ((*piVarA != 0) && (*pcVar4 != 0)) {
                        int a;
                        int b;
                        __builtin_memcpy(&a, pcVar4 - 0x10, 4);
                        if (a != -1) {
                            FUN_00423890(iVar8, 0, 1);
                        }
                        if (iVar8 == DAT_0071ecf4) {
                            DAT_007092c0 = 1;
                        }
                        *pcVar4 = 0;
                        FUN_00438970();
                        iVar2 = FUN_00432ca0();
                        __builtin_memcpy(&b, pcVar4 + 0x578, 4);
                        if (iVar2 != b) {
                            FUN_0042c670(iVar8);
                        }
                        piVar3 = FUN_00432cc0(iVar8);
                        FUN_004a3490(0, piVar3);
                        FUN_004a1d70(0x172, DAT_0071ecf4);
                        if ((iVar8 == DAT_0071ecf4) && (iVar2 = FUN_00431d30(), iVar2 != 0)) {
                            FUN_0042c580();
                        }
                    }
                    pcVar4 = pcVar4 + 0x5bd;
                    piVarA = piVarA + 1;
                    iVar8 = iVar8 + 1;
                } while (piVarA < DAT_0070ab10);
            }
            return;
        }
    } while (1);
}
