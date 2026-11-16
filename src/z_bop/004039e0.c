/* SPDX-License-Identifier: MIT */
/* z_bop 004039e0 */
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_009a1ff8;
extern unsigned short DAT_009a73b0;
extern unsigned char DAT_009a73b6[4];
extern int DAT_009d1308;
extern int DAT_00a07ce8;
extern unsigned int DAT_00a08104;
extern char DAT_009d1320[];
extern unsigned char DAT_00521f2e[];
extern char DAT_0051a1d0[];
extern char s_stationaryldrpln_0051a1bc[];
extern char s_stationaryflwpln_0051a1a8[];
extern char s_boardtogivepln_0051a254[];
extern char s_boardtotakepln_0051a244[];
extern char s_boardtoexchangepln_0051a230[];
extern char s_boardtocapturepln_0051a278[];
extern char s_boardtodestroypln_0051a264[];
extern char s_boardtopickuppln_0051a21c[];
extern char s_boardtocontactpln_0051a208[];
extern char s_boardtorepairpln_0051a1f4[];

unsigned char FUN_00433390(unsigned int param_1, unsigned short param_2, unsigned short param_3);
unsigned short FUN_00432fa0(unsigned short param_1, unsigned short param_2, unsigned short param_3);
void FUN_00403540(unsigned int param_1, unsigned int param_2);

unsigned int __cdecl FUN_004039e0(unsigned short param_1, unsigned short param_2, short param_3, unsigned short param_4, unsigned short param_5)
{
    unsigned char *psVar1;
    unsigned char bVar2;
    int bVar3;
    int bVar4;
    unsigned short uVar5;
    unsigned short uVar13;
    unsigned int iVar6;
    unsigned int iVar7;
    unsigned int iVar8;
    short sVar9;
    const char *pcVar10;
    const char *pcVar11;
    const char *pcVar14;
    unsigned int uVar12;
    unsigned short local_12;
    short local_10;
    unsigned short local_e;
    unsigned int local_4;
    unsigned int iTmp;
    unsigned int iPtr;
    unsigned short sTmp;

    local_e = DAT_00a07cec;
    local_4 = 0xffffffff;
    local_12 = 0xffff;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            iVar6 = (unsigned int)local_e * 0x23;
            if (*(char *)(DAT_009a1fe8 + 3 + iVar6) != '\0') {
                bVar4 = FUN_00433390(local_e, param_1, param_2);
                uVar13 = (unsigned short)(unsigned char)bVar4;
                bVar4 = FUN_00433390(local_e, param_4, param_5);
                uVar5 = (unsigned short)(unsigned char)bVar4;
                if (param_3 == 1) {
                    uVar13 = uVar13 | uVar5;
                } else {
                    uVar13 = uVar13 & uVar5;
                }
                bVar4 = (uVar13 == 0);
                if (!bVar4) {
                    bVar3 = 0;
                    psVar1 = (unsigned char *)(DAT_009a1fe8 + iVar6);
                    __builtin_memcpy(&iPtr, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar6), 4);
                    __builtin_memcpy(&iVar6, (unsigned char *)(iPtr + 0xa9), 4);
                    pcVar11 = DAT_009d1320 + (unsigned int)*(unsigned char *)(iVar6 + 0x35) * 0x55;
                    iVar7 = 8;
                    pcVar10 = pcVar11;
                    pcVar14 = DAT_0051a1d0;
                    do {
                        if (iVar7 == 0) break;
                        iVar7 = iVar7 - 1;
                        bVar4 = (*pcVar10 == *pcVar14);
                        pcVar10 = pcVar10 + 1;
                        pcVar14 = pcVar14 + 1;
                    } while (bVar4);
                    if (bVar4) {
LAB_00403b78:
                        bVar3 = 1;
                    } else {
                        iVar7 = 0x11;
                        bVar4 = 0;
                        pcVar10 = pcVar11;
                        pcVar14 = s_stationaryldrpln_0051a1bc;
                        do {
                            if (iVar7 == 0) break;
                            iVar7 = iVar7 - 1;
                            bVar4 = (*pcVar10 == *pcVar14);
                            pcVar10 = pcVar10 + 1;
                            pcVar14 = pcVar14 + 1;
                        } while (bVar4);
                        if (bVar4) goto LAB_00403b78;
                        iVar7 = 0x11;
                        bVar4 = 0;
                        pcVar10 = s_stationaryflwpln_0051a1a8;
                        do {
                            if (iVar7 == 0) break;
                            iVar7 = iVar7 - 1;
                            bVar4 = (*pcVar11 == *pcVar10);
                            pcVar11 = pcVar11 + 1;
                            pcVar10 = pcVar10 + 1;
                        } while (bVar4);
                        if (bVar4 || psVar1[2] == 5) goto LAB_00403b78;
                        iVar7 = 0x12;
                        bVar4 = 0;
                        __builtin_memcpy(&iTmp, DAT_009a73b6, 4);
                        pcVar11 = DAT_009d1320 + (unsigned int)*(unsigned char *)(iTmp + 0xb) * 0x55;
                        pcVar10 = s_boardtocapturepln_0051a278;
                        do {
                            if (iVar7 == 0) break;
                            iVar7 = iVar7 - 1;
                            bVar4 = (*pcVar11 == *pcVar10);
                            pcVar11 = pcVar11 + 1;
                            pcVar10 = pcVar10 + 1;
                        } while (bVar4);
                        if (bVar4) {
LAB_00403b6e:
                            __builtin_memcpy(&sTmp, (unsigned char *)(iVar6 + 0x1e5), 2);
                            if (sTmp == 0) goto LAB_00403b78;
                        } else {
                            iVar7 = 0x12;
                            bVar4 = 0;
                            __builtin_memcpy(&iTmp, DAT_009a73b6, 4);
                            pcVar11 = DAT_009d1320 + (unsigned int)*(unsigned char *)(iTmp + 0xb) * 0x55;
                            pcVar10 = s_boardtodestroypln_0051a264;
                            do {
                                if (iVar7 == 0) break;
                                iVar7 = iVar7 - 1;
                                bVar4 = (*pcVar11 == *pcVar10);
                                pcVar11 = pcVar11 + 1;
                                pcVar10 = pcVar10 + 1;
                            } while (bVar4);
                            if (bVar4) goto LAB_00403b6e;
                            __builtin_memcpy(&sTmp, (unsigned char *)(iVar6 + 0x1e5), 2);
                            if (sTmp == 0 || *(char *)(iVar6 + 0x5e) == '\x13' ||
                                *(char *)(iVar6 + 0x5e) == '\x19') goto LAB_00403b78;
                        }
                    }
                    if (bVar3) {
                        local_10 = 0;
                        for (uVar13 = DAT_00a07cec; (int)(unsigned int)uVar13 < DAT_009a1fe0; uVar13 = uVar13 + 1) {
                            iVar6 = (unsigned int)uVar13 * 0x23 + DAT_009a1fe8;
                            bVar4 = (DAT_009a73b0 == uVar13);
                            if ((*(char *)(iVar6 + 3) != '\0') && !bVar4) {
                                __builtin_memcpy(&iPtr, (unsigned char *)(iVar6 + 0x1f), 4);
                                __builtin_memcpy(&iVar6, (unsigned char *)(iPtr + 0xa9), 4);
                                pcVar11 = DAT_009d1320 + (unsigned int)*(unsigned char *)(iVar6 + 0x35) * 0x55;
                                iVar7 = 0xf;
                                pcVar10 = pcVar11;
                                pcVar14 = s_boardtogivepln_0051a254;
                                do {
                                    if (iVar7 == 0) break;
                                    iVar7 = iVar7 - 1;
                                    bVar4 = (*pcVar10 == *pcVar14);
                                    pcVar10 = pcVar10 + 1;
                                    pcVar14 = pcVar14 + 1;
                                } while (bVar4);
                                if (!bVar4) {
                                    iVar7 = 0xf;
                                    pcVar10 = pcVar11;
                                    pcVar14 = s_boardtotakepln_0051a244;
                                    do {
                                        if (iVar7 == 0) break;
                                        iVar7 = iVar7 - 1;
                                        bVar4 = (*pcVar10 == *pcVar14);
                                        pcVar10 = pcVar10 + 1;
                                        pcVar14 = pcVar14 + 1;
                                    } while (bVar4);
                                    if (!bVar4) {
                                        iVar7 = 0x13;
                                        pcVar10 = pcVar11;
                                        pcVar14 = s_boardtoexchangepln_0051a230;
                                        do {
                                            if (iVar7 == 0) break;
                                            iVar7 = iVar7 - 1;
                                            bVar4 = (*pcVar10 == *pcVar14);
                                            pcVar10 = pcVar10 + 1;
                                            pcVar14 = pcVar14 + 1;
                                        } while (bVar4);
                                        if (!bVar4) {
                                            iVar7 = 0x12;
                                            pcVar10 = pcVar11;
                                            pcVar14 = s_boardtocapturepln_0051a278;
                                            do {
                                                if (iVar7 == 0) break;
                                                iVar7 = iVar7 - 1;
                                                bVar4 = (*pcVar10 == *pcVar14);
                                                pcVar10 = pcVar10 + 1;
                                                pcVar14 = pcVar14 + 1;
                                            } while (bVar4);
                                            if (!bVar4) {
                                                iVar7 = 0x12;
                                                pcVar10 = pcVar11;
                                                pcVar14 = s_boardtodestroypln_0051a264;
                                                do {
                                                    if (iVar7 == 0) break;
                                                    iVar7 = iVar7 - 1;
                                                    bVar4 = (*pcVar10 == *pcVar14);
                                                    pcVar10 = pcVar10 + 1;
                                                    pcVar14 = pcVar14 + 1;
                                                } while (bVar4);
                                                if (!bVar4) {
                                                    iVar7 = 0x11;
                                                    pcVar10 = pcVar11;
                                                    pcVar14 = s_boardtopickuppln_0051a21c;
                                                    do {
                                                        if (iVar7 == 0) break;
                                                        iVar7 = iVar7 - 1;
                                                        bVar4 = (*pcVar10 == *pcVar14);
                                                        pcVar10 = pcVar10 + 1;
                                                        pcVar14 = pcVar14 + 1;
                                                    } while (bVar4);
                                                    if (!bVar4) {
                                                        iVar7 = 0x12;
                                                        pcVar10 = pcVar11;
                                                        pcVar14 = s_boardtocontactpln_0051a208;
                                                        do {
                                                            if (iVar7 == 0) break;
                                                            iVar7 = iVar7 - 1;
                                                            bVar4 = (*pcVar10 == *pcVar14);
                                                            pcVar10 = pcVar10 + 1;
                                                            pcVar14 = pcVar14 + 1;
                                                        } while (bVar4);
                                                        if (!bVar4) {
                                                            iVar7 = 0x11;
                                                            pcVar10 = s_boardtorepairpln_0051a1f4;
                                                            do {
                                                                if (iVar7 == 0) break;
                                                                iVar7 = iVar7 - 1;
                                                                bVar4 = (*pcVar11 == *pcVar10);
                                                                pcVar11 = pcVar11 + 1;
                                                                pcVar10 = pcVar10 + 1;
                                                            } while (bVar4);
                                                            if (!bVar4) goto LAB_00403c9b;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if ((*(unsigned short *)(iVar6 + 0x42) == local_e) ||
                                    (*(unsigned short *)(iVar6 + 0x66) == local_e)) {
                                    local_10 = (short)((unsigned short)local_10 + 1);
                                }
                            }
LAB_00403c9b:
                            ;
                        }
                        uVar12 = 0;
                        if (*(unsigned char *)(DAT_00a08104 + 0x7d) != 0) {
                            do {
                                if (*(short *)(DAT_00a08104 + 0x7e + uVar12 * 2) == *(short *)psVar1) {
                                    local_10 = (short)((unsigned short)local_10 + 1);
                                }
                                uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
                            } while (uVar12 < *(unsigned char *)(DAT_00a08104 + 0x7d));
                        }
                        if (local_10 == 0) {
                            FUN_00403540((unsigned int)DAT_009a73b0, (unsigned int)local_e);
                            if (DAT_009a1ff8 < local_4) {
                                local_4 = DAT_009a1ff8;
                                local_12 = local_e;
                            }
                        }
                    }
                }
            }
            local_e = local_e + 1;
        } while ((int)(unsigned int)local_e < DAT_009a1fe0);
    }
    local_e = (unsigned short)DAT_00a07ce8;
    iVar6 = (unsigned int)(DAT_009d1308 + DAT_00a07ce8);
    if ((int)(DAT_00a07ce8 & 0xffff) < (int)iVar6) {
        do {
            iVar6 = (unsigned int)local_e * 0x23;
            bVar2 = *(unsigned char *)(DAT_009a1fe8 + iVar6 + 3);
            if ((bVar2 != 0) && ((DAT_00521f2e[(unsigned int)bVar2 * 0x1a] & 2) != 0)) {
                uVar13 = FUN_00432fa0((unsigned short)*(unsigned char *)(DAT_009a1fe8 + iVar6 + 0x16), param_1, param_2);
                uVar5 = FUN_00432fa0((unsigned short)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar6), param_4, param_5);
                if (param_3 == 1) {
                    uVar13 = uVar13 | uVar5;
                } else {
                    uVar13 = uVar13 & uVar5;
                }
                if (uVar13 != 0) {
                    sVar9 = 0;
                    for (uVar13 = DAT_00a07cec; (int)(unsigned int)uVar13 < DAT_009a1fe0; uVar13 = uVar13 + 1) {
                        iVar7 = (unsigned int)uVar13 * 0x23 + DAT_009a1fe8;
                        bVar4 = (DAT_009a73b0 == uVar13);
                        if ((*(char *)(iVar7 + 3) != '\0') && !bVar4) {
                            __builtin_memcpy(&iPtr, (unsigned char *)(iVar7 + 0x1f), 4);
                            __builtin_memcpy(&iVar7, (unsigned char *)(iPtr + 0xa9), 4);
                            iVar8 = 0xf;
                            pcVar11 = DAT_009d1320 + (unsigned int)*(unsigned char *)(iVar7 + 0x35) * 0x55;
                            pcVar10 = pcVar11;
                            pcVar14 = s_boardtogivepln_0051a254;
                            do {
                                if (iVar8 == 0) break;
                                iVar8 = iVar8 - 1;
                                bVar4 = (*pcVar10 == *pcVar14);
                                pcVar10 = pcVar10 + 1;
                                pcVar14 = pcVar14 + 1;
                            } while (bVar4);
                            if (!bVar4) {
                                iVar8 = 0xf;
                                pcVar10 = pcVar11;
                                pcVar14 = s_boardtotakepln_0051a244;
                                do {
                                    if (iVar8 == 0) break;
                                    iVar8 = iVar8 - 1;
                                    bVar4 = (*pcVar10 == *pcVar14);
                                    pcVar10 = pcVar10 + 1;
                                    pcVar14 = pcVar14 + 1;
                                } while (bVar4);
                                if (!bVar4) {
                                    iVar8 = 0x13;
                                    pcVar10 = pcVar11;
                                    pcVar14 = s_boardtoexchangepln_0051a230;
                                    do {
                                        if (iVar8 == 0) break;
                                        iVar8 = iVar8 - 1;
                                        bVar4 = (*pcVar10 == *pcVar14);
                                        pcVar10 = pcVar10 + 1;
                                        pcVar14 = pcVar14 + 1;
                                    } while (bVar4);
                                    if (!bVar4) {
                                        iVar8 = 0x12;
                                        pcVar10 = pcVar11;
                                        pcVar14 = s_boardtocapturepln_0051a278;
                                        do {
                                            if (iVar8 == 0) break;
                                            iVar8 = iVar8 - 1;
                                            bVar4 = (*pcVar10 == *pcVar14);
                                            pcVar10 = pcVar10 + 1;
                                            pcVar14 = pcVar14 + 1;
                                        } while (bVar4);
                                        if (!bVar4) {
                                            iVar8 = 0x12;
                                            pcVar10 = pcVar11;
                                            pcVar14 = s_boardtodestroypln_0051a264;
                                            do {
                                                if (iVar8 == 0) break;
                                                iVar8 = iVar8 - 1;
                                                bVar4 = (*pcVar10 == *pcVar14);
                                                pcVar10 = pcVar10 + 1;
                                                pcVar14 = pcVar14 + 1;
                                            } while (bVar4);
                                            if (!bVar4) {
                                                iVar8 = 0x12;
                                                pcVar10 = pcVar11;
                                                pcVar14 = s_boardtocontactpln_0051a208;
                                                do {
                                                    if (iVar8 == 0) break;
                                                    iVar8 = iVar8 - 1;
                                                    bVar4 = (*pcVar10 == *pcVar14);
                                                    pcVar10 = pcVar10 + 1;
                                                    pcVar14 = pcVar14 + 1;
                                                } while (bVar4);
                                                if (!bVar4) {
                                                    iVar8 = 0x11;
                                                    pcVar10 = s_boardtorepairpln_0051a1f4;
                                                    do {
                                                        if (iVar8 == 0) break;
                                                        iVar8 = iVar8 - 1;
                                                        bVar4 = (*pcVar11 == *pcVar10);
                                                        pcVar11 = pcVar11 + 1;
                                                        pcVar10 = pcVar10 + 1;
                                                    } while (bVar4);
                                                    if (!bVar4) goto LAB_00403f11;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if (*(unsigned short *)(iVar7 + 0x42) == local_e) {
                                sVar9 = (short)((unsigned short)sVar9 + 1);
                            }
                        }
LAB_00403f11:
                        ;
                    }
                    uVar12 = 0;
                    if (*(unsigned char *)(DAT_00a08104 + 0x7d) != 0) {
                        do {
                            if (*(short *)(DAT_00a08104 + 0x7e + uVar12 * 2) == *(short *)(DAT_009a1fe8 + iVar6)) {
                                sVar9 = (short)((unsigned short)sVar9 + 1);
                            }
                            uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
                        } while (uVar12 < *(unsigned char *)(DAT_00a08104 + 0x7d));
                    }
                    if (sVar9 == 0) {
                        FUN_00403540((unsigned int)DAT_009a73b0, (unsigned int)local_e);
                        if (DAT_009a1ff8 < local_4) {
                            local_4 = DAT_009a1ff8;
                            local_12 = local_e;
                        }
                    }
                }
            }
            local_e = local_e + 1;
            iVar6 = (unsigned int)(DAT_009d1308 + DAT_00a07ce8);
        } while ((int)(unsigned int)local_e < (int)iVar6);
    }
    return ((iVar6 >> 0x10) << 0x10) | (unsigned int)local_12;
}
