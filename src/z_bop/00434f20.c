/* SPDX-License-Identifier: MIT */
/* z_bop 00434f20 */
extern unsigned int FUN_00435a40(unsigned int param_1);
extern int FUN_00435640(int param_1, unsigned int param_2, int param_3, unsigned short param_4, int param_5, int param_6);

extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_009fd458[];
extern unsigned char DAT_009d8c88[];
extern short DAT_009d69e0[];
extern short DAT_009d69cc[];
extern short DAT_009d69b8[];
extern unsigned char DAT_009d6954;
extern short DAT_009d6990[];
extern unsigned int unaff_EBX;

void __cdecl FUN_00434f20(unsigned int param_1, int param_2, int param_3, int param_4)
{
    unsigned char bVar1;
    int iVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned short uVar5;

    uVar4 = param_1 & 0xffff;
    uVar5 = 1;
    bVar1 = DAT_009a1fe8[uVar4 * 0x23 + 0x16];

    if (DAT_009fd458[param_4 * 0x1e5 + DAT_009d8c88[bVar1 * 0x566]] == '\0') {
        iVar2 = FUN_00435a40(uVar4);
        if (param_3 == 1) {
            DAT_009d69e0[param_4] = DAT_009d69e0[param_4] + 1;
            uVar5 = 10;
            iVar2 = iVar2 / 10;
        }
        else if (param_3 == 2) {
            DAT_009d69cc[param_4] = DAT_009d69cc[param_4] + 1;
            uVar5 = 6;
            iVar2 = iVar2 / 2;
        }
        else if (param_3 == 3) {
            DAT_009d69b8[param_4] = DAT_009d69b8[param_4] + 1;
            uVar5 = 1;
        }
        if (DAT_009d6954 == '\x02') {
            iVar2 = iVar2 * 2;
        }
        else if (DAT_009d6954 == '\0') {
            iVar2 = iVar2 / 2;
        }
        iVar3 = FUN_00435640(2, (((unsigned int)unaff_EBX >> 8) << 8 | bVar1) & 0xffff00ff, -1, uVar5, param_2, param_4);
        if (-1 < iVar3) {
            DAT_009d6990[param_4] = DAT_009d6990[param_4] + iVar2;
            return;
        }
    }
    else if ((param_3 == 2) || (param_3 == 3)) {
        iVar2 = FUN_00435a40(uVar4);
        DAT_009d6990[param_4] = DAT_009d6990[param_4] - iVar2;
    }
    return;
}
