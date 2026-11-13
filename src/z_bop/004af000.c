/* SPDX-License-Identifier: MIT */
/* z_bop 004af000 */
extern int __cdecl FUN_004ac720(int param_1);
extern void __cdecl FUN_00471ff0(int *param_1);
extern int __cdecl FUN_004add10(int param_1);
extern void __cdecl FUN_004ac740(void);
extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];

unsigned int __cdecl FUN_004af000(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  unsigned int uVar3;

  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720((((unsigned int)(param_1 * 5) >> 16) << 16) | (unsigned short)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  if (*(int *)(*(int *)((char *)piVar1 + 10) + 4) != 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((char *)piVar1 + 6) <= param_2) {
    param_2 = *(int *)((char *)piVar1 + 6) - 1;
  }
  uVar3 = 0;
  iVar2 = FUN_004add10((*(int **)((char *)piVar1 + 10))[param_2]);
  if (iVar2 != 0) {
    uVar3 = *(unsigned int *)(iVar2 + 0x14);
  }
  FUN_004ac740();
  return uVar3;
}
