/* SPDX-License-Identifier: MIT */
/* z_bop 0049d010 */
extern unsigned char DAT_00521f18[];
extern unsigned char DAT_00521f19[];
extern unsigned char DAT_00521f2f[];
extern unsigned char DAT_00521f30[];
extern unsigned char DAT_00521f31[];
extern short DAT_00521f24[];
extern short DAT_009a7ed0[];
extern char DAT_00527620[];
extern char DAT_00527710[];
extern char DAT_0052770c[];
extern char DAT_00527704[];
extern char DAT_00523434[];
extern char s_ivfiles__00527714[];
extern int DAT_005233ec;
extern int DAT_005233d8;
extern void *DAT_00a07bd8;
extern unsigned char DAT_009d6940[];
extern int DAT_00999488;
extern unsigned char DAT_0099947e[]; /* 4-byte field at unaligned address */

void FUN_0049cba0(void);
int FUN_0049e720(char *, char *, int, int);
char *_fgets(char *, int, void *);
short FUN_00411e00(char *);
int *FUN_004ac720(unsigned int);
int FUN_0049ea10(void *, int, int, void *);
short FUN_004ac5f0(int, int);
void FUN_0049eb60(int);
void FUN_0049e9c0(int);
void FUN_0040e6c0(int *);
void FUN_0040e7f0(int *);
void FUN_0049d860(unsigned char, unsigned char);
void FUN_004ac740(void);
void FUN_00485cd0(void);
void FUN_0049cc90(void);

void __stdcall FUN_0049d010(void)
{
    char cVar1;
    unsigned char bVar2;
    int bVar3;
    void *_File;
    void *pFVar4;
    short sVar5;
    unsigned int uVar6;
    int *piVar7;
    unsigned short uVar8;
    unsigned int uVar9;
    int iVar10;
    unsigned int uVar11;
    unsigned short uVar12;
    char *pcVar15;
    char *pcVar16;
    char *pcVar17;
    unsigned char bStack_151;
    unsigned short uStack_150;
    unsigned short uStack_14e;
    int iStack_148;
    unsigned int uStack_144;
    int iStack_140;
    char acStack_13c[60];
    char acStack_100[256];
    unsigned int uDword;
    int iZero;

    uVar12 = 0;
    do {
        uVar8 = (unsigned short)(uVar12 + 1);
        uVar6 = (unsigned int)uVar12;
        DAT_009a7ed0[uVar6] = 0;
        DAT_00521f24[uVar6 * 0xd] = 0;
        uVar12 = uVar8;
    } while (uVar8 < 0xc9);
    FUN_0049cba0();
    DAT_00999488 = 0;
    iZero = 0;
    __builtin_memcpy(DAT_0099947e, &iZero, 4);
    uStack_14e = 0;
    do {
        acStack_13c[0] = s_ivfiles__00527714[0];
        acStack_13c[1] = s_ivfiles__00527714[1];
        acStack_13c[2] = s_ivfiles__00527714[2];
        acStack_13c[3] = s_ivfiles__00527714[3];
        acStack_13c[4] = s_ivfiles__00527714[4];
        acStack_13c[5] = s_ivfiles__00527714[5];
        acStack_13c[6] = s_ivfiles__00527714[6];
        acStack_13c[7] = s_ivfiles__00527714[7];
        uStack_144 = (unsigned int)uStack_14e;
        acStack_13c[8] = s_ivfiles__00527714[8];

        /* strcat(acStack_13c, &DAT_00527620[uStack_144 * 9]) */
        uVar9 = 0xffffffffu;
        pcVar15 = &DAT_00527620[uStack_144 * 9];
        do {
            pcVar17 = pcVar15;
            if (uVar9 == 0) break;
            uVar9 = uVar9 - 1;
            pcVar17 = pcVar15 + 1;
            cVar1 = *pcVar15;
            pcVar15 = pcVar17;
        } while (cVar1 != '\0');
        uVar9 = ~uVar9;
        iVar10 = -1;
        pcVar15 = acStack_13c;
        do {
            pcVar16 = pcVar15;
            if (iVar10 == 0) break;
            iVar10 = iVar10 + -1;
            pcVar16 = pcVar15 + 1;
            cVar1 = *pcVar15;
            pcVar15 = pcVar16;
        } while (cVar1 != '\0');
        pcVar15 = pcVar17 - uVar9;
        pcVar17 = pcVar16 - 1;
        for (uVar11 = uVar9 >> 2; uVar11 != 0; uVar11 = uVar11 - 1) {
            __builtin_memcpy(&uDword, pcVar15, 4);
            __builtin_memcpy(pcVar17, &uDword, 4);
            pcVar15 = pcVar15 + 4;
            pcVar17 = pcVar17 + 4;
        }
        for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
            *pcVar17 = *pcVar15;
            pcVar15 = pcVar15 + 1;
            pcVar17 = pcVar17 + 1;
        }

        if (DAT_005233ec == 0x101) {
            pcVar15 = DAT_00527710;
        } else if (DAT_005233ec == 0x112) {
            pcVar15 = DAT_00527710;
        } else {
            pcVar15 = DAT_0052770c;
        }

        /* strcat(acStack_13c, pcVar15) */
        uVar9 = 0xffffffffu;
        do {
            pcVar17 = pcVar15;
            if (uVar9 == 0) break;
            uVar9 = uVar9 - 1;
            pcVar17 = pcVar15 + 1;
            cVar1 = *pcVar15;
            pcVar15 = pcVar17;
        } while (cVar1 != '\0');
        uVar9 = ~uVar9;
        iVar10 = -1;
        pcVar15 = acStack_13c;
        do {
            pcVar16 = pcVar15;
            if (iVar10 == 0) break;
            iVar10 = iVar10 + -1;
            pcVar16 = pcVar15 + 1;
            cVar1 = *pcVar15;
            pcVar15 = pcVar16;
        } while (cVar1 != '\0');
        pcVar15 = pcVar17 - uVar9;
        pcVar17 = pcVar16 - 1;
        for (uVar11 = uVar9 >> 2; uVar11 != 0; uVar11 = uVar11 - 1) {
            __builtin_memcpy(&uDword, pcVar15, 4);
            __builtin_memcpy(pcVar17, &uDword, 4);
            pcVar15 = pcVar15 + 4;
            pcVar17 = pcVar17 + 4;
        }
        for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
            *pcVar17 = *pcVar15;
            pcVar15 = pcVar15 + 1;
            pcVar17 = pcVar17 + 1;
        }

        /* strcat(acStack_13c, DAT_00527704) */
        uVar9 = 0xffffffffu;
        pcVar15 = DAT_00527704;
        do {
            pcVar17 = pcVar15;
            if (uVar9 == 0) break;
            uVar9 = uVar9 - 1;
            pcVar17 = pcVar15 + 1;
            cVar1 = *pcVar15;
            pcVar15 = pcVar17;
        } while (cVar1 != '\0');
        uVar9 = ~uVar9;
        iVar10 = -1;
        pcVar15 = acStack_13c;
        do {
            pcVar16 = pcVar15;
            if (iVar10 == 0) break;
            iVar10 = iVar10 + -1;
            pcVar16 = pcVar15 + 1;
            cVar1 = *pcVar15;
            pcVar15 = pcVar16;
        } while (cVar1 != '\0');
        pcVar15 = pcVar17 - uVar9;
        pcVar17 = pcVar16 - 1;
        for (uVar11 = uVar9 >> 2; uVar11 != 0; uVar11 = uVar11 - 1) {
            __builtin_memcpy(&uDword, pcVar15, 4);
            __builtin_memcpy(pcVar17, &uDword, 4);
            pcVar15 = pcVar15 + 4;
            pcVar17 = pcVar17 + 4;
        }
        for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
            *pcVar17 = *pcVar15;
            pcVar15 = pcVar15 + 1;
            pcVar17 = pcVar17 + 1;
        }

        FUN_0049e720(acStack_13c, DAT_00523434, 1, 0);
        _File = DAT_00a07bd8;
        uStack_150 = 0;
        pcVar15 = _fgets(acStack_100, 0x100, DAT_00a07bd8);
        while (pcVar15 != (char *)0) {
            cVar1 = acStack_100[0];
            for (pcVar15 = acStack_100; (cVar1 != '\n' && (*pcVar15 != '\r')); pcVar15 = pcVar15 + 1) {
                cVar1 = pcVar15[1];
            }
            *pcVar15 = '\0';
            if (acStack_100[0] != '\0') {
                uVar9 = 0;
                bVar3 = 0;
                uStack_150 = (unsigned short)(uStack_150 + 1);
                do {
                    iVar10 = (int)(uVar9 * 0x1a);
                    if ((((DAT_00521f18[iVar10] & 2) != 0) &&
                         (DAT_00521f30[iVar10] == uStack_144) &&
                         ((unsigned int)DAT_00521f31[iVar10] - (unsigned int)uStack_150 == 0xffffffffu)) &&
                        (bVar2 = DAT_00521f19[iVar10], (bVar2 & 0x18) != 0) &&
                        (((bVar2 & 0x40) == 0) || (DAT_009d6940[1] != 0))) {
                        bVar3 = 1;
                        bStack_151 = bVar2;
                    }
                    uVar12 = (unsigned short)((short)uVar9 + 1);
                    uVar9 = (unsigned int)uVar12;
                } while (uVar12 < 0xc9);
                if (bVar3) {
                    if ((bStack_151 & 1) == 0) {
                        if ((bStack_151 & 2) != 0) {
                            FUN_0049e720(acStack_100, DAT_00523434, 1, 0);
                            pFVar4 = DAT_00a07bd8;
                            FUN_0049ea10(&iStack_148, 4, 1, DAT_00a07bd8);
                            FUN_0049ea10(&iStack_140, 4, 1, pFVar4);
                            sVar5 = FUN_004ac5f0((int)((unsigned int)iStack_140 * (unsigned int)DAT_005233d8 + (unsigned int)iStack_148), 0);
                            uVar6 = (unsigned int)(unsigned short)sVar5 | (uVar6 & 0xffff0000u);
                            if (sVar5 == 0) {
                                FUN_0049eb60(0);
                            }
                            piVar7 = FUN_004ac720(uVar6);
                            *piVar7 = iStack_148;
                            piVar7[1] = iStack_140;
                            FUN_0049ea10(piVar7 + 2, (int)((unsigned int)iStack_148 - 8u), 1, pFVar4);
                            DAT_00a07bd8 = pFVar4;
                            FUN_0049e9c0(0);
                            if (DAT_005233d8 == 2) {
                                FUN_0040e6c0(piVar7);
                            } else {
                                FUN_0040e7f0(piVar7);
                            }
                        }
                    } else {
                        sVar5 = FUN_00411e00(acStack_100);
                        uVar6 = (unsigned int)(unsigned short)sVar5 | (uVar6 & 0xffff0000u);
                        FUN_004ac720(uVar6);
                    }
                    uVar9 = 0;
                    do {
                        iVar10 = (int)(uVar9 * 0x1a);
                        if (((DAT_00521f18[iVar10] & 2) != 0) &&
                            (DAT_00521f30[iVar10] == uStack_144) &&
                            ((unsigned int)DAT_00521f31[iVar10] - (unsigned int)uStack_150 == 0xffffffffu) &&
                            ((DAT_00521f19[iVar10] & 0x18) != 0) &&
                            (((DAT_00521f19[iVar10] & 0x40) == 0) || (DAT_009d6940[1] != 0))) {
                            DAT_00521f24[uVar9 * 0xd] = (short)uVar6;
                            DAT_009a7ed0[uVar9] = (short)uVar6;
                            if ((bStack_151 & 1) != 0) {
                                FUN_0049d860(DAT_00521f2f[iVar10], (unsigned char)uVar9);
                            }
                        }
                        uVar12 = (unsigned short)((short)uVar9 + 1);
                        uVar9 = (unsigned int)uVar12;
                    } while (uVar12 < 0xc9);
                    FUN_004ac740();
                }
            }
            pcVar15 = _fgets(acStack_100, 0x100, _File);
        }
        DAT_00a07bd8 = _File;
        FUN_0049e9c0(0);
        uStack_14e = (unsigned short)(uStack_14e + 1);
        if (2 < uStack_14e) {
            FUN_00485cd0();
            FUN_0049cc90();
            return;
        }
    } while (1);
}
