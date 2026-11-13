/* SPDX-License-Identifier: MIT */
/* z_xvt 0042b1e0 */
int __cdecl FUN_0042b1e0(int param_1, int param_2)
{
  short *psVar1;
  int iVar2;

  psVar1 = *(short **)(param_1 + 0x3f2 + param_2 * 4);
  if (psVar1 == (short *)0x0) {
    return param_1 + 0x386 + param_2 * 4;
  }
  if (*(short *)(param_1 + 0x3c6 + param_2 * 2) != *psVar1) {
    *(short *)(param_1 + 0x3c6 + param_2 * 2) = 0;
    return param_1 + 0x386 + param_2 * 4;
  }
  if (*(int *)((int)psVar1 + 0x1f) == 0) {
    return param_1 + 0x386 + param_2 * 4;
  }
  iVar2 = *(int *)(*(int *)((int)psVar1 + 0x1f) + 0xab);
  if (iVar2 == 0) {
    return param_1 + 0x386 + param_2 * 4;
  }
  return iVar2 + 0x44;
}
