/* SPDX-License-Identifier: MIT */
/* z_bop 004d64e0 */
extern int DAT_00aa7b82;
extern int DAT_00aa7b86;
extern int DAT_00aa7b8a;
extern int DAT_00aa7b8e;

uint __cdecl FUN_004d64e0(int *param_1)
{
  int iVar1;
  uint uVar2;
  
  iVar1 = DAT_00aa7b82;
  uVar2 = 0;
  if (*param_1 < DAT_00aa7b82) {
    *param_1 = DAT_00aa7b82;
    if (param_1[2] < iVar1) {
      param_1[2] = iVar1 - 1;
    }
    uVar2 = 1;
  }
  iVar1 = DAT_00aa7b86;
  if (DAT_00aa7b86 < param_1[2]) {
    param_1[2] = DAT_00aa7b86;
    if (iVar1 < *param_1) {
      *param_1 = iVar1 + 1;
    }
    uVar2 = uVar2 | 4;
  }
  iVar1 = DAT_00aa7b8a;
  if (param_1[1] < DAT_00aa7b8a) {
    param_1[1] = DAT_00aa7b8a;
    if (param_1[3] < iVar1) {
      param_1[3] = iVar1 - 1;
    }
    uVar2 = uVar2 | 2;
  }
  iVar1 = DAT_00aa7b8e;
  if (DAT_00aa7b8e < param_1[3]) {
    param_1[3] = DAT_00aa7b8e;
    if (iVar1 < param_1[1]) {
      param_1[1] = iVar1 + 1;
    }
    uVar2 = uVar2 | 8;
  }
  return uVar2;
}
