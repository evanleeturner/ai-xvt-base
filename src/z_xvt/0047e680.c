/* SPDX-License-Identifier: MIT */
/* z_xvt 0047e680 */
extern undefined2 DAT_00732e8c;
extern undefined2 DAT_00732e58;
extern undefined2 DAT_00732e52;
extern undefined1 DAT_00732e50;
extern undefined2 DAT_00732a34;
extern undefined2 DAT_007329c0;
extern undefined2 DAT_00730894;
extern undefined1 DAT_0071e524;
extern undefined1 DAT_0071b012;
extern undefined2 DAT_0070acc0;
extern char       DAT_0070a188;
extern undefined2 DAT_006ffbf0;
extern undefined1 DAT_006da9fa;
extern undefined2 DAT_006d9980;
extern undefined1 DAT_006d4a4e;
extern int        DAT_006db594;
extern int        DAT_006db5b0;
extern int        DAT_006da538;
extern int        DAT_00525518;
extern char       DAT_0070ab20;
extern char       DAT_00525e04;
extern char      *DAT_006a8974;
extern char      *DAT_006a8978;
extern void (*DAT_0070ac2c)(int, int, int, int, int);
extern void (*DAT_00732a30)(int, int, int, int, int);
extern void (*DAT_00703c80)(void);

void FUN_004a6a80(void);
void FUN_0048ee20(char param_1);
void FUN_0048ed00(short param_1, short param_2, short param_3, short param_4);
void FUN_0048ece0(short param_1, short param_2);
void __cdecl FUN_0048f270(unsigned char *param_1);
int  FUN_004a6a70(void);
void FUN_004a6f90(void);
void FUN_004a7130(void);
void FUN_004a7040(void);
unsigned int FUN_004a5470(void);

undefined1 __stdcall FUN_0047e680(void)
{
    char           cVar1;
    unsigned char  bVar2;
    undefined2     uVar3;
    undefined1     uVar4;
    undefined2     uVar5;
    char           cVar6;
    undefined2     uVar7;
    undefined1     uVar8;
    undefined1     uVar9;
    undefined2     uVar10;
    undefined2     uVar11;
    undefined2     uVar12;
    undefined1     uVar13;
    undefined2     uVar14;
    undefined2     uVar15;
    undefined2     uVar16;
    int            iVar17;
    short          sVar18;
    unsigned int   uVar19;
    unsigned int   uVar20;
    int            iVar21;
    short          sVar22;
    int            iVar23;
    unsigned char *pbVar24;
    char          *pcVar25;
    char          *pcVar26;
    unsigned char *pbVar27;
    unsigned char  abStack_100[256];

    uVar16 = DAT_00732e8c;
    uVar15 = DAT_00732e58;
    uVar14 = DAT_00732e52;
    uVar13 = DAT_00732e50;
    uVar12 = DAT_00732a34;
    uVar11 = DAT_007329c0;
    uVar10 = DAT_00730894;
    uVar9  = DAT_0071e524;
    uVar8  = DAT_0071b012;
    uVar7  = DAT_0070acc0;
    cVar6  = DAT_0070a188;
    uVar5  = DAT_006ffbf0;
    uVar4  = DAT_006da9fa;
    uVar3  = DAT_006d9980;

    FUN_004a6a80();
    FUN_0048ee20('\x01');

    uVar19 = (unsigned int)DAT_006d4a4e;
    iVar23 = (int)((unsigned int)DAT_006db5b0 +
                   ((unsigned int)DAT_006db594 + uVar19 * (unsigned int)-4 + (unsigned int)-1) *
                   (unsigned int)DAT_00525518 * (unsigned int)DAT_006da538);
    DAT_0070ac2c(iVar23, 0,
                 (int)((unsigned int)(DAT_006db594 >> 1) + uVar19 * (unsigned int)-2),
                 DAT_006da538,
                 (int)(uVar19 * 4u + 1u));

    sVar18 = (short)(DAT_006db594 >> 1);
    sVar22 = (short)(DAT_006da538 >> 4);
    FUN_0048ed00(sVar22,
                 (short)(sVar18 + (unsigned short)DAT_006d4a4e * -2),
                 (short)((short)DAT_006da538 - sVar22),
                 (short)((unsigned short)DAT_006d4a4e * 2 + sVar18));
    DAT_00732e50 = 0xf9;
    DAT_00703c80();

    sVar22 = (short)(DAT_006db594 >> 1);
    sVar18 = (short)(DAT_006da538 >> 4);
    FUN_0048ed00((short)(sVar18 + 1),
                 (short)(sVar22 + (unsigned short)DAT_006d4a4e * -2 + 1),
                 (short)(((short)DAT_006da538 - sVar18) + -1),
                 (short)((unsigned short)DAT_006d4a4e * 2 + sVar22 + -1));
    DAT_00732e50 = 0;
    DAT_00703c80();

    DAT_006da9fa = 0xf9;
    DAT_0071b012 = 0;
    DAT_0071e524 = 0;
    FUN_0048ece0(0, (short)(((short)(DAT_006db594 >> 1) - (unsigned short)DAT_006d4a4e) + -2));

    /* strcpy(abStack_100, &DAT_0070ab20) */
    uVar19 = 0xffffffff;
    pcVar25 = &DAT_0070ab20;
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
        *(undefined4 *)pbVar27 = *(undefined4 *)pbVar24;
        pbVar24 = pbVar24 + 4;
        pbVar27 = pbVar27 + 4;
    }
    for (uVar19 = uVar19 & 3; uVar19 != 0; uVar19 = uVar19 - 1) {
        *pbVar27 = *pbVar24;
        pbVar24 = pbVar24 + 1;
        pbVar27 = pbVar27 + 1;
    }

    /* strcat(abStack_100, &DAT_00525e04) */
    uVar19 = 0xffffffff;
    pcVar25 = &DAT_00525e04;
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
        *(undefined4 *)pbVar27 = *(undefined4 *)pbVar24;
        pbVar24 = pbVar24 + 4;
        pbVar27 = pbVar27 + 4;
    }
    for (uVar19 = uVar19 & 3; uVar19 != 0; uVar19 = uVar19 - 1) {
        *pbVar27 = *pbVar24;
        pbVar24 = pbVar24 + 1;
        pbVar27 = pbVar27 + 1;
    }

    /* strcat(abStack_100, DAT_006a8974) */
    uVar19 = 0xffffffff;
    pcVar25 = DAT_006a8974;
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
        *(undefined4 *)pbVar27 = *(undefined4 *)pbVar24;
        pbVar24 = pbVar24 + 4;
        pbVar27 = pbVar27 + 4;
    }
    for (uVar20 = uVar19 & 3; uVar20 != 0; uVar20 = uVar20 - 1) {
        *pbVar27 = *pbVar24;
        pbVar24 = pbVar24 + 1;
        pbVar27 = pbVar27 + 1;
    }

    FUN_0048f270(abStack_100);
    FUN_0048ece0(0, (short)((short)(DAT_006db594 >> 1) + 2));
    FUN_0048f270((unsigned char *)DAT_006a8978);

    iVar17 = FUN_004a6a70();
    iVar21 = iVar17;
    if (0 < iVar17) {
        do {
            FUN_004a6f90();
            iVar21 = iVar21 + -1;
        } while (iVar21 != 0);
    }
    FUN_004a7130();
    FUN_004a7040();
    uVar19 = FUN_004a5470();
    if (0 < iVar17) {
        do {
            FUN_004a6a80();
            iVar17 = iVar17 + -1;
        } while (iVar17 != 0);
    }

    DAT_00732a30(iVar23, 0,
                 (int)((unsigned int)(DAT_006db594 >> 1) + (unsigned int)DAT_006d4a4e * (unsigned int)-2),
                 DAT_006da538,
                 (int)((unsigned int)DAT_006d4a4e * 4u + 1u));
    FUN_0048ee20(cVar6);

    DAT_006d9980 = uVar3;
    DAT_006da9fa = uVar4;
    DAT_006ffbf0 = uVar5;
    DAT_0070acc0 = uVar7;
    DAT_0071b012 = uVar8;
    DAT_0071e524 = uVar9;
    DAT_00730894 = uVar10;
    DAT_007329c0 = uVar11;
    DAT_00732a34 = uVar12;
    DAT_00732e50 = uVar13;
    DAT_00732e52 = uVar14;
    DAT_00732e58 = uVar15;
    DAT_00732e8c = uVar16;

    FUN_004a6f90();
    iVar21 = FUN_004a6a70();
    iVar23 = iVar21;
    if (0 < iVar21) {
        do {
            FUN_004a6f90();
            iVar23 = iVar23 + -1;
        } while (iVar23 != 0);
    }
    FUN_004a7130();
    FUN_004a7040();
    if (0 < iVar21) {
        do {
            FUN_004a6a80();
            iVar21 = iVar21 + -1;
        } while (iVar21 != 0);
    }
    return (undefined1)uVar19;
}
