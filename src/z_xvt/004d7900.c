/* SPDX-License-Identifier: MIT */
/* z_xvt 004d7900 */
/* Prototypes for every callee */
void  FUN_004dbea0(void);
void  FUN_004ddc70(int *a, int *b);
void  FUN_004d0e60(void *a, int b, int c);
void  FUN_004dcfd0(int *r, int a, int b, int c, int d);
char *FUN_004dcfa0(int a);
void  FUN_004da920(int a, char *b, int *c, unsigned int d);
int   FUN_004dd7d0(int *a, int b, int c);
unsigned char FUN_004db570(void);
unsigned char FUN_004db590(void);
void  FUN_004de190(char *a, int b, int c, int d);
int   FUN_004d9660(int *a, char *b, int c, int d, int e, char *f);
int   FUN_004cdbc0(void);
int   FUN_004cdca0(int a);
void  FUN_004cb830(int a, unsigned int *b, unsigned int c);
int   FUN_004daee0(int a, int b, int *c, int d, int e, int f);
int   FUN_004d9010(int *a, int b, int c, int d, int e, int f, int g);

/* Globals (types inferred from the decompile; unresolved ones noted) */
extern int  DAT_0064f654;
extern int  DAT_00532a38;
extern int  DAT_0081bcb8;
extern int  DAT_00532ab4;               /* type unresolved, only address is used */
extern int  DAT_00532aac;               /* type unresolved, only address is used */
extern unsigned int DAT_0081b6fc;
extern int  DAT_0081b700;
extern char DAT_0085e7c5;
extern char s_jewelsound_00531990[];
extern char DAT_007585b0[];             /* 100-byte buffer */
extern int  DAT_00758620;
extern char DAT_00758624[];
extern char DAT_0081b740[];
extern char DAT_0081bcf0[];
extern char DAT_00525e04[];
extern int  DAT_007579b0[8];
extern int  DAT_00757870[];
extern unsigned char DAT_0081bfb6[4];   /* packed int: address 0x81bfb6 is not 4-aligned */
extern int  DAT_00859c2c;

void __cdecl FUN_004d7900(int param_1)
{
    char cVar1;
    char *pcVar3;
    char *pcVar8;
    char *pcVar9;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int *puVar7;
    int *piVar11;
    unsigned int uVar12;
    int aiStack_18[4];
    int iStack_8;
    int iStack_4;
    int iBase;

    if (param_1 == 0) {
        FUN_004dbea0();
        DAT_0064f654 = 0;
    }
    if (DAT_0081bcb8 == 0) {
        DAT_00532a38 = 0;
    }
    else {
        FUN_004ddc70(&iStack_4, &iStack_8);
        if (DAT_00532a38 == 0) {
            FUN_004d0e60(&DAT_00532ab4, 0, 0);
            FUN_004dcfd0(aiStack_18, 0x22c, 0x5f, 0x24c, 0x69);
            piVar11 = aiStack_18;
            uVar12 = DAT_0081b6fc;
            pcVar3 = FUN_004dcfa0(0xcc);
            FUN_004da920(10, pcVar3, piVar11, uVar12);
            iVar4 = FUN_004dd7d0(aiStack_18, iStack_4, iStack_8);
            if ((iVar4 != 0) &&
                (FUN_004db570() != 0 || FUN_004db590() != 0)) {
                if (DAT_0085e7c5 != '\0') {
                    FUN_004de190(s_jewelsound_00531990, 1, 0, 0xff);
                }
                DAT_00532a38 = 1;
            }
            FUN_004d0e60(&DAT_00532aac, 0, 0);
            FUN_004dcfd0(aiStack_18, 0x208, 0x5f, 0x228, 0x69);
            piVar11 = aiStack_18;
            uVar12 = 0xffff;
            pcVar3 = FUN_004dcfa0(0x185);
            FUN_004da920(10, pcVar3, piVar11, uVar12);
        }
        else {
            FUN_004d0e60(&DAT_00532aac, 0, 0);
            FUN_004dcfd0(aiStack_18, 0x208, 0x5f, 0x228, 0x69);
            piVar11 = aiStack_18;
            uVar12 = DAT_0081b6fc;
            pcVar3 = FUN_004dcfa0(0x185);
            FUN_004da920(10, pcVar3, piVar11, uVar12);
            iVar4 = FUN_004dd7d0(aiStack_18, iStack_4, iStack_8);
            if ((iVar4 != 0) &&
                (FUN_004db570() != 0 || FUN_004db590() != 0)) {
                if (DAT_0085e7c5 != '\0') {
                    FUN_004de190(s_jewelsound_00531990, 1, 0, 0xff);
                }
                DAT_00532a38 = 0;
            }
            FUN_004d0e60(&DAT_00532ab4, 0, 0);
            FUN_004dcfd0(aiStack_18, 0x22c, 0x5f, 0x24c, 0x69);
            piVar11 = aiStack_18;
            uVar12 = 0xffff;
            pcVar3 = FUN_004dcfa0(0xcc);
            FUN_004da920(10, pcVar3, piVar11, uVar12);
        }
    }
    FUN_004dcfd0(aiStack_18, 0x1cd, 0x196, 0x253, 0x1a8);
    iVar4 = FUN_004d9660(aiStack_18, DAT_007585b0, 100, 0, 0xc, (char *)0);
    if ((iVar4 == 0) || (DAT_007585b0[0] == '\0')) goto LAB_004d7d91;

    DAT_00758620 = 0x4d;
    iVar4 = FUN_004cdbc0();
    iVar5 = FUN_004cdca0(iVar4);
    iVar4 = DAT_00532a38;
    if (iVar5 == 0) {
        DAT_0081b740[0] = 4;
    }
    else if (DAT_00532a38 == 1) {
        DAT_0081b740[0] = 2;
    }
    else {
        DAT_0081b740[0] = 3;
    }
    uVar12 = 0xffffffff;
    DAT_0081b740[1] = 0;

    /* strcat(DAT_0081b740, DAT_0081bcf0) */
    pcVar3 = DAT_0081bcf0;
    do {
        pcVar9 = pcVar3;
        if (uVar12 == 0) break;
        uVar12 = uVar12 - 1;
        pcVar9 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar9;
    } while (cVar1 != '\0');
    uVar12 = ~uVar12;
    iVar5 = -1;
    pcVar3 = DAT_0081b740;
    do {
        pcVar8 = pcVar3;
        if (iVar5 == 0) break;
        iVar5 = iVar5 + -1;
        pcVar8 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar1 != '\0');
    pcVar3 = pcVar9 + -uVar12;
    pcVar9 = pcVar8 + -1;
    for (uVar6 = uVar12 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        pcVar9[0] = pcVar3[0];
        pcVar9[1] = pcVar3[1];
        pcVar9[2] = pcVar3[2];
        pcVar9[3] = pcVar3[3];
        pcVar3 = pcVar3 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar12 = uVar12 & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
        *pcVar9 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar9 = pcVar9 + 1;
    }

    /* strcat(DAT_0081b740, DAT_00525e04) */
    uVar12 = 0xffffffff;
    pcVar3 = DAT_00525e04;
    do {
        pcVar9 = pcVar3;
        if (uVar12 == 0) break;
        uVar12 = uVar12 - 1;
        pcVar9 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar9;
    } while (cVar1 != '\0');
    uVar12 = ~uVar12;
    iVar5 = -1;
    pcVar3 = DAT_0081b740;
    do {
        pcVar8 = pcVar3;
        if (iVar5 == 0) break;
        iVar5 = iVar5 + -1;
        pcVar8 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar1 != '\0');
    pcVar3 = pcVar9 + -uVar12;
    pcVar9 = pcVar8 + -1;
    for (uVar6 = uVar12 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        pcVar9[0] = pcVar3[0];
        pcVar9[1] = pcVar3[1];
        pcVar9[2] = pcVar3[2];
        pcVar9[3] = pcVar3[3];
        pcVar3 = pcVar3 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar12 = uVar12 & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
        *pcVar9 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar9 = pcVar9 + 1;
    }

    /* strlen + terminator patch */
    uVar12 = 0xffffffff;
    pcVar3 = DAT_0081b740;
    do {
        if (uVar12 == 0) break;
        uVar12 = uVar12 - 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    DAT_0081b740[~uVar12 - 1] = 1;
    DAT_0081b740[~uVar12] = 0;

    /* strcat(DAT_0081b740, DAT_007585b0) */
    uVar12 = 0xffffffff;
    pcVar3 = DAT_007585b0;
    do {
        pcVar9 = pcVar3;
        if (uVar12 == 0) break;
        uVar12 = uVar12 - 1;
        pcVar9 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar9;
    } while (cVar1 != '\0');
    uVar12 = ~uVar12;
    iVar5 = -1;
    pcVar3 = DAT_0081b740;
    do {
        pcVar8 = pcVar3;
        if (iVar5 == 0) break;
        iVar5 = iVar5 + -1;
        pcVar8 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar1 != '\0');
    pcVar3 = pcVar9 + -uVar12;
    pcVar9 = pcVar8 + -1;
    for (uVar6 = uVar12 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        pcVar9[0] = pcVar3[0];
        pcVar9[1] = pcVar3[1];
        pcVar9[2] = pcVar3[2];
        pcVar9[3] = pcVar3[3];
        pcVar3 = pcVar3 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar12 = uVar12 & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
        *pcVar9 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar9 = pcVar9 + 1;
    }

    /* copy strlen(DAT_0081b740) bytes to DAT_00758624 */
    uVar12 = 0xffffffff;
    pcVar3 = DAT_0081b740;
    do {
        if (uVar12 == 0) break;
        uVar12 = uVar12 - 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    pcVar8 = DAT_0081b740;
    pcVar9 = DAT_00758624;
    for (uVar6 = (~uVar12 - 1) >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        pcVar9[0] = pcVar8[0];
        pcVar9[1] = pcVar8[1];
        pcVar9[2] = pcVar8[2];
        pcVar9[3] = pcVar8[3];
        pcVar8 = pcVar8 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar12 = (~uVar12 - 1) & 3; uVar12 != 0; uVar12 = uVar12 - 1) {
        *pcVar9 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar9 = pcVar9 + 1;
    }

    if (iVar4 == 0) {
        uVar12 = 0xffffffff;
        pcVar3 = DAT_0081b740;
        do {
            if (uVar12 == 0) break;
            uVar12 = uVar12 - 1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar3 + 1;
        } while (cVar1 != '\0');
LAB_004d7d7b:
        FUN_004cb830(0, (unsigned int *)&DAT_00758620, ~uVar12 + 3);
    }
    else {
        iVar4 = 0;
        piVar11 = DAT_007579b0;
        do {
            iVar5 = FUN_004cdbc0();
            if (iVar5 == *piVar11) break;
            piVar11 = piVar11 + 1;
            iVar4 = iVar4 + 1;
        } while (piVar11 < DAT_007579b0 + 8);
        if (iVar4 == 8) {
            uVar12 = 0xffffffff;
            pcVar3 = DAT_0081b740;
            do {
                if (uVar12 == 0) break;
                uVar12 = uVar12 - 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar3 + 1;
            } while (cVar1 != '\0');
            goto LAB_004d7d7b;
        }
        iVar4 = 0;
        do {
            __builtin_memcpy(&iBase, DAT_0081bfb6, 4);
            iVar5 = (int)((unsigned int)iVar4 + (unsigned int)iBase * 8u);
            if (DAT_00757870[iVar5] != 0) {
                uVar12 = 0xffffffff;
                pcVar3 = DAT_0081b740;
                do {
                    if (uVar12 == 0) break;
                    uVar12 = uVar12 - 1;
                    cVar1 = *pcVar3;
                    pcVar3 = pcVar3 + 1;
                } while (cVar1 != '\0');
                FUN_004cb830(DAT_00757870[iVar5], (unsigned int *)&DAT_00758620, ~uVar12 + 3);
            }
            iVar4 = iVar4 + 1;
        } while (iVar4 < 8);
    }
    puVar7 = (unsigned int *)DAT_007585b0;
    for (iVar4 = 0x19; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
    }
LAB_004d7d91:
    FUN_004dcfd0(aiStack_18, 0x1cd, 0x75, 0x253, 0x197);
    iVar5 = FUN_004daee0(0xc, DAT_0081b700, aiStack_18, 0xffff, 2, 0x400);
    iVar4 = (int)((unsigned int)iVar5 - 0x14u);
    if (iVar5 < 0x15) {
        iVar4 = 0;
    }
    if (0 < iVar4) {
        iVar4 = (int)((unsigned int)iVar4 + 1u);
        if (DAT_0064f654 == iVar4) {
            DAT_0064f654 = (int)((unsigned int)DAT_0064f654 - 1u);
        }
        FUN_004dcfd0(aiStack_18, 0x254, 0x75, 0x25d, 0x197);
        iVar5 = FUN_004d9010(aiStack_18,
                             (int)((unsigned int)iVar4 - (unsigned int)DAT_0064f654 - 1u),
                             iVar4, 0, 5, DAT_00859c2c, 7);
        DAT_0064f654 = (int)((unsigned int)iVar4 - (unsigned int)iVar5 - 1u);
    }
    FUN_004dcfd0(aiStack_18, 0x1cd, 0x75, 0x253, 0x18e);
    FUN_004daee0(0xc, DAT_0081b700, aiStack_18, 0xffff, 2,
                 (int)((unsigned int)iVar4 - (unsigned int)DAT_0064f654 - 1u));
    return;
}
