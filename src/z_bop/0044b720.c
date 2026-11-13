/* SPDX-License-Identifier: MIT */
/* z_bop 0044b720 */
extern void *DAT_005569d4;
extern int __cdecl FUN_0044b790(int, char *, int, void **, unsigned int *);
extern int __cdecl FUN_0044bad0(int *, void *, unsigned int);
extern void __cdecl _free(void *);

int __cdecl FUN_0044b720(int *param_1, char *param_2)
{
  int iVar1;
  int uVar2;
  unsigned int local_8;
  void *local_4;
  
  uVar2 = 0;
  DAT_005569d4 = 0;
  iVar1 = FUN_0044b790(0, param_2, 0, &local_4, &local_8);
  if (iVar1 != 0) {
    iVar1 = ((int (__stdcall *)(int *))(*(int **)param_1)[0x14])(param_1);
    if (iVar1 >= 0) {
      iVar1 = FUN_0044bad0(param_1, local_4, local_8);
      if (iVar1 != 0) {
        uVar2 = 1;
      }
    }
  }
  if (DAT_005569d4 != 0) {
    _free(DAT_005569d4);
  }
  return uVar2;
}
