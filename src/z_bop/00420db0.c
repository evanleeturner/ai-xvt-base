/* SPDX-License-Identifier: MIT */
/* z_bop 00420db0 */
void __cdecl FUN_00420fe0(float *param_1, float *param_2);

void __cdecl FUN_00420db0(int param_1, float *param_2, int param_3)
{
    *(float *)(param_1 + 0x10) = *param_2;
    *(float *)(param_1 + 0x14) = param_2[1];
    *(float *)(param_1 + 0x18) = param_2[2];
    FUN_00420fe0((float *)(param_1 + 0x10), (float *)(param_3 + 0xc));
    *(float *)(param_1 + 0x1c) = param_2[3];
    *(float *)(param_1 + 0x20) = param_2[4];
    *(float *)(param_1 + 0x24) = param_2[5];
    FUN_00420fe0((float *)(param_1 + 0x1c), (float *)(param_3 + 0xc));
    return;
}
