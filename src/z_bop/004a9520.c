/* SPDX-License-Identifier: MIT */
/* z_bop 004a9520 */
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern short DAT_00a0813c;
extern short DAT_00a07ce4;
extern short DAT_009a6fe0;
extern short DAT_009d8c00;

void __cdecl FUN_004a9520(short param_1, short param_2, short param_3, short param_4)
{
  if (param_2 < 0) {
    param_2 = 0;
  }
  if (param_1 < 0) {
    param_1 = 0;
  }
  if ((unsigned int)DAT_009a7bac < (unsigned int)(int)param_3) {
    param_3 = (short)DAT_009a7bac;
  }
  if ((unsigned int)DAT_009a8c14 < (unsigned int)(int)param_4) {
    param_4 = (short)DAT_009a8c14;
  }
  DAT_00a0813c = param_2;
  DAT_00a07ce4 = param_4;
  DAT_009a6fe0 = param_1;
  DAT_009d8c00 = param_3;
  return;
}
