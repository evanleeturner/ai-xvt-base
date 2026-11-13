/* SPDX-License-Identifier: MIT */
/* z_bop 004c9bf0 */
extern char DAT_00b69d20[];
extern char s__02d__02d_0052b5c0[];
extern char s__02d__02d__02d_0052b5b0[];
int _sprintf(char *, const char *, ...);

void __cdecl FUN_004c9bf0(uint param_1)
{
  if (param_1 / 0xe10 == 0) {
    _sprintf((char *)&DAT_00b69d20, s__02d__02d_0052b5c0);
    return;
  }
  _sprintf((char *)&DAT_00b69d20, s__02d__02d__02d_0052b5b0, param_1 / 0xe10,
           (int)(((unsigned __int64)param_1 / 0x3c) % 0x3c), param_1 % 0x3c);
  return;
}
