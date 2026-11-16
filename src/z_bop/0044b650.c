/* SPDX-License-Identifier: MIT */
/* z_bop 0044b650 */
extern int __cdecl FUN_0044b790(int param_1, char *param_2, int *param_3, int **param_4, int *param_5);
extern int __cdecl FUN_0044bad0(int *param_1, int *param_2, int param_3);
extern void __cdecl _free(void *ptr);
extern void *DAT_005569d4;

int * __cdecl FUN_0044b650(int *param_1, char *param_2, int param_3)
{
  int iVar1;
  int *local_1c;
  int *local_18;
  int local_14;
  int local_10;
  int local_c[3];

  local_1c = (int *)0x0;
  local_14 = 0;
  local_10 = 0;
  local_c[0] = 0;
  local_c[1] = 0;
  local_c[2] = 0;
  DAT_005569d4 = (void *)0x0;
  iVar1 = FUN_0044b790(0, param_2, &local_c[2], &local_18, local_c);
  if (iVar1 != 0) {
    local_14 = 0x14;
    local_10 = 0xc2;
    if (param_3 == 0) {
      local_10 = 0xea;
    }
    iVar1 = ((int (__cdecl *)(int *, int *, int **, int))((int **)*param_1)[3])(param_1, &local_14, &local_1c, 0);
    if (-1 < iVar1) {
      iVar1 = FUN_0044bad0(local_1c, local_18, local_c[0]);
      if (iVar1 != 0) {
        goto LAB_0044b6fa;
      }
      ((void (__cdecl *)(int *))((int **)*local_1c)[2])(local_1c);
    }
    local_1c = (int *)0x0;
  }
LAB_0044b6fa:
  if (DAT_005569d4 != (void *)0x0) {
    _free(DAT_005569d4);
  }
  (void)local_10;
  return local_1c;
}
