/* SPDX-License-Identifier: MIT */
/* z_xvt 004d0b60 */
int __cdecl FUN_004d3570(char *param_1);
uint __cdecl FUN_004d0bc0(int *param_1, int *param_2, int param_3, int param_4, uint param_5);
extern int DAT_00758d41;

uint __cdecl FUN_004d0b60(char *param_1, int *param_2, int param_3, int param_4, uint param_5)
{
  int *piVar1;
  int iVar2;
  uint uVar3;
  
  iVar2 = FUN_004d3570(param_1);
  if (iVar2 == -1) {
    return 0;
  }
  piVar1 = *(int **)(DAT_00758d41 + 0x40 + iVar2 * 0x44);
  if (piVar1[2] != 0) {
    return 0;
  }
  uVar3 = FUN_004d0bc0(piVar1, param_2, param_3, param_4, param_5);
  return uVar3;
}
