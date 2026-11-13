/* SPDX-License-Identifier: MIT */
/* z_xvt 004f3680 */
void __cdecl FUN_004f2e40(int param_1);
void __cdecl FUN_004f2c10(void);

short DAT_0065047a;
short DAT_00650478;
int DAT_00650438;
int DAT_0065043c;

void __stdcall FUN_004f3680(void)
{
  if (DAT_0065047a < DAT_00650478) {
    FUN_004f2e40(0);
    return;
  }
  DAT_00650438 = 0;
  DAT_0065043c = 0;
  FUN_004f2c10();
  return;
}
