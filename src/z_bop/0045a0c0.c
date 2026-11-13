/* SPDX-License-Identifier: MIT */
/* z_bop 0045a0c0 */
extern int DAT_00a07ce8;
extern int DAT_009d6820;
extern int DAT_009a1fe8;
extern int DAT_0099f9a4;
extern int DAT_009a8da0;
extern int DAT_00a07bd0;
extern int DAT_009a1090[];
extern int DAT_009a8e18;

void __stdcall FUN_0045a0c0(void)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  
  iVar1 = 0;
  if (0 < DAT_00a07ce8) {
    iVar3 = 0;
    iVar2 = 0;
    do {
      iVar4 = DAT_009d6820 + iVar3;
      iVar2 = iVar2 + 0x23;
      iVar3 = iVar3 + 0xb1;
      iVar1 = iVar1 + 1;
      __builtin_memcpy((void *)(DAT_009a1fe8 - 4 + iVar2), &iVar4, 4);
    } while (iVar1 < DAT_00a07ce8);
  }
  iVar2 = 0;
  iVar1 = 0;
  if (0 < DAT_00a07ce8) {
    piVar5 = (int *)&DAT_0099f9a4;
    do {
      if (piVar5[-1] == -1) {
        if (*piVar5 == -1) {
          if (piVar5[1] != -1) {
            int rhs = DAT_009a8da0 + piVar5[1] * 0x4c;
            __builtin_memcpy((void *)(DAT_009d6820 + 0xad + iVar2), &rhs, 4);
          }
        }
        else {
          int rhs2 = *piVar5 * 0x462 + DAT_00a07bd0;
          __builtin_memcpy((void *)(DAT_009d6820 + 0xa9 + iVar2), &rhs2, 4);
          if (DAT_009a1090[*piVar5] != -1) {
            int inner_val;
            __builtin_memcpy(&inner_val, (void *)(DAT_009d6820 + 0xa9 + iVar2), 4);
            int rhs3 = DAT_009a1090[*piVar5] * 0x23 + DAT_009a1fe8;
            __builtin_memcpy((void *)(inner_val + 0x45e), &rhs3, 4);
          }
        }
      }
      else {
        int rhs4 = DAT_009a8e18 + piVar5[-1] * 10;
        __builtin_memcpy((void *)(DAT_009d6820 + 0xa5 + iVar2), &rhs4, 4);
      }
      piVar5 = piVar5 + 3;
      iVar2 = iVar2 + 0xb1;
      iVar1 = iVar1 + 1;
    } while (iVar1 < DAT_00a07ce8);
  }
  return;
}
