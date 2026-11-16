/* SPDX-License-Identifier: MIT */
/* z_xvt 0046372a */
extern short DAT_00730894;
extern unsigned short DAT_00732e58;
extern short DAT_0070acc0;
extern unsigned short DAT_005f10ec;
extern unsigned short DAT_005f10f4;
extern unsigned short DAT_005f10e8;
extern unsigned short DAT_005f10f8;
extern unsigned char DAT_006d4a4e;
extern unsigned short DAT_00732e52;
extern unsigned short DAT_006d9980;
extern unsigned short DAT_00732e8c;
extern unsigned short DAT_00732a34;
extern char DAT_0070a188;
extern char DAT_006d4998;
extern unsigned char DAT_0070acc2;
extern unsigned int DAT_00709ff4;
extern int DAT_006ffbf0;
extern unsigned int DAT_005f1d20;
extern unsigned int *DAT_005f1d24;
extern int DAT_0052552c;
extern unsigned int DAT_00525534;
extern unsigned int DAT_0052550c;
extern unsigned int DAT_0070acc8;
extern char DAT_0071e524;
extern unsigned char DAT_006da9fa;
extern unsigned char DAT_00732e50;
extern unsigned char DAT_0071b012;

void __cdecl FUN_00463e10(int param_1);

void __stdcall FUN_0046372a(void)
{
    unsigned char bVar1;
    unsigned char bVar2;
    unsigned char in_AL;
    unsigned char in_ZF;
    int iVar3;
    unsigned char uVar4;
    unsigned char *pbVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned char *puVar10;
    int iVar11;
    unsigned int uVar12;
    unsigned char *puVar13;
    unsigned int uTmp;
    int iStack00000004;
    unsigned int uStack00000008;

    /* Register inputs: must be the first statement; setz/mov do not alter flags/AL. */
    __asm__ volatile("setz %1\n\tmovb %%al, %0" : "=m"(in_AL), "=m"(in_ZF));

    if (in_ZF) {
        if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
            DAT_005f10ec = DAT_0070acc0;
            DAT_005f10f4 = DAT_00732e58;
            DAT_005f10e8 = (unsigned short)(DAT_006d4a4e + DAT_00732e52);
            DAT_005f10f8 = DAT_00732e52;
            if ((int)(unsigned int)DAT_00732e58 < (int)(short)DAT_006d9980) {
                DAT_005f10f4 = DAT_006d9980;
            }
            if ((int)(unsigned int)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                DAT_005f10f8 = DAT_00732e8c;
            }
            if ((int)(short)DAT_00732a34 < (int)(unsigned int)DAT_005f10e8) {
                DAT_005f10e8 = DAT_00732a34;
            }
            if (DAT_005f10f8 < DAT_005f10e8) {
                FUN_00463e10(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        DAT_00732e52 = (unsigned short)(DAT_00732e52 + DAT_006d4a4e + 1);
        return;
    }
    if (in_AL < 0x20) {
        return;
    }
    if ((((DAT_0070a188 != '\0') && (DAT_006d4998 == '\0')) && (0x60 < in_AL)) && (in_AL < 0x7b)) {
        in_AL = (unsigned char)(in_AL - 0x20);
    }
    pbVar5 = (unsigned char *)((unsigned int)(unsigned char)(in_AL - 0x20) * (unsigned int)DAT_0070acc2 + DAT_00709ff4);
    bVar1 = pbVar5[1];
    bVar2 = *pbVar5;
    uStack00000008 = (unsigned int)(unsigned short)bVar2;
    if (((int)DAT_0070acc0 <= (int)((int)(short)DAT_00732e58 + uStack00000008)) && (DAT_006ffbf0 != 0)) {
        if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
            DAT_005f10ec = DAT_0070acc0;
            DAT_005f10f4 = DAT_00732e58;
            DAT_005f10e8 = (unsigned short)(DAT_006d4a4e + DAT_00732e52);
            DAT_005f10f8 = DAT_00732e52;
            if ((int)(unsigned int)DAT_00732e58 < (int)(short)DAT_006d9980) {
                DAT_005f10f4 = DAT_006d9980;
            }
            if ((int)(unsigned int)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                DAT_005f10f8 = DAT_00732e8c;
            }
            if ((int)(short)DAT_00732a34 < (int)(unsigned int)DAT_005f10e8) {
                DAT_005f10e8 = DAT_00732a34;
            }
            if (DAT_005f10f8 < DAT_005f10e8) {
                FUN_00463e10(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        DAT_00732e52 = (unsigned short)(DAT_00732e52 + bVar1 + 2);
    }
    iVar11 = (int)(short)DAT_00732e52;
    uVar12 = 0;
    iStack00000004 = 0;
    iVar3 = iVar11;
    if (iVar11 < (short)DAT_00732e8c) {
        iVar3 = (int)(short)DAT_00732e8c;
    }
    uVar6 = *(unsigned int *)(DAT_005f1d20 + (unsigned int)iVar3 * 4);
    if (DAT_0052552c == 0x13) {
LAB_00463a27:
        if (DAT_00525534 == 0xa0000) goto LAB_00463a3d;
    }
    else if (DAT_00525534 == 0xa0000) {
        uVar6 = uVar6 % DAT_0052550c;
        iVar9 = (int)((unsigned int)iVar11 + 1u + (unsigned int)bVar1);
        if ((short)DAT_00732a34 < iVar9) {
            iVar9 = (int)(short)DAT_00732a34;
        }
        if (0xffff < (int)((unsigned int)(iVar9 - iVar3) * DAT_0070acc8 + uVar6)) {
            iStack00000004 = 1;
        }
        goto LAB_00463a27;
    }
    iStack00000004 = 1;
LAB_00463a3d:
    if (iStack00000004 == 0) {
        iVar3 = (int)(uVar6 + DAT_00525534);
        puVar13 = pbVar5 + 2;
        if (iVar11 < (int)((unsigned int)bVar1 + (unsigned int)iVar11)) {
            do {
                __builtin_memcpy(&uVar6, puVar13, 4);
                uVar7 = uStack00000008;
                if (DAT_0071e524 != '\0') {
                    uVar7 = uStack00000008 + 1;
                }
                iVar9 = (int)(short)DAT_00732e58;
                iStack00000004 = (int)(short)DAT_006d9980;
                if (iVar9 < iStack00000004) {
                    if ((int)uVar7 <= iStack00000004 - iVar9) break;
                    uVar7 = (uVar7 - (unsigned int)iStack00000004) + (unsigned int)iVar9;
                    uVar6 = uVar6 << (((char)DAT_006d9980 - (char)DAT_00732e58) & 0x1f);
                    iVar9 = iStack00000004;
                }
                if ((short)DAT_00732a34 <= iVar11) break;
                if ((short)DAT_00732e8c <= iVar11) {
                    iStack00000004 = (int)((unsigned int)iVar9 + uVar7);
                    if ((DAT_0070acc0 < iStack00000004) && (uVar7 = (unsigned int)DAT_0070acc0 - (unsigned int)iVar9, (int)uVar7 < 1))
                        break;
                    puVar10 = (unsigned char *)((unsigned int)iVar9 + (unsigned int)iVar3);
                    while (uVar7 != 0) {
                        uVar7 = uVar7 - 1;
                        uVar4 = DAT_006da9fa;
                        if ((((uVar6 & 0x80000000) == 0) && (uVar4 = DAT_00732e50, DAT_0071e524 != '\0')) &&
                            ((uVar12 & 0x80000000) != 0)) {
                            uVar4 = DAT_0071b012;
                        }
                        *puVar10 = uVar4;
                        puVar10 = puVar10 + 1;
                        uVar6 = uVar6 * 2;
                        uVar12 = uVar12 * 2;
                    }
                    iVar3 = (int)((unsigned int)iVar3 + *DAT_005f1d24);
                }
                iVar11 = iVar11 + 1;
                __builtin_memcpy(&uTmp, puVar13, 4);
                uVar12 = uTmp >> 1;
                puVar13 = puVar13 + 8;
            } while (iVar11 < (int)((int)(short)DAT_00732e52 + (unsigned int)bVar1));
        }
    }
    else {
        puVar13 = pbVar5 + 2;
        if (iVar11 < (int)((unsigned int)bVar1 + (unsigned int)iVar11)) {
            do {
                __builtin_memcpy(&uVar6, puVar13, 4);
                uVar7 = uStack00000008;
                if (DAT_0071e524 != '\0') {
                    uVar7 = uStack00000008 + 1;
                }
                iVar3 = (int)(short)DAT_00732e58;
                iVar9 = (int)(short)DAT_006d9980;
                if (iVar3 < iVar9) {
                    if ((int)uVar7 <= iVar9 - iVar3) break;
                    uVar7 = (uVar7 - (unsigned int)iVar9) + (unsigned int)iVar3;
                    uVar6 = uVar6 << (((char)DAT_006d9980 - (char)DAT_00732e58) & 0x1f);
                    iVar3 = iVar9;
                }
                if ((short)DAT_00732a34 <= iVar11) break;
                if ((short)DAT_00732e8c <= iVar11) {
                    if (((int)DAT_0070acc0 < (int)((unsigned int)iVar3 + uVar7)) &&
                        (uVar7 = (unsigned int)DAT_0070acc0 - (unsigned int)iVar3, (int)uVar7 < 1)) break;
                    uVar8 = *(unsigned int *)(DAT_005f1d20 + (unsigned int)iVar11 * 4) + (unsigned int)iVar3;
                    if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
                        uVar8 = uVar8 % DAT_0052550c;
                    }
                    puVar10 = (unsigned char *)(uVar8 + DAT_00525534);
                    while (uVar7 != 0) {
                        uVar7 = uVar7 - 1;
                        uVar4 = DAT_006da9fa;
                        if ((((uVar6 & 0x80000000) == 0) && (uVar4 = DAT_00732e50, DAT_0071e524 != '\0')) &&
                            ((uVar12 & 0x80000000) != 0)) {
                            uVar4 = DAT_0071b012;
                        }
                        *puVar10 = uVar4;
                        puVar10 = puVar10 + 1;
                        uVar6 = uVar6 * 2;
                        uVar12 = uVar12 * 2;
                    }
                }
                iVar11 = iVar11 + 1;
                __builtin_memcpy(&uTmp, puVar13, 4);
                uVar12 = uTmp >> 1;
                puVar13 = puVar13 + 8;
            } while (iVar11 < (int)((int)(short)DAT_00732e52 + (unsigned int)bVar1));
        }
    }
    DAT_00732e58 = (unsigned short)(DAT_00732e58 + bVar2);
    if ((DAT_0070acc0 <= (short)DAT_00732e58) && (DAT_006ffbf0 != 0)) {
        if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
            DAT_005f10ec = DAT_0070acc0;
            DAT_005f10f4 = DAT_00732e58;
            DAT_005f10e8 = (unsigned short)(DAT_006d4a4e + DAT_00732e52);
            DAT_005f10f8 = DAT_00732e52;
            if ((int)(unsigned int)DAT_00732e58 < (int)(short)DAT_006d9980) {
                DAT_005f10f4 = DAT_006d9980;
            }
            if ((int)(unsigned int)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                DAT_005f10f8 = DAT_00732e8c;
            }
            if ((int)(short)DAT_00732a34 < (int)(unsigned int)DAT_005f10e8) {
                DAT_005f10e8 = DAT_00732a34;
            }
            if (DAT_005f10f8 < DAT_005f10e8) {
                FUN_00463e10(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        DAT_00732e52 = (unsigned short)(DAT_00732e52 + bVar1 + 2);
    }
    return;
}
