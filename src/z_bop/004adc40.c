/* SPDX-License-Identifier: MIT */
/* z_bop 004adc40 */
extern short DAT_009a7ed0[];
extern unsigned char DAT_00521f19[];

int *FUN_004ac720(int param);
void FUN_00471ff0(int *param);
void FUN_004ac740(void);

int __cdecl FUN_004adc40(int param_1)
{
  int *piVar1;
  int iVar2;

  if (DAT_009a7ed0[param_1] == 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = FUN_004ac720(((unsigned int)param_1 & 0xFFFF0000) | (unsigned int)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  iVar2 = *(int *)((int)piVar1 + 6);
  if (*(int *)(*(int *)((int)piVar1 + 10) + 4) == 0x14) {
    iVar2 = iVar2 - 1;
  }
  FUN_004ac740();
  if (0x32 < iVar2) {
    iVar2 = 0x32;
  }
  return iVar2;
}
