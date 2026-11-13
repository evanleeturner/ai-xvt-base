/* SPDX-License-Identifier: MIT */
/* z_bop 004fa500 */
extern int DAT_00aa5e68;
extern int DAT_00b6a5a6;
extern int DAT_00aa5e10[];
extern int DAT_00aa5cb0[];
extern int DAT_00aa5ac0[];

int __stdcall FUN_004fa500(void)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = 0;
  if (0 < DAT_00aa5e68) {
    do {
      iVar2 = 0;
      if (0 < DAT_00aa5e10[DAT_00b6a5a6]) {
        do {
          iVar1 = DAT_00b6a5a6 * 8 + iVar2;
          if (DAT_00aa5cb0[iVar1] != 0 && DAT_00aa5ac0[iVar1] == -1) {
            return 0;
          }
          iVar2 = iVar2 + 1;
        } while (iVar2 < DAT_00aa5e10[DAT_00b6a5a6]);
      }
      iVar3 = iVar3 + 1;
    } while (iVar3 < DAT_00aa5e68);
  }
  return 1;
}
