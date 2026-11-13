/* SPDX-License-Identifier: MIT */
/* z_xvt 00462920 */
uint __cdecl FUN_00462920(byte *param_1, int param_2, uint param_3, uint param_4)
{
    int iVar1;
    byte *pbVar2;
    uint uVar3;
    int local_10;

    local_10 = 0x7fffffff;
    if (param_3 < param_4) {
        pbVar2 = (byte *)(param_2 + param_3 * 3);
        uVar3 = param_3;
        do {
            iVar1 = ((uint)param_1[1] - (uint)pbVar2[1]) * ((uint)param_1[1] - (uint)pbVar2[1]) +
                    ((uint)param_1[2] - (uint)pbVar2[2]) * ((uint)param_1[2] - (uint)pbVar2[2]) +
                    ((uint)*param_1 - (uint)*pbVar2) * ((uint)*param_1 - (uint)*pbVar2);
            if (iVar1 < local_10) {
                param_3 = uVar3;
                local_10 = iVar1;
            }
            pbVar2 = pbVar2 + 3;
            uVar3 = uVar3 + 1;
        } while (uVar3 < param_4);
    }
    return param_3;
}
