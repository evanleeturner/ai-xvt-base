/* SPDX-License-Identifier: MIT */
/* z_xvt 004ac210 */
extern unsigned char *DAT_006d4988;
extern unsigned char DAT_006d4870;
extern char *DAT_00655a80;
extern char *DAT_00655190[];
extern char *DAT_0073b268[];
extern char *DAT_0073b1b4[];
extern char DAT_0052b070[];
extern char DAT_0070acf0[];
extern char DAT_0070ad43[];
extern char DAT_0070ad47[];
extern char DAT_0070ad51[];
extern char DAT_0070b210[];
extern char DAT_0070b216[];

void __cdecl FUN_0048f040(unsigned char param_1);
void __cdecl FUN_0048f000(char *param_1);
void __cdecl FUN_0048efd0(char *param_1);

void __cdecl FUN_004ac210(unsigned short param_1, unsigned char param_2)
{
    unsigned char bVar1;
    char cVar2;
    char *pcVar3;
    unsigned int *puVar4;
    unsigned int uVar5;
    unsigned int iVar6;
    unsigned short uVar7;
    unsigned char *iVar8;
    unsigned int uVar9;
    unsigned char uVar10;
    int iVar11;

    DAT_006d4870 = 0;
    if (0x7fff < param_1) {
        if ((param_2 & 1) == 0) {
            DAT_006d4870 = 0;
            return;
        }
        FUN_0048f040(0xfe);
        FUN_0048f040(0x43);
        FUN_0048f000(DAT_00655190[(unsigned short)(param_1 + 0x8000)]);
        return;
    }
    iVar6 = (unsigned int)param_1 * 0x23;
    iVar8 = DAT_006d4988 + iVar6;
    __builtin_memcpy(&iVar11, iVar8 + 0x1f, 4);
    if (iVar11 == 0) {
        uVar7 = (unsigned short)*(unsigned char *)(iVar8 + 3);
        if (uVar7 == 0) {
            FUN_0048efd0(DAT_00655a80);
            return;
        }
        FUN_0048f040(0xfe);
        cVar2 = DAT_0070ad47[(unsigned int)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar6) * 0x566];
        if (cVar2 == '\0') {
            uVar10 = 0x51;
        } else if ((cVar2 == '\x01') || (cVar2 == '\x04')) {
            uVar10 = 0x49;
        } else if (cVar2 == '\x02') {
            uVar10 = 0x45;
        } else if (cVar2 == '\x05') {
            uVar10 = 0x56;
        } else {
            uVar10 = 0x4d;
        }
        FUN_0048f040(uVar10);
        if ((((param_2 & 1) != 0) && (0x45 < uVar7)) && (uVar7 < 0x56)) {
            FUN_0048f000(DAT_0073b268[(unsigned int)uVar7]);
        }
        if ((param_2 & 4) == 0) {
            if ((param_2 & 3) != 3) goto LAB_004ac65b;
            FUN_0048f040(0x3a);
            uVar10 = 0x20;
        } else {
            uVar10 = 0x3a;
        }
        FUN_0048f040(uVar10);
LAB_004ac65b:
        if ((param_2 & 2) == 0) {
            return;
        }
        FUN_0048f040(0xfe);
        if (cVar2 == '\0') {
            uVar10 = 0x52;
        } else if ((cVar2 == '\x01') || (cVar2 == '\x04')) {
            uVar10 = 0x4a;
        } else if (cVar2 == '\x02') {
            uVar10 = 0x46;
        } else if (cVar2 == '\x05') {
            uVar10 = 0x56;
        } else {
            uVar10 = 0x4e;
        }
        FUN_0048f040(uVar10);
        FUN_0048f000(DAT_0070acf0 + (unsigned int)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar6) * 0x566);
        return;
    }
    bVar1 = *(unsigned char *)(iVar8 + 3);
    if (bVar1 == 0) {
        FUN_0048efd0(DAT_00655a80);
        return;
    }
    FUN_0048f040(0xfe);
    {
        unsigned char *pTmp;
        __builtin_memcpy(&pTmp, DAT_006d4988 + 0x1f + iVar6, 4);
        cVar2 = *(char *)(pTmp + 0x86);
    }
    if (cVar2 == '\0') {
        uVar10 = 0x51;
    } else if ((cVar2 == '\x01') || (cVar2 == '\x04')) {
        uVar10 = 0x49;
    } else if (cVar2 == '\x02') {
        uVar10 = 0x45;
    } else if (cVar2 == '\x05') {
        uVar10 = 0x55;
    } else {
        uVar10 = 0x4d;
    }
    FUN_0048f040(uVar10);
    __builtin_memcpy(&pcVar3, DAT_006d4988 + 0x1f + iVar6, 4);
    if (*pcVar3 != '\0') {
        if ((param_2 & 1) == 0) {
            return;
        }
        if ((0x8e < bVar1) && (bVar1 < 0x9c)) {
            FUN_0048f000(DAT_0073b1b4[(unsigned int)bVar1]);
            return;
        }
        if (bVar1 < 0x46) {
            return;
        }
        if (0x54 < bVar1) {
            return;
        }
        FUN_0048f000(DAT_0073b268[(unsigned int)bVar1]);
        return;
    }
    __builtin_memcpy(&puVar4, pcVar3 + 0xa7, 4);
    if ((param_2 & 1) != 0) {
        FUN_0048f000(DAT_0052b070 + (unsigned int)(unsigned char)puVar4[1] * 0xef);
    }
    if ((param_2 & 4) == 0) {
        if ((param_2 & 3) != 3) goto LAB_004ac359;
        FUN_0048f040(0x3a);
        uVar10 = 0x20;
    } else {
        uVar10 = 0x3a;
    }
    FUN_0048f040(uVar10);
LAB_004ac359:
    if ((param_2 & 2) != 0) {
        FUN_0048f040(0xfe);
        {
            unsigned char *pTmp2;
            __builtin_memcpy(&pTmp2, DAT_006d4988 + 0x1f + iVar6, 4);
            cVar2 = *(char *)(pTmp2 + 0x86);
        }
        if (cVar2 == '\0') {
            uVar10 = 0x52;
        } else if ((cVar2 == '\x01') || (cVar2 == '\x04')) {
            uVar10 = 0x4a;
        } else if (cVar2 == '\x02') {
            uVar10 = 0x46;
        } else if (cVar2 == '\x05') {
            uVar10 = 0x56;
        } else {
            uVar10 = 0x4e;
        }
        FUN_0048f040(uVar10);
        uVar9 = (unsigned int)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar6);
        FUN_0048f000(DAT_0070acf0 + uVar9 * 0x566);
        iVar6 = uVar9 * 0x566;
        if ((DAT_0070b210[iVar6] == '\x01') ||
            (((DAT_0070b216[iVar6] == '\0') && (DAT_0070ad43[iVar6] == '\x01')) &&
             (DAT_0070ad51[iVar6] == '\0'))) {
            uVar9 = 0;
        } else {
            uVar9 = *puVar4;
        }
        if ((unsigned short)uVar9 != 0) {
            FUN_0048f040(0x20);
            if (999 < (unsigned short)uVar9) {
                uVar9 = 999;
            }
            if ((unsigned short)uVar9 < 100) {
                if ((unsigned short)uVar9 < 10) {
                    FUN_0048f040((char)uVar9 + '0');
                    return;
                }
                FUN_0048f040((char)((uVar9 & 0xffff) / 10) + '0');
                FUN_0048f040((char)((uVar9 & 0xffff) % 10) + '0');
                return;
            }
            uVar5 = (uVar9 & 0xffff) % 100;
            FUN_0048f040((char)((uVar9 & 0xffff) / 100) + '0');
            FUN_0048f040((char)(uVar5 / 10) + '0');
            FUN_0048f040((char)(uVar5 % 10) + '0');
            return;
        }
    }
    return;
}
