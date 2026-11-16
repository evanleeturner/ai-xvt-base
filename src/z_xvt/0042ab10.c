/* SPDX-License-Identifier: MIT */
/* z_xvt 0042ab10 */
extern char DAT_005250b4[];
extern char DAT_00524fd4[];
extern char DAT_00703ca0[];
extern unsigned char DAT_0070ad94[];
extern unsigned char DAT_0070ad95[];
extern unsigned short DAT_006d4a6a[];
extern unsigned char DAT_006d9d56[4];
extern unsigned short DAT_006d9d60;

extern char s_formevadeldr1pln_005250a0[];
extern char s_starshipformpln_00525090[];
extern char s_rendezvous1pln_00525080[];
extern char s_disabledpln_00524914[];
extern char s_waitforboardpln_00525070[];
extern char s_capfreeldr1pln_00525060[];
extern char s_capescortersldr1pln_0052504c[];
extern char s_caprespondldr1pln_00525038[];
extern char s_escortldr1pln_00524904[];
extern char s_disableldr1pln_00525028[];
extern char s_starshipprotectpln_00525014[];
extern char s_starshipattackpln_00525000[];
extern char s_starshipdisablepln_00524fec[];
extern char s_boardtogivepln_00523c18[];
extern char s_boardtotakepln_00524f6c[];
extern char s_boardtoexchangepln_00524f58[];
extern char s_boardtocapturepln_00524f90[];
extern char s_boardtodestroypln_00524f7c[];
extern char s_boardtopickuppln_00524f44[];
extern char s_boardtocontactpln_00524f30[];
extern char s_boardtorepairpln_00524f1c[];
extern char s_dropoffldr1pln_00524fdc[];
extern char s_starshipwaitreturnpln_00524fbc[];
extern char s_starshipwaitcreatepln_00524fa4[];

_Bool FUN_0044c7b0(int param_1, unsigned int param_2, unsigned short param_3);
_Bool FUN_0044fd90(unsigned int param_1);
unsigned int FUN_00456330(void);
unsigned int FUN_00456470(void);

_Bool __cdecl FUN_0042ab10(unsigned int param_1, unsigned short param_2)
{
    unsigned int uVar1;
    unsigned int uVar2;
    int iVar3;
    unsigned short uVar7;
    char *pcVar4;
    char *pcVar5;
    char *pcVar6;
    char *pcVar8;
    _Bool bVar9;
    unsigned int uBase;
    unsigned int uAddr;
    int iVal;

    uVar1 = param_1 & 0xffff;
    bVar9 = uVar1 == 0;
    pcVar6 = DAT_00703ca0 + uVar1 * 0x55;

    iVar3 = 0xc;
    pcVar5 = pcVar6;
    pcVar8 = DAT_005250b4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto final_ret;

    iVar3 = 0x11;
    pcVar5 = pcVar6;
    pcVar8 = s_formevadeldr1pln_005250a0;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto final_ret;

    iVar3 = 0x10;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipformpln_00525090;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto final_ret;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_rendezvous1pln_00525080;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto final_ret;

    iVar3 = 0xc;
    pcVar5 = pcVar6;
    pcVar8 = s_disabledpln_00524914;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto final_ret;

    iVar3 = 0x10;
    pcVar5 = pcVar6;
    pcVar8 = s_waitforboardpln_00525070;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto final_ret;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_capfreeldr1pln_00525060;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0x14;
    pcVar5 = pcVar6;
    pcVar8 = s_capescortersldr1pln_0052504c;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_caprespondldr1pln_00525038;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0xe;
    pcVar5 = pcVar6;
    pcVar8 = s_escortldr1pln_00524904;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_disableldr1pln_00525028;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0x13;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipprotectpln_00525014;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipattackpln_00525000;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0x13;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipdisablepln_00524fec;
    do {
        pcVar4 = pcVar5;
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        pcVar4 = pcVar5 + 1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar4;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    uVar7 = (unsigned short)((unsigned int)pcVar4 >> 0x10);
    if (bVar9) goto call_section;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtogivepln_00523c18;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtotakepln_00524f6c;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0x13;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtoexchangepln_00524f58;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtocapturepln_00524f90;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtodestroypln_00524f7c;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0x11;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtopickuppln_00524f44;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0x12;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtocontactpln_00524f30;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0x11;
    pcVar5 = pcVar6;
    pcVar8 = s_boardtorepairpln_00524f1c;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) goto board_ret;

    iVar3 = 0xf;
    pcVar5 = pcVar6;
    pcVar8 = s_dropoffldr1pln_00524fdc;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) {
        __builtin_memcpy(&uBase, DAT_006d9d56, 4);
        if ((unsigned short)*(unsigned char *)((unsigned int)param_2 + 5 + uBase) <
            DAT_006d4a6a[(unsigned short)((unsigned char)DAT_0070ad95[(unsigned int)DAT_006d9d60 * 0x566 + (unsigned int)param_2 * 0x52] - 1) * 0x93]) {
            return 0;
        }
        return 1;
    }

    iVar3 = 8;
    bVar9 = 0;
    pcVar5 = pcVar6;
    pcVar8 = DAT_00524fd4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) {
        __builtin_memcpy(&uBase, DAT_006d9d56, 4);
        uAddr = uBase + 0x36;
        __builtin_memcpy(&iVal, (void *)uAddr, 4);
        if (iVal != 0) {
            return 0;
        }
        return 1;
    }

    iVar3 = 0x16;
    bVar9 = 0;
    pcVar5 = pcVar6;
    pcVar8 = s_starshipwaitreturnpln_00524fbc;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar5 == *pcVar8;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
    } while (bVar9);
    if (bVar9) {
        uVar2 = FUN_00456330();
        if ((short)uVar2 == 0) {
            return 0;
        }
        return 1;
    }

    iVar3 = 0x16;
    bVar9 = 0;
    pcVar5 = s_starshipwaitcreatepln_00524fa4;
    do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 + -1;
        bVar9 = *pcVar6 == *pcVar5;
        pcVar6 = pcVar6 + 1;
        pcVar5 = pcVar5 + 1;
    } while (bVar9);
    if (!bVar9) {
        return 0;
    }
    uVar2 = FUN_00456470();
    if ((short)uVar2 == 0) {
        return 0;
    }
    return 1;

call_section:
    bVar9 = FUN_0044c7b0(iVar3, uVar1 * 5, param_2);
    if ((short)bVar9 != 0) {
        return 0;
    }
    bVar9 = FUN_0044fd90(((unsigned int)uVar7 << 16) | (unsigned int)param_2);
    if ((short)bVar9 != 0) {
        return 0;
    }
    return 1;

board_ret:
    __builtin_memcpy(&uBase, DAT_006d9d56, 4);
    if (*(unsigned char *)((unsigned int)param_2 + 5 + uBase) <
        (unsigned char)DAT_0070ad95[(unsigned int)DAT_006d9d60 * 0x566 + (unsigned int)param_2 * 0x52]) {
        return 0;
    }
    return 1;

final_ret:
    __builtin_memcpy(&uBase, DAT_006d9d56, 4);
    return (unsigned char)DAT_0070ad94[(unsigned int)DAT_006d9d60 * 0x566 + (unsigned int)param_2 * 0x52] <=
           *(unsigned char *)((unsigned int)param_2 + 5 + uBase);
}
