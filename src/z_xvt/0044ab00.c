/* SPDX-License-Identifier: MIT */
/* z_xvt 0044ab00 */
extern unsigned short DAT_006da850[];
extern int DAT_00525cc8;

int __cdecl FUN_0048f4f0(int param_1);
void __cdecl FUN_00485fa0(int *param_1);
void * __cdecl FUN_0044a830(int param_1, int param_2, int param_3);
long long __cdecl __ftol(void);
void __cdecl FUN_0048f510(void);

int __cdecl FUN_0044ab00(int param_1, int param_2, int param_3)
{
  int *piVar1;
  void *puVar2;
  int iVar3;
  long long lVar4;
  int v10;
  int v6;

  piVar1 = (int *)FUN_0048f4f0((int)(unsigned short)DAT_006da850[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00485fa0(piVar1);
  }
  __builtin_memcpy(&v10, (char *)piVar1 + 10, 4);
  __builtin_memcpy(&v6, (char *)piVar1 + 6, 4);
  if (*(int *)((int)v10 + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (v6 <= param_2) {
    param_2 = v6 - 1;
  }
  iVar3 = 0;
  DAT_00525cc8 = 0;
  puVar2 = FUN_0044a830(*(int *)((int)v10 + param_2 * 4), (int)piVar1, param_3);
  if (puVar2 != (void *)0x0) {
    lVar4 = __ftol();
    iVar3 = (int)lVar4;
  }
  FUN_0048f510();
  return -iVar3;
}
