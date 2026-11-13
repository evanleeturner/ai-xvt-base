/* SPDX-License-Identifier: MIT */
/* z_xvt 00485c90 */
long long __ftol(float);
extern uint DAT_006d4984;
extern float _DAT_005201dc;

uint __cdecl FUN_00485c90(uint *param_1, float *param_2, float *param_3)
{
  float fVar1;
  float fVar2;
  float fVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  float *pfVar7;
  long long lVar8;
  
  if ((uint)param_2[1] < 0x80000001) {
    lVar8 = __ftol(param_2[1]);
    uVar6 = (uint)lVar8;
    if ((float)(int)uVar6 != param_2[1]) {
      uVar6 = uVar6 + 1;
    }
  }
  else {
    uVar6 = 0;
  }
  if ((uint)param_3[1] < 0x80000001) {
    lVar8 = __ftol(param_3[1]);
    uVar4 = (uint)lVar8;
    if ((float)(int)uVar4 != param_3[1]) {
      uVar4 = uVar4 + 1;
    }
  }
  else {
    uVar4 = 0;
  }
  if (uVar4 == uVar6) {
    return 0xffffffff;
  }
  uVar5 = uVar4;
  pfVar7 = param_2;
  if ((int)uVar4 < (int)uVar6) {
    uVar5 = uVar6;
    uVar6 = uVar4;
    pfVar7 = param_3;
    param_3 = param_2;
  }
  if ((int)uVar5 < 1) {
    return 0xffffffff;
  }
  uVar4 = (uint)DAT_006d4984;
  if ((int)uVar4 <= (int)uVar6) {
    return 0xffffffff;
  }
  if ((int)uVar4 < (int)uVar5) {
    uVar5 = uVar4;
  }
  *param_1 = uVar5;
  fVar3 = _DAT_005201dc / (param_3[1] - pfVar7[1]);
  param_1[4] = (uint)((*param_3 - *pfVar7) * fVar3);
  fVar1 = param_3[3];
  fVar2 = pfVar7[3];
  param_1[6] = 0;
  param_1[5] = (uint)((fVar1 - fVar2) * fVar3);
  fVar1 = pfVar7[1];
  param_1[2] = (uint)((float)param_1[4] * ((float)(int)uVar6 - fVar1) + *pfVar7);
  fVar2 = pfVar7[3];
  param_1[1] = uVar6;
  param_1[3] = (uint)(((float)(int)uVar6 - fVar1) * (float)param_1[5] + fVar2);
  return uVar6;
}
