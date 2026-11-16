/* SPDX-License-Identifier: MIT */
/* z_bop 004f9750 */
/* Prototypes */
void  FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004ddc10(int param_1);
void  FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
int   FUN_004d11e0(void);
void  FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
void  FUN_004d4cf0(unsigned int *param_1);
void  FUN_004d4d20(void *param_1);
int   _sprintf(char *, const char *, ...);
unsigned char FUN_004dc2e0(int param_1);
void  FUN_004de090(int *param_1, int *param_2);
unsigned char FUN_004dc1e0(void);
unsigned char FUN_004dc200(void);
int   FUN_004d6b90(int *param_1, int param_2, int param_3);
void  FUN_004dc1c0(int param_1);
void  FUN_004cef70(int param_1, unsigned int *param_2, int param_3);
void  FUN_004d64a0(int *param_1, int param_2, int param_3);
unsigned char FUN_004dc2a0(int param_1);
void  FUN_004de5a0(char *param_1, int param_2, int param_3, int param_4);
unsigned char FUN_004dc2c0(int param_1);
void  FUN_004dc1d0(void);

/* Globals */
extern int          DAT_00aa5e10[];
extern unsigned int DAT_00aa5cb0[];
extern int          DAT_00aa5ac0[];
extern unsigned int DAT_00aa5e40[];
extern unsigned int DAT_00aa5e38;
extern int          DAT_00aa5e60;
extern int          DAT_00aa5aac;
extern unsigned char DAT_00b6a5a6[4];      /* unaligned int */
extern unsigned char DAT_00aa6150[];
extern unsigned char DAT_00aa615e[];       /* unaligned ints, stride 0x2a */
extern unsigned char DAT_00aa6162[];       /* unaligned ints, stride 0x2a */
extern unsigned char DAT_00aa62ae[];
extern unsigned int DAT_00aa6af0;
extern unsigned int DAT_00aa6af4;
extern unsigned int DAT_00aa6af8;
extern unsigned int DAT_00aa6afc;
extern char         DAT_00b69d20[];
extern int          DAT_00b69e30;
extern unsigned int DAT_00b69e38;
extern unsigned int DAT_00b6a2a0;
extern unsigned int DAT_00b6a2c0;
extern unsigned int DAT_00b69ccc;
extern unsigned char DAT_00b6a270[];
extern char         DAT_00bb73a7;
extern char         DAT_00a91d75[];

#define RD32(p) ({ int _t; __builtin_memcpy(&_t, (p), 4); _t; })
#define SEL RD32(DAT_00b6a5a6)

int FUN_004f9750(int param_1)
{
    int iVar1;
    char cVar2;
    unsigned char bVar3;
    unsigned char uVar4;
    int iVar5;
    char *pcVar6;
    unsigned int *puVar7;
    char *pcVar8;
    unsigned char bVar9;
    int *piVar10;
    unsigned char *pbVar10;
    char *pcVar11;
    int iVar12;
    int iVar13;
    unsigned int uVar14;
    int iVar15;
    int iVar16;
    unsigned int uVar17;
    unsigned int uVar18;
    char *local_44;
    int local_40;
    int local_3c;
    int local_38;
    int local_34[4];
    unsigned int local_24;
    int local_20[4];
    unsigned int local_10[4];

    iVar5 = -0x44;
    if (DAT_00aa5e10[SEL] < 5) {
        iVar5 = 0;
    }
    iVar13 = iVar5 + 0x15d;
    iVar16 = iVar5 + 0x14f;
    FUN_004d6460(local_34, 0x58, iVar16, 0xe5, iVar13);
    piVar10 = local_34;
    uVar18 = 0xffff;
    iVar15 = 1;
    iVar12 = 0;
    pcVar6 = FUN_004ddc10(0x273);
    FUN_004db7d0(0xc, pcVar6, piVar10, iVar12, iVar15, uVar18);
    FUN_004d6460(local_34, 0xe8, iVar16, 0x150, iVar13);
    piVar10 = local_34;
    uVar18 = 0xffff;
    iVar15 = 1;
    iVar12 = 0;
    pcVar6 = FUN_004ddc10(0x274);
    FUN_004db7d0(0xc, pcVar6, piVar10, iVar12, iVar15, uVar18);
    FUN_004d6460(local_34, 0x153, iVar16, 0x1bb, iVar13);
    piVar10 = local_34;
    uVar18 = 0xffff;
    iVar16 = 1;
    iVar13 = 0;
    pcVar6 = FUN_004ddc10(0x2ad);
    FUN_004db7d0(0xc, pcVar6, piVar10, iVar13, iVar16, uVar18);
    iVar13 = FUN_004d11e0();
    local_24 = (unsigned int)(iVar13 == (int)DAT_00aa5cb0[SEL * 8]);
    if (1 < DAT_00aa5e10[SEL]) {
        FUN_004d6460(local_34, 0x58, 0x1a6, 0x1bb, 0x1b4);
        if ((local_24 == 0) && (DAT_00b69e30 != 2)) {
            uVar14 = 0x276;
            uVar18 = DAT_00b69e38;
        }
        else {
            uVar14 = 0x275;
            uVar18 = DAT_00b6a2a0;
        }
        piVar10 = local_34;
        pcVar6 = FUN_004ddc10(uVar14);
        FUN_004db640(0xc, pcVar6, piVar10, uVar18);
    }
    local_40 = 0;
    iVar16 = 0;
    FUN_004d6460(local_34, 0x153, iVar5 + 0x160, 0x1bb, iVar5 + 0x170);
    iVar13 = DAT_00aa5e60;
    if (0 < DAT_00aa5e10[SEL]) {
        do {
            iVar15 = 0;
            pbVar10 = DAT_00aa615e;
            iVar12 = iVar16 + SEL * 8;
            do {
                if (RD32(pbVar10) == (int)DAT_00aa5cb0[iVar12]) break;
                pbVar10 = pbVar10 + 0x2a;
                iVar15 = iVar15 + 1;
            } while (pbVar10 < DAT_00aa62ae);
            if ((iVar15 != 8) && (DAT_00aa5ac0[iVar12] == -1)) {
                iVar1 = iVar15 * 0x2a;
                uVar18 = (unsigned int)RD32(DAT_00aa615e + iVar15 * 0x2a);
                if ((DAT_00aa5e38 != uVar18) && (DAT_00aa5cb0[iVar12] != 0)) {
                    iVar12 = 0;
                    if (0 < iVar13) {
                        puVar7 = DAT_00aa5e40;
                        do {
                            if (*puVar7 == uVar18) break;
                            puVar7 = puVar7 + 1;
                            iVar12 = iVar12 + 1;
                        } while (iVar12 < iVar13);
                    }
                    FUN_004d4cf0(local_10);
                    FUN_004d4d20(local_34);
                    iVar13 = (int)(DAT_00aa6150 + iVar1);
                    if (iVar12 == DAT_00aa5e60) {
                        uVar17 = 1;
                        pcVar6 = FUN_004ddc10(RD32(DAT_00aa6162 + iVar1) + 0x9a);
                        _sprintf(DAT_00b69d20, "%c%s %c%s", 6, pcVar6, uVar17, (char *)iVar13);
                        iVar13 = FUN_004d11e0();
                        if ((iVar13 == RD32(DAT_00aa615e + iVar1)) ||
                            (uVar18 = DAT_00b6a2c0, DAT_00b69e30 == 2)) {
                            uVar18 = *(unsigned int *)(DAT_00b6a270 + (((unsigned int)(param_1 % 0x18)) & 0xfffffffeU) * 2);
                        }
                    }
                    else {
                        pcVar6 = FUN_004ddc10(RD32(DAT_00aa6162 + iVar1) + 0x9a);
                        _sprintf(DAT_00b69d20, "%s %s", pcVar6, (char *)iVar13);
                        uVar18 = DAT_00b69ccc;
                    }
                    FUN_004db7d0(0xc, DAT_00b69d20, local_34, 0, 1, uVar18);
                    FUN_004d4d20(local_10);
                    local_40 = local_40 + 1;
                    iVar13 = FUN_004d11e0();
                    if (((iVar13 == (int)DAT_00aa5cb0[SEL * 8]) || (DAT_00b69e30 == 2)) &&
                        ((bVar3 = FUN_004dc2e0(3)) == 0)) {
                        FUN_004de090(&local_38, &local_3c);
                        uVar4 = FUN_004dc1e0();
                        if (((uVar4 != 0) ||
                             (uVar4 = FUN_004dc200(), uVar4 != 0)) &&
                            (iVar13 = FUN_004d6b90(local_34, local_38, local_3c), iVar13 != 0)) {
                            if (DAT_00b69e30 == 2) {
                                DAT_00aa5e38 = (unsigned int)RD32(DAT_00aa615e + iVar1);
                                FUN_004dc1c0(3);
                            }
                            else {
                                iVar13 = 0;
                                if (0 < DAT_00aa5e60) {
                                    puVar7 = DAT_00aa5e40;
                                    do {
                                        if ((int)*puVar7 == RD32(DAT_00aa615e + iVar1)) break;
                                        puVar7 = puVar7 + 1;
                                        iVar13 = iVar13 + 1;
                                    } while (iVar13 < DAT_00aa5e60);
                                }
                                if (iVar13 == DAT_00aa5e60) {
                                    DAT_00aa6af4 = (unsigned int)RD32(DAT_00aa615e + iVar1);
                                    DAT_00aa5e60 = DAT_00aa5e60 + 1;
                                    DAT_00aa5e40[iVar13] = DAT_00aa6af4;
                                    DAT_00aa6af0 = 0x6b;
                                    DAT_00aa6af8 = FUN_004d11e0();
                                    FUN_004cef70(0, &DAT_00aa6af0, 0xc);
                                    DAT_00aa5e38 = (unsigned int)RD32(DAT_00aa615e + iVar1);
                                    iVar13 = 0;
                                    FUN_004dc1c0(3);
                                    uVar18 = DAT_00aa5e38;
                                    if (0 < DAT_00aa5e10[SEL]) {
                                        do {
                                            iVar12 = iVar13 + SEL * 8;
                                            if ((DAT_00aa5cb0[iVar12] == uVar18) && (DAT_00aa5ac0[iVar12] != -1)) {
                                                DAT_00aa6af0 = 0x50;
                                                DAT_00aa6af4 = SEL;
                                                DAT_00aa6afc = (unsigned int)-1;
                                                DAT_00aa6af8 = iVar13;
                                                FUN_004cef70(0, &DAT_00aa6af0, 0x14);
                                                uVar18 = DAT_00aa5e38;
                                            }
                                            iVar13 = iVar13 + 1;
                                        } while (iVar13 < DAT_00aa5e10[SEL]);
                                    }
                                }
                            }
                        }
                    }
                    FUN_004d64a0(local_34, 0, 0x11);
                    iVar13 = DAT_00aa5e60;
                    if (3 < local_40) break;
                }
            }
            iVar16 = iVar16 + 1;
        } while (iVar16 < DAT_00aa5e10[SEL]);
    }
    local_40 = 0;
    FUN_004de090(&local_38, &local_3c);
    FUN_004d6460(local_34, 0x58, iVar5 + 0x160, 0xe5, iVar5 + 0x170);
    iVar13 = 0;
    FUN_004d6460(local_20, 0xe8, iVar5 + 0x160, 0x14c, iVar5 + 0x170);
    if (0 < DAT_00aa5aac) {
        local_44 = DAT_00a91d75;
        do {
            if ((local_44[-0x51] != '\0') && ((unsigned char)local_44[-0x5d] == SEL)) {
                switch (local_44[-0x5e]) {
                case '\0':
                    bVar9 = 2;
                    break;
                case '\x01':
                case '\x04':
                    bVar9 = 3;
                    break;
                case '\x02':
                    bVar9 = 5;
                    break;
                case '\x03':
                    bVar9 = 4;
                    break;
                case '\x05':
                    bVar9 = 6;
                    break;
                default:
                    bVar9 = 1;
                }
                uVar17 = 1;
                pcVar6 = local_44 + -0xb5;
                pcVar8 = FUN_004ddc10((unsigned char)local_44[-99] + 0x261);
                _sprintf(DAT_00b69d20, "%u. %c%s %s: %c", local_40 + 1, (unsigned int)bVar9, pcVar8,
                         pcVar6, uVar17);
                pcVar6 = local_44;
                if (*local_44 == '\0') {
                    pcVar6 = FUN_004ddc10(0x272);
                }
                uVar18 = 0xffffffff;
                do {
                    pcVar8 = pcVar6;
                    if (uVar18 == 0) break;
                    uVar18 = uVar18 - 1;
                    pcVar8 = pcVar6 + 1;
                    cVar2 = *pcVar6;
                    pcVar6 = pcVar8;
                } while (cVar2 != '\0');
                uVar18 = ~uVar18;
                iVar5 = -1;
                pcVar6 = DAT_00b69d20;
                do {
                    pcVar11 = pcVar6;
                    if (iVar5 == 0) break;
                    iVar5 = iVar5 + -1;
                    pcVar11 = pcVar6 + 1;
                    cVar2 = *pcVar6;
                    pcVar6 = pcVar11;
                } while (cVar2 != '\0');
                pcVar6 = pcVar8 + -uVar18;
                pcVar8 = pcVar11 + -1;
                for (uVar14 = uVar18 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
                    __builtin_memcpy(pcVar8, pcVar6, 4);
                    pcVar6 = pcVar6 + 4;
                    pcVar8 = pcVar8 + 4;
                }
                for (uVar18 = uVar18 & 3; uVar18 != 0; uVar18 = uVar18 - 1) {
                    *pcVar8 = *pcVar6;
                    pcVar6 = pcVar6 + 1;
                    pcVar8 = pcVar8 + 1;
                }
                iVar5 = -1;
                iVar16 = 0;
                FUN_004db7d0(0xc, DAT_00b69d20, local_34, 0, 1, 0xffff);
                if (0 < DAT_00aa5e10[SEL]) {
                    do {
                        iVar12 = iVar16 + SEL * 8;
                        if (DAT_00aa5ac0[iVar12] == iVar13) {
                            iVar15 = 0;
                            pbVar10 = DAT_00aa615e;
                            do {
                                iVar5 = iVar16;
                                if (RD32(pbVar10) == (int)DAT_00aa5cb0[iVar12]) {
                                    iVar12 = (int)(DAT_00aa6150 + iVar15 * 0x2a);
                                    uVar17 = 1;
                                    pcVar6 = FUN_004ddc10(RD32(DAT_00aa6162 + iVar15 * 0x2a) + 0x9a);
                                    _sprintf(DAT_00b69d20, "%c%s %c%s", 6, pcVar6, uVar17, (char *)iVar12);
                                    FUN_004d4cf0(local_10);
                                    FUN_004d4d20(local_20);
                                    iVar12 = FUN_004d11e0();
                                    if ((iVar12 == RD32(DAT_00aa615e + iVar15 * 0x2a)) ||
                                        (uVar18 = DAT_00b6a2c0, DAT_00b69e30 == 2)) {
                                        uVar18 = *(unsigned int *)(DAT_00b6a270 + (((unsigned int)(param_1 % 0x18)) & 0xfffffffeU) * 2);
                                    }
                                    FUN_004db7d0(0xc, DAT_00b69d20, local_20, 0, 1, uVar18);
                                    FUN_004d4d20(local_10);
                                    break;
                                }
                                pbVar10 = pbVar10 + 0x2a;
                                iVar15 = iVar15 + 1;
                            } while (pbVar10 < DAT_00aa62ae);
                        }
                        iVar16 = iVar16 + 1;
                    } while (iVar16 < DAT_00aa5e10[SEL]);
                }
                if (1 < DAT_00aa5e10[SEL]) {
                    if (iVar5 == -1) {
                        bVar3 = FUN_004dc2e0(3);
                        if ((bVar3 != 0) &&
                            (((uVar4 = FUN_004dc2a0(3), uVar4 != 0) ||
                              (uVar4 = FUN_004dc2a0(3), uVar4 != 0)) &&
                             ((iVar5 = FUN_004d6b90(local_20, local_38, local_3c)) != 0))) {
                            if (DAT_00b69e30 == 2) {
                                if (DAT_00bb73a7 != '\0') {
                                    FUN_004de5a0("slotsound", 1, 0, 0xff);
                                }
                                iVar5 = 0;
                                if (0 < DAT_00aa5e10[SEL]) {
                                    do {
                                        if (DAT_00aa5cb0[iVar5 + SEL * 8] == DAT_00aa5e38) {
                                            DAT_00aa5ac0[SEL * 8 + iVar5] = iVar13;
                                            break;
                                        }
                                        iVar5 = iVar5 + 1;
                                    } while (iVar5 < DAT_00aa5e10[SEL]);
                                }
                            }
                            else {
                                DAT_00aa6af8 = 0;
                                if (0 < DAT_00aa5e10[SEL]) {
                                    do {
                                        if (DAT_00aa5cb0[DAT_00aa6af8 + SEL * 8] == DAT_00aa5e38) break;
                                        DAT_00aa6af8 = DAT_00aa6af8 + 1;
                                    } while ((int)DAT_00aa6af8 < DAT_00aa5e10[SEL]);
                                }
                                DAT_00aa6af4 = SEL;
                                DAT_00aa6af0 = 0x50;
                                DAT_00aa6afc = iVar13;
                                FUN_004cef70(0, &DAT_00aa6af0, 0x10);
                                DAT_00aa6af0 = 0x6a;
                                DAT_00aa6af4 = DAT_00aa5e38;
                                FUN_004cef70(0, &DAT_00aa6af0, 8);
                            }
LAB_004fa3ac:
                            DAT_00aa5e38 = 0;
                            FUN_004dc1d0();
                        }
                    }
                    else {
                        bVar3 = FUN_004dc2e0(3);
                        if (bVar3 == 0) {
                            if ((((local_24 != 0) || (DAT_00b69e30 == 2)) &&
                                 ((uVar4 = FUN_004dc1e0(), uVar4 != 0) ||
                                  (uVar4 = FUN_004dc200(), uVar4 != 0))) &&
                                ((iVar16 = FUN_004d6b90(local_20, local_38, local_3c)) != 0)) {
                                if (DAT_00b69e30 == 2) {
                                    DAT_00aa5e38 = DAT_00aa5cb0[iVar5 + SEL * 8];
                                    FUN_004dc1c0(3);
                                    DAT_00aa5ac0[iVar5 + SEL * 8] = -1;
                                }
                                else {
                                    iVar16 = 0;
                                    if (0 < DAT_00aa5e60) {
                                        puVar7 = DAT_00aa5e40;
                                        do {
                                            if (*puVar7 == DAT_00aa5cb0[iVar5 + SEL * 8]) break;
                                            puVar7 = puVar7 + 1;
                                            iVar16 = iVar16 + 1;
                                        } while (iVar16 < DAT_00aa5e60);
                                    }
                                    if (iVar16 == DAT_00aa5e60) {
                                        DAT_00aa5e60 = DAT_00aa5e60 + 1;
                                        DAT_00aa6af0 = 0x6b;
                                        DAT_00aa6af4 = DAT_00aa5cb0[iVar5 + SEL * 8];
                                        DAT_00aa5e40[iVar16] = DAT_00aa6af4;
                                        DAT_00aa6af8 = FUN_004d11e0();
                                        FUN_004cef70(0, &DAT_00aa6af0, 0xc);
                                        iVar16 = 0;
                                        DAT_00aa5e38 = DAT_00aa5cb0[iVar5 + SEL * 8];
                                        FUN_004dc1c0(3);
                                        uVar18 = DAT_00aa5e38;
                                        if (0 < DAT_00aa5e10[SEL]) {
                                            do {
                                                iVar5 = iVar16 + SEL * 8;
                                                if ((DAT_00aa5cb0[iVar5] == uVar18) && (DAT_00aa5ac0[iVar5] != -1)) {
                                                    DAT_00aa6af0 = 0x50;
                                                    DAT_00aa6af4 = SEL;
                                                    DAT_00aa6afc = (unsigned int)-1;
                                                    DAT_00aa6af8 = iVar16;
                                                    FUN_004cef70(0, &DAT_00aa6af0, 0x14);
                                                    uVar18 = DAT_00aa5e38;
                                                }
                                                iVar16 = iVar16 + 1;
                                            } while (iVar16 < DAT_00aa5e10[SEL]);
                                        }
                                    }
                                }
                            }
                        }
                        else {
                            uVar4 = FUN_004dc2a0(3);
                            if (((uVar4 != 0) ||
                                 (uVar4 = FUN_004dc2c0(3), uVar4 != 0)) &&
                                ((iVar16 = FUN_004d6b90(local_20, local_38, local_3c)) != 0)) {
                                if (DAT_00b69e30 == 2) {
                                    if (DAT_00bb73a7 != '\0') {
                                        FUN_004de5a0("slotsound", 1, 0, 0xff);
                                    }
                                    iVar15 = 0;
                                    iVar12 = SEL * 8;
                                    iVar16 = DAT_00aa5e10[SEL];
                                    DAT_00aa5ac0[iVar5 + iVar12] = -1;
                                    if (0 < iVar16) {
                                        do {
                                            if (DAT_00aa5cb0[iVar15 + iVar12] == DAT_00aa5e38) {
                                                DAT_00aa5ac0[iVar15 + iVar12] = iVar13;
                                                break;
                                            }
                                            iVar15 = iVar15 + 1;
                                        } while (iVar15 < iVar16);
                                    }
                                }
                                else {
                                    DAT_00aa6af4 = SEL;
                                    DAT_00aa6af0 = 0x50;
                                    DAT_00aa6afc = 0xffffffff;
                                    DAT_00aa6af8 = iVar5;
                                    FUN_004cef70(0, &DAT_00aa6af0, 0x14);
                                    DAT_00aa6af8 = 0;
                                    if (0 < DAT_00aa5e10[SEL]) {
                                        do {
                                            if (DAT_00aa5cb0[DAT_00aa6af8 + SEL * 8] == DAT_00aa5e38) break;
                                            DAT_00aa6af8 = DAT_00aa6af8 + 1;
                                        } while ((int)DAT_00aa6af8 < DAT_00aa5e10[SEL]);
                                    }
                                    DAT_00aa6af4 = SEL;
                                    DAT_00aa6af0 = 0x50;
                                    DAT_00aa6afc = iVar13;
                                    FUN_004cef70(0, &DAT_00aa6af0, 0x10);
                                    DAT_00aa6af0 = 0x6a;
                                    DAT_00aa6af4 = DAT_00aa5e38;
                                    FUN_004cef70(0, &DAT_00aa6af0, 8);
                                }
                                goto LAB_004fa3ac;
                            }
                        }
                    }
                }
                FUN_004d64a0(local_20, 0, 0x11);
                FUN_004d64a0(local_34, 0, 0x11);
                local_40 = local_40 + 1;
            }
            local_44 = local_44 + 0x562;
            iVar13 = iVar13 + 1;
        } while (iVar13 < DAT_00aa5aac);
    }
    return 1;
}
