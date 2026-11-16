/* SPDX-License-Identifier: MIT */
/* z_xvt 00463e10 */
extern unsigned short DAT_005f10f8;
extern unsigned int   DAT_005f1110;
extern unsigned short DAT_005f10e8;
extern unsigned int   DAT_005f1d28;
extern unsigned short DAT_005f10ec;
extern unsigned short DAT_005f10f4;
extern unsigned int  *DAT_005f1d20;
extern int           *DAT_005f1d24;
extern unsigned int   DAT_0052552c;
extern unsigned int   DAT_00525534;
extern unsigned int   DAT_0052550c;
extern unsigned int   DAT_0070acc8;
extern unsigned char  DAT_00732e50;

void __cdecl FUN_00463e10(unsigned short param_1)
{
    int iVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned char *puVar5;
    unsigned int uVar6;
    short sVar7;
    unsigned char bVar8;

    DAT_005f1110 = (unsigned int)DAT_005f10f8;
    DAT_005f1d28 = (unsigned int)DAT_005f10e8 - DAT_005f1110;
    bVar8 = 0;
    if ((short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4) < 1) {
        return;
    }
    uVar3 = *(unsigned int *)((unsigned char *)DAT_005f1d20 + DAT_005f1110 * 4);
    if (DAT_0052552c == 0x13) {
LAB_00463e9a:
        if (DAT_00525534 == 0xa0000) goto LAB_00463ead;
    }
    else if (DAT_00525534 == 0xa0000) {
        uVar3 = uVar3 % DAT_0052550c;
        if (0xffffu < DAT_0070acc8 * DAT_005f1d28 + uVar3) {
            bVar8 = 1;
        }
        goto LAB_00463e9a;
    }
    bVar8 = 1;
LAB_00463ead:
    if (bVar8) {
        if (param_1 == 0) {
            for (; DAT_005f1d28 != 0; DAT_005f1d28 = DAT_005f1d28 - 1) {
                uVar3 = (unsigned int)DAT_005f10f4 + *(unsigned int *)((unsigned char *)DAT_005f1d20 + DAT_005f1110 * 4);
                if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
                    uVar3 = uVar3 % DAT_0052550c;
                }
                sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
                puVar5 = (unsigned char *)(uVar3 + DAT_00525534);
                if (sVar7 < 1) {
                    return;
                }
                for (; sVar7 != 0; sVar7 = (short)(sVar7 + -1)) {
                    *puVar5 = DAT_00732e50;
                    puVar5 = puVar5 + 1;
                }
                DAT_005f1110 = DAT_005f1110 + 1;
            }
        }
        else {
            uVar3 = 0;
            do {
                uVar6 = *(unsigned int *)((unsigned char *)DAT_005f1d20 + DAT_005f1110 * 4) + (unsigned int)DAT_005f10f4;
                if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
                    uVar6 = uVar6 % DAT_0052550c;
                }
                sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
                puVar5 = (unsigned char *)(uVar6 + DAT_00525534);
                if (sVar7 < 1) {
                    return;
                }
                for (; sVar7 != 0; sVar7 = (short)(sVar7 + -1)) {
                    *puVar5 = DAT_00732e50;
                    puVar5 = puVar5 + 1;
                }
                uVar3 = uVar3 + 1;
                DAT_005f1d28 = DAT_005f1d28 - 1;
                DAT_005f1110 = DAT_005f1110 + 1;
            } while (uVar3 < param_1);
            uVar3 = (unsigned int)param_1;
            for (; uVar3 < DAT_005f1d28; DAT_005f1d28 = DAT_005f1d28 - 1) {
                uVar6 = (unsigned int)DAT_005f10f4 + *(unsigned int *)((unsigned char *)DAT_005f1d20 + DAT_005f1110 * 4);
                if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
                    uVar6 = uVar6 % DAT_0052550c;
                }
                sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
                puVar5 = (unsigned char *)(uVar6 + DAT_00525534);
                iVar1 = (int)(uVar3 - 1);
                if (param_1 != 0) {
                    do {
                        *puVar5 = DAT_00732e50;
                        puVar5 = puVar5 + 1;
                        bVar8 = (iVar1 != 0);
                        iVar1 = (int)((unsigned int)iVar1 - 1u);
                    } while (bVar8);
                }
                puVar5 = puVar5 + (short)((unsigned int)(int)sVar7 + (unsigned int)param_1 * 0xFFFFFFFEu);
                iVar1 = (int)(uVar3 - 1);
                if (param_1 != 0) {
                    do {
                        *puVar5 = DAT_00732e50;
                        bVar8 = (iVar1 != 0);
                        puVar5 = puVar5 + 1;
                        iVar1 = (int)((unsigned int)iVar1 - 1u);
                    } while (bVar8);
                }
                DAT_005f1110 = DAT_005f1110 + 1;
            }
            uVar6 = 0;
            if (param_1 != 0) {
                while (1) {
                    uVar4 = (unsigned int)DAT_005f10f4 + *(unsigned int *)((unsigned char *)DAT_005f1d20 + DAT_005f1110 * 4);
                    if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
                        uVar4 = uVar4 % DAT_0052550c;
                    }
                    sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
                    puVar5 = (unsigned char *)(uVar4 + DAT_00525534);
                    if (sVar7 < 1) break;
                    for (; sVar7 != 0; sVar7 = (short)(sVar7 + -1)) {
                        *puVar5 = DAT_00732e50;
                        puVar5 = puVar5 + 1;
                    }
                    uVar6 = uVar6 + 1;
                    DAT_005f1d28 = DAT_005f1d28 - 1;
                    DAT_005f1110 = DAT_005f1110 + 1;
                    if (uVar3 <= uVar6) {
                        return;
                    }
                }
            }
        }
    }
    else if (param_1 == 0) {
        iVar1 = (int)(uVar3 + DAT_00525534);
        if (DAT_005f1d28 != 0) {
            while (1) {
                sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
                puVar5 = (unsigned char *)((unsigned int)DAT_005f10f4 + (unsigned int)iVar1);
                if (sVar7 < 1) break;
                for (; sVar7 != 0; sVar7 = (short)(sVar7 + -1)) {
                    *puVar5 = DAT_00732e50;
                    puVar5 = puVar5 + 1;
                }
                DAT_005f1d28 = DAT_005f1d28 - 1;
                DAT_005f1110 = DAT_005f1110 + 1;
                iVar1 = (int)((unsigned int)iVar1 + (unsigned int)*DAT_005f1d24);
                if (DAT_005f1d28 == 0) {
                    return;
                }
            }
        }
    }
    else {
        iVar1 = (int)(uVar3 + DAT_00525534);
        uVar3 = 0;
        if (param_1 != 0) {
            do {
                sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
                puVar5 = (unsigned char *)((unsigned int)DAT_005f10f4 + (unsigned int)iVar1);
                if (sVar7 < 1) {
                    return;
                }
                for (; sVar7 != 0; sVar7 = (short)(sVar7 + -1)) {
                    *puVar5 = DAT_00732e50;
                    puVar5 = puVar5 + 1;
                }
                uVar3 = uVar3 + 1;
                DAT_005f1d28 = DAT_005f1d28 - 1;
                iVar1 = (int)((unsigned int)iVar1 + (unsigned int)*DAT_005f1d24);
                DAT_005f1110 = DAT_005f1110 + 1;
            } while (uVar3 < param_1);
        }
        uVar3 = (unsigned int)param_1;
        for (; uVar3 < DAT_005f1d28; DAT_005f1d28 = DAT_005f1d28 - 1) {
            sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
            puVar5 = (unsigned char *)((unsigned int)DAT_005f10f4 + (unsigned int)iVar1);
            iVar2 = (int)(uVar3 - 1);
            if (param_1 != 0) {
                do {
                    *puVar5 = DAT_00732e50;
                    puVar5 = puVar5 + 1;
                    bVar8 = (iVar2 != 0);
                    iVar2 = (int)((unsigned int)iVar2 - 1u);
                } while (bVar8);
            }
            iVar2 = (int)(uVar3 - 1);
            puVar5 = puVar5 + (short)((unsigned int)(int)sVar7 + (unsigned int)param_1 * 0xFFFFFFFEu);
            if (param_1 != 0) {
                do {
                    *puVar5 = DAT_00732e50;
                    bVar8 = (iVar2 != 0);
                    iVar2 = (int)((unsigned int)iVar2 - 1u);
                    puVar5 = puVar5 + 1;
                } while (bVar8);
            }
            DAT_005f1110 = DAT_005f1110 + 1;
            iVar1 = (int)((unsigned int)iVar1 + (unsigned int)*DAT_005f1d24);
        }
        uVar6 = 0;
        if (param_1 != 0) {
            while (1) {
                sVar7 = (short)((unsigned int)DAT_005f10ec - (unsigned int)DAT_005f10f4);
                puVar5 = (unsigned char *)((unsigned int)DAT_005f10f4 + (unsigned int)iVar1);
                if (sVar7 < 1) break;
                for (; sVar7 != 0; sVar7 = (short)(sVar7 + -1)) {
                    *puVar5 = DAT_00732e50;
                    puVar5 = puVar5 + 1;
                }
                uVar6 = uVar6 + 1;
                DAT_005f1d28 = DAT_005f1d28 - 1;
                iVar1 = (int)((unsigned int)iVar1 + (unsigned int)*DAT_005f1d24);
                DAT_005f1110 = DAT_005f1110 + 1;
                if (uVar3 <= uVar6) {
                    return;
                }
            }
        }
    }
    return;
}
