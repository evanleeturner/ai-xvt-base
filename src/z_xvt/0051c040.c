/* SPDX-License-Identifier: MIT */
/* z_xvt 0051c040 */
void __cdecl FUN_0051c040(int param_1, int param_2, uint param_3)
{
  uint uVar1;
  uint uVar2;

  if (param_3 != 0) {
    uVar1 = 0;
    do {
      uVar2 = (uint)(unsigned short)(uVar1 + 1);
      *(unsigned short *)(uVar1 * 2 + param_1) = *(unsigned short *)(uVar1 * 2 + param_2);
      uVar1 = uVar2;
    } while (uVar2 < param_3);
  }
  return;
}
