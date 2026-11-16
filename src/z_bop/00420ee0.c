/* SPDX-License-Identifier: MIT */
/* z_bop 00420ee0 */
extern int DAT_00a081ec;
extern float _DAT_005180d4;
extern short DAT_009d682c;
extern short DAT_009a1fe4;
extern int DAT_009d7680;

void __cdecl FUN_00420fe0(float *param_1, float *param_2);

void __cdecl FUN_00420ee0(float *param_1, float *param_2, float *param_3)
{
  float fVar1;
  float fVar2;
  float local_10;
  float fStack_c;
  float local_8;
  float local_4;
  
  local_10 = *param_2;
  fStack_c = param_2[1];
  local_8 = param_2[2];
  local_4 = ((float)DAT_00a081ec / param_3[2]) * _DAT_005180d4;
  FUN_00420fe0(&local_10, param_3 + 3);
  fVar1 = *param_3;
  fVar2 = param_3[1];
  local_4 = local_4 / (param_3[2] + local_8 + _DAT_005180d4);
  param_1[2] = local_4;
  *param_1 = local_4 * (fVar1 + local_10);
  param_1[1] = param_1[2] * (fVar2 + fStack_c);
  *param_1 = (float)(DAT_009d682c >> 1) + *param_1;
  param_1[1] = (float)(int)((unsigned int)(DAT_009a1fe4 >> 1) + DAT_009d7680) + param_1[1];
  return;
}
