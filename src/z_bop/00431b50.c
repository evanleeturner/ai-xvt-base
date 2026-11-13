/* SPDX-License-Identifier: MIT */
/* z_bop 00431b50 */
int FUN_00431c10(ushort, ushort, ushort, ushort, short, ushort);

uint __cdecl FUN_00431b50(byte *param_1, short param_2)
{
    ushort uVar2;
    uint uVar3;
    uint uVar4;

    uVar2 = 10;
    if (param_1[4] == 0x27 && param_1[5] == 0xc) {
        uVar2 = (ushort)param_1[6];
    }
    uVar3 = FUN_00431c10((ushort)*param_1, (ushort)param_1[1], (ushort)param_1[2], (ushort)param_1[3],
                         param_2, uVar2) & 0xffff;
    if (param_1[4] == 0x27) {
        uVar4 = 0;
    }
    else {
        uVar4 = FUN_00431c10((ushort)param_1[4], (ushort)param_1[5], (ushort)param_1[6], (ushort)param_1[7],
                             param_2, 10) & 0xffff;
    }
    if (param_1[10] != 1 && param_1[4] != 0x27) {
        return uVar4 & uVar3;
    }
    return uVar4 | uVar3;
}
