/* SPDX-License-Identifier: MIT */
/* z_bop 004b4d40 */
int __cdecl FUN_004b7a70(char *param_1);
unsigned int __cdecl FUN_004b4d90(int *param_1, int param_2, int param_3);
extern int *DAT_00aa7201;

unsigned int __cdecl FUN_004b4d40(char *param_1, int param_2, int param_3)
{
  int *piVar1;
  int iVar2;
  unsigned int uVar3;

  iVar2 = FUN_004b7a70(param_1);
  if (iVar2 == -1) {
    return 0;
  }
  piVar1 = *(int **)(DAT_00aa7201 + 0x40 + iVar2 * 0x44);
  if (piVar1[2] != 0) {
    return 0;
  }
  uVar3 = FUN_004b4d90(piVar1, param_2, param_3);
  return uVar3;
}
