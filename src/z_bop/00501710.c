/* SPDX-License-Identifier: MIT */
/* z_bop 00501710 */
/* Declarations: unaligned globals (address % 4 == 2) are declared as bytes
   and are only read through __builtin_memcpy. */
extern int DAT_00b69e30;
extern int DAT_00b69e38;
extern int DAT_00b6a2c0;
extern char DAT_00aa477b;
extern char DAT_00b69d20[];
extern int DAT_00bb2810;
extern unsigned char DAT_00a91d18;

extern int DAT_0066d918;
extern int DAT_0066da70;
extern int DAT_0066da6c;
extern int DAT_0066da50;
extern int DAT_0066da54;
extern int DAT_0066da60;
extern int DAT_0066da64;
extern int DAT_0066d9e8;
extern int DAT_0066d958;
extern int DAT_0066da40;
extern int DAT_0066d948;
extern int DAT_0066d8c8;
extern int DAT_0066d908;
extern int DAT_0066d9b4;
extern int DAT_0066d9b0;
extern int DAT_0066d9e4;
extern int DAT_0066d9f4;
extern int DAT_0066da4c;
extern int DAT_0066d904;
extern int DAT_0066d900;

extern char DAT_00b6d5aa;
extern char DAT_00b6d86a;
extern char DAT_00b6a5aa;
extern char DAT_00b6a5a6;
extern char DAT_00b6a616;
extern char DAT_00b6da12;
extern char DAT_00b6da1a;
extern char DAT_00b6d962;
extern char DAT_00b6c13e;
extern char DAT_00b6c2ce;
extern char DAT_00b6bcc2;
extern char DAT_00b6bce2;
extern char DAT_00b6be82;
extern char DAT_00b6bea2;
extern char DAT_00b6ca9e;
extern char DAT_00b6cf4e;
extern char DAT_00b6d2d2;
extern char DAT_00b6d416;
extern char DAT_00b6d542;
extern char DAT_00b6d84e;
extern char DAT_00b6d856;
extern char DAT_00b6d5a6;
extern char DAT_00b6d5ae;
extern char DAT_00b6d5b2;
extern char DAT_00b6bc32;
extern char DAT_00b6c126;
extern char DAT_00b6d3aa;
extern char DAT_00b6d3b6;
extern char DAT_00b6d3c2;
extern char DAT_00b6d3ce;
extern char DAT_00b6d3da;
extern char DAT_00b6d3e6;
extern char DAT_00b6d3f2;
extern char DAT_00b6d3fe;
extern char DAT_00b6d40a;

extern char s__c_s__c_s__c_s__c_d__c_s_0052d4e8[];
extern char s__c_s__c_s_0052a6b0[];
extern char s__c_s__c_s__s__0052d4d8[];
extern char s__c_s__c_d_0052c518[];
extern char s_rank_d_0052b4e0[];
extern char s_rcitlvl_d_0052b4b4[];
extern char s_citlvl_d_0052b4a8[];
extern char s_medlvl_d_0052b49c[];
extern char s__s____s_0052d4d0[];
extern char s__d___d__0052b464[];
extern char s__d__d__0052b450[];
extern char DAT_0052b46c[];
extern char DAT_0052b460[];
extern char DAT_0052b458[];

int   _sprintf(char *buf, const char *fmt, ...);
void  FUN_004d6460(int *rect, int a, int b, int c, int d);
char *FUN_004ddc10(unsigned int id);
void  FUN_004db640(int a, char *s, int *rect, unsigned int col);
int   FUN_004d11e0(void);
int   FUN_004d9d70(int *rect, int a, int b, int c, int d, int e, int f);
void  FUN_004db4e0(int a, char *s, int x, int y, unsigned int col);
void  FUN_004c9bf0(int a);
void  FUN_004b5570(char *s, int x, int y);
int   FUN_004dbfa0(char *s, int a);

#define LD32(p) ({ int _t; __builtin_memcpy(&_t, (p), 4); _t; })
#define LDI(p, n) LD32((char *)(p) + (n) * 4)
#define IN(x) (DAT_0066da6c <= (x) && (int)((unsigned int)(x) - (unsigned int)DAT_0066da6c) < 0x15)

unsigned int __stdcall FUN_00501710(void)
{
    char cVar1;
    char *pcVar2;
    char *pcVar3;
    char *pcVar4;
    char *pcVar5;
    char *piVar6;
    char *piVar12;
    int *piVar7;
    int *piVar13;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    int iVar10;
    int iVar11;
    unsigned int uVar13;
    unsigned int uVar14;
    unsigned int uVar15;
    int iVar16;
    unsigned int uVar17;
    unsigned int uVar18;
    unsigned int uVar19;
    int local_28;
    int local_24[4];
    char local_14[20];

    iVar10 = 0;
    FUN_004d6460(local_24, 0x54, 0x5a, 0x1b2, 0x6a);
    uVar19 = 0xffff;
    pcVar2 = FUN_004ddc10(0x138);
    FUN_004db640(0xf, pcVar2, local_24, uVar19);
    if (DAT_00b69e30 == 2) {
        local_28 = 0;
    } else {
        local_28 = 0;
        piVar12 = &DAT_00b6d5aa;
        do {
            iVar8 = FUN_004d11e0();
            if (iVar8 == LD32(piVar12)) break;
            piVar12 = piVar12 + 0x16 * 4;
            local_28 = local_28 + 1;
        } while (piVar12 < &DAT_00b6d86a);
    }
    if (local_28 == 8) {
        return 0;
    }
    if (DAT_0066d918 != 0) {
        DAT_0066da70 = 0xd;
        DAT_0066da6c = 0;
        DAT_0066d918 = 0;
        if (((DAT_00aa477b == '\0') && (LD32(&DAT_00b6a5aa) != 1)) && (LD32(&DAT_00b6a5aa) != 2)) {
            DAT_0066da70 = 0xe;
        }
        if (DAT_00b69e30 != 2) {
            DAT_0066da70 = DAT_0066da70 + 5;
        }
        if (LD32(&DAT_00b6a5aa) == 1) {
            DAT_0066da70 = DAT_0066da70 + 1;
        }
        if (LD32(&DAT_00b6a616) != 0) {
            DAT_0066da70 = DAT_0066da70 + 1;
        }
        iVar8 = 4;
        piVar12 = &DAT_00b6da12 + LD32(&DAT_00b6d962) * 0x4278 * 4;
        do {
            if (LD32(piVar12) != 0) {
                DAT_0066da70 = DAT_0066da70 + 1;
            }
            piVar12 = piVar12 + 4;
            iVar8 = iVar8 + -1;
        } while (iVar8 != 0);
        DAT_0066da50 = 0;
        DAT_0066da60 = 0;
        DAT_0066d9e8 = 0;
        DAT_0066d958 = 0;
        DAT_0066da40 = 0;
        DAT_0066d948 = 0;
        DAT_0066d8c8 = 0;
        DAT_0066d908 = 0;
        DAT_0066d9b4 = 0;
        piVar12 = &DAT_00b6c13e;
        do {
            iVar8 = 1;
            DAT_0066d9f4 = 0;
            piVar6 = piVar12;
            do {
                if ((LD32(piVar6) != 0) || (LDI(piVar6, 300) != 0)) {
                    DAT_0066d9f4 = 1;
                    DAT_0066d9b4 = 1;
                }
                piVar6 = piVar6 + 100 * 4;
                iVar8 = iVar8 + -1;
            } while (iVar8 != 0);
            if (DAT_0066d9f4 != 0) {
                DAT_0066da70 = DAT_0066da70 + 1;
            }
            piVar12 = piVar12 + 4;
        } while (piVar12 < &DAT_00b6c2ce);
        if (DAT_0066d9b4 != 0) {
            DAT_0066da70 = DAT_0066da70 + 2;
        }
        DAT_0066d9b0 = 0;
        if (DAT_00b69e30 != 2) {
            piVar12 = &DAT_00b6bcc2;
            do {
                if ((LD32(piVar12) != 0) || (LDI(piVar12, 8) != 0)) {
                    DAT_0066d9b0 = 1;
                    DAT_0066da70 = DAT_0066da70 + 1;
                }
                piVar12 = piVar12 + 4;
            } while (piVar12 < &DAT_00b6bce2);
            if (DAT_0066d9b0 != 0) {
                DAT_0066da70 = DAT_0066da70 + 2;
            }
        }
        DAT_0066d9e4 = 0;
        if (DAT_00b69e30 != 2) {
            piVar12 = &DAT_00b6be82;
            do {
                if ((LD32(piVar12) != 0) || (LDI(piVar12, 8) != 0)) {
                    DAT_0066d9e4 = 1;
                    DAT_0066da70 = DAT_0066da70 + 1;
                }
                piVar12 = piVar12 + 4;
            } while (piVar12 < &DAT_00b6bea2);
            if (DAT_0066d9e4 != 0) {
                DAT_0066da70 = DAT_0066da70 + 2;
            }
        }
        iVar8 = 0;
        piVar12 = &DAT_00b6ca9e;
        do {
            iVar9 = 100;
            do {
                iVar11 = LD32(piVar12);
                iVar16 = LDI(piVar12, -300);
                piVar12 = piVar12 + 4;
                *(unsigned int *)((char *)&DAT_0066d908 + iVar8) =
                    *(unsigned int *)((char *)&DAT_0066d908 + iVar8) + (unsigned int)iVar11;
                *(unsigned int *)((char *)&DAT_0066d8c8 + iVar8) =
                    *(unsigned int *)((char *)&DAT_0066d8c8 + iVar8) + (unsigned int)iVar16;
                iVar9 = iVar9 + -1;
            } while (iVar9 != 0);
            iVar8 = iVar8 + 4;
        } while (iVar8 < 4);
        iVar8 = 0;
        piVar12 = &DAT_00b6cf4e;
        do {
            iVar9 = 0x19;
            do {
                iVar11 = LD32(piVar12);
                iVar16 = LDI(piVar12, 0x4b);
                piVar12 = piVar12 + 4;
                *(unsigned int *)((char *)&DAT_0066d948 + iVar8) =
                    *(unsigned int *)((char *)&DAT_0066d948 + iVar8) + (unsigned int)iVar11;
                *(unsigned int *)((char *)&DAT_0066da40 + iVar8) =
                    *(unsigned int *)((char *)&DAT_0066da40 + iVar8) + (unsigned int)iVar16;
                iVar9 = iVar9 + -1;
            } while (iVar9 != 0);
            iVar8 = iVar8 + 4;
        } while (iVar8 < 4);
        iVar8 = 0;
        piVar12 = &DAT_00b6d2d2;
        do {
            iVar9 = 6;
            do {
                iVar11 = LD32(piVar12);
                iVar16 = LDI(piVar12, 0x12);
                piVar12 = piVar12 + 4;
                *(unsigned int *)((char *)&DAT_0066d958 + iVar8) =
                    *(unsigned int *)((char *)&DAT_0066d958 + iVar8) + (unsigned int)iVar11;
                *(unsigned int *)((char *)&DAT_0066d9e8 + iVar8) =
                    *(unsigned int *)((char *)&DAT_0066d9e8 + iVar8) + (unsigned int)iVar16;
                iVar9 = iVar9 + -1;
            } while (iVar9 != 0);
            iVar8 = iVar8 + 4;
        } while (iVar8 < 4);
        piVar7 = &DAT_0066da60;
        piVar6 = &DAT_00b6d416;
        do {
            iVar8 = 0x19;
            do {
                iVar9 = LD32(piVar6);
                piVar6 = piVar6 + 4;
                *piVar7 = (int)((unsigned int)*piVar7 + (unsigned int)iVar9);
                iVar8 = iVar8 + -1;
            } while (iVar8 != 0);
            piVar7 = piVar7 + 1;
        } while (piVar7 < &DAT_0066da64);
        piVar13 = &DAT_0066da50;
        piVar12 = &DAT_00b6d542;
        do {
            iVar8 = 6;
            do {
                iVar9 = LD32(piVar12);
                piVar12 = piVar12 + 4;
                *piVar13 = (int)((unsigned int)*piVar13 + (unsigned int)iVar9);
                iVar8 = iVar8 + -1;
            } while (iVar8 != 0);
            piVar13 = piVar13 + 1;
        } while (piVar13 < &DAT_0066da54);
    }
    FUN_004d6460(local_24, 0x1a9, 0x6b, 0x1b2, 0x1b1);
    if (0x15 < DAT_0066da70) {
        DAT_0066da6c = FUN_004d9d70(local_24, DAT_0066da6c, DAT_0066da70, 0, 5, DAT_00bb2810, 10);
    }
    iVar8 = 0x6f;
    if ((DAT_00b69e30 != 2) && (LD32(&DAT_00b6a5aa) == 1)) {
        if ((DAT_0066da6c < 1) && (-0x15 < DAT_0066da6c)) {
            if (DAT_0066da4c == 0) {
                uVar19 = 0x2b3;
            } else {
                uVar19 = 0x170;
            }
            pcVar2 = FUN_004ddc10(uVar19);
            uVar17 = 4;
            uVar15 = 1;
            uVar18 = DAT_0066d904;
            pcVar3 = FUN_004ddc10(0x14f);
            uVar14 = 4;
            pcVar4 = FUN_004ddc10(DAT_0066d900 + 0x13e);
            uVar13 = 1;
            pcVar5 = FUN_004ddc10(0x16f);
            _sprintf(DAT_00b69d20, s__c_s__c_s__c_s__c_d__c_s_0052d4e8, 4, pcVar5, uVar13, pcVar4,
                     uVar14, pcVar3, uVar15, uVar18, uVar17, pcVar2);
            iVar8 = 0x7e;
            FUN_004db4e0(0xc, DAT_00b69d20, 0x58, 0x6f, 0xffff);
        }
        iVar10 = 1;
    }
    iVar9 = iVar8;
    if (((DAT_00aa477b != '\0') || (LD32(&DAT_00b6a5aa) == 1)) || (LD32(&DAT_00b6a5aa) == 2)) goto LAB_00501ce1;
    if (IN(iVar10)) {
        uVar19 = (unsigned int)LDI(&DAT_00b6d84e, LD32(&DAT_00b6a5a6) * 7);
        if (LD32(&DAT_00b6a5aa) == 0) {
            uVar7 = (unsigned int)(uVar19 == 0);
        } else {
            uVar7 = (unsigned int)LDI(&DAT_00b6d84e, (LD32(&DAT_00b6a5a6) ^ 1) * 7);
        }
        if (uVar7 == uVar19) {
            uVar19 = 0x15b;
LAB_00501c94:
            pcVar2 = FUN_004ddc10(uVar19);
            uVar18 = 1;
            pcVar3 = FUN_004ddc10(0x171);
            _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 4, pcVar3, uVar18, pcVar2);
        } else {
            if (uVar19 != 1) {
                uVar19 = 0x173;
                goto LAB_00501c94;
            }
            FUN_004c9bf0(LDI(&DAT_00b6d856,
                             (unsigned int)(&DAT_00a91d18)[LDI(&DAT_00b6d5a6, local_28 * 0x16) * 0x562] * 7));
            uVar19 = 0xffffffff;
            pcVar2 = DAT_00b69d20;
            do {
                pcVar3 = pcVar2;
                if (uVar19 == 0) break;
                uVar19 = uVar19 - 1;
                pcVar3 = pcVar2 + 1;
                cVar1 = *pcVar2;
                pcVar2 = pcVar3;
            } while (cVar1 != '\0');
            uVar19 = ~uVar19;
            pcVar2 = pcVar3 + -uVar19;
            pcVar3 = local_14;
            for (uVar7 = uVar19 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                __builtin_memcpy(pcVar3, pcVar2, 4);
                pcVar2 = pcVar2 + 4;
                pcVar3 = pcVar3 + 4;
            }
            for (uVar19 = uVar19 & 3; uVar19 != 0; uVar19 = uVar19 - 1) {
                *pcVar3 = *pcVar2;
                pcVar2 = pcVar2 + 1;
                pcVar3 = pcVar3 + 1;
            }
            pcVar2 = local_14;
            pcVar3 = FUN_004ddc10(0x172);
            uVar18 = 1;
            pcVar4 = FUN_004ddc10(0x171);
            _sprintf(DAT_00b69d20, s__c_s__c_s__s__0052d4d8, 4, pcVar4, uVar18, pcVar3, pcVar2);
        }
        iVar9 = iVar8 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar8, 0xffff);
    }
    iVar10 = iVar10 + 1;
LAB_00501ce1:
    iVar8 = iVar9;
    if (IN(iVar10)) {
        uVar18 = (unsigned int)LDI(&DAT_00b6d5b2, local_28 * 0x16);
        uVar13 = 1;
        pcVar2 = FUN_004ddc10(0x14d);
        _sprintf(DAT_00b69d20, s__c_s__c_d_0052c518, 4, pcVar2, uVar13, uVar18);
        iVar8 = iVar9 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar9, 0xffff);
    }
    iVar11 = iVar10 + 1;
    iVar9 = iVar8;
    if (LD32(&DAT_00b6a616) != 0) {
        if (IN(iVar11)) {
            _sprintf(DAT_00b69d20, s_rank_d_0052b4e0, LD32(&DAT_00b6bc32));
            FUN_004b5570(DAT_00b69d20, 0x58, iVar8 + 1);
            pcVar2 = FUN_004ddc10(LD32(&DAT_00b6bc32) + 0x7a);
            uVar18 = 6;
            pcVar3 = FUN_004ddc10(LD32(&DAT_00b6a616) + 0x177);
            _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 4, pcVar3, uVar18, pcVar2);
            iVar9 = iVar8 + 0xf;
            FUN_004db4e0(0xc, DAT_00b69d20, 0x91, iVar8, 0xffff);
        }
        iVar11 = iVar10 + 2;
    }
    DAT_0066d9f4 = 0;
    iVar10 = 0;
    do {
        iVar8 = iVar9;
        if (LDI(&DAT_00b6da12, iVar10 + LD32(&DAT_00b6d962) * 0x4278) != 0) {
            if (IN(iVar11)) {
                if (DAT_0066d9f4 == 0) {
                    iVar16 = 0x58;
                    uVar19 = DAT_00b6a2c0;
                    pcVar2 = FUN_004ddc10(0x179);
                    FUN_004db4e0(0xc, pcVar2, iVar16, iVar8, uVar19);
                    DAT_0066d9f4 = 1;
                }
                iVar8 = 0xc;
                pcVar2 = FUN_004ddc10(0x179);
                iVar16 = FUN_004dbfa0(pcVar2, iVar8);
                if (iVar10 == 2) {
                    if (LD32(&DAT_00b6d962) == 0) {
                        pcVar2 = s_rcitlvl_d_0052b4b4;
                        uVar18 = (unsigned int)LD32(&DAT_00b6da1a);
                    } else {
                        uVar18 = (unsigned int)LDI(&DAT_00b6da12, LD32(&DAT_00b6d962) * 0x4278 + 2);
                        pcVar2 = s_citlvl_d_0052b4a8;
                    }
                } else {
                    uVar18 = (unsigned int)LDI(&DAT_00b6da12, iVar10 + LD32(&DAT_00b6d962) * 0x4278);
                    pcVar2 = s_medlvl_d_0052b49c;
                }
                _sprintf(DAT_00b69d20, pcVar2, uVar18);
                FUN_004b5570(DAT_00b69d20, iVar16 + 0x62, iVar9);
                if (iVar10 == 2) {
                    uVar19 = (unsigned int)LDI(&DAT_00b6da12, LD32(&DAT_00b6d962) * 0x4278 + 2) + 0x293;
                } else {
                    uVar19 = (unsigned int)LDI(&DAT_00b6da12, iVar10 + LD32(&DAT_00b6d962) * 0x4278) + 0x17d;
                }
                pcVar2 = FUN_004ddc10(uVar19);
                pcVar3 = FUN_004ddc10(iVar10 + 0x17a);
                _sprintf(DAT_00b69d20, s__s____s_0052d4d0, pcVar3, pcVar2);
                iVar8 = iVar9 + 0xf;
                FUN_004db4e0(0xc, DAT_00b69d20, iVar16 + 0x8a, iVar9, 0xffff);
            }
            iVar11 = iVar11 + 1;
        }
        iVar10 = iVar10 + 1;
        iVar9 = iVar8;
    } while (iVar10 < 4);
    if (IN(iVar11)) {
        iVar8 = iVar8 + 0xf;
    }
    iVar10 = iVar8;
    if (IN(iVar11 + 1)) {
        iVar9 = 0x58;
        iVar10 = iVar8 + 0xf;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x15e);
        FUN_004db4e0(0xc, pcVar2, iVar9, iVar8, uVar19);
    }
    iVar8 = iVar10;
    if (IN(iVar11 + 2)) {
        iVar9 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x129);
        FUN_004db4e0(0xc, pcVar2, iVar9, iVar8, uVar19);
        _sprintf(DAT_00b69d20, s__d___d__0052b464, LD32(&DAT_00b6c126), DAT_0066d8c8);
        iVar8 = iVar10 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar10, 0xffff);
    }
    iVar10 = iVar11 + 3;
    if (DAT_00b69e30 != 2) {
        iVar9 = iVar8;
        if (IN(iVar10)) {
            iVar9 = 0x58;
            iVar10 = iVar8;
            uVar19 = DAT_00b6a2c0;
            pcVar2 = FUN_004ddc10(0x15f);
            FUN_004db4e0(0xc, pcVar2, iVar9, iVar10, uVar19);
            if (LD32(&DAT_00b6a5aa) == 0) {
                _sprintf(DAT_00b69d20, DAT_0052b46c);
            } else {
                _sprintf(DAT_00b69d20, s__d___d__0052b464, DAT_0066d948, DAT_0066da40);
            }
            iVar9 = iVar8 + 0xf;
            FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar8, 0xffff);
        }
        iVar8 = iVar9;
        if (IN(iVar11 + 4)) {
            iVar8 = 0x58;
            iVar10 = iVar9;
            uVar19 = DAT_00b6a2c0;
            pcVar2 = FUN_004ddc10(0x160);
            FUN_004db4e0(0xc, pcVar2, iVar8, iVar10, uVar19);
            _sprintf(DAT_00b69d20, s__d___d__0052b464, DAT_0066d958, DAT_0066d9e8);
            iVar8 = iVar9 + 0xf;
            FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar9, 0xffff);
        }
        iVar10 = iVar11 + 5;
    }
    iVar9 = iVar8;
    if (IN(iVar10)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x161);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar9, uVar19);
        _sprintf(DAT_00b69d20, DAT_0052b460, DAT_0066d908);
        iVar9 = iVar8 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar8, 0xffff);
    }
    iVar8 = iVar9;
    if (IN(iVar10 + 1)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x162);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar8, uVar19);
        _sprintf(DAT_00b69d20, DAT_0052b460, LD32(&DAT_00b6d3aa));
        iVar8 = iVar9 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar9, 0xffff);
    }
    iVar9 = iVar8;
    if (IN(iVar10 + 2)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x163);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar9, uVar19);
        uVar19 = 0;
        if (LD32(&DAT_00b6d3c2) != 0) {
            uVar19 = ((unsigned int)LD32(&DAT_00b6d3b6) * 100u) / (unsigned int)LD32(&DAT_00b6d3c2);
        }
        _sprintf(DAT_00b69d20, DAT_0052b458, uVar19);
        iVar9 = iVar8 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar8, 0xffff);
    }
    iVar8 = iVar9;
    if (IN(iVar10 + 3)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x164);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar8, uVar19);
        uVar19 = 0;
        if (LD32(&DAT_00b6d3da) != 0) {
            uVar19 = ((unsigned int)LD32(&DAT_00b6d3ce) * 100u) / (unsigned int)LD32(&DAT_00b6d3da);
        }
        _sprintf(DAT_00b69d20, DAT_0052b458, uVar19);
        iVar8 = iVar9 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar9, 0xffff);
    }
    iVar9 = iVar10 + 4;
    if (DAT_0066d9b0 != 0) {
        iVar11 = iVar8;
        if (IN(iVar9)) {
            iVar11 = iVar8 + 0xf;
        }
        iVar8 = iVar11;
        if (IN(iVar10 + 5)) {
            iVar9 = 0x58;
            iVar8 = iVar11 + 0xf;
            uVar19 = DAT_00b6a2c0;
            pcVar2 = FUN_004ddc10(0x165);
            FUN_004db4e0(0xc, pcVar2, iVar9, iVar11, uVar19);
        }
        local_28 = 0x18;
        iVar9 = iVar10 + 6;
        do {
            piVar6 = &DAT_00b6d5ae;
            piVar12 = &DAT_00b6bcc2;
            iVar10 = iVar8;
            do {
                iVar8 = iVar10;
                if ((LD32(piVar6) == local_28) && ((LD32(piVar12) != 0 || (LDI(piVar12, 8) != 0)))) {
                    if (IN(iVar9)) {
                        iVar8 = (int)piVar6 + -0x16;
                        uVar18 = 1;
                        pcVar2 = FUN_004ddc10(local_28 + 0x9a);
                        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar2, uVar18, iVar8);
                        FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar10, DAT_00b6a2c0);
                        _sprintf(DAT_00b69d20, s__d__d__0052b450, LD32(piVar12), LDI(piVar12, 8));
                        iVar8 = iVar10 + 0xf;
                        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar10, 0xffff);
                    }
                    iVar9 = iVar9 + 1;
                }
                piVar6 = piVar6 + 0x16 * 4;
                piVar12 = piVar12 + 4;
                iVar10 = iVar8;
            } while (piVar12 < &DAT_00b6bce2);
            local_28 = local_28 + -1;
        } while (-1 < local_28);
    }
    if (DAT_0066d9b4 != 0) {
        iVar10 = iVar8;
        if (IN(iVar9)) {
            iVar10 = iVar8 + 0xf;
        }
        iVar8 = iVar10;
        if (IN(iVar9 + 1)) {
            iVar11 = 0x58;
            iVar8 = iVar10 + 0xf;
            uVar19 = DAT_00b6a2c0;
            pcVar2 = FUN_004ddc10(0x166);
            FUN_004db4e0(0xc, pcVar2, iVar11, iVar10, uVar19);
        }
        iVar9 = iVar9 + 2;
    }
    iVar10 = 0;
    piVar12 = &DAT_00b6c13e;
    do {
        iVar11 = 1;
        DAT_0066d9f4 = 0;
        piVar6 = piVar12;
        do {
            if ((LD32(piVar6) != 0) || (LDI(piVar6, 300) != 0)) {
                DAT_0066d9f4 = 1;
            }
            piVar6 = piVar6 + 100 * 4;
            iVar11 = iVar11 + -1;
        } while (iVar11 != 0);
        iVar11 = iVar8;
        if (DAT_0066d9f4 != 0) {
            if (IN(iVar9)) {
                iVar16 = 0x58;
                uVar19 = DAT_00b69e38;
                pcVar2 = FUN_004ddc10(iVar10 + 0x15);
                FUN_004db4e0(0xc, pcVar2, iVar16, iVar11, uVar19);
                _sprintf(DAT_00b69d20, s__d___d__0052b464, LD32(piVar12), LDI(piVar12, 300));
                iVar11 = iVar8 + 0xf;
                FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar8, 0xffff);
            }
            iVar9 = iVar9 + 1;
        }
        piVar12 = piVar12 + 4;
        iVar10 = iVar10 + 1;
        iVar8 = iVar11;
    } while (piVar12 < &DAT_00b6c2ce);
    if (IN(iVar9)) {
        iVar11 = iVar11 + 0xf;
    }
    iVar10 = iVar11;
    if (IN(iVar9 + 1)) {
        iVar8 = 0x58;
        iVar10 = iVar11 + 0xf;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x168);
        FUN_004db4e0(0xc, pcVar2, iVar8, iVar11, uVar19);
    }
    iVar8 = iVar10;
    if (IN(iVar9 + 2)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x169);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar8, uVar19);
        _sprintf(DAT_00b69d20, DAT_0052b460, LD32(&DAT_00b6d3e6));
        iVar8 = iVar10 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar10, 0xffff);
    }
    iVar10 = iVar9 + 3;
    if (DAT_00b69e30 != 2) {
        iVar11 = iVar8;
        if (IN(iVar10)) {
            iVar11 = 0x58;
            iVar10 = iVar8;
            uVar19 = DAT_00b6a2c0;
            pcVar2 = FUN_004ddc10(0x16a);
            FUN_004db4e0(0xc, pcVar2, iVar11, iVar10, uVar19);
            _sprintf(DAT_00b69d20, DAT_0052b460, DAT_0066da60);
            iVar11 = iVar8 + 0xf;
            FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar8, 0xffff);
        }
        iVar8 = iVar11;
        if (IN(iVar9 + 4)) {
            iVar8 = 0x58;
            iVar10 = iVar11;
            uVar19 = DAT_00b6a2c0;
            pcVar2 = FUN_004ddc10(0x16b);
            FUN_004db4e0(0xc, pcVar2, iVar8, iVar10, uVar19);
            _sprintf(DAT_00b69d20, DAT_0052b460, DAT_0066da50);
            iVar8 = iVar11 + 0xf;
            FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar11, 0xffff);
        }
        iVar10 = iVar9 + 5;
    }
    iVar9 = iVar8;
    if (IN(iVar10)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x16c);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar9, uVar19);
        _sprintf(DAT_00b69d20, DAT_0052b460, LD32(&DAT_00b6d3fe));
        iVar9 = iVar8 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar8, 0xffff);
    }
    iVar8 = iVar9;
    if (IN(iVar10 + 1)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x16d);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar8, uVar19);
        _sprintf(DAT_00b69d20, DAT_0052b460, LD32(&DAT_00b6d40a));
        iVar8 = iVar9 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar9, 0xffff);
    }
    iVar9 = iVar8;
    if (IN(iVar10 + 2)) {
        iVar11 = 0x58;
        uVar19 = DAT_00b6a2c0;
        pcVar2 = FUN_004ddc10(0x16e);
        FUN_004db4e0(0xc, pcVar2, iVar11, iVar9, uVar19);
        _sprintf(DAT_00b69d20, DAT_0052b460, LD32(&DAT_00b6d3f2));
        iVar9 = iVar8 + 0xf;
        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar8, 0xffff);
    }
    if (DAT_0066d9e4 != 0) {
        if (IN(iVar10 + 3)) {
            iVar9 = iVar9 + 0xf;
        }
        iVar8 = iVar9;
        if (IN(iVar10 + 4)) {
            iVar11 = 0x58;
            iVar8 = iVar9 + 0xf;
            uVar19 = DAT_00b6a2c0;
            pcVar2 = FUN_004ddc10(0x184);
            FUN_004db4e0(0xc, pcVar2, iVar11, iVar9, uVar19);
        }
        local_28 = 0x18;
        iVar10 = iVar10 + 5;
        do {
            piVar12 = &DAT_00b6d5ae;
            piVar6 = &DAT_00b6be82;
            iVar9 = iVar8;
            do {
                iVar8 = iVar9;
                if ((LD32(piVar12) == local_28) && ((LD32(piVar6) != 0 || (LDI(piVar6, 8) != 0)))) {
                    if (IN(iVar10)) {
                        iVar8 = (int)piVar12 + -0x16;
                        uVar18 = 1;
                        pcVar2 = FUN_004ddc10(local_28 + 0x9a);
                        _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar2, uVar18, iVar8);
                        FUN_004db4e0(0xc, DAT_00b69d20, 0x58, iVar9, DAT_00b6a2c0);
                        _sprintf(DAT_00b69d20, s__d__d__0052b450, LD32(piVar6), LDI(piVar6, 8));
                        iVar8 = iVar9 + 0xf;
                        FUN_004db4e0(0xc, DAT_00b69d20, 0x10c, iVar9, 0xffff);
                    }
                    iVar10 = iVar10 + 1;
                }
                piVar12 = piVar12 + 0x16 * 4;
                piVar6 = piVar6 + 4;
                iVar9 = iVar8;
            } while (piVar6 < &DAT_00b6bea2);
            local_28 = local_28 + -1;
        } while (-1 < local_28);
    }
    return 1;
}
