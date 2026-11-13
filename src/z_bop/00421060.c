/* SPDX-License-Identifier: MIT */
/* z_bop 00421060 */
float10 __cdecl FUN_00421060(float *param_1, int param_2)
{
  return (*(float *)(param_2 + 0x1c) * param_1[2] + *(float *)(param_2 + 0x10) * param_1[1]) + *(float *)(param_2 + 4) * param_1[0];
}
