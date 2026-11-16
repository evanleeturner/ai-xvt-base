/* SPDX-License-Identifier: MIT */
/* z_bop 004aeec0 */
extern char DAT_00521f19[];
extern short DAT_009a7ed0[];
extern void *FUN_004ac720(int param_1);
extern void FUN_00471ff0(int *param_1);
extern int FUN_004ae1a0(int param_1, int param_2);
extern void FUN_004ac740(void);

uint __cdecl FUN_004aeec0(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  uint uVar3;
  int ptr1;
  int ptr2;
  int val_check;
  int val6;
  int arr_val;
  uint val_iVar2;

  if (param_2 < 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720((((uint)(param_1 * 5) >> 0x10) << 16) | (uint)(unsigned short)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  __builtin_memcpy(&ptr1, (char *)piVar1 + 10, 4);
  __builtin_memcpy(&ptr2, (char *)ptr1, 4);
  __builtin_memcpy(&val_check, (char *)ptr2 + 4, 4);
  if (val_check == 0x14) {
    param_2 = param_2 + 1;
  }
  __builtin_memcpy(&val6, (char *)piVar1 + 6, 4);
  if (val6 <= param_2) {
    param_2 = val6 - 1;
  }
  uVar3 = 0;
  __builtin_memcpy(&arr_val, (char *)ptr1 + param_2 * 4, 4);
  iVar2 = FUN_004ae1a0(arr_val, (int)piVar1);
  if (iVar2 != 0) {
    __builtin_memcpy(&val_iVar2, (char *)iVar2 + 4, 4);
    uVar3 = val_iVar2 & 2;
  }
  FUN_004ac740();
  return uVar3;
}
