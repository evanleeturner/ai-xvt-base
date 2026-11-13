/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7810 */
extern int DAT_0052b050;
extern int DAT_0052b05c;
extern int DAT_0052b060;
extern int DAT_0052b064;
extern int DAT_0052b068;
extern int DAT_00645bd8;

void __stdcall FUN_00490490(void);
int * __stdcall FUN_004904e0(void);
void __stdcall FUN_00490510(void);

void __stdcall FUN_004a7810(void)
{
  int *piVar1;
  
  if (DAT_0052b050 != 0) {
    if (DAT_0052b060 == 0) {
      if (DAT_00645bd8 >> 1 < DAT_0052b05c) {
        FUN_00490490();
        DAT_0052b060 = 1;
        return;
      }
    }
    else if (DAT_0052b060 == 2) {
      piVar1 = FUN_004904e0();
      if ((DAT_0052b064 < (int)piVar1) && ((int)piVar1 < DAT_0052b068)) {
        FUN_00490510();
        DAT_0052b060 = 0;
        DAT_0052b05c = 0;
      }
    }
  }
  return;
}
