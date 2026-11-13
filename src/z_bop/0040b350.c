/* SPDX-License-Identifier: MIT */
/* z_bop 0040b350 */
long __cdecl __ftol(float f);

extern float DAT_00a081ec;
extern float _DAT_00518060;
extern float _DAT_00518080;
extern int DAT_00a90b20;
extern int DAT_0054f994;
extern int DAT_0053f970;
extern float _DAT_0054f9a4;
extern float _DAT_0053f974;
extern int DAT_0051a54c;

void __cdecl FUN_0040b350(int param_1, int param_2)
{
  float fVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  float fVar4;
  int iVar5;
  long lVar6;
  float local_18;

  iVar5 = param_2 + param_1 * 0x18;
  local_18 = *(float *)(iVar5 + 8);
  fVar1 = *(float *)(iVar5 + 4);
  uVar2 = *(unsigned int *)(iVar5 + 0x10);
  uVar3 = *(unsigned int *)(iVar5 + 0x14);
  if (0x80000000 < (unsigned int)local_18) {
    local_18 = (float)DAT_00a081ec;
  }
  fVar4 = _DAT_00518060 / (((float)DAT_00a081ec / local_18) * _DAT_00518080 + _DAT_00518060);
  if (DAT_00a90b20 != 2) {
    fVar4 = _DAT_00518060 - fVar4;
  }
  *(float *)(DAT_0054f994 * 0x20 + DAT_0053f970) = *(float *)(param_2 + param_1 * 0x18) + _DAT_0054f9a4;
  *(float *)(DAT_0054f994 * 0x20 + 4 + DAT_0053f970) = fVar1 + _DAT_0053f974;
  *(float *)(DAT_0054f994 * 0x20 + 8 + DAT_0053f970) = fVar4;
  *(float *)(DAT_0054f994 * 0x20 + 0xc + DAT_0053f970) = local_18;
  *(unsigned int *)(DAT_0054f994 * 0x20 + 0x18 + DAT_0053f970) = uVar2;
  *(unsigned int *)(DAT_0054f994 * 0x20 + 0x1c + DAT_0053f970) = uVar3;
  lVar6 = __ftol(local_18);
  iVar5 = (int)lVar6 + 0x30;
  if (0xff < iVar5) {
    iVar5 = 0xff;
  }
  if (DAT_0051a54c == 0) {
    iVar5 = iVar5 * 0x10101 + -0x1000000;
  }
  else {
    iVar5 = iVar5 * 0x10101 + -0x2000000;
  }
  *(int *)(DAT_0054f994 * 0x20 + 0x10 + DAT_0053f970) = iVar5;
  *(unsigned int *)(DAT_0054f994 * 0x20 + 0x14 + DAT_0053f970) = 0;
  DAT_0054f994 = DAT_0054f994 + 1;
  return;
}
