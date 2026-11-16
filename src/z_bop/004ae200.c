/* SPDX-License-Identifier: MIT */
/* z_bop 004ae200 */
extern unsigned short DAT_009a7ed0[];
extern unsigned char DAT_00521f19[];

int __cdecl FUN_004ac720(int param_1);
void __stdcall FUN_004ac740(void);
void __cdecl FUN_00471ff0(int param_1);
int __cdecl FUN_004ae1a0(int param_1, int param_2);

int __cdecl FUN_004ae200(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  int **ptr_at_10;
  int val_at_6;

  if (DAT_009a7ed0[param_1] == 0) {
    return 0;
  }
  if (param_2 < 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720((int)DAT_009a7ed0[param_1]);
  FUN_004ac740();
  if ((int *)*piVar1 != piVar1) {
    FUN_00471ff0((int)piVar1);
  }
  __builtin_memcpy(&ptr_at_10, (void *)((int)piVar1 + 10), 4);
  __builtin_memcpy(&val_at_6, (void *)((int)piVar1 + 6), 4);
  if (*(int *)(*ptr_at_10 + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (val_at_6 <= param_2) {
    param_2 = val_at_6 - 1;
  }
  iVar2 = FUN_004ae1a0((*ptr_at_10)[param_2], (int)piVar1);
  return iVar2;
}
