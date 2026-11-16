/* SPDX-License-Identifier: MIT */
/* z_xvt 0051c080 */
extern undefined2 DAT_00742460[];

void __cdecl FUN_0051c080(undefined2 *param_1, unsigned int param_2, int param_3)
{
  undefined2 uVar1;
  
  uVar1 = DAT_00742460[param_2 & 0xffff];
  if (param_3 != 0) {
    do {
      *param_1 = uVar1;
      param_1++;
    } while (--param_3 != 0);
  }
  return;
}
