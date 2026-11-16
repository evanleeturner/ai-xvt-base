/* SPDX-License-Identifier: MIT */
/* z_bop 00463c80 */
extern int DAT_00b6d5aa;
extern int DAT_009e9bf8;
extern int DAT_00b6d86a;

int __cdecl FUN_00463c80(int param_1)
{
  int iVar1;
  int *piVar2;
  
  iVar1 = 0;
  piVar2 = &DAT_00b6d5aa;
  do {
    if (*piVar2 == *(int *)((int)&DAT_009e9bf8 + param_1 * 0x5bd)) {
      return iVar1;
    }
    piVar2 = piVar2 + 0x16;
    iVar1 = iVar1 + 1;
  } while (piVar2 < &DAT_00b6d86a);
  return 0;
}
