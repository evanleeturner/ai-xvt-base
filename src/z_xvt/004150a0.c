/* SPDX-License-Identifier: MIT */
/* z_xvt 004150a0 */
void __cdecl FUN_004150a0(float *param_1, float *param_2)
{
  float v0 = param_1[0];
  float v1 = param_1[1];
  float v2 = param_1[2];
  float v3 = param_1[3];
  float v4 = param_1[4];
  float v5 = param_1[5];
  float v6 = param_1[6];
  float v7 = param_1[7];
  float v8 = param_1[8];

  param_1[1] = param_2[1] * v0 + v1 * param_2[4] + v2 * param_2[7];
  param_1[2] = param_2[2] * v0 + v2 * param_2[8] + param_2[5] * v1;
  param_1[3] = param_2[3] * v4 + v5 * param_2[6] + param_2[0] * v3;
  param_1[0] = param_2[6] * v2 + param_2[0] * v0 + v1 * param_2[3];
  param_1[5] = param_2[2] * v3 + param_2[5] * v4 + v5 * param_2[8];
  param_1[4] = param_2[4] * v4 + param_2[1] * v3 + v5 * param_2[7];
  param_1[6] = v7 * param_2[3] + param_2[6] * v8 + param_2[0] * v6;
  param_1[7] = v7 * param_2[4] + param_2[7] * v8 + param_2[1] * v6;
  param_1[8] = param_2[8] * v8 + v6 * param_2[2] + param_2[5] * v7;
}
