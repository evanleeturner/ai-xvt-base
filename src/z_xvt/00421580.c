/* SPDX-License-Identifier: MIT */
/* z_xvt 00421580 */
void __cdecl FUN_00421580(int param_1)
{
  int *piVar2;
  int iVar1;

  if (*(int *)(param_1 + 0x432) != 0) {
    *(unsigned char *)(*(int *)(param_1 + 0x432) + 3) = 0;
    *(int *)(param_1 + 0x432) = 0;
  }
  piVar2 = (int *)(param_1 + 0x3f2);
  iVar1 = 0x10;
  do {
    if (*piVar2 != 0) {
      *(unsigned char *)(*piVar2 + 3) = 0;
      *piVar2 = 0;
    }
    piVar2++;
    iVar1--;
  } while (iVar1 != 0);
  return;
}
