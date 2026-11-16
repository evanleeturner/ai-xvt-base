/* SPDX-License-Identifier: MIT */
/* z_bop 00403fe0 */
typedef unsigned char bool8;

extern char DAT_009d1320[];
extern char DAT_0051a3a8[];
extern char DAT_0051a2bc[];
extern char s_formevadeldr1pln_0051a394[];
extern char s_starshipformpln_0051a384[];
extern char s_rendezvous1pln_0051a374[];
extern char s_disabledpln_0051a368[];
extern char s_waitforboardpln_0051a358[];
extern char s_capfreeldr1pln_0051a348[];
extern char s_capescortersldr1pln_0051a334[];
extern char s_caprespondldr1pln_0051a320[];
extern char s_escortldr1pln_0051a1e4[];
extern char s_disableldr1pln_0051a310[];
extern char s_starshipprotectpln_0051a2fc[];
extern char s_starshipattackpln_0051a2e8[];
extern char s_starshipdisablepln_0051a2d4[];
extern char s_boardtogivepln_0051a254[];
extern char s_boardtotakepln_0051a244[];
extern char s_boardtoexchangepln_0051a230[];
extern char s_boardtocapturepln_0051a278[];
extern char s_boardtodestroypln_0051a264[];
extern char s_boardtopickuppln_0051a21c[];
extern char s_boardtocontactpln_0051a208[];
extern char s_boardtorepairpln_0051a1f4[];
extern char s_dropoffldr1pln_0051a2c4[];
extern char s_starshipwaitreturnpln_0051a2a4[];
extern char s_starshipwaitcreatepln_0051a28c[];

extern unsigned char DAT_009a73b6[4]; /* packed int at odd address 0x009a73b6 */
extern unsigned short DAT_009a73c0;
extern unsigned char DAT_009d8cd4[];
extern unsigned char DAT_009d8cd5[];
extern unsigned short DAT_009a20ca[];

unsigned long long __fastcall FUN_00468cf0(unsigned int param_1, unsigned int param_2);
unsigned int FUN_00468e40(void);
unsigned short FUN_0045ddf0(unsigned short param_1);
unsigned short FUN_00462830(unsigned int param_1);

bool8 __cdecl FUN_00403fe0(unsigned int param_1, unsigned short param_2)
{
    unsigned int uVar1;
    unsigned int uVar2;
    int iVar3;
    int iBase;
    int iTmp;
    char *pcVar5;
    char *pcVar6;
    char *pcVar8;
    bool8 bVar9;
    unsigned short uVar11;
    unsigned long long uVar10;

    uVar1 = param_1 & 0xffff;
    bVar9 = (uVar1 == 0);
    pcVar6 = &DAT_009d1320[uVar1 * 0x55];

    iVar3 = 0xc;
    pcVar5 = pcVar6;
    pcVar8 = DAT_0051a3a8;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_final;

    iVar3 = 0x11;
    pcVar5 = pcVar6;
    pcVar8 = s_formevadeldr1pln_0051a394;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_final;

    iVar3 = 0x10;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipformpln_0051a384;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_final;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_rendezvous1pln_0051a374;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_final;

    iVar3 = 0xc;
    pcVar5 = pcVar6;
    pcVar8 = s_disabledpln_0051a368;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_final;

    iVar3 = 0x10;
    pcVar5 = pcVar6;
    pcVar8 = s_waitforboardpln_0051a358;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_final;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_capfreeldr1pln_0051a348;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0x14;
    pcVar5 = pcVar6;
    pcVar8 = s_capescortersldr1pln_0051a334;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_caprespondldr1pln_0051a320;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0xe;
    pcVar5 = pcVar6;
    pcVar8 = s_escortldr1pln_0051a1e4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_disableldr1pln_0051a310;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0x13;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipprotectpln_0051a2fc;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipattackpln_0051a2e8;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0x13;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipdisablepln_0051a2d4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_check;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtogivepln_0051a254;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtotakepln_0051a244;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0x13;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtoexchangepln_0051a230;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtocapturepln_0051a278;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtodestroypln_0051a264;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0x11;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtopickuppln_0051a21c;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtocontactpln_0051a208;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0x11;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtorepairpln_0051a1f4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto L_board;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_dropoffldr1pln_0051a2c4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) {
        __builtin_memcpy(&iBase, DAT_009a73b6, 4);
        if ((unsigned short)*(unsigned char *)(iBase + 5 + (unsigned int)param_2) <
            (unsigned short)DAT_009a20ca[(unsigned int)(unsigned short)((unsigned char)DAT_009d8cd5[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)param_2 * 0x52] - 1) * 0x93]) {
            return 0;
        }
        return 1;
    }

    iVar3 = 8;
    bVar9 = 0;
    pcVar5 = pcVar6;
    pcVar8 = DAT_0051a2bc;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) {
        __builtin_memcpy(&iBase, DAT_009a73b6, 4);
        __builtin_memcpy(&iTmp, (char *)(iBase + 0x36), 4);
        if (iTmp != 0) {
            return 0;
        }
        return 1;
    }

    iVar3 = 0x16;
    bVar9 = 0;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipwaitreturnpln_0051a2a4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar5 == *pcVar8);
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) {
        uVar10 = FUN_00468cf0((unsigned int)iVar3, uVar1 * 5);
        if ((short)uVar10 == 0) {
            return 0;
        }
        return 1;
    }

    iVar3 = 0x16;
    bVar9 = 0;
    pcVar5 = s_starshipwaitcreatepln_0051a28c;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = (*pcVar6 == *pcVar5);
        pcVar6 = pcVar6 + 1;
        pcVar5 = pcVar5 + 1;
    } while (bVar9);
    if (!bVar9) {
        return 0;
    }
    uVar2 = FUN_00468e40();
    if ((short)uVar2 == 0) {
        return 0;
    }
    return 1;

L_check:
    uVar11 = FUN_0045ddf0(param_2);
    if ((short)uVar11 != 0) {
        return 0;
    }
    uVar11 = FUN_00462830((unsigned int)param_2);
    if ((short)uVar11 != 0) {
        return 0;
    }
    return 1;

L_board:
    __builtin_memcpy(&iBase, DAT_009a73b6, 4);
    if (*(unsigned char *)(iBase + 5 + (unsigned int)param_2) <
        (unsigned char)DAT_009d8cd5[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)param_2 * 0x52]) {
        return 0;
    }
    return 1;

L_final:
    __builtin_memcpy(&iBase, DAT_009a73b6, 4);
    return (unsigned char)DAT_009d8cd4[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)param_2 * 0x52] <=
           *(unsigned char *)(iBase + 5 + (unsigned int)param_2);
}
