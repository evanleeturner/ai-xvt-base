/* SPDX-License-Identifier: MIT */
/* z_bop 0049ec40 */
extern int            DAT_005235dc;
extern int            DAT_009d1148;
extern int            DAT_009a7ec0;
extern int            DAT_009a7b44;
extern int            DAT_009a806c;
extern int            DAT_009a7bac;
extern int            DAT_009a8c14;
extern int            DAT_005233d8;
extern unsigned short DAT_00a08374;
extern short          DAT_00a0a2ca[];
extern short          DAT_00a08ba4;
extern unsigned char  DAT_005233c8;
extern void         (*DAT_009d1300)(void);
extern short          DAT_00521548;
extern short          DAT_0052154c;
extern int            DAT_009ec464;
extern unsigned char  DAT_009a20ae;
extern short          DAT_005235d0;
extern short          DAT_005235d4;
extern short          DAT_005235d8;
extern unsigned short DAT_009ec5fc;
extern int            DAT_009993fc;
extern unsigned short DAT_009d6930;
extern unsigned short DAT_00622bbc;
extern short          DAT_00622bb0;
extern unsigned int   DAT_00622bb8;
extern signed char    DAT_009a807a;
extern void         (*DAT_009a6fe4)(int);
extern unsigned char  DAT_009e9683[];
extern int            DAT_009ecc34;
extern void         (*DAT_009eca20)(int, int, int, int, int);
extern char           s_BJFNREBRJRFNJN_005240a0[];
extern char           s_RJFNJN_005240b0[];
extern unsigned char  DAT_00524078[];
extern char           s_00_00_00527748[];
extern char           s_00_00_00_00527750[];

void __cdecl FUN_0040e0c0(int a, int b, int c, int d);
void __cdecl FUN_004a95c0(unsigned int a);
void __cdecl FUN_004a9520(short a, short b, short c, short d);
unsigned int __stdcall FUN_00480530(void);
void __cdecl FUN_004a9620(int a);
void __cdecl FUN_004a9630(int a);
void __cdecl FUN_004a9640(char a);
int __cdecl FUN_004a9590(unsigned int a);
int __cdecl FUN_004ac720(int a);
void __stdcall FUN_004ac740(void);
void __cdecl FUN_004a9500(short a, short b);
int __cdecl FUN_0049f330(int a);
/* RET 0 (caller cleanup); one stack argument. Ghidra's ECX/EDX inputs are
   unresolved register state and are not expressible in C. */
unsigned long long __cdecl FUN_004278c0(unsigned char *c);
void __cdecl FUN_004277f0(unsigned short a, unsigned int b, unsigned int c);

short __stdcall FUN_0049ec40(void)
{
    unsigned char *pbVar1;
    unsigned char *pbVar20;
    unsigned char bVar2;
    unsigned char bVar17;
    unsigned int uVar13;
    int iVar14;
    int iVar19;
    int iVar22;
    unsigned long long uVar24;
    short sVar5, sVar6, sVar7, sVar8, sVar9, sVar10, sVar11, sVar12;
    /* original leaves this slot unassigned on the early-return path
       (indeterminate value); 0 is used here to avoid reading it uninitialized */
    short sStack_10 = 0;
    short sStack_a;

    sVar5 = (short)DAT_005235dc;
    sVar7 = (short)DAT_009d1148;
    sVar6 = (short)(sVar7 + 2);
    sVar9 = (short)DAT_009a7ec0;
    sVar8 = (short)(sVar9 + -2);
    sVar11 = (short)DAT_009a7b44;
    sVar10 = (short)(sVar11 + -2);

    FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                 (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));

    if (*(short *)((char *)DAT_00a0a2ca + (unsigned int)DAT_00a08374 * 2) != DAT_00a08ba4) {
        FUN_004a95c0((unsigned int)DAT_005233c8);
        FUN_004a9520((short)DAT_005235dc, (short)DAT_009d1148, (short)DAT_009a7ec0, (short)DAT_009a7b44);
        DAT_009d1300();
        if (DAT_00a08ba4 == -2) {
            if (DAT_00521548 == 2) {
                DAT_00521548 = DAT_0052154c;
                uVar13 = FUN_00480530();
                DAT_0052154c = (short)uVar13;
            }
            FUN_0040e0c0(0, 0x140, 200, 0);
            return sStack_10;
        }
    }

    FUN_004a9520((short)(sVar5 + 2), sVar6, sVar8, sVar10);
    iVar19 = 0;
    FUN_004a9620(1);
    FUN_004a9630(1);
    FUN_004a9640('\0');
    FUN_004a95c0((unsigned int)DAT_005233c8);
    DAT_009ec464 = 1;
    sVar12 = (short)(DAT_009a20ae + 2);
    FUN_004a9590(0x43);
    iVar22 = DAT_005235d0;
    DAT_009993fc = FUN_004ac720((int)DAT_009ec5fc);
    FUN_004ac740();

    if (*(short *)((char *)DAT_00a0a2ca + (unsigned int)DAT_00a08374 * 2) == DAT_00a08ba4) {
        if (DAT_00521548 == 2) {
            switch (DAT_009d6930) {
            case 0xa6:
                if (DAT_00622bbc != 0) {
                    DAT_00622bbc = (unsigned short)(DAT_00622bbc - 1);
LAB_0049ef0b:
                    DAT_00622bb0 = 1;
                }
                break;
            case 0xa7:
                iVar19 = 300;
                if (DAT_005235d8 == 0) {
                    iVar19 = iVar22;
                }
                if ((int)(unsigned int)DAT_00622bbc <=
                    ((int)sVar6 - (int)sVar10) / (int)sVar12 + (int)(short)iVar19) {
                    DAT_00622bbc = (unsigned short)(DAT_00622bbc + 1);
                    goto LAB_0049ef0b;
                }
                break;
            case 0xac:
                if (4 < DAT_00622bbc) {
                    DAT_00622bbc = (unsigned short)(DAT_00622bbc - 4);
                    goto LAB_0049ef0b;
                }
                break;
            case 0xad:
                iVar19 = 300;
                if (DAT_005235d8 == 0) {
                    iVar19 = iVar22;
                }
                if ((int)(unsigned int)DAT_00622bbc <=
                    (int)(short)iVar19 + ((int)sVar6 - (int)sVar10) / (int)sVar12 + -4) {
                    DAT_00622bbc = (unsigned short)(DAT_00622bbc + 4);
                    goto LAB_0049ef0b;
                }
                break;
            }
        }
        if ((unsigned int)(int)DAT_005235d4 != DAT_00622bb8) {
            DAT_00622bb0 = 1;
        }
    } else {
        DAT_00622bbc = 0;
        DAT_00622bb8 = 0;
        DAT_00622bb0 = 1;
        DAT_009d1300();
    }

    sVar10 = (short)((short)(((int)sVar10 - (int)sVar6) / (int)sVar12) + DAT_00622bbc);
    sStack_10 = sVar6;

    if (DAT_00622bb0 != 0) {
        DAT_009d1300();
        sStack_a = 0;
        if (-1 < sVar10) {
            bVar17 = (unsigned char)sVar6;
            do {
                sStack_10 = sVar6;
                iVar19 = (int)sStack_a;
                if (((int)(unsigned int)DAT_00622bbc <= iVar19) && (sStack_a < 300)) {
                    FUN_004a9500((short)(sVar5 + 2), sStack_10);
                    iVar14 = FUN_0049f330(iVar19);
                    if (-1 < iVar14) {
                        iVar22 = (int)((unsigned int)iVar14 * 0x54u);
                        pbVar1 = (unsigned char *)((unsigned int)DAT_009993fc + 0xeu + (unsigned int)iVar22);
                        bVar2 = *pbVar1;
                        if (bVar2 < 9) {
                            pbVar20 = pbVar1 + 1;
                            FUN_004a9590((unsigned int)(unsigned char)s_BJFNREBRJRFNJN_005240a0[bVar2]);
                            if (bVar2 == 1) {
                                bVar2 = *pbVar20;
                                if ((0x2f < bVar2) && (bVar2 < 0x34)) {
                                    pbVar20 = pbVar1 + 2;
                                    FUN_004a9590((unsigned int)DAT_00524078[bVar2]);
                                }
                            } else if (bVar2 == 2) {
                                uVar13 = (unsigned int)(unsigned char)s_RJFNJN_005240b0[*(unsigned short *)((unsigned int)DAT_009993fc + 10u + (unsigned int)iVar22)];
                                goto LAB_0049f045;
                            }
                        } else {
                            uVar13 = 0x42;
                            pbVar20 = pbVar1;
LAB_0049f045:
                            FUN_004a9590(uVar13);
                        }
                        bVar2 = *pbVar20;
                        while (bVar2 != 0) {
                            bVar2 = *pbVar20;
                            if (bVar2 == 0x5b) {
                                if (DAT_009a807a == -0x2c) {
                                    DAT_009a807a = -0x2d;
                                } else {
                                    DAT_009a807a = (signed char)(DAT_009a807a + 1);
                                }
                            } else if (bVar2 == 0x5d) {
                                if (DAT_009a807a == -0x2d) {
                                    DAT_009a807a = -0x2c;
                                } else {
                                    DAT_009a807a = (signed char)(DAT_009a807a - 1);
                                }
                            } else {
                                DAT_009a6fe4((int)bVar2);
                                bVar17 = *pbVar20;
                            }
                            pbVar1 = pbVar20 + 1;
                            pbVar20 = pbVar20 + 1;
                            bVar2 = *pbVar1;
                        }
                        if ((((bVar17 != 0x3f) && (bVar17 != 0x21)) && (bVar17 != 0x3a)) && (bVar17 != 0x20)) {
                            DAT_009a6fe4(0x2e);
                        }
                        FUN_004a95c0((unsigned int)DAT_005233c8);
                        DAT_009a6fe4(10);
                        FUN_004a9590(0x42);
                        FUN_004a95c0((unsigned int)DAT_005233c8);
                        if (*(char *)((unsigned int)DAT_009993fc + 8u + (unsigned int)iVar22) == '\0') {
                            uVar24 = FUN_004278c0((unsigned char *)s_00_00_00527748);
                            iVar19 = (int)((unsigned int)(int)sVar8 - ((unsigned int)uVar24 & 0xffffu));
                            FUN_004a9500((short)iVar19, sStack_10);
                            uVar13 = 1;
                        } else {
                            uVar24 = FUN_004278c0((unsigned char *)s_00_00_00_00527750);
                            iVar19 = (int)((unsigned int)(int)sVar8 - ((unsigned int)uVar24 & 0xffffu));
                            FUN_004a9500((short)iVar19, sStack_10);
                            FUN_004277f0((unsigned short)*(unsigned char *)((unsigned int)DAT_009993fc + 8u + (unsigned int)iVar22), 2, 1);
                            DAT_009a6fe4(0x3a);
                            uVar13 = 2;
                        }
                        FUN_004277f0((unsigned short)*(unsigned char *)((unsigned int)DAT_009993fc + 7u + (unsigned int)iVar22), 2, uVar13);
                        DAT_009a6fe4(0x3a);
                        FUN_004277f0((unsigned short)*(unsigned char *)((unsigned int)DAT_009993fc + 6u + (unsigned int)iVar22), 2, 2);
                        DAT_009a6fe4(0x20);
                        sStack_10 = (short)(sStack_10 + sVar12);
                    }
                }
                sStack_a = (short)(sStack_a + 1);
                sVar6 = sStack_10;
            } while (sStack_a <= sVar10);
        }
    }

    FUN_004a9620(0);
    DAT_00622bb0 = 0;
    if ((DAT_009e9683[(unsigned int)DAT_009ecc34 * 0x5bdu] == '\0') && (DAT_00521548 == -1)) {
        DAT_00521548 = 2;
    }

    if (DAT_00521548 == 2) {
        FUN_004a95c0(0x46);
        FUN_004a9520(sVar5, sVar7, sVar9, sVar11);
    } else {
        if (DAT_0052154c != 2) goto LAB_0049f2d3;
        FUN_004a95c0((unsigned int)DAT_005233c8);
        FUN_004a9520(sVar5, sVar7, sVar9, sVar11);
    }
    DAT_009eca20((int)sVar5, (int)sVar7, (int)sVar9, (int)sVar11, 1);

LAB_0049f2d3:
    FUN_0040e0c0(0, 0x140, 200, 0);
    DAT_00622bb8 = (unsigned int)(int)DAT_005235d4;
    return sStack_10;
}
