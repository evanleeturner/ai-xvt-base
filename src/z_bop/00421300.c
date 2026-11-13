/* SPDX-License-Identifier: MIT */
/* z_bop 00421300 */
extern float _DAT_005180c4;
extern float sinf(float);
extern float cosf(float);

void __cdecl FUN_00421300(float *param_1, float *param_2)
{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  
  fVar5 = *param_2;
  fVar6 = param_2[1];
  fVar7 = param_2[2];
  fVar8 = sinf(param_2[3]);
  fVar9 = cosf(param_2[3]);
  fVar10 = _DAT_005180c4 - fVar9;
  fVar1 = fVar10 * fVar6 * fVar5;
  fVar2 = fVar10 * fVar7;
  fVar3 = fVar10 * fVar7 * fVar5;
  fVar4 = fVar2 * fVar6;
  *param_1 = fVar10 * fVar5 * fVar5 + fVar9;
  param_1[1] = fVar1 + fVar8 * fVar7;
  param_1[3] = fVar1 - fVar8 * fVar7;
  param_1[2] = fVar3 - fVar8 * fVar6;
  param_1[4] = fVar6 * (fVar10 * fVar6) + fVar9;
  param_1[5] = fVar4 + fVar8 * fVar5;
  param_1[6] = fVar3 + fVar8 * fVar6;
  param_1[8] = fVar7 * fVar2 + fVar9;
  param_1[7] = fVar4 - fVar8 * fVar5;
  return;
}
