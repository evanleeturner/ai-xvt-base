/* SPDX-License-Identifier: MIT */
/* z_xvt 0044ab90 */
extern int in_EAX;
extern short DAT_006da850[];
extern int DAT_00525cc8;

extern int * __cdecl FUN_0048f4f0(int param_1);
extern void __cdecl FUN_00485fa0(int *param_1);
extern void * __cdecl FUN_0044a830(void *param_1, int param_2, int param_3);
extern long long __cdecl __ftol(void);
extern void __cdecl FUN_0048f510(void);

int __cdecl FUN_0044ab90(int param_1, int param_2, int param_3)
{
  int *piVar1;
  void *puVar2;
  int uVar3;
  long long lVar4;
  int ptr_at_a;
  int val_at_6;

  piVar1 = FUN_0048f4f0(((unsigned int)in_EAX & 0xFFFF0000) | (unsigned short)DAT_006da850[param_1]);
  if ((int *)*piVar1 != piVar1) {
    FUN_00485fa0(piVar1);
  }
  __builtin_memcpy(&ptr_at_a, (void *)((int)piVar1 + 10), 4);
  if (*(int *)(ptr_at_a + 4) != 0x14) {
    param_2 = param_2 + 1;
  }
  __builtin_memcpy(&val_at_6, (void *)((int)piVar1 + 6), 4);
  if (param_2 >= val_at_6) {
    param_2 = val_at_6 - 1;
  }
  uVar3 = 0;
  DAT_00525cc8 = 0;
  puVar2 = FUN_0044a830(((void **)ptr_at_a)[param_2], (int)piVar1, param_3);
  if (puVar2 != NULL) {
    lVar4 = __ftol();
    uVar3 = (int)lVar4;
  }
  FUN_0048f510();
  return uVar3;
}
