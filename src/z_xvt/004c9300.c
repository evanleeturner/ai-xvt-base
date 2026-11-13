/* SPDX-License-Identifier: MIT */
/* z_xvt 004c9300 */
void __cdecl _sprintf(char *dst, const char *fmt, ...);
extern char DAT_0081b740[];
extern const char s__02d__02d_005322f4[];
extern const char s__02d__02d__02d_005322e4[];

void __cdecl FUN_004c9300(uint param_1)
{
  if (param_1 / 0xe10 == 0) {
    _sprintf((char *)&DAT_0081b740, s__02d__02d_005322f4);
    return;
  }
  _sprintf((char *)&DAT_0081b740, s__02d__02d__02d_005322e4, param_1 / 0xe10,
           (int)(((unsigned long long)param_1 / 0x3c) % 0x3c), param_1 % 0x3c);
  return;
}
