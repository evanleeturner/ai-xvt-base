/* SPDX-License-Identifier: MIT */
/* z_xvt 00415080 */
float __cdecl FUN_00415080(float *param_1, int param_2)
{
  return *(float *)(param_2 + 8) * param_1[0] +
         param_1[2] * *(float *)(param_2 + 0x20) +
         param_1[1] * *(float *)(param_2 + 0x14);
}
