/* SPDX-License-Identifier: MIT */
/* z_bop 004b52c0 */
extern int FUN_004b7a70(char *param_1);
extern uint FUN_004b5310(int *param_1, int *param_2, int param_3, int param_4, uint param_5);
extern void *DAT_00aa7201;

uint __cdecl FUN_004b52c0(char *param_1, int *param_2, int param_3, int param_4, uint param_5)
{
  int *piVar1;
  int iVar2;
  uint uVar3;
  
  iVar2 = FUN_004b7a70(param_1);
  if (iVar2 == -1) {
    return 0;
  }
  piVar1 = *(int **)((char *)DAT_00aa7201 + 0x40 + iVar2 * 0x44);
  if (piVar1[2] != 0) {
    return 0;
  }
  uVar3 = FUN_004b5310(piVar1, param_2, param_3, param_4, param_5);
  return uVar3;
}
