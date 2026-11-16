/* SPDX-License-Identifier: MIT */
/* z_xvt 00443440 */
extern unsigned char *DAT_006d4988;
extern int DAT_006d4980;
extern unsigned short DAT_00732a3c;
extern int DAT_0071ecf4;
extern int DAT_00732e54;

extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad4b[];
extern unsigned char DAT_0070ad54[];
extern char DAT_0070b216[];
extern char DAT_0072f508[];
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071b73e[];
extern char DAT_00703ca0[];

extern char DAT_00524948[];
extern char s_stationaryldrpln_00524934[];
extern char s_stationaryflwpln_00524920[];
extern char DAT_005250b4[];
extern char DAT_00525b80[];
extern char s_formevadeldr1pln_005250a0[];
extern char s_formevadeflw1pln_00525b6c[];
extern char s_flyhomeevadepln_00525b50[];
extern char s_intohyperspacepln_00525b10[];
extern char s_enterhangarpln_00523be4[];
extern char s_craftwaitforgopln_00525b24[];

int FUN_0042b150(int param_1);
void FUN_004296b0(unsigned int param_1);
void FUN_00429310(unsigned int param_1);
void FUN_004a35b0(unsigned int param_1, unsigned int *param_2, short param_3,
                  unsigned int param_4, int param_5, unsigned int param_6);

void __cdecl FUN_00443440(unsigned short param_1, short param_2,
                          unsigned int param_3, int param_4)
{
    unsigned char *iVar2;
    unsigned char *iVar6;
    unsigned char *q;
    unsigned int iVar4;
    int iVar5;
    unsigned int iVar1;
    unsigned int uVar7;
    unsigned int unaff_EBP = 0; /* stale EBP in original; unresolved */
    unsigned int *puVar3;
    char *pcVar8;
    char *pcVar9;
    char *pcVar10;
    int bVar11;
    short local_e;
    unsigned short local_c;
    unsigned short uStack_a;
    unsigned short t16;
    unsigned short u16;
    unsigned int t32;
    unsigned int p32;

    if (param_1 != 0xffff) {
        __builtin_memcpy(&t16, DAT_0071b73c + (unsigned int)param_4 * 0x5bd, 2);
        uVar7 = (unsigned int)t16;
        bVar11 = 0;
        if ((unsigned char)DAT_0070ad48[(unsigned int)DAT_006d4988[(unsigned int)param_1 * 0x23 + 0x16] * 0x566]
            != uVar7) {
            bVar11 = DAT_0072f508[(unsigned int)(unsigned char)DAT_0070ad48[(unsigned int)DAT_006d4988[(unsigned int)param_1 * 0x23 + 0x16] * 0x566]
                                  + uVar7 * 0x1e5] == '\0';
        }
        if (!bVar11) {
            return;
        }
    }
    local_e = 0;
    local_c = 0xffff;
    uStack_a = DAT_00732a3c;
    if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
        iVar4 = (unsigned int)param_4 * 0x5bd;
        do {
            uVar7 = (unsigned int)uStack_a;
            __builtin_memcpy(&t32, DAT_0071b730 + iVar4, 4);
            if (t32 != uVar7) {
                iVar6 = DAT_006d4988 + uVar7 * 0x23;
                __builtin_memcpy(&t32, iVar6 + 0x1b, 4);
                if ((t32 == 0xffffffffUL) && (iVar6[3] != 0)) {
                    iVar5 = (int)((unsigned int)iVar6[0x16] * 0x566);
                    __builtin_memcpy(&t16, DAT_0071b73c + iVar4, 2);
                    if ((unsigned short)DAT_0070ad48[iVar5] == t16) {
                        __builtin_memcpy(&t32, iVar6 + 0x1f, 4);
                        q = (unsigned char *)t32;
                        __builtin_memcpy(&p32, q + 0xa7, 4);
                        iVar2 = (unsigned char *)p32;
                        if (iVar2[7] == 0) {
                            __builtin_memcpy(&u16, DAT_0071b73e + iVar4, 2);
                            iVar1 = (unsigned int)u16 * 0x566;
                            if ((((unsigned int)iVar6[0x16] == (unsigned int)u16) ||
                                 ((DAT_0070b216[iVar5] != '\0' && (DAT_0070b216[iVar1] == DAT_0070b216[iVar5])))) ||
                                ((unsigned int)DAT_0070ad4b[iVar5] - (unsigned int)((unsigned char)DAT_0070ad54[iVar1] - 1) == 9)) {
                                iVar6 = (unsigned char *)FUN_0042b150((int)iVar2);
                                if (param_2 == 0x9a) {
                                    *(unsigned short *)(iVar2 + 0x342) = param_1;
                                    __builtin_memcpy(&t16, iVar6 + 0x2b, 2);
                                    if (t16 == param_1) {
                                        t16 = 0xffff;
                                        __builtin_memcpy(iVar6 + 0x2b, &t16, 2);
                                    }
LAB_0044375f:
                                    local_e = local_e + 1;
                                    local_c = uStack_a;
                                } else {
                                    pcVar9 = DAT_00703ca0 + (unsigned int)iVar6[10] * 0x55;
                                    iVar5 = 8;
                                    bVar11 = 0;
                                    pcVar8 = pcVar9;
                                    pcVar10 = DAT_00524948;
                                    do {
                                        if (iVar5 == 0) break;
                                        iVar5 = iVar5 + -1;
                                        bVar11 = *pcVar8 == *pcVar10;
                                        pcVar8 = pcVar8 + 1;
                                        pcVar10 = pcVar10 + 1;
                                    } while (bVar11);
                                    if (!bVar11) {
                                        iVar5 = 0x11;
                                        pcVar8 = pcVar9;
                                        pcVar10 = s_stationaryldrpln_00524934;
                                        do {
                                            if (iVar5 == 0) break;
                                            iVar5 = iVar5 + -1;
                                            bVar11 = *pcVar8 == *pcVar10;
                                            pcVar8 = pcVar8 + 1;
                                            pcVar10 = pcVar10 + 1;
                                        } while (bVar11);
                                        if (!bVar11) {
                                            iVar5 = 0x11;
                                            pcVar8 = pcVar9;
                                            pcVar10 = s_stationaryflwpln_00524920;
                                            do {
                                                if (iVar5 == 0) break;
                                                iVar5 = iVar5 + -1;
                                                bVar11 = *pcVar8 == *pcVar10;
                                                pcVar8 = pcVar8 + 1;
                                                pcVar10 = pcVar10 + 1;
                                            } while (bVar11);
                                            if (!bVar11) {
                                                iVar5 = 0xc;
                                                pcVar8 = pcVar9;
                                                pcVar10 = DAT_005250b4;
                                                do {
                                                    if (iVar5 == 0) break;
                                                    iVar5 = iVar5 + -1;
                                                    bVar11 = *pcVar8 == *pcVar10;
                                                    pcVar8 = pcVar8 + 1;
                                                    pcVar10 = pcVar10 + 1;
                                                } while (bVar11);
                                                if (!bVar11) {
                                                    iVar5 = 0xc;
                                                    pcVar8 = pcVar9;
                                                    pcVar10 = DAT_00525b80;
                                                    do {
                                                        if (iVar5 == 0) break;
                                                        iVar5 = iVar5 + -1;
                                                        bVar11 = *pcVar8 == *pcVar10;
                                                        pcVar8 = pcVar8 + 1;
                                                        pcVar10 = pcVar10 + 1;
                                                    } while (bVar11);
                                                    if (!bVar11) {
                                                        iVar5 = 0x11;
                                                        pcVar8 = pcVar9;
                                                        pcVar10 = s_formevadeldr1pln_005250a0;
                                                        do {
                                                            if (iVar5 == 0) break;
                                                            iVar5 = iVar5 + -1;
                                                            bVar11 = *pcVar8 == *pcVar10;
                                                            pcVar8 = pcVar8 + 1;
                                                            pcVar10 = pcVar10 + 1;
                                                        } while (bVar11);
                                                        if (!bVar11) {
                                                            iVar5 = 0x11;
                                                            pcVar8 = pcVar9;
                                                            pcVar10 = s_formevadeflw1pln_00525b6c;
                                                            do {
                                                                if (iVar5 == 0) break;
                                                                iVar5 = iVar5 + -1;
                                                                bVar11 = *pcVar8 == *pcVar10;
                                                                pcVar8 = pcVar8 + 1;
                                                                pcVar10 = pcVar10 + 1;
                                                            } while (bVar11);
                                                            if (!bVar11) {
                                                                iVar5 = 0x10;
                                                                pcVar8 = pcVar9;
                                                                pcVar10 = s_flyhomeevadepln_00525b50;
                                                                do {
                                                                    if (iVar5 == 0) break;
                                                                    iVar5 = iVar5 + -1;
                                                                    bVar11 = *pcVar8 == *pcVar10;
                                                                    pcVar8 = pcVar8 + 1;
                                                                    pcVar10 = pcVar10 + 1;
                                                                } while (bVar11);
                                                                if (!bVar11) {
                                                                    iVar5 = 0x12;
                                                                    pcVar8 = pcVar9;
                                                                    pcVar10 = s_intohyperspacepln_00525b10;
                                                                    do {
                                                                        if (iVar5 == 0) break;
                                                                        iVar5 = iVar5 + -1;
                                                                        bVar11 = *pcVar8 == *pcVar10;
                                                                        pcVar8 = pcVar8 + 1;
                                                                        pcVar10 = pcVar10 + 1;
                                                                    } while (bVar11);
                                                                    if (!bVar11) {
                                                                        iVar5 = 0xf;
                                                                        pcVar8 = pcVar9;
                                                                        pcVar10 = s_enterhangarpln_00523be4;
                                                                        do {
                                                                            if (iVar5 == 0) break;
                                                                            iVar5 = iVar5 + -1;
                                                                            bVar11 = *pcVar8 == *pcVar10;
                                                                            pcVar8 = pcVar8 + 1;
                                                                            pcVar10 = pcVar10 + 1;
                                                                        } while (bVar11);
                                                                        if (!bVar11) {
                                                                            iVar5 = 0x12;
                                                                            pcVar8 = s_craftwaitforgopln_00525b24;
                                                                            do {
                                                                                if (iVar5 == 0) break;
                                                                                iVar5 = iVar5 + -1;
                                                                                bVar11 = *pcVar9 == *pcVar8;
                                                                                pcVar9 = pcVar9 + 1;
                                                                                pcVar8 = pcVar8 + 1;
                                                                            } while (bVar11);
                                                                            if (bVar11) {
                                                                                iVar6[10] = iVar6[0xd];
                                                                                DAT_00732e54 = (int)iVar2;
                                                                                FUN_004296b0(uStack_a);
                                                                                FUN_00429310(uVar7);
                                                                            }
                                                                            __builtin_memcpy(iVar6 + 0x2b, &param_1, 2);
                                                                            if (*(unsigned short *)(iVar2 + 0x342) == param_1) {
                                                                                *(unsigned short *)(iVar2 + 0x342) = 0xffff;
                                                                            }
                                                                            goto LAB_0044375f;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            uStack_a = uStack_a + 1;
        } while ((int)(unsigned int)uStack_a < DAT_006d4980);
    }
    if ((DAT_0071ecf4 == param_4) && (local_c != 0xffff)) {
        __builtin_memcpy(&t32, DAT_006d4988 + (unsigned int)local_c * 0x23 + 0x1f, 4);
        q = (unsigned char *)t32;
        __builtin_memcpy(&p32, q + 0xa7, 4);
        puVar3 = (unsigned int *)p32;
        if (local_e == 1) {
            FUN_004a35b0(((unsigned int)uStack_a << 16) | (unsigned int)local_c, puVar3, param_2, param_3, 0, unaff_EBP);
            return;
        }
        FUN_004a35b0(((unsigned int)uStack_a << 16) | (unsigned int)local_c, puVar3, param_2, param_3, 1, unaff_EBP);
    }
    return;
}
