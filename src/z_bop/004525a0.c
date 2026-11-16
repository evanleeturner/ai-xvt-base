/* SPDX-License-Identifier: MIT */
/* z_bop 004525a0 */
typedef unsigned char  byte;
typedef unsigned short ushort;
typedef unsigned int   uint;

extern unsigned char *DAT_009a1fe8;
extern char           DAT_009d8c30[];
extern char          *DAT_00a60698[];
extern char          *DAT_00a605e4[];
extern char           DAT_0051c560[];
extern unsigned char  DAT_0051c56a[];

void FUN_00452830(char *param_1, char *param_2);
void FUN_00452860(char param_1, char *param_2);
uint FUN_0043c740(uint param_1, unsigned int *param_2);

void __cdecl FUN_004525a0(uint param_1, short param_2, char *param_3)
{
    byte bVar1;
    unsigned int *puVar2;
    uint iVar3;
    int bVar4;
    char *pcVar5;
    uint uVar6;
    unsigned char *iVar7;

    bVar4 = 0;
    *param_3 = '\0';
    iVar3 = (param_1 & 0xffff) * 0x23;
    iVar7 = DAT_009a1fe8 + iVar3;
    __builtin_memcpy(&pcVar5, iVar7 + 0x1f, 4);
    if (pcVar5 == (char *)0) {
        if ((param_2 != 1) && (param_2 != 0)) {
            FUN_00452830(DAT_009d8c30 + (uint)*(byte *)(iVar7 + 0x16) * 0x566, param_3);
            return;
        }
        FUN_00452830(DAT_00a60698[*(byte *)(iVar7 + 3)], param_3);
        if (*(char *)(DAT_009d8c30 + (uint)*(byte *)(DAT_009a1fe8 + 0x16 + iVar3) * 0x566) == '\0') {
            return;
        }
        FUN_00452860(' ', param_3);
        FUN_00452830(DAT_009d8c30 + (uint)*(byte *)(DAT_009a1fe8 + 0x16 + iVar3) * 0x566, param_3);
        return;
    }
    bVar1 = *(byte *)(iVar7 + 3);
    if (*pcVar5 != '\0') {
        if ((0x8e < bVar1) && (bVar1 < 0x9c)) {
            FUN_00452830(DAT_00a605e4[bVar1], param_3);
            return;
        }
        if (bVar1 < 0x46) {
            return;
        }
        if (0x54 < bVar1) {
            return;
        }
        FUN_00452830(DAT_00a60698[bVar1], param_3);
        return;
    }
    __builtin_memcpy(&puVar2, pcVar5 + 0xa9, 4);
    bVar1 = *(byte *)(iVar7 + 0x16);
    if (param_2 == 1) {
        __builtin_memcpy(&pcVar5, DAT_0051c56a + (uint)*(byte *)(puVar2 + 1) * 0x101, 4);
        bVar4 = 1;
        FUN_00452830(pcVar5, param_3);
    } else if (param_2 == 0) {
        pcVar5 = DAT_0051c560 + (uint)*(byte *)(puVar2 + 1) * 0x101;
        bVar4 = 1;
        FUN_00452830(pcVar5, param_3);
    }
    pcVar5 = DAT_009d8c30 + (uint)bVar1 * 0x566;
    if (*pcVar5 != '\0') {
        if (bVar4) {
            FUN_00452860(' ', param_3);
        }
        bVar4 = 1;
        FUN_00452830(pcVar5, param_3);
    }
    uVar6 = FUN_0043c740((uint)bVar1, puVar2);
    if ((ushort)uVar6 == 0) {
        return;
    }
    if (bVar4) {
        FUN_00452860(' ', param_3);
    }
    if ((ushort)uVar6 < 10) {
        FUN_00452860((char)uVar6 + '0', param_3);
        return;
    }
    FUN_00452860((char)((uVar6 & 0xffff) / 10) + '0', param_3);
    FUN_00452860((char)((uVar6 & 0xffff) % 10) + '0', param_3);
    return;
}
