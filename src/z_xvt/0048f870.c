/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f870 */
extern unsigned int DAT_0052a570;
extern unsigned int DAT_0052a574;
extern unsigned int DAT_0052a578;
extern unsigned int DAT_0052a57c;

extern unsigned int __cdecl FUN_004a57a0(unsigned int param_1, unsigned int *param_2, unsigned int *param_3, unsigned int *param_4, unsigned int *param_5);

unsigned int __stdcall FUN_0048f870(void)
{
  unsigned int local_10;
  unsigned int local_c;
  unsigned int local_8;
  unsigned int local_4;
  
  if (DAT_0052a570 == 0) {
    DAT_0052a574 = 1;
    FUN_004a57a0(0, &local_10, &local_c, &local_4, &local_8);
    if ((local_10 == 32000) && (local_c == 32000)) {
      FUN_004a57a0(1, &local_10, &local_c, &local_4, &local_8);
      if ((local_10 == 32000) && (local_c == 32000)) {
        DAT_0052a578 = 0;
      }
      else {
        DAT_0052a57c = 1;
        DAT_0052a578 = 1;
      }
    }
    else {
      DAT_0052a57c = 0;
      DAT_0052a578 = 1;
    }
    DAT_0052a570 = 1;
  }
  return DAT_0052a574;
}
