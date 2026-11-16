/* SPDX-License-Identifier: MIT */
/* z_bop 0049e060 */
void __stdcall FUN_004aba50(void);
void __cdecl FUN_004a9640(char param_1);
void __cdecl FUN_004a9520(short a, short b, short c, short d);
void __cdecl FUN_004a9500(short a, short b);
void __cdecl FUN_004a9a50(unsigned char *c);
int __stdcall FUN_004aba40(void);
void __stdcall FUN_004abe50(void);
void __stdcall FUN_004ac260(void);
void __stdcall FUN_004abee0(void);
unsigned int __stdcall FUN_004aa870(void);

extern unsigned char DAT_009a20ae;
extern int DAT_009a8c14;
extern int DAT_009a8c34;
extern int DAT_005233d8;
extern int DAT_009a7bac;
extern void (*DAT_009d8b6c)(int, int, int, int, unsigned int);
extern void (*DAT_00a07ce0)(int, int, int, int, unsigned int);
extern void (*DAT_009d1300)(void);

extern unsigned short DAT_009a6fe0;
extern unsigned char DAT_009a807a;
extern unsigned short DAT_009cd270;
extern char DAT_009d80c8;
extern unsigned short DAT_009d8c00;
extern unsigned char DAT_009e8f52;
extern unsigned char DAT_009ec464;
extern unsigned short DAT_009fe7e4;
extern unsigned short DAT_00a07c64;
extern unsigned short DAT_00a07ce4;
extern unsigned char DAT_00a08100;
extern unsigned short DAT_00a08102;
extern unsigned short DAT_00a08108;
extern unsigned short DAT_00a0813c;

extern char DAT_009d8a60[];
extern char DAT_0051bf3c[];
extern char *DAT_00a609d4;
extern unsigned char *DAT_00a609d8;

unsigned char __stdcall FUN_0049e060(void)
{
    unsigned short uVar16 = DAT_00a0813c;
    unsigned short uVar15 = DAT_00a08108;
    unsigned short uVar14 = DAT_00a08102;
    unsigned char uVar13 = DAT_00a08100;
    unsigned short uVar12 = DAT_00a07ce4;
    unsigned short uVar11 = DAT_00a07c64;
    unsigned short uVar10 = DAT_009fe7e4;
    unsigned char uVar9 = DAT_009ec464;
    unsigned char uVar8 = DAT_009e8f52;
    unsigned short uVar7 = DAT_009d8c00;
    char cVar6 = DAT_009d80c8;
    unsigned short uVar5 = DAT_009cd270;
    unsigned char uVar4 = DAT_009a807a;
    unsigned short uVar3 = DAT_009a6fe0;
    char cVar1;
    unsigned char bVar2;
    int iVar18;
    unsigned int uVar19;
    unsigned int uVar20;
    int iVar21;
    int iVar23;
    short sVar17;
    short sVar22;
    unsigned char *pbVar24;
    unsigned char *pbVar27;
    char *pcVar25;
    char *pcVar26;
    unsigned char abStack_100[256];
    unsigned int tmp;

    FUN_004aba50();
    FUN_004a9640('\x01');
    uVar19 = (unsigned int)DAT_009a20ae;
    iVar23 = (int)((unsigned int)DAT_009a8c34 +
                   ((unsigned int)DAT_009a8c14 + uVar19 * (unsigned int)-4 + (unsigned int)-1) *
                       (unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac);
    (*DAT_009d8b6c)(iVar23, 0,
                    (int)((unsigned int)(DAT_009a8c14 >> 1) + uVar19 * (unsigned int)-2),
                    DAT_009a7bac, uVar19 * 4 + 1);
    sVar22 = (short)(DAT_009a8c14 >> 1);
    sVar17 = (short)(DAT_009a7bac >> 4);
    FUN_004a9520(sVar17,
                 (short)((unsigned int)sVar22 + (unsigned int)DAT_009a20ae * 0xFFFFFFFEu),
                 (short)((short)DAT_009a7bac - sVar17),
                 (short)((unsigned int)DAT_009a20ae * 2u + (unsigned int)sVar22));
    DAT_00a08100 = 0xf9;
    (*DAT_009d1300)();
    sVar22 = (short)(DAT_009a8c14 >> 1);
    sVar17 = (short)(DAT_009a7bac >> 4);
    FUN_004a9520((short)(sVar17 + 1),
                 (short)((unsigned int)sVar22 + (unsigned int)DAT_009a20ae * 0xFFFFFFFEu + 1u),
                 (short)(((short)DAT_009a7bac - sVar17) + -1),
                 (short)((unsigned int)DAT_009a20ae * 2u + (unsigned int)sVar22 + 0xFFFFFFFFu));
    DAT_00a08100 = 0;
    (*DAT_009d1300)();
    DAT_009a807a = 0xf9;
    DAT_009e8f52 = 0;
    DAT_009ec464 = 0;
    FUN_004a9500(0, (short)(((short)(DAT_009a8c14 >> 1) - (unsigned short)DAT_009a20ae) + -2));

    uVar19 = 0xffffffff;
    pcVar25 = DAT_009d8a60;
    do {
        pcVar26 = pcVar25;
        if (uVar19 == 0) break;
        uVar19 = uVar19 - 1;
        pcVar26 = pcVar25 + 1;
        cVar1 = *pcVar25;
        pcVar25 = pcVar26;
    } while (cVar1 != '\0');
    uVar19 = ~uVar19;
    pbVar24 = (unsigned char *)(pcVar26 + -uVar19);
    pbVar27 = abStack_100;
    for (uVar20 = uVar19 >> 2; uVar20 != 0; uVar20 = uVar20 - 1) {
        __builtin_memcpy(&tmp, pbVar24, 4);
        __builtin_memcpy(pbVar27, &tmp, 4);
        pbVar24 = pbVar24 + 4;
        pbVar27 = pbVar27 + 4;
    }
    for (uVar19 = uVar19 & 3; uVar19 != 0; uVar19 = uVar19 - 1) {
        *pbVar27 = *pbVar24;
        pbVar24 = pbVar24 + 1;
        pbVar27 = pbVar27 + 1;
    }

    uVar19 = 0xffffffff;
    pcVar25 = DAT_0051bf3c;
    do {
        pcVar26 = pcVar25;
        if (uVar19 == 0) break;
        uVar19 = uVar19 - 1;
        pcVar26 = pcVar25 + 1;
        cVar1 = *pcVar25;
        pcVar25 = pcVar26;
    } while (cVar1 != '\0');
    uVar19 = ~uVar19;
    iVar21 = -1;
    pbVar24 = abStack_100;
    do {
        pbVar27 = pbVar24;
        if (iVar21 == 0) break;
        iVar21 = iVar21 + -1;
        pbVar27 = pbVar24 + 1;
        bVar2 = *pbVar24;
        pbVar24 = pbVar27;
    } while (bVar2 != 0);
    pbVar24 = (unsigned char *)(pcVar26 + -uVar19);
    pbVar27 = pbVar27 + -1;
    for (uVar20 = uVar19 >> 2; uVar20 != 0; uVar20 = uVar20 - 1) {
        __builtin_memcpy(&tmp, pbVar24, 4);
        __builtin_memcpy(pbVar27, &tmp, 4);
        pbVar24 = pbVar24 + 4;
        pbVar27 = pbVar27 + 4;
    }
    for (uVar19 = uVar19 & 3; uVar19 != 0; uVar19 = uVar19 - 1) {
        *pbVar27 = *pbVar24;
        pbVar24 = pbVar24 + 1;
        pbVar27 = pbVar27 + 1;
    }

    uVar19 = 0xffffffff;
    pcVar25 = DAT_00a609d4;
    do {
        pcVar26 = pcVar25;
        if (uVar19 == 0) break;
        uVar19 = uVar19 - 1;
        pcVar26 = pcVar25 + 1;
        cVar1 = *pcVar25;
        pcVar25 = pcVar26;
    } while (cVar1 != '\0');
    uVar19 = ~uVar19;
    iVar21 = -1;
    pbVar24 = abStack_100;
    do {
        pbVar27 = pbVar24;
        if (iVar21 == 0) break;
        iVar21 = iVar21 + -1;
        pbVar27 = pbVar24 + 1;
        bVar2 = *pbVar24;
        pbVar24 = pbVar27;
    } while (bVar2 != 0);
    pbVar24 = (unsigned char *)(pcVar26 + -uVar19);
    pbVar27 = pbVar27 + -1;
    for (uVar20 = uVar19 >> 2; uVar20 != 0; uVar20 = uVar20 - 1) {
        __builtin_memcpy(&tmp, pbVar24, 4);
        __builtin_memcpy(pbVar27, &tmp, 4);
        pbVar24 = pbVar24 + 4;
        pbVar27 = pbVar27 + 4;
    }
    for (uVar20 = uVar19 & 3; uVar20 != 0; uVar20 = uVar20 - 1) {
        *pbVar27 = *pbVar24;
        pbVar24 = pbVar24 + 1;
        pbVar27 = pbVar27 + 1;
    }
    FUN_004a9a50(abStack_100);

    FUN_004a9500(0, (short)((short)(DAT_009a8c14 >> 1) + 2));
    FUN_004a9a50(DAT_00a609d8);
    iVar18 = FUN_004aba40();
    iVar21 = iVar18;
    if (0 < iVar18) {
        do {
            FUN_004abe50();
            iVar21 = iVar21 + -1;
        } while (iVar21 != 0);
    }
    FUN_004ac260();
    FUN_004abee0();
    uVar19 = FUN_004aa870();
    if (0 < iVar18) {
        do {
            FUN_004aba50();
            iVar18 = iVar18 + -1;
        } while (iVar18 != 0);
    }
    (*DAT_00a07ce0)(iVar23, 0,
                    (int)((unsigned int)(DAT_009a8c14 >> 1) + (unsigned int)DAT_009a20ae * 0xFFFFFFFEu),
                    DAT_009a7bac, (unsigned int)DAT_009a20ae * 4u + 1u);
    FUN_004a9640(cVar6);
    DAT_009a6fe0 = uVar3;
    DAT_009a807a = uVar4;
    DAT_009cd270 = uVar5;
    DAT_009d8c00 = uVar7;
    DAT_009e8f52 = uVar8;
    DAT_009ec464 = uVar9;
    DAT_009fe7e4 = uVar10;
    DAT_00a07c64 = uVar11;
    DAT_00a07ce4 = uVar12;
    DAT_00a08100 = uVar13;
    DAT_00a08102 = uVar14;
    DAT_00a08108 = uVar15;
    DAT_00a0813c = uVar16;
    FUN_004abe50();
    iVar21 = FUN_004aba40();
    iVar23 = iVar21;
    if (0 < iVar21) {
        do {
            FUN_004abe50();
            iVar23 = iVar23 + -1;
        } while (iVar23 != 0);
    }
    FUN_004ac260();
    FUN_004abee0();
    if (0 < iVar21) {
        do {
            FUN_004aba50();
            iVar21 = iVar21 + -1;
        } while (iVar21 != 0);
    }
    return (unsigned char)uVar19;
}
