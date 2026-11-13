/* SPDX-License-Identifier: MIT */
/* z_bop 00421080 */
float __cdecl FUN_00421080(float *param_1, void *param_2)
{
    return *(float *)((char *)param_2 + 8) * *param_1 +
           param_1[2] * *(float *)((char *)param_2 + 0x20) +
           *(float *)((char *)param_2 + 0x14) * param_1[1];
}
