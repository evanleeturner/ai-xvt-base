/* SPDX-License-Identifier: MIT */
/* z_bop 0040e940 */
unsigned int __cdecl FUN_0040e940(unsigned char *param_1, int param_2, unsigned int param_3, unsigned int param_4)
{
    int local_10 = 0x7fffffff;
    unsigned int uVar3;
    unsigned char *pbVar2;
    int iVar1;

    if (param_3 < param_4) {
        pbVar2 = (unsigned char *)(param_2 + param_3 * 3);
        uVar3 = param_3;
        do {
            iVar1 = ((unsigned int)param_1[1] - (unsigned int)pbVar2[1]) * ((unsigned int)param_1[1] - (unsigned int)pbVar2[1]) +
                    ((unsigned int)*param_1 - (unsigned int)*pbVar2) * ((unsigned int)*param_1 - (unsigned int)*pbVar2) +
                    ((unsigned int)param_1[2] - (unsigned int)pbVar2[2]) * ((unsigned int)param_1[2] - (unsigned int)pbVar2[2]);
            if (iVar1 < local_10) {
                param_3 = uVar3;
                local_10 = iVar1;
            }
            pbVar2 += 3;
            uVar3++;
        } while (uVar3 < param_4);
    }
    return param_3;
}
