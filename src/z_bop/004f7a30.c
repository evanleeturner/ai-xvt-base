/* SPDX-License-Identifier: MIT */
/* z_bop 004f7a30 */
extern short DAT_00aa5aac;
extern short DAT_00a921c6[];
extern short DAT_00a92142[];
extern short DAT_00a9216e[];
extern short _DAT_006691e8;

void __cdecl FUN_004f7b10(int *param_1, short param_2, short param_3, short *param_4, short *param_5);

unsigned int __cdecl FUN_004f7a30(int *param_1, short param_2, short param_3)
{
    short sVar1;
    unsigned short uVar4;
    int iVar2;
    int iVar3;
    unsigned int uVar5;
    short sVar6;
    short local_6;
    short local_4;
    short local_2;

    uVar4 = 0;
    local_2 = 0;
    sVar1 = 999;
    sVar6 = 0;
    if (0 < DAT_00aa5aac) {
        do {
            iVar2 = (int)sVar6;
            iVar3 = iVar2 * 0x562;
            if (DAT_00a921c6[iVar2 * 0x2b1] != 0) {
                FUN_004f7b10(param_1, DAT_00a92142[iVar2 * 0x2b1], DAT_00a9216e[iVar2 * 0x2b1], &local_6, &local_4);
                uVar5 = ((int)param_2 - (int)local_6) >> 0x1f;
                iVar2 = ((int)param_2 - (int)local_6 ^ uVar5) - uVar5;
                iVar3 = iVar2;
                if (((iVar2 < sVar1) &&
                    (uVar5 = ((int)param_3 - (int)local_4) >> 0x1f,
                    iVar3 = ((int)param_3 - (int)local_4 ^ uVar5) - uVar5, iVar3 < sVar1)) &&
                   (sVar1 = (short)iVar3, local_2 = sVar6, iVar3 <= iVar2)) {
                    sVar1 = (short)iVar2;
                }
            }
            uVar4 = (unsigned short)((unsigned int)iVar3 >> 0x10);
            sVar6 = sVar6 + 1;
        } while (sVar6 < DAT_00aa5aac);
    }
    if (sVar1 == 999) {
        return (unsigned int)uVar4 << 0x10;
    }
    _DAT_006691e8 = local_2;
    return ((unsigned int)uVar4 << 0x10) | 1;
}
