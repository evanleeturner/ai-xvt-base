/* SPDX-License-Identifier: MIT */
/* z_xvt 0045f410 */
extern int DAT_00732f3c;
extern float _DAT_00520194;
extern float _DAT_005201b4;
extern int DAT_005f10d4;
extern int DAT_005f10b0;
extern float _DAT_005f10e4;
extern float _DAT_005f10b4;
extern int DAT_005279f4;

long __cdecl __ftol(float);

void __cdecl FUN_0045f410(int param_1, int param_2)
{
  float local_20;
  float fVar1;
  float fVar4;
  int iVar5;
  long lVar6;
  int uVar2;
  int uVar3;

  iVar5 = param_2 + param_1 * 0x18;
  local_20 = *(float *)(iVar5 + 8);
  fVar1 = *(float *)(iVar5 + 4);
  uVar2 = *(int *)(iVar5 + 0x10);
  uVar3 = *(int *)(iVar5 + 0x14);
  if (0x80000000 < (unsigned int)local_20) {
    local_20 = (float)DAT_00732f3c;
  }
  fVar4 = (_DAT_00520194 / local_20) * _DAT_005201b4;
  *(float *)(DAT_005f10d4 * 0x20 + DAT_005f10b0) = *(float *)(param_2 + param_1 * 0x18) + _DAT_005f10e4;
  *(float *)(DAT_005f10d4 * 0x20 + 4 + DAT_005f10b0) = fVar1 + _DAT_005f10b4;
  *(float *)(DAT_005f10d4 * 0x20 + 8 + DAT_005f10b0) = fVar4;
  *(float *)(DAT_005f10d4 * 0x20 + 0xc + DAT_005f10b0) = local_20;
  *(int *)(DAT_005f10d4 * 0x20 + 0x18 + DAT_005f10b0) = uVar2;
  *(int *)(DAT_005f10d4 * 0x20 + 0x1c + DAT_005f10b0) = uVar3;
  lVar6 = __ftol(local_20);
  iVar5 = (int)lVar6 + 0x30;
  if (0xff < iVar5) {
    iVar5 = 0xff;
  }
  if (DAT_005279f4 == 0) {
    *(int *)(DAT_005f10d4 * 0x20 + 0x10 + DAT_005f10b0) = iVar5 * 0x10101 + -0x1000000;
  }
  else {
    *(int *)(DAT_005f10d4 * 0x20 + 0x10 + DAT_005f10b0) = iVar5 * 0x10101 + -0x2000000;
  }
  *(int *)(DAT_005f10d4 * 0x20 + 0x14 + DAT_005f10b0) = 0;
  DAT_005f10d4 = DAT_005f10d4 + 1;
  return;
}
