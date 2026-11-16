/* SPDX-License-Identifier: MIT */
/* z_xvt 004a72b0 */
extern int *DAT_00657c00;

void __cdecl FUN_004a72b0(int *param_1)
{
  int iVar1;
  int iVar2;
  int local_64[20];
  
  local_64[0] = 100;
  while (1) {
    iVar1 = (*(int (__cdecl **)(int *, int, int, int, int, int *))(*param_1 + 0x14))(param_1, 0, 0, 0, 0x400, local_64);
    if (iVar1 == 0) {
      return;
    }
    if (iVar1 == (int)0x887601c2) {
      iVar2 = (*(int (__cdecl **)(int *))(*DAT_00657c00 + 0x6c))(DAT_00657c00);
      if (iVar2 != 0) {
        break;
      }
    }
    if (iVar1 != (int)0x8876021c) {
      return;
    }
  }
  return;
}
