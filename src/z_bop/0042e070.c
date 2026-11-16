/* SPDX-License-Identifier: MIT */
/* z_bop 0042e070 */
extern char DAT_009a8c10;
extern char DAT_00bb737d;
extern char DAT_00bb7375;
extern char DAT_00bb7376;
extern char DAT_00bb7374;
extern char DAT_00bb7377;
extern unsigned int DAT_005210b4;
extern unsigned short DAT_005210b8;
extern short DAT_009e967a;
extern int DAT_009ecc34;
extern short DAT_009a2000;
extern char DAT_009d8c94;
extern char DAT_009d9192;
extern unsigned char DAT_009d8c83[];
extern unsigned int DAT_005210bc[3];
extern unsigned int DAT_005210c8[3];
extern char DAT_0052111c[];
extern char DAT_005210ec[];
extern char s_RTO2_LST_00521110[];
extern char s_ITO2_LST_005210f8[];
extern char s_ITO1_LST_00521104[];
extern char s_ICMD_LST_005210e0[];
extern char s_PCMD_LST_005210d4[];
extern char s_DEMO_TIE_00523448[];

unsigned int FUN_00425d10(void);
void FUN_0042ded0(char *param_1, int param_2);
unsigned short FUN_004305d0(int param_1);

void __stdcall FUN_0042e070(void)
{
    unsigned int bufw[16];
    char *buf = (char *)bufw;
    char path[0x40];
    char tmp[12];
    char cVar1;
    short sVar2;
    unsigned short uVar3;
    unsigned int uVar4;
    int iVar5;
    int iVar7;
    int iVar8;
    unsigned int uVar9;
    unsigned int uVar11;
    unsigned short uVar12;
    char *pcVar13;
    char *pcVar14;
    char *pcVar15;
    unsigned int *puVar10;
    unsigned char *pb;
    unsigned char *s;
    int iVar_cnt;
    int bVar17;

    if ((DAT_009a8c10 != '\0') && (DAT_00bb737d != '\0')) {
        if (DAT_00bb7375 != '\0') {
            bufw[0] = DAT_005210b4;
            *(unsigned short *)(buf + 4) = DAT_005210b8;
            pb = (unsigned char *)&DAT_009e967a + (unsigned int)DAT_009ecc34 * 0x5bd;
            __builtin_memcpy(&sVar2, pb, 2);
            if (sVar2 == 0) {
                uVar4 = FUN_00425d10();
                if ((uVar4 & 1) == 0) {
                    pcVar13 = s_RTO2_LST_00521110;
                } else {
                    pcVar13 = DAT_0052111c;
                }
            } else {
                uVar4 = FUN_00425d10();
                if ((uVar4 & 1) == 0) {
                    pcVar13 = s_ITO2_LST_005210f8;
                } else {
                    pcVar13 = s_ITO1_LST_00521104;
                }
            }
            uVar4 = 0xffffffff;
            do {
                pcVar15 = pcVar13;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar15 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar15;
            } while (cVar1 != '\0');
            uVar4 = ~uVar4;
            iVar8 = -1;
            pcVar13 = buf;
            do {
                pcVar14 = pcVar13;
                if (iVar8 == 0) break;
                iVar8 = iVar8 + -1;
                pcVar14 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar14;
            } while (cVar1 != '\0');
            pcVar13 = pcVar15 + -uVar4;
            pcVar15 = pcVar14 + -1;
            for (uVar9 = uVar4 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar15, pcVar13, 4);
                pcVar13 = pcVar13 + 4;
                pcVar15 = pcVar15 + 4;
            }
            for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar15 = *pcVar13;
                pcVar13 = pcVar13 + 1;
                pcVar15 = pcVar15 + 1;
            }
            FUN_0042ded0(buf, 0x2b8);
        }
        if (DAT_00bb7376 != '\0') {
            bufw[0] = DAT_005210b4;
            *(unsigned short *)(buf + 4) = DAT_005210b8;
            pb = (unsigned char *)&DAT_009e967a + (unsigned int)DAT_009ecc34 * 0x5bd;
            __builtin_memcpy(&sVar2, pb, 2);
            if (sVar2 == 0) {
                pcVar13 = DAT_005210ec;
            } else if (sVar2 == 1) {
                pcVar13 = s_ICMD_LST_005210e0;
            } else {
                pcVar13 = s_PCMD_LST_005210d4;
            }
            uVar4 = 0xffffffff;
            do {
                pcVar15 = pcVar13;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar15 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar15;
            } while (cVar1 != '\0');
            uVar4 = ~uVar4;
            iVar8 = -1;
            pcVar13 = buf;
            do {
                pcVar14 = pcVar13;
                if (iVar8 == 0) break;
                iVar8 = iVar8 + -1;
                pcVar14 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar14;
            } while (cVar1 != '\0');
            pcVar13 = pcVar15 + -uVar4;
            pcVar15 = pcVar14 + -1;
            for (uVar9 = uVar4 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar15, pcVar13, 4);
                pcVar13 = pcVar13 + 4;
                pcVar15 = pcVar15 + 4;
            }
            for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar15 = *pcVar13;
                pcVar13 = pcVar13 + 1;
                pcVar15 = pcVar15 + 1;
            }
            FUN_0042ded0(buf, 0x324);
        }
        if (DAT_00bb7374 != '\0') {
            iVar8 = 0;
            if (0 < DAT_009a2000) {
                iVar5 = (int)DAT_009a2000;
                pcVar13 = &DAT_009d8c94;
                do {
                    if (*pcVar13 != '\0') {
                        iVar8 = iVar8 + 1;
                    }
                    pcVar13 = pcVar13 + 0x566;
                    iVar5 = iVar5 + -1;
                } while (iVar5 != 0);
            }
            iVar5 = 0;
            if (0 < DAT_009a2000) {
                unsigned char *pId = (unsigned char *)&DAT_009d9192;
                int idv;
                do {
                    __builtin_memcpy(&idv, pId, 4);
                    if (idv == DAT_009ecc34) break;
                    pId = pId + 0x566;
                    iVar5 = iVar5 + 1;
                } while (iVar5 < DAT_009a2000);
            }
            if ((iVar5 < DAT_009a2000) && ((1 < DAT_009d8c83[iVar5 * 0x566]) || (1 < iVar8))) {
                iVar_cnt = (int)(unsigned int)DAT_009d8c83[iVar5 * 0x566];
                uVar3 = FUN_004305d0(6);
                bVar17 = iVar_cnt != 0;
                iVar_cnt = iVar_cnt - 1;
                if (bVar17) {
                    uVar12 = 0x72;
                    do {
                        bufw[0] = DAT_005210b4;
                        *(unsigned short *)(buf + 4) = DAT_005210b8;
                        pb = (unsigned char *)&DAT_009e967a + (unsigned int)DAT_009ecc34 * 0x5bd;
                        __builtin_memcpy(&sVar2, pb, 2);
                        if (sVar2 == 1) {
                            puVar10 = DAT_005210c8;
                        } else {
                            puVar10 = DAT_005210bc;
                        }
                        s = (unsigned char *)puVar10;
                        tmp[0] = (char)s[0];
                        tmp[1] = (char)s[1];
                        tmp[2] = (char)s[2];
                        tmp[3] = (char)((char)uVar3 + '1');
                        tmp[4] = (char)s[4];
                        tmp[5] = (char)s[5];
                        tmp[6] = (char)s[6];
                        tmp[7] = (char)s[7];
                        tmp[8] = (char)s[8];
                        uVar4 = 0xffffffff;
                        pcVar13 = tmp;
                        do {
                            pcVar15 = pcVar13;
                            if (uVar4 == 0) break;
                            uVar4 = uVar4 - 1;
                            pcVar15 = pcVar13 + 1;
                            cVar1 = *pcVar13;
                            pcVar13 = pcVar15;
                        } while (cVar1 != '\0');
                        uVar4 = ~uVar4;
                        iVar8 = -1;
                        pcVar13 = buf;
                        do {
                            pcVar14 = pcVar13;
                            if (iVar8 == 0) break;
                            iVar8 = iVar8 + -1;
                            pcVar14 = pcVar13 + 1;
                            cVar1 = *pcVar13;
                            pcVar13 = pcVar14;
                        } while (cVar1 != '\0');
                        uVar3 = uVar3 + 1;
                        pcVar13 = pcVar15 + -uVar4;
                        pcVar15 = pcVar14 + -1;
                        for (uVar9 = uVar4 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                            __builtin_memcpy(pcVar15, pcVar13, 4);
                            pcVar13 = pcVar13 + 4;
                            pcVar15 = pcVar15 + 4;
                        }
                        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                            *pcVar15 = *pcVar13;
                            pcVar13 = pcVar13 + 1;
                            pcVar15 = pcVar15 + 1;
                        }
                        FUN_0042ded0(buf, uVar12);
                        if (5 < uVar3) {
                            uVar3 = 0;
                        }
                        bVar17 = iVar_cnt != 0;
                        iVar_cnt = iVar_cnt + -1;
                        uVar12 = (unsigned short)(uVar12 + 0x61);
                    } while (bVar17);
                }
            }
        }
        if (DAT_00bb7377 != '\0') {
            bufw[0] = DAT_005210b4;
            *(unsigned short *)(buf + 4) = DAT_005210b8;
            uVar4 = 0xffffffff;
            pcVar13 = s_DEMO_TIE_00523448;
            do {
                pcVar15 = pcVar13;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar15 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar15;
            } while (cVar1 != '\0');
            uVar4 = ~uVar4;
            pcVar13 = pcVar15 + -uVar4;
            pcVar15 = path;
            for (uVar9 = uVar4 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar15, pcVar13, 4);
                pcVar13 = pcVar13 + 4;
                pcVar15 = pcVar15 + 4;
            }
            for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar15 = *pcVar13;
                pcVar13 = pcVar13 + 1;
                pcVar15 = pcVar15 + 1;
            }
            uVar4 = 0xffffffff;
            pcVar13 = path;
            do {
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar13 + 1;
            } while (cVar1 != '\0');
            uVar4 = ~uVar4;
            iVar5 = 0;
            iVar8 = (int)(uVar4 - 1);
            path[uVar4 - 4] = 'l';
            path[uVar4 - 3] = 's';
            path[uVar4 - 2] = 't';
            while (path[0] != '\\') {
                if (iVar8 <= iVar5) goto LAB_0042e40d;
                iVar7 = iVar5 + 1;
                iVar5 = iVar5 + 1;
                path[0] = path[iVar7];
            }
            if (iVar5 < iVar8) {
                iVar5 = iVar5 + 1;
            } else {
LAB_0042e40d:
                iVar5 = 0;
            }
            uVar4 = 0xffffffff;
            pcVar13 = buf;
            do {
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar13 + 1;
            } while (cVar1 != '\0');
            iVar7 = (int)(~uVar4 - 1);
            if (iVar5 < iVar8) {
                uVar11 = (unsigned int)(iVar8 - iVar5);
                pcVar13 = path + iVar5;
                pcVar14 = buf + (~uVar4 - 1);
                for (uVar9 = uVar11 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    __builtin_memcpy(pcVar14, pcVar13, 4);
                    pcVar13 = pcVar13 + 4;
                    pcVar14 = pcVar14 + 4;
                }
                for (uVar4 = uVar11 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                    *pcVar14 = *pcVar13;
                    pcVar13 = pcVar13 + 1;
                    pcVar14 = pcVar14 + 1;
                }
                iVar7 = (int)((unsigned int)iVar7 + uVar11);
            }
            buf[iVar7] = 0;
            FUN_0042ded0(buf, 0x5f);
        }
    }
    return;
}
