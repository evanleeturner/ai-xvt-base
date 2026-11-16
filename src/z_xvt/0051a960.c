/* SPDX-License-Identifier: MIT */
/* z_xvt 0051a960 */
extern ushort DAT_005366fc;
extern int DAT_00742660;
extern int DAT_0074278c;
extern int DAT_005366d4;
extern int DAT_00742668;
extern int DAT_00742730;
extern int DAT_00757c30;
extern int DAT_007427c0;
extern int DAT_007426d0;

short __cdecl FUN_0051aa30(undefined4 param_1, undefined4 param_2, ushort param_3);
void __cdecl FUN_0051c6f0(char *param_1);

short __cdecl FUN_0051a960(undefined4 param_1, undefined4 param_2, ushort param_3)
{
  short sVar1;
  
  DAT_005366fc = param_3;
  sVar1 = 1;
  if (DAT_00742660 < DAT_0074278c) {
    if (DAT_005366d4 != 0) {
      sVar1 = FUN_0051aa30(param_1, param_2, param_3);
      return sVar1;
    }
    FUN_0051aa30(param_1, param_2, param_3);
    if (DAT_00742660 < DAT_0074278c + -1) {
      DAT_00742660 = DAT_00742660 + 1;
      FUN_0051c6f0((char *)(&DAT_007426d0)[DAT_00742660]);
      DAT_00742668 = (&DAT_00742730)[DAT_00742660];
      (**(void (**)(undefined4))(DAT_00757c30 + 0x30))((&DAT_007427c0)[DAT_00742660]);
      return 1;
    }
  }
  else {
    sVar1 = 0;
  }
  return sVar1;
}
