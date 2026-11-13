/* SPDX-License-Identifier: MIT */
/* z_xvt 004cddb0 */
extern int DAT_007806fa;
extern char DAT_00759645;
extern int DAT_00758830;

void FUN_004d4d90(void);
void FUN_004cdc60(int param_1);
int FUN_004d4d00(void);

void __cdecl FUN_004cddb0(int param_1)
{
  if (DAT_007806fa != 0) {
    FUN_004d4d90();
    FUN_004cdc60(param_1);
    if (DAT_00759645 != 0) {
      DAT_00758830 = FUN_004d4d00();
    }
  }
  return;
}
