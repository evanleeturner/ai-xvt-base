/* SPDX-License-Identifier: MIT */
/* z_bop 004af5b0 */
extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];

extern int * __cdecl FUN_004ac720(int param_1);
extern void __cdecl FUN_00471ff0(int *param_1);
extern int __cdecl FUN_004ae1a0(int param_1, int param_2);
extern void __stdcall FUN_004ac740(void);

int __cdecl FUN_004af5b0(int param_1)
{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int field_6;
  int field_10;

  if (((DAT_00521f19[param_1 * 0x1a] & 1) == 0)) {
    return 0;
  }
  piVar2 = (int *)FUN_004ac720((((unsigned int)(param_1 * 5) >> 0x10) << 16) | (unsigned int)(short)DAT_009a7ed0[param_1]);
  if (*piVar2 != (int)piVar2) {
    FUN_00471ff0(piVar2);
  }
  iVar5 = 0;
  iVar4 = 0;
  __builtin_memcpy(&field_6, (char *)piVar2 + 6, 4);
  if (0 < field_6) {
    do {
      __builtin_memcpy(&field_10, (char *)piVar2 + 10, 4);
      iVar1 = *(int *)(field_10 + iVar5);
      if ((iVar1 != 0) && (*(int *)(iVar1 + 4) != 0x14)) {
        piVar3 = (int *)FUN_004ae1a0(iVar1, (int)piVar2);
        if ((piVar3 != (int *)0x0) && (*piVar3 == 3)) {
          FUN_004ac740();
          return 1;
        }
      }
      iVar5 = iVar5 + 4;
      iVar4 = iVar4 + 1;
    } while (iVar4 < field_6);
  }
  FUN_004ac740();
  return 0;
}
