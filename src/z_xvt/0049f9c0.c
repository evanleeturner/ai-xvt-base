/* SPDX-License-Identifier: MIT */
/* z_xvt 0049f9c0 */
extern int DAT_006d4988;
extern int DAT_006d4980;
extern short DAT_006d49a0;
extern int _DAT_00732a3c;
extern unsigned char DAT_006d4a6a[];
extern unsigned char DAT_006d4a70[];
extern unsigned char DAT_006d4a74[];
extern unsigned char DAT_006d4a76[];
extern unsigned char DAT_006d4a78[];
extern unsigned char DAT_006d4a80[];
extern unsigned char DAT_006d4a84[];
extern unsigned char DAT_006d4a88[];
extern unsigned char DAT_006d4a8c[];
extern unsigned char DAT_006d4a8e[];
extern unsigned char DAT_006d4a90[];
extern unsigned char DAT_006d4a92[];
extern unsigned char DAT_006d4a94[];
extern unsigned char DAT_006d4a96[];
extern unsigned char DAT_006d4a9a[];
extern unsigned char DAT_006d4aa2[];
extern unsigned char DAT_006d4aa7[];
extern unsigned char DAT_006d4aa8[];
extern unsigned char DAT_006d4aa9[];
extern unsigned char DAT_006d4aad[];
extern unsigned char DAT_006d4aaf[];
extern unsigned char DAT_006d4ab1[];
extern unsigned char DAT_006d4ab7[];
extern unsigned char DAT_006d4ab8[];
extern unsigned char DAT_006d4aba[];
extern unsigned char DAT_006d4ad2[];
extern unsigned char DAT_006d4adc[];
extern unsigned char DAT_006d4afa[];
extern unsigned char DAT_006d4b04[];
extern unsigned char DAT_0070ad40[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad8a[];
extern unsigned char DAT_0070ad8b[];
extern unsigned char DAT_0070ad8c[];
extern unsigned char DAT_0070ad8d[];
extern unsigned char DAT_0070ad8e[];
extern unsigned char DAT_0070ad8f[];
extern unsigned char DAT_0070ad90[];
extern unsigned char DAT_0070ad91[];
extern unsigned char DAT_0070ad92[];
extern unsigned char DAT_00709d0a[];
extern unsigned char DAT_00703ca0[];
extern unsigned char DAT_006da3e0[];
extern unsigned char DAT_00524798[];
extern char s_dropoffldr1pln_00524fdc[];
extern char DAT_0071b740[];
extern char DAT_0071e528[];

void __cdecl FUN_004a00e0(unsigned int param_1);

void __cdecl FUN_0049f9c0(unsigned short param_1, unsigned short param_2, unsigned short param_3)
{
    char cVar1;
    unsigned char bVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar6;
    char *pcVar7;
    int iVar8;
    unsigned int uVar9;
    unsigned char *pbVar10;
    unsigned short uVar11;
    char *pcVar12;
    int bVar13;
    int local_4;
    int iTmp;
    int iVar5;
    unsigned short sTmp;
    char *pp;

    __builtin_memcpy(&iTmp, (unsigned char *)DAT_006d4988 + 0x1f + (unsigned int)param_1 * 0x23, 4);
    __builtin_memcpy(&iVar6, (unsigned char *)iTmp + 0xa7, 4);
    if (*(char *)(iVar6 + 8) != '\x01') {
        uVar3 = (unsigned int)param_2;
        *(unsigned char *)(iVar6 + 8) = 1;
        *(unsigned short *)(DAT_006d4a6a + ((unsigned int)param_3 * 2 + uVar3 * 0x126)) =
            (unsigned short)(*(unsigned short *)(DAT_006d4a6a + ((unsigned int)param_3 * 2 + uVar3 * 0x126)) + 1);
        iVar8 = uVar3 * 0x566;
        cVar1 = DAT_0070ad40[iVar8];
        if (*(char *)(iVar6 + 0xb0) == cVar1) {
            DAT_006d4aa2[uVar3 * 0x126 + (unsigned int)param_3] = 1;
        }
        if (param_3 == 2) {
            if (*(char *)(iVar6 + 0x1e5) != '\0') {
                *(short *)(DAT_006d4a76 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a76 + uVar3 * 0x126) + -1);
                if (*(char *)(iVar6 + 0xb0) == cVar1) {
                    DAT_006d4aa8[uVar3 * 0x126] = 0;
                }
                *(unsigned char *)(iVar6 + 0x1e5) = 0;
            }
            if (*(char *)(iVar6 + 0x74) != '\0') {
                *(short *)(DAT_006d4a96 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a96 + uVar3 * 0x126) + -1);
                if ((char)DAT_0070ad40[iVar8] == *(char *)(iVar6 + 0xb0)) {
                    DAT_006d4ab8[uVar3 * 0x126] = 0;
                }
            }
            if (*(char *)(iVar6 + 0x73) != '\0') {
                *(short *)(DAT_006d4a94 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a94 + uVar3 * 0x126) + -1);
                if ((char)DAT_0070ad40[iVar8] == *(char *)(iVar6 + 0xb0)) {
                    DAT_006d4ab7[uVar3 * 0x126] = 0;
                }
            }
            *(short *)(DAT_006d4a9a + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a9a + uVar3 * 0x126) + 1);
            if ((char)DAT_0070ad40[iVar8] == *(char *)(iVar6 + 0xb0)) {
                DAT_006d4aba[uVar3 * 0x126] = 1;
            }
        }
        else {
            iVar4 = DAT_006d4988 + (unsigned int)param_1 * 0x23;
            __builtin_memcpy(&iTmp, (unsigned char *)iVar4 + 0x1b, 4);
            __builtin_memcpy(&iVar5, (unsigned char *)iVar4 + 0x1f, 4);
            if ((((iTmp != -1) && (*(char *)(iVar6 + 0x74) == '\0')) &&
                 (*(char *)(iVar6 + 0x73) == '\0')) &&
                ((char)DAT_00709d0a[(unsigned int)*(unsigned char *)(iVar5 + 0x87) * 3] != '\x01')) {
                *(short *)(DAT_006d4a96 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a96 + uVar3 * 0x126) + 1);
                if (*(char *)(iVar6 + 0xb0) == cVar1) {
                    DAT_006d4ab8[uVar3 * 0x126] = 1;
                }
            }
        }
        uVar9 = 0;
        do {
            if (*(char *)(iVar6 + 0x1f0 + uVar9) == '\0') {
                DAT_006d4ad2[uVar3 * 0x126 + uVar9] = (unsigned char)(DAT_006d4ad2[uVar3 * 0x126 + uVar9] + 1);
                if ((char)DAT_0070ad40[iVar8] == *(char *)(iVar6 + 0xb0)) {
                    DAT_006d4adc[uVar3 * 0x126 + uVar9] = 1;
                }
            }
            uVar11 = (unsigned short)((short)uVar9 + 1);
            uVar9 = (unsigned int)uVar11;
        } while (uVar11 < 10);
        if (*(char *)(iVar6 + 0x1e4) == '\0') {
            *(short *)(DAT_006d4a88 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a88 + uVar3 * 0x126) + 1);
            if ((char)DAT_0070ad40[iVar8] == *(char *)(iVar6 + 0xb0)) {
                DAT_006d4ab1[uVar3 * 0x126] = 1;
            }
        }
        if (*(char *)(iVar6 + 0x1e5) == '\0') {
            cVar1 = (char)DAT_0070ad40[iVar8];
            *(short *)(DAT_006d4a78 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a78 + uVar3 * 0x126) + 1);
            iVar4 = uVar3 * 0x126;
            if (cVar1 == *(char *)(iVar6 + 0xb0)) {
                DAT_006d4aa9[iVar4] = 1;
            }
            uVar9 = 0;
            bVar2 = DAT_0070ad48[iVar8];
            do {
                if (bVar2 != uVar9) {
                    DAT_006d4afa[iVar4 + uVar9] = (unsigned char)(DAT_006d4afa[iVar4 + uVar9] + 1);
                    DAT_006d4b04[iVar4 + uVar9] = 1;
                }
                uVar11 = (unsigned short)((short)uVar9 + 1);
                uVar9 = (unsigned int)uVar11;
            } while (uVar11 < 10);
        }
        bVar13 = 0;
        uVar11 = 0;
        do {
            if (*(char *)(uVar11 + 0x1e6 + iVar6) == '\x01') {
                bVar13 = 1;
            }
            uVar11 = (unsigned short)(uVar11 + 1);
        } while (uVar11 < 10);
        if (!bVar13) {
            *(short *)(DAT_006d4a74 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a74 + uVar3 * 0x126) + 1);
            if ((char)DAT_0070ad40[iVar8] == *(char *)(iVar6 + 0xb0)) {
                DAT_006d4aa7[uVar3 * 0x126] = 1;
            }
        }
        if (*(char *)(iVar6 + 0x7b) == '\0') {
            *(short *)(DAT_006d4a80 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a80 + uVar3 * 0x126) + 1);
            if ((char)DAT_0070ad40[iVar8] == *(char *)(iVar6 + 0xb0)) {
                DAT_006d4aad[uVar3 * 0x126] = 1;
            }
        }
        if (*(char *)(iVar6 + 0x7d) == '\0') {
            cVar1 = (char)DAT_0070ad40[iVar8];
            *(short *)(DAT_006d4a84 + uVar3 * 0x126) = (short)(*(short *)(DAT_006d4a84 + uVar3 * 0x126) + 1);
            if (cVar1 == *(char *)(iVar6 + 0xb0)) {
                DAT_006d4aaf[uVar3 * 0x126] = 1;
            }
        }
        if (param_3 == 2) {
            uVar11 = 0;
            bVar13 = DAT_006d49a0 == 0;
            if (0 < DAT_006d49a0) {
                do {
                    if (uVar11 != param_2) {
                        uVar9 = (unsigned int)uVar11;
                        iVar6 = uVar9 * 0x566;
                        if ((DAT_0070ad8b[uVar9 * 0x566] != 0) && ((unsigned char)DAT_0070ad8a[iVar6] == uVar3)) {
                            FUN_004a00e0(uVar9);
                        }
                        if ((DAT_0070ad8d[iVar6] != 0) && ((unsigned char)DAT_0070ad8c[iVar6] == uVar3)) {
                            *(short *)(DAT_006d4a70 + uVar9 * 0x126) =
                                (short)(*(short *)(DAT_006d4a70 + uVar9 * 0x126) + *(short *)(DAT_006d4a8e + uVar9 * 0x126));
                            *(short *)(DAT_006d4a8e + uVar9 * 0x126) = 0;
                        }
                        if ((DAT_0070ad8f[iVar6] != 0) && ((unsigned char)DAT_0070ad8e[iVar6] == uVar3)) {
                            *(short *)(DAT_006d4a70 + uVar9 * 0x126) =
                                (short)(*(short *)(DAT_006d4a70 + uVar9 * 0x126) + *(short *)(DAT_006d4a90 + uVar9 * 0x126));
                            *(short *)(DAT_006d4a90 + uVar9 * 0x126) = 0;
                        }
                        if ((DAT_0070ad91[iVar6] != 0) && ((unsigned char)DAT_0070ad90[iVar6] == uVar3)) {
                            *(short *)(DAT_006d4a70 + uVar9 * 0x126) =
                                (short)(*(short *)(DAT_006d4a70 + uVar9 * 0x126) + *(short *)(DAT_006d4a92 + uVar9 * 0x126));
                            *(short *)(DAT_006d4a92 + uVar9 * 0x126) = 0;
                        }
                    }
                    uVar11 = (unsigned short)(uVar11 + 1);
                    bVar13 = (unsigned int)(int)DAT_006d49a0 == (unsigned int)uVar11;
                } while (!bVar13 && (int)(unsigned int)uVar11 <= (int)DAT_006d49a0);
            }
            local_4 = 4;
            pbVar10 = DAT_0070ad92 + iVar8;
            do {
                iVar6 = 0xf;
                pcVar7 = (char *)(DAT_00703ca0 +
                                  (unsigned int)DAT_006da3e0[(unsigned int)DAT_00524798[*pbVar10]] * 0x55);
                pcVar12 = s_dropoffldr1pln_00524fdc;
                do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 + -1;
                    bVar13 = *pcVar7 == *pcVar12;
                    pcVar7 = pcVar7 + 1;
                    pcVar12 = pcVar12 + 1;
                } while (bVar13);
                if (bVar13) {
                    FUN_004a00e0((unsigned int)(unsigned short)(pbVar10[3] - 1));
                }
                pbVar10 = pbVar10 + 0x52;
                local_4 = local_4 + -1;
                bVar13 = local_4 == 0;
            } while (!bVar13);
        }
        if ((param_3 == 0x11) && (uVar11 = 0, 0 < DAT_006d49a0)) {
            iVar6 = (int)DAT_006d49a0;
            do {
                if (uVar11 != param_2) {
                    uVar9 = (unsigned int)uVar11;
                    iVar8 = uVar9 * 0x566;
                    if ((DAT_0070ad8d[uVar9 * 0x566] != 0) && ((unsigned char)DAT_0070ad8c[iVar8] == uVar3)) {
                        *(short *)(DAT_006d4a8c + uVar9 * 0x126) =
                            (short)(*(short *)(DAT_006d4a8c + uVar9 * 0x126) + *(short *)(DAT_006d4a8e + uVar9 * 0x126));
                        *(short *)(DAT_006d4a8e + uVar9 * 0x126) = 0;
                    }
                    if ((DAT_0070ad8f[iVar8] != 0) && ((unsigned char)DAT_0070ad8e[iVar8] == uVar3)) {
                        *(short *)(DAT_006d4a8c + uVar9 * 0x126) =
                            (short)(*(short *)(DAT_006d4a8c + uVar9 * 0x126) + *(short *)(DAT_006d4a90 + uVar9 * 0x126));
                        *(short *)(DAT_006d4a90 + uVar9 * 0x126) = 0;
                    }
                    if ((DAT_0070ad91[iVar8] != 0) && ((unsigned char)DAT_0070ad90[iVar8] == uVar3)) {
                        *(short *)(DAT_006d4a8c + uVar9 * 0x126) =
                            (short)(*(short *)(DAT_006d4a8c + uVar9 * 0x126) + *(short *)(DAT_006d4a92 + uVar9 * 0x126));
                        *(short *)(DAT_006d4a92 + uVar9 * 0x126) = 0;
                    }
                }
                uVar11 = (unsigned short)(uVar11 + 1);
            } while ((int)(unsigned int)uVar11 < iVar6);
        }
        if (_DAT_00732a3c < DAT_006d4980) {
            iVar6 = _DAT_00732a3c * 0x23;
            iVar8 = _DAT_00732a3c;
            do {
                if (*(char *)(DAT_006d4988 + iVar6 + 3) != '\0') {
                    __builtin_memcpy(&iTmp, (unsigned char *)DAT_006d4988 + iVar6 + 0x1f, 4);
                    __builtin_memcpy(&iVar4, (unsigned char *)iTmp + 0xa7, 4);
                    if (*(unsigned short *)(iVar4 + 0x6a) == param_1) {
                        *(unsigned short *)(iVar4 + 0x6a) = 0xffff;
                    }
                }
                iVar6 = iVar6 + 0x23;
                iVar8 = iVar8 + 1;
            } while (iVar8 < DAT_006d4980);
        }
        pcVar7 = DAT_0071b740;
        do {
            if (*pcVar7 != '\0') {
                pp = pcVar7 + 0xe;
                iVar6 = 4;
                do {
                    __builtin_memcpy(&sTmp, pp, 2);
                    if (sTmp == param_1) {
                        sTmp = 0xffff;
                        __builtin_memcpy(pp, &sTmp, 2);
                    }
                    pp = pp + 2;
                    iVar6 = iVar6 + -1;
                } while (iVar6 != 0);
            }
            pcVar7 = pcVar7 + 0x5bd;
        } while (pcVar7 < DAT_0071e528);
    }
    return;
}
