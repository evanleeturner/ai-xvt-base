/* SPDX-License-Identifier: MIT */
/* z_bop 00450650 */
extern int            DAT_00523410;
extern int            DAT_009ecc34;
extern char           DAT_009d695c;
extern unsigned short DAT_009ed22e;
extern unsigned char  DAT_009ed22d;
extern unsigned char  DAT_009ed22c;
extern unsigned int   DAT_009ed228;
extern unsigned int   DAT_009d8b74;
extern unsigned int   DAT_009d8b70;
extern unsigned short DAT_00a08292;
extern unsigned short DAT_009d7684;
extern unsigned char *DAT_009a1840[];
extern unsigned short DAT_00a07be0[];
extern unsigned char *DAT_00a08120[];
extern unsigned short DAT_00520eb0[];
extern int            DAT_00a00860;
extern int            DAT_005235d4;
extern unsigned short DAT_005235d0;
extern int            DAT_005235d8;
extern int            DAT_009ecc3c;
extern unsigned short DAT_009ec5fc;
extern unsigned int   DAT_009993fc;
extern unsigned char  DAT_009d77f8;
extern int            DAT_009a7b54;
extern int            DAT_005569f8;
extern char           DAT_00556a01;
extern unsigned int   DAT_00556a50;
extern unsigned int   DAT_009a6ff0;
extern unsigned char  DAT_009a6ff9;
extern unsigned short DAT_009a7044[];

void         __stdcall FUN_00450550(void);
unsigned int __cdecl   FUN_004ac720(unsigned int param_1);
void         __stdcall FUN_004ac740(void);
void         __cdecl   FUN_00450c90(unsigned short param_1);
void         __cdecl   FUN_00450bc0(void);

void __cdecl FUN_00450650(int param_1, int param_2)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    int bVar3;
    unsigned int uVar4;
    unsigned short uVar5;
    unsigned char *pbVar6;
    unsigned short uVar9;
    unsigned int uVar11;
    unsigned char *pbVar12;
    unsigned int *puVar13;
    unsigned int *puVar14;
    int iVar10;
    unsigned short uStack_5c;
    unsigned int uStack_5a;
    struct {
        unsigned short uStack_54_lo;
        unsigned short uStack_54_hi;
        unsigned short uStack_50;
        unsigned char  uStack_4e;
        unsigned char  uStack_4d;
        unsigned char  uStack_4c;
        unsigned char  bStack_4b;
        unsigned short uStack_4a;
        unsigned char  uStack_48;
        unsigned char  uStack_47;
        unsigned char  abStack_46[70]; /* [69] is Ghidra's uStack_1 */
    } blk;

    bVar2 = DAT_009d77f8;
    if ((DAT_00523410 == 0) && (param_2 == DAT_009ecc34)) {
        blk.uStack_54_lo = (unsigned short)param_1;
        if (DAT_009d695c == '\0') {
            blk.uStack_50 = DAT_009ed22e;
            blk.uStack_4e = DAT_009ed22d;
            blk.uStack_4d = DAT_009ed22c;
            blk.uStack_4c = (unsigned char)(DAT_009ed228 >> 24);
        } else {
            blk.uStack_50 = (unsigned short)(DAT_009d8b74 >> 16);
            blk.uStack_4e = (unsigned char)(DAT_009d8b74 >> 8);
            blk.uStack_4d = (unsigned char)DAT_009d8b74;
            blk.uStack_4c = (unsigned char)(DAT_009d8b70 >> 24);
        }
        blk.uStack_48 = 0;
        blk.uStack_47 = 0;
        blk.uStack_4a = DAT_00a08292;
        if ((param_1 == 0xcf) || (param_1 == 0xc4)) {
            blk.uStack_54_hi = DAT_009d7684;
        } else {
            blk.uStack_54_hi = 0;
        }
        uVar9 = 0;
        pbVar12 = DAT_009a1840[param_1];
        bVar2 = *pbVar12;
        uStack_5a = (unsigned int)bVar2 << 0x10;
        uVar4 = uStack_5a;
        while (bVar2 != 0) {
            if (0x45 < uVar9) goto LAB_0045087a;
            bVar2 = *pbVar12;
            uStack_5a = uVar4;
            if (bVar2 == 0x2a) {
                pbVar12 = pbVar12 + 1;
                uStack_5a = (uStack_5a & 0xffff0000u) |
                            (unsigned int)(unsigned short)((unsigned short)uStack_5a + 1u);
                uVar5 = DAT_00a07be0[uVar4 & 0xffff];
                if (uVar5 < 0x8000) {
                    pbVar6 = DAT_009a1840[uVar5];
                } else {
                    pbVar6 = DAT_00a08120[uVar5 & 0x7fff];
                }
                bVar2 = *pbVar6;
                for (; (bVar2 != 0 && (uVar9 < 0x46)); uVar9 = (unsigned short)(uVar9 + 1)) {
                    bVar2 = *pbVar6;
                    pbVar6 = pbVar6 + 1;
                    blk.abStack_46[uVar9] = bVar2;
                    bVar2 = *pbVar6;
                }
            } else if (bVar2 == 0x26) {
                pbVar1 = pbVar12 + 1;
                uVar11 = (unsigned int)*pbVar1;
                pbVar12 = pbVar12 + 2;
                bVar3 = 0;
                uStack_5a = (uStack_5a & 0xffff0000u) |
                            (unsigned int)(unsigned short)((unsigned short)uStack_5a + 1u);
                uStack_5c = DAT_00a07be0[uVar4 & 0xffff];
                pbVar6 = (unsigned char *)0;
                if (*pbVar1 != 0) {
                    do {
                        pbVar6 = (unsigned char *)0;
                        if (0x45 < uVar9) break;
                        uVar5 = (unsigned short)(uStack_5c / DAT_00520eb0[uVar11]);
                        uStack_5c = (unsigned short)(uStack_5c -
                                    (unsigned short)((unsigned int)DAT_00520eb0[uVar11] *
                                                     (unsigned int)uVar5));
                        if (((bVar3) || ((unsigned short)uVar11 < 2)) || (uVar5 != 0)) {
                            bVar3 = 1;
                            if (9 < uVar5) {
                                uVar5 = 9;
                            }
                            pbVar6 = (unsigned char *)(unsigned int)(unsigned short)(uVar5 + 0x30);
                        } else {
                            pbVar6 = (unsigned char *)0x20;
                        }
                        if ((short)(unsigned int)pbVar6 != 0x20) {
                            blk.abStack_46[uVar9] = (unsigned char)(unsigned int)pbVar6;
                            uVar9 = (unsigned short)(uVar9 + 1);
                        }
                        uVar5 = (unsigned short)((unsigned short)uVar11 - 1);
                        uVar11 = (unsigned int)uVar5;
                    } while (uVar5 != 0);
                }
            } else {
                pbVar12 = pbVar12 + 1;
                pbVar6 = (unsigned char *)(unsigned int)uVar9;
                blk.abStack_46[(int)pbVar6] = bVar2;
                uStack_5a = uVar4;
                uVar9 = (unsigned short)(uVar9 + 1);
            }
            uVar4 = uStack_5a;
            bVar2 = *pbVar12;
        }
        if (uVar9 < 0x46) {
            blk.abStack_46[uVar9] = 0;
        } else {
LAB_0045087a:
            blk.abStack_46[69] = 0; /* uStack_1 */
        }
        blk.bStack_4b = (unsigned char)(uVar4 >> 0x10);
        if (8 < (unsigned char)(uVar4 >> 0x10)) {
            blk.bStack_4b = 6;
        }
        uVar9 = (unsigned short)blk.bStack_4b;
        if ((DAT_00a00860 == 0) && ((uVar9 == 2 || (uVar9 == 1)))) {
            DAT_005235d4 = (int)((unsigned int)DAT_005235d4 + 1u);
            DAT_005235d0 = (unsigned short)(DAT_005235d0 + 1);
            if (DAT_005235d0 == 300) {
                if (DAT_009ecc3c != 0) {
                    FUN_00450550();
                }
                DAT_005235d0 = 0;
                DAT_005235d8 = 1;
            }
            DAT_009993fc = FUN_004ac720((unsigned int)DAT_009ec5fc);
            FUN_004ac740();
            puVar13 = (unsigned int *)&blk;
            puVar14 = (unsigned int *)(DAT_009993fc + (unsigned int)DAT_005235d0 * 0x54u);
            for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                *puVar14 = *puVar13;
                puVar13 = puVar13 + 1;
                puVar14 = puVar14 + 1;
            }
        }
        bVar2 = DAT_009d77f8;
        if (((uVar9 == 3) || (uVar9 == 4)) || (uVar9 == 7)) {
            if (((DAT_009a7b54 != 0) || (param_1 == 400)) &&
                ((DAT_005569f8 == -1 || ((DAT_00556a01 != '\x04' || (uVar9 == 7)))))) {
                puVar13 = (unsigned int *)&blk;
                puVar14 = (unsigned int *)&DAT_005569f8;
                for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                    *puVar14 = *puVar13;
                    puVar13 = puVar13 + 1;
                    puVar14 = puVar14 + 1;
                }
                FUN_00450c90(uVar9);
                bVar2 = DAT_009d77f8;
            }
        } else {
            if (uVar9 == 8) {
                puVar13 = (unsigned int *)&blk;
                puVar14 = &DAT_00556a50;
                for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                    *puVar14 = *puVar13;
                    puVar13 = puVar13 + 1;
                    puVar14 = puVar14 + 1;
                }
                FUN_00450c90(8);
                return;
            }
            if ((short)DAT_009a6ff0 == -1) {
                puVar13 = (unsigned int *)&blk;
                puVar14 = &DAT_009a6ff0;
                for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                    *puVar14 = *puVar13;
                    puVar13 = puVar13 + 1;
                    puVar14 = puVar14 + 1;
                }
                FUN_00450c90(uVar9);
                return;
            }
            switch (DAT_009a6ff9) {
            case 1:
                if ((uVar9 != 2) && (uVar9 != 1)) {
                    FUN_00450bc0();
                    puVar13 = (unsigned int *)&blk;
                    puVar14 = &DAT_009a6ff0;
                    for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                        *puVar14 = *puVar13;
                        puVar13 = puVar13 + 1;
                        puVar14 = puVar14 + 1;
                    }
                    FUN_00450c90(uVar9);
                    return;
                }
                puVar13 = (unsigned int *)&blk;
                puVar14 = (unsigned int *)(DAT_009a7044 + (unsigned int)DAT_009d77f8 * 0x2a);
                for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                    *puVar14 = *puVar13;
                    puVar13 = puVar13 + 1;
                    puVar14 = puVar14 + 1;
                }
                bVar2 = (unsigned char)(DAT_009d77f8 + 1);
                if (9 < (unsigned char)(DAT_009d77f8 + 1)) {
                    return;
                }
                break;
            case 2:
            case 5:
                if (uVar9 != 2) {
                    FUN_00450bc0();
                    puVar13 = (unsigned int *)&blk;
                    puVar14 = &DAT_009a6ff0;
                    for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                        *puVar14 = *puVar13;
                        puVar13 = puVar13 + 1;
                        puVar14 = puVar14 + 1;
                    }
                    FUN_00450c90(uVar9);
                    return;
                }
                puVar13 = (unsigned int *)&blk;
                puVar14 = (unsigned int *)(DAT_009a7044 + (unsigned int)DAT_009d77f8 * 0x2a);
                for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                    *puVar14 = *puVar13;
                    puVar13 = puVar13 + 1;
                    puVar14 = puVar14 + 1;
                }
                bVar2 = (unsigned char)(DAT_009d77f8 + 1);
                if (9 < (unsigned char)(DAT_009d77f8 + 1)) {
                    return;
                }
                break;
            case 3:
            case 6:
            case 7:
            case 8:
                puVar13 = (unsigned int *)&blk;
                puVar14 = &DAT_009a6ff0;
                for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                    *puVar14 = *puVar13;
                    puVar13 = puVar13 + 1;
                    puVar14 = puVar14 + 1;
                }
                FUN_00450c90(uVar9);
                return;
            case 4:
                if ((uVar9 != 3) && (uVar9 != 8)) {
                    if ((uVar9 != 2) && (uVar9 != 1)) {
                        puVar13 = (unsigned int *)&blk;
                        puVar14 = &DAT_009a6ff0;
                        for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                            *puVar14 = *puVar13;
                            puVar13 = puVar13 + 1;
                            puVar14 = puVar14 + 1;
                        }
                        FUN_00450c90(uVar9);
                        return;
                    }
                    puVar13 = (unsigned int *)&blk;
                    puVar14 = (unsigned int *)(DAT_009a7044 + (unsigned int)DAT_009d77f8 * 0x2a);
                    for (iVar10 = 0x15; iVar10 != 0; iVar10 = iVar10 + -1) {
                        *puVar14 = *puVar13;
                        puVar13 = puVar13 + 1;
                        puVar14 = puVar14 + 1;
                    }
                    bVar2 = (unsigned char)(DAT_009d77f8 + 1);
                    if (9 < (unsigned char)(DAT_009d77f8 + 1)) {
                        return;
                    }
                }
            }
        }
    }
    DAT_009d77f8 = bVar2;
    return;
}
