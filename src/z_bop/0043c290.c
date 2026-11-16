/* SPDX-License-Identifier: MIT */
/* z_bop 0043c290 */
extern unsigned char DAT_009a1ed0;
extern unsigned char *DAT_009a1fe8;
extern char *DAT_00a08380[];
extern char *DAT_00a08c70;
extern char DAT_009d8c87[];
extern char DAT_009d8c30[];
extern char *DAT_00a60698[];
extern char *DAT_00a605e4[];
extern char DAT_0051c560[];

void __cdecl FUN_004a9860(char param_1);
void __cdecl FUN_004a9820(char *param_1);
void __cdecl FUN_004a97f0(char *param_1);
unsigned int __cdecl FUN_0043c740(unsigned int param_1, unsigned int *param_2);

void __cdecl FUN_0043c290(unsigned short param_1, unsigned char param_2)
{
    unsigned char bVar1;
    char cVar2;
    char *pcVar3;
    unsigned int *puVar4;
    unsigned int iVar5;
    unsigned char *iVar6;
    unsigned int uVar7;
    unsigned short uVar8;
    unsigned int uVar9;
    unsigned char uVar10;
    unsigned int tmp;

    DAT_009a1ed0 = 0;
    if (0x7fff < param_1) {
        if ((param_2 & 1) == 0) {
            DAT_009a1ed0 = 0;
            return;
        }
        FUN_004a9860((char)0xfe);
        FUN_004a9860(0x43);
        FUN_004a9820(DAT_00a08380[(unsigned short)(param_1 + 0x8000)]);
        return;
    }
    iVar5 = (unsigned int)param_1 * 0x23;
    iVar6 = DAT_009a1fe8 + iVar5;
    __builtin_memcpy(&tmp, iVar6 + 0x1f, 4);
    if (tmp == 0) {
        uVar8 = (unsigned short)*(unsigned char *)(iVar6 + 3);
        if (uVar8 == 0) {
            FUN_004a97f0(DAT_00a08c70);
            return;
        }
        FUN_004a9860((char)0xfe);
        cVar2 = DAT_009d8c87[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar5) * 0x566];
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
        FUN_004a9860((char)uVar10);
        if ((((param_2 & 1) != 0) && (0x45 < uVar8)) && (uVar8 < 0x56)) {
            FUN_004a9820(DAT_00a60698[(unsigned int)uVar8]);
        }
        if ((param_2 & 4) == 0) {
            if ((param_2 & 3) != 3) goto LAB_0043c69f;
            FUN_004a9860(0x3a);
            uVar10 = 0x20;
        } else {
            uVar10 = 0x3a;
        }
        FUN_004a9860((char)uVar10);
LAB_0043c69f:
        if ((param_2 & 2) == 0) {
            return;
        }
        FUN_004a9860((char)0xfe);
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
        FUN_004a9860((char)uVar10);
        FUN_004a9820(DAT_009d8c30 + (unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar5) * 0x566);
        return;
    }
    bVar1 = *(unsigned char *)(iVar6 + 3);
    if (bVar1 == 0) {
        FUN_004a97f0(DAT_00a08c70);
        return;
    }
    FUN_004a9860((char)0xfe);
    __builtin_memcpy(&pcVar3, DAT_009a1fe8 + 0x1f + iVar5, 4);
    cVar2 = *(char *)(pcVar3 + 0x88);
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
    FUN_004a9860((char)uVar10);
    __builtin_memcpy(&pcVar3, DAT_009a1fe8 + 0x1f + iVar5, 4);
    if (*pcVar3 != '\0') {
        if ((param_2 & 1) == 0) {
            return;
        }
        if ((0x8e < bVar1) && (bVar1 < 0x9c)) {
            FUN_004a9820(DAT_00a605e4[(unsigned int)bVar1]);
            return;
        }
        if (bVar1 < 0x46) {
            return;
        }
        if (0x54 < bVar1) {
            return;
        }
        FUN_004a9820(DAT_00a60698[(unsigned int)bVar1]);
        return;
    }
    __builtin_memcpy(&puVar4, pcVar3 + 0xa9, 4);
    if ((param_2 & 1) != 0) {
        FUN_004a9820(DAT_0051c560 + (unsigned int)*(unsigned char *)(puVar4 + 1) * 0x101);
    }
    if ((param_2 & 4) == 0) {
        if ((param_2 & 3) != 3) goto LAB_0043c3d3;
        FUN_004a9860(0x3a);
        uVar10 = 0x20;
    } else {
        uVar10 = 0x3a;
    }
    FUN_004a9860((char)uVar10);
LAB_0043c3d3:
    if ((param_2 & 2) != 0) {
        FUN_004a9860((char)0xfe);
        __builtin_memcpy(&pcVar3, DAT_009a1fe8 + 0x1f + iVar5, 4);
        cVar2 = *(char *)(pcVar3 + 0x88);
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
        FUN_004a9860((char)uVar10);
        uVar9 = (unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar5);
        FUN_004a9820(DAT_009d8c30 + uVar9 * 0x566);
        uVar7 = FUN_0043c740(uVar9, puVar4);
        uVar8 = (unsigned short)uVar7;
        if (uVar8 != 0) {
            FUN_004a9860(0x20);
            if (999 < uVar8) {
                uVar8 = 999;
            }
            if (uVar8 < 100) {
                if (uVar8 < 10) {
                    FUN_004a9860((char)uVar8 + '0');
                    return;
                }
                FUN_004a9860((char)(uVar8 / 10) + '0');
                FUN_004a9860((char)((unsigned int)uVar8 % 10) + '0');
                return;
            }
            FUN_004a9860((char)(uVar8 / 100) + '0');
            FUN_004a9860((char)(((unsigned int)uVar8 % 100) / 10) + '0');
            FUN_004a9860((char)(((unsigned int)uVar8 % 100) % 10) + '0');
            return;
        }
    }
    return;
}
