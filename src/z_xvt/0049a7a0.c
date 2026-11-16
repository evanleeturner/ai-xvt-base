/* SPDX-License-Identifier: MIT */
/* z_xvt 0049a7a0 */
extern int DAT_00732e48;
extern int DAT_00525550;
extern int DAT_00525554;
extern int DAT_0071b71c;
extern int DAT_006d4988;
extern unsigned short DAT_0071e6be;
extern short DAT_0070a1d2;
extern int DAT_0071e690;
extern unsigned int DAT_0071ecf4;
extern int DAT_0073290c;
extern unsigned char DAT_0071b730[];
extern int DAT_006db730[];
extern unsigned char DAT_006da500[];
extern unsigned char DAT_0071f720[];

void __stdcall FUN_00444a30(void);
void __stdcall FUN_004253a0(void);
void __cdecl FUN_00439a30(unsigned int param_1);

/* Unaligned-safe accessors (records are 0x12 / 0x23 / 0x5bd / 6 byte strides) */
#define LD32(v, p) __builtin_memcpy(&(v), (const void *)(p), 4)
#define ST32(p, v) __builtin_memcpy((void *)(p), &(v), 4)
#define LD16(v, p) __builtin_memcpy(&(v), (const void *)(p), 2)
#define ST16(p, v) __builtin_memcpy((void *)(p), &(v), 2)

void __cdecl FUN_0049a7a0(int param_1)
{
    int iVar1;
    int iVar2;
    unsigned short uVar3;
    unsigned short uVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    int iVar14;
    int iVar15;
    int iVar21;
    int iVar22;
    short sVar16;
    short sVar17;
    unsigned char *pbVar19;
    unsigned char *pbVar20;
    unsigned char *psVar12;
    unsigned char *piVar8;
    unsigned char *piVar9;
    int *piVar10;
    unsigned char *piVar11;
    unsigned char *piStack_1c;
    unsigned char *piStack_18;
    unsigned int uStack_14;
    int iStack_10;
    int iStack_20;

    if (DAT_00732e48 == 1) {
        iStack_20 = 1;
    } else {
        iStack_20 = DAT_00525550;
    }
    piVar11 = DAT_0071b730;
    piVar10 = DAT_006db730;
    uStack_14 = 0;
    piStack_1c = DAT_006da500;
    piStack_18 = DAT_0071f720;
    do {
        if (piVar11[0x10] != 0) {
            iVar1 = *piVar10;
            iStack_10 = 0;
            piVar9 = piStack_18;
            if (0 < iVar1) {
                do {
                    iVar5 = DAT_0071b71c;
                    LD32(iVar8, piVar9);
                    LD32(iVar10, piVar9 + 8);
                    LD32(iVar13, piVar11 + 0x58c);
                    if ((((iStack_20 == 0) || (DAT_00732e48 == 1)) && (iVar10 <= iVar13)) && (iVar8 == 0)) {
                        if ((*piVar10 != 0) && (piStack_18 <= piVar9)) {
                            iVar5 = (int)((unsigned int)*piVar10 - 1u);
                            iVar6 = 0;
                            *piVar10 = iVar5;
                            piVar8 = piStack_18;
                            if (0 < iVar5) {
                                do {
                                    if (piVar9 <= piVar8) {
                                        LD32(iVar7, piVar8 + 0x12);
                                        ST32(piVar8, iVar7);
                                        LD32(iVar7, piVar8 + 0x16);
                                        ST32(piVar8 + 4, iVar7);
                                        LD32(iVar7, piVar8 + 0x1a);
                                        ST32(piVar8 + 8, iVar7);
                                        LD32(iVar7, piVar8 + 0x1e);
                                        ST32(piVar8 + 0xc, iVar7);
                                        LD16(sVar16, piVar8 + 0x22);
                                        ST16(piVar8 + 0x10, sVar16);
                                    }
                                    iVar6 = iVar6 + 1;
                                    piVar8 = piVar8 + 0x12;
                                } while (iVar6 < *piVar10);
                            }
                        }
                        piVar9 = piVar9 - 0x12;
                    } else {
                        iVar6 = iVar10;
                        DAT_0071b71c = iVar5;
                        LD32(iVar9, piVar9 + 4);
                        if (((iVar6 <= param_1) && ((iStack_20 != 0 || (iVar9 == 0)))) &&
                            (LD32(iVar2, piVar11 + 0x58c), iVar2 < iVar6)) {
                            LD32(iVar14, piVar11);
                            if ((iVar6 <= DAT_0071b71c) && (iVar14 != -1)) {
                                psVar12 = (unsigned char *)((unsigned int)DAT_006d4988 + (unsigned int)iVar14 * 0x23u);
                                LD32(iVar6, psVar12 + 0x1f);
                                DAT_0071b71c = iVar5;
                                if (iVar6 != 0) {
                                    LD32(iVar15, (unsigned char *)iVar6 + 0xa7);
                                }
                                if ((iVar6 == 0) || (iVar15 == 0)) goto LAB_0049ad67;
                                LD16(sVar16, piVar11 + 0x5aa);
                                LD16(sVar17, psVar12);
                                if ((sVar16 == sVar17) && (piVar11[0x5ac] == piVar11[0x11])) {
                                    LD32(iVar15, (unsigned char *)iVar6 + 2);
                                    if (iVar2 < iVar15) {
                                        ST32((unsigned char *)iVar6 + 2, iVar2);
                                        LD32(iVar14, piVar11);
                                        psVar12 = (unsigned char *)((unsigned int)DAT_006d4988 + (unsigned int)iVar14 * 0x23u);
                                        LD32(iVar7, piVar11 + 0x590);
                                        ST32(psVar12 + 4, iVar7);
                                        LD32(iVar7, piVar11 + 0x594);
                                        ST32(psVar12 + 8, iVar7);
                                        LD32(iVar7, piVar11 + 0x598);
                                        ST32(psVar12 + 0xc, iVar7);
                                        LD32(iVar7, piVar11 + 0x59c);
                                        sVar16 = (short)iVar7;
                                        ST16(psVar12 + 0x14, sVar16);
                                        LD16(sVar16, piVar11 + 0x59e);
                                        ST16(psVar12 + 0x12, sVar16);
                                        LD32(iVar7, piVar11 + 0x5a0);
                                        sVar16 = (short)iVar7;
                                        ST16(psVar12 + 0x10, sVar16);
                                        LD32(iVar21, psVar12 + 0x1f);
                                        pbVar19 = (unsigned char *)iVar21;
                                        LD16(sVar16, piVar11 + 0x5a2);
                                        ST16(pbVar19 + 0x7f, sVar16);
                                        LD32(iVar7, piVar11 + 0x5a8);
                                        sVar16 = (short)iVar7;
                                        ST16(pbVar19 + 0x77, sVar16);
                                        LD32(iVar7, piVar11 + 0x5a4);
                                        sVar16 = (short)iVar7;
                                        ST16(pbVar19 + 0x79, sVar16);
                                        LD16(sVar16, piVar11 + 0x5a6);
                                        ST16(pbVar19 + 0x7b, sVar16);
                                        LD32(iVar22, pbVar19 + 0xa7);
                                        pbVar20 = (unsigned char *)iVar22;
                                        LD16(sVar16, piVar11 + 0x59e);
                                        ST16(pbVar20 + 0xd, sVar16);
                                        pbVar19[0x90] = 1;
                                        pbVar19[0x89] = 1;
                                    }
                                    LD32(iVar14, piVar11);
                                    psVar12 = (unsigned char *)((unsigned int)DAT_006d4988 + (unsigned int)iVar14 * 0x23u);
                                    LD32(iVar21, psVar12 + 0x1f);
                                    LD32(iVar6, (unsigned char *)iVar21 + 2);
                                    if (iVar6 < DAT_0071b71c) {
                                        DAT_0071b71c = iVar6;
                                    }
                                } else {
                                    if (iStack_20 != 0) goto LAB_0049ad67;
                                    iVar7 = (int)((unsigned int)DAT_0071b71c + 4u);
                                    ST32(piVar9 + 8, iVar7);
                                    ST32(piVar11 + 0x58c, DAT_0071b71c);
                                }
                            }
                            uVar4 = DAT_0071e6be;
                            uVar3 = (unsigned short)DAT_0070a1d2;
                            LD32(iVar6, piVar11);
                            if (iVar6 != -1) {
                                psVar12 = (unsigned char *)((unsigned int)DAT_006d4988 + (unsigned int)iVar6 * 0x23u);
                                LD32(iVar21, psVar12 + 0x1f);
                                if (iVar21 != 0) {
                                    LD32(iVar10, piVar9 + 8);
                                    DAT_0071e6be = (unsigned short)((short)iVar10 - (short)DAT_0071b71c);
                                    if (DAT_0071e6be == 0) {
                                        DAT_0070a1d2 = 0xec;
                                    } else {
                                        DAT_0070a1d2 = (short)(0xecu / (unsigned int)DAT_0071e6be);
                                    }
                                    if (DAT_0070a1d2 == 0) {
                                        DAT_0070a1d2 = 1;
                                    }
                                    DAT_0071e690 = iVar6;
                                    FUN_00444a30();
                                    FUN_004253a0();
                                    psVar12 = (unsigned char *)((unsigned int)DAT_0071e690 * 0x23u + (unsigned int)DAT_006d4988);
                                    LD32(iVar21, psVar12 + 0x1f);
                                    LD32(iVar10, piVar9 + 8);
                                    ST32((unsigned char *)iVar21 + 2, iVar10);
                                    psVar12 = (unsigned char *)((unsigned int)DAT_0071e690 * 0x23u + (unsigned int)DAT_006d4988);
                                    LD32(iVar7, psVar12 + 4);
                                    ST32(piVar11 + 0x590, iVar7);
                                    LD32(iVar7, psVar12 + 8);
                                    ST32(piVar11 + 0x594, iVar7);
                                    LD32(iVar7, psVar12 + 0xc);
                                    ST32(piVar11 + 0x598, iVar7);
                                    LD16(sVar16, psVar12 + 0x14);
                                    ST16(piVar11 + 0x59c, sVar16);
                                    LD16(sVar16, psVar12 + 0x12);
                                    ST16(piVar11 + 0x59e, sVar16);
                                    LD16(sVar16, psVar12 + 0x10);
                                    ST16(piVar11 + 0x5a0, sVar16);
                                    LD32(iVar21, psVar12 + 0x1f);
                                    pbVar19 = (unsigned char *)iVar21;
                                    LD16(sVar16, pbVar19 + 0x7f);
                                    ST16(piVar11 + 0x5a2, sVar16);
                                    LD16(sVar16, pbVar19 + 0x77);
                                    ST16(piVar11 + 0x5a8, sVar16);
                                    LD16(sVar16, pbVar19 + 0x79);
                                    ST16(piVar11 + 0x5a4, sVar16);
                                    LD16(sVar16, pbVar19 + 0x7b);
                                    ST16(piVar11 + 0x5a6, sVar16);
                                    LD16(sVar16, psVar12);
                                    ST16(piVar11 + 0x5aa, sVar16);
                                    piVar11[0x5ac] = piVar11[0x11];
                                }
                                DAT_0071e690 = -1;
                            }
                            LD32(iVar10, piVar9 + 8);
                            LD32(iVar7, piVar11 + 0x58c);
                            DAT_0071e6be = (unsigned short)((short)iVar10 - (short)iVar7);
                            if (DAT_0071e6be < 4) {
                                DAT_0071e6be = 4;
                            }
                            if (DAT_0071e6be == 0) {
                                DAT_0070a1d2 = 0xec;
                            } else {
                                DAT_0070a1d2 = (short)(0xecu / (unsigned int)DAT_0071e6be);
                            }
                            if (DAT_0070a1d2 == 0) {
                                DAT_0070a1d2 = 1;
                            }
                            LD32(iVar10, piVar9 + 8);
                            ST32(piVar11 + 0x58c, iVar10);
                            LD32(iVar11, piVar9 + 0xc);
                            ST32(piStack_1c, iVar11);
                            LD32(iVar12, piVar9 + 0x10);
                            sVar16 = (short)iVar12;
                            ST16(piStack_1c + 4, sVar16);
                            LD32(iVar10, piVar9 + 8);
                            if ((iStack_20 != 0) && (iVar10 <= iVar5)) {
                                piStack_1c[2] = 0;
                                piStack_1c[5] = 0;
                            }
                            LD32(iVar10, piVar9 + 8);
                            if (((1 < DAT_00732e48) && (uStack_14 == DAT_0071ecf4)) &&
                                (DAT_00525554 = 2, DAT_0073290c < iVar10)) {
                                DAT_00525554 = 1;
                                LD32(iVar10, piVar9 + 8);
                                DAT_0073290c = iVar10;
                            }
                            FUN_00439a30(uStack_14);
                            DAT_00525554 = 0;
                            DAT_0070a1d2 = (short)uVar3;
                            DAT_0071b71c = iVar5;
                            DAT_0071e6be = uVar4;
                            if (piVar11[0x10] == 0) break;
                        }
                    }
LAB_0049ad67:
                    piVar9 = piVar9 + 0x12;
                    iStack_10 = iStack_10 + 1;
                } while (iStack_10 < iVar1);
            }
        }
        piVar11 = piVar11 + 0x5bd;
        piStack_1c = piStack_1c + 6;
        piStack_18 = piStack_18 + 0x7e9 * 4;
        piVar10 = piVar10 + 1;
        uStack_14 = uStack_14 + 1;
        if ((int *)0x6db74f < piVar10) {
            return;
        }
    } while (1);
}
