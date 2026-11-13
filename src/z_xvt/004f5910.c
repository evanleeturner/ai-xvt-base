/* SPDX-License-Identifier: MIT */
/* z_xvt 004f5910 */
extern int DAT_00757a28;
extern int DAT_0081bfb6;
extern int DAT_007579d0[];
extern int DAT_00757870[];
extern int DAT_00757680[];

undefined8 __fastcall FUN_004f5910(undefined4 param_1, undefined4 param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = 0;
  if (0 < DAT_00757a28) {
    do {
      iVar2 = 0;
      if (0 < DAT_007579d0[DAT_0081bfb6]) {
        param_2 = 0;
        do {
          iVar1 = DAT_0081bfb6 * 8 + iVar2;
          if ((DAT_00757870[iVar1] != 0) && (DAT_00757680[iVar1] == -1)) {
            return 0;
          }
          iVar2 = iVar2 + 1;
        } while (iVar2 < DAT_007579d0[DAT_0081bfb6]);
      }
      iVar3 = iVar3 + 1;
    } while (iVar3 < DAT_00757a28);
  }
  return ((undefined8)param_2 << 32) | 1;
}
