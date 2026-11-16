/* SPDX-License-Identifier: MIT */
/* z_bop 00441c30 */
#define CONCAT22(h, l) ((((unsigned int)(unsigned short)(h)) << 16) | (unsigned int)(unsigned short)(l))

extern short DAT_00a08ba0[];
extern unsigned int DAT_005233d8;
extern unsigned char DAT_005233c8;
extern short DAT_009a7bac;
extern unsigned short DAT_00a08a00;
extern unsigned short DAT_00a08604;
extern unsigned char DAT_009e9683[];
extern int DAT_009ecc34;
extern unsigned short DAT_00a08374;
extern unsigned char DAT_009e9670[];
extern int DAT_009a1fe8;
extern unsigned int DAT_009a806c;
extern unsigned char DAT_009a20ae;
extern unsigned char DAT_009ed232;
extern unsigned char DAT_0051c5ae[];
extern unsigned char DAT_0051c5af[];

extern unsigned char DAT_00a092ea[];
extern unsigned char DAT_00a092e8[];
extern unsigned char DAT_00a092d2[];
extern unsigned char DAT_00a092d0[];
extern unsigned char DAT_00a092de[];
extern unsigned char DAT_00a092dc[];
extern unsigned char DAT_00a092ba[];
extern unsigned char DAT_00a092b8[];
extern unsigned char DAT_00a092c6[];
extern unsigned char DAT_00a092c4[];
extern unsigned char DAT_00a08de4[];
extern unsigned char DAT_00a08de6[];
extern unsigned char DAT_00a08de8[];

extern unsigned short DAT_00a0836a;
extern unsigned short DAT_00a0836e;
extern unsigned short DAT_00a08366;
extern unsigned short DAT_00a0836c;
extern unsigned short DAT_00a08c92;
extern unsigned short DAT_00a08c84;
extern unsigned short DAT_00a08c8c;
extern unsigned short DAT_00a08c80;
extern unsigned short DAT_00a08c90;
extern unsigned short DAT_00a08c82;
extern unsigned short DAT_00a08c8e;
extern unsigned short DAT_00a08c8a;
extern unsigned short DAT_00a08608;
extern unsigned short DAT_00a08606;
extern unsigned short DAT_00a08376;
extern unsigned short DAT_00a083bc;
extern unsigned short DAT_00a08378;
extern unsigned short DAT_00a083b8;

void __cdecl FUN_00427150(unsigned int a1, unsigned int a2, unsigned int a3, unsigned int a4,
                          unsigned int a5, unsigned int a6, unsigned int a7, unsigned int a8,
                          unsigned int a9);
void __cdecl FUN_004a9640(char a1);

void __stdcall FUN_00441c30(void)
{
    unsigned char bVar1;
    unsigned char bVar2;
    short sVar3;
    int iVar4;
    short sVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned short uVar9;
    unsigned int uVar10;
    unsigned int uVar11;
    unsigned short uVar12;
    unsigned short uVar13;
    unsigned short uVar14;
    int iTmp1;
    int iTmp2;

    uVar12 = 0;
    do {
        sVar3 = DAT_00a08ba0[uVar12];
        sVar5 = (short)DAT_005233d8;
        uVar9 = (unsigned short)(DAT_005233d8 >> 0x10);
        uVar13 = DAT_00a08a00;
        uVar14 = DAT_00a08604;
        switch ((unsigned int)uVar12) {
        case 0:
            if (sVar3 != 0) {
                if (DAT_009e9683[(unsigned int)DAT_009ecc34 * 0x5bd] != 0) goto LAB_00441ed6;
                uVar6 = CONCAT22(uVar9, (unsigned int)(int)sVar5 * (unsigned int)(int)DAT_009a7bac);
                iVar4 = (int)((unsigned int)DAT_00a08374 * 0xc);
                uVar8 = CONCAT22(uVar9, *(unsigned short *)(DAT_00a092ea + iVar4));
                uVar10 = CONCAT22((unsigned short)((unsigned int)iVar4 >> 0x10), *(unsigned short *)(DAT_00a092e8 + iVar4));
                uVar11 = CONCAT22((unsigned short)(((unsigned int)DAT_00a08374 * 3) >> 0x10), DAT_00a0836a);
                uVar7 = CONCAT22(uVar9, DAT_00a0836e);
                uVar13 = DAT_00a08366;
                uVar14 = DAT_00a0836c;
LAB_00441f24:
                FUN_00427150(DAT_009a806c, (unsigned short)DAT_005233c8, uVar7, uVar11, uVar10, uVar8, uVar13, uVar14, uVar6);
            }
            break;
        case 1:
            if (sVar3 != 0) {
                if (DAT_009e9683[(unsigned int)DAT_009ecc34 * 0x5bd] != 0) goto LAB_00441ed6;
                uVar6 = CONCAT22(uVar9, (unsigned int)(int)sVar5 * (unsigned int)(int)DAT_009a7bac);
                iVar4 = (int)((unsigned int)DAT_00a08374 * 0xc);
                uVar8 = CONCAT22(uVar9, *(unsigned short *)(DAT_00a092d2 + iVar4));
                uVar10 = CONCAT22((unsigned short)((unsigned int)iVar4 >> 0x10), *(unsigned short *)(DAT_00a092d0 + iVar4));
                uVar11 = CONCAT22((unsigned short)(((unsigned int)DAT_00a08374 * 3) >> 0x10), DAT_00a08c92);
                uVar7 = CONCAT22(uVar9, DAT_00a08c84);
                uVar13 = DAT_00a08c8c;
                uVar14 = DAT_00a08c80;
                goto LAB_00441f24;
            }
            break;
        case 3:
            if ((sVar3 != 0) && (DAT_009e9683[(unsigned int)DAT_009ecc34 * 0x5bd] == 0)) {
                uVar6 = CONCAT22(uVar9, (unsigned int)(int)sVar5 * (unsigned int)(int)DAT_009a7bac);
                iVar4 = (int)((unsigned int)DAT_00a08374 * 0xc);
                uVar8 = CONCAT22(uVar9, *(unsigned short *)(DAT_00a092de + iVar4));
                uVar10 = CONCAT22((unsigned short)((unsigned int)iVar4 >> 0x10), *(unsigned short *)(DAT_00a092dc + iVar4));
                uVar11 = CONCAT22((unsigned short)(((unsigned int)DAT_00a08374 * 3) >> 0x10), DAT_00a08c90);
                uVar7 = CONCAT22(uVar9, DAT_00a08c82);
                uVar13 = DAT_00a08c8e;
                uVar14 = DAT_00a08c8a;
                goto LAB_00441f24;
            }
            break;
        case 4:
            if (sVar3 != 0) {
                if (DAT_009e9683[(unsigned int)DAT_009ecc34 * 0x5bd] != 0) goto LAB_00441ed6;
                uVar6 = CONCAT22(uVar9, (unsigned int)(int)sVar5 * (unsigned int)(int)DAT_009a7bac);
                iVar4 = (int)((unsigned int)DAT_00a08374 * 0xc);
                uVar8 = CONCAT22(uVar9, *(unsigned short *)(DAT_00a092ba + iVar4));
                uVar10 = CONCAT22((unsigned short)((unsigned int)iVar4 >> 0x10), *(unsigned short *)(DAT_00a092b8 + iVar4));
                uVar11 = CONCAT22((unsigned short)(((unsigned int)DAT_00a08374 * 3) >> 0x10), DAT_00a08608);
                uVar7 = CONCAT22(uVar9, DAT_00a08606);
                goto LAB_00441f24;
            }
            break;
        case 5:
            if (sVar3 != 0) {
                uVar6 = CONCAT22(uVar9, (unsigned int)(int)sVar5 * (unsigned int)(int)DAT_009a7bac);
                iVar4 = (int)((unsigned int)DAT_00a08374 * 0xc);
                uVar8 = CONCAT22(uVar9, *(unsigned short *)(DAT_00a092ba + iVar4));
                uVar10 = CONCAT22((unsigned short)((unsigned int)iVar4 >> 0x10), *(unsigned short *)(DAT_00a092b8 + iVar4));
                uVar11 = CONCAT22((unsigned short)(((unsigned int)DAT_00a08374 * 3) >> 0x10), DAT_00a08608);
                uVar7 = CONCAT22(uVar9, DAT_00a08606);
                goto LAB_00441f24;
            }
            break;
        case 6:
            if ((sVar3 != 0) && (DAT_009e9683[(unsigned int)DAT_009ecc34 * 0x5bd] != 0)) {
LAB_00441ed6:
                uVar6 = CONCAT22(uVar9, (unsigned int)(int)sVar5 * (unsigned int)(int)DAT_009a7bac);
                iVar4 = (int)((unsigned int)DAT_00a08374 * 0xc);
                uVar8 = CONCAT22(uVar9, *(unsigned short *)(DAT_00a092c6 + iVar4));
                uVar10 = CONCAT22((unsigned short)((unsigned int)iVar4 >> 0x10), *(unsigned short *)(DAT_00a092c4 + iVar4));
                uVar11 = CONCAT22((unsigned short)(((unsigned int)DAT_00a08374 * 3) >> 0x10), DAT_00a08376);
                uVar7 = CONCAT22(uVar9, DAT_00a083bc);
                uVar13 = DAT_00a08378;
                uVar14 = DAT_00a083b8;
                goto LAB_00441f24;
            }
        }
        uVar12 = (unsigned short)(uVar12 + 1);
        if (7 < uVar12) {
            if (DAT_00a08374 == 0x90) {
                __builtin_memcpy(&iTmp1, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, sizeof(int));
                __builtin_memcpy(&iTmp2, (unsigned char *)(int)((unsigned int)iTmp1 * 0x23 + 0x1f + (unsigned int)DAT_009a1fe8), sizeof(int));
                __builtin_memcpy(&iVar4, (unsigned char *)(int)((unsigned int)iTmp2 + 0xa9), sizeof(int));
                if ((*(unsigned char *)(iVar4 + 0x1e1) & 8) != 0) {
                    bVar1 = *(unsigned char *)(iVar4 + 4);
                    FUN_004a9640('\0');
                    uVar12 = 0;
                    iVar4 = (int)((unsigned int)bVar1 * 0x101);
                    bVar1 = DAT_0051c5af[iVar4];
                    bVar2 = DAT_0051c5ae[iVar4];
                    if ((unsigned short)((unsigned short)bVar1 + (unsigned short)bVar2) != 0) {
                        do {
                            iVar4 = (int)(((unsigned int)uVar12 + (unsigned int)DAT_00a08374) * 0xc);
                            if (*(short *)(DAT_00a08de8 + iVar4) != 0) {
                                uVar6 = CONCAT22((unsigned short)(DAT_005233d8 >> 0x10),
                                                 (unsigned int)(int)(short)DAT_005233d8 * (unsigned int)(int)DAT_009a7bac);
                                uVar8 = ((((uVar6 >> 8) & 0xffffff) << 8) | (unsigned int)DAT_009a20ae) & 0xffff00ff;
                                uVar9 = (unsigned short)(uVar8 >> 0x10);
                                FUN_00427150(DAT_009a806c, (unsigned short)DAT_005233c8,
                                             CONCAT22(uVar9, (unsigned int)uVar12 * ((unsigned int)DAT_009ed232 + 1) + 2),
                                             2,
                                             CONCAT22((unsigned short)((unsigned int)iVar4 >> 0x10), *(unsigned short *)(DAT_00a08de4 + iVar4)),
                                             CONCAT22(uVar9, *(unsigned short *)(DAT_00a08de6 + iVar4)),
                                             (unsigned int)(int)*(short *)(DAT_00a08de8 + iVar4) * ((unsigned int)DAT_009ed232 + 1),
                                             (unsigned short)uVar8,
                                             uVar6);
                            }
                            uVar12 = (unsigned short)(uVar12 + 1);
                        } while (uVar12 < (unsigned short)((unsigned short)bVar1 + (unsigned short)bVar2));
                    }
                }
            }
            return;
        }
    } while (1);
}
