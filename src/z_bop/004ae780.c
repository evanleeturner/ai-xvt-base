/* SPDX-License-Identifier: MIT */
/* z_bop 004ae780 */
extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];

int * __cdecl FUN_004ac720(int param_1);
void __cdecl FUN_00471ff0(int *param_1);
int __cdecl FUN_004ae1a0(int param_1, int *param_2);
long long __cdecl __ftol(void);
void __stdcall FUN_004ac740(void);

int __cdecl FUN_004ae780(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  int uVar3;
  long long lVar4;
  int field_6;
  int *field_10;

  if (param_2 < 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720((((int)(short)((param_1 * 5) >> 16)) << 16) | (unsigned short)DAT_009a7ed0[param_1]);
  if (*(int *)piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  __builtin_memcpy(&field_6, (char *)piVar1 + 6, 4);
  __builtin_memcpy(&field_10, (char *)piVar1 + 10, 4);
  if (*(int *)(*field_10 + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (field_6 <= param_2) {
    param_2 = field_6 - 1;
  }
  uVar3 = 0;
  iVar2 = FUN_004ae1a0(field_10[param_2], piVar1);
  if (iVar2 != 0) {
    lVar4 = __ftol();
    uVar3 = (int)lVar4;
  }
  FUN_004ac740();
  return uVar3;
}
