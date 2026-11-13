/* SPDX-License-Identifier: MIT */
/* z_xvt 0048faa0 */
extern unsigned int DAT_0052a578;
extern unsigned int DAT_0052a57c;
extern void __cdecl FUN_004a57a0(unsigned int, unsigned int *, unsigned int *, unsigned int *, unsigned int *);

unsigned int __cdecl FUN_0048faa0(unsigned int *param_1, unsigned int *param_2, unsigned int *param_3)
{
  unsigned int local_4;
  
  if (DAT_0052a578 == 0) {
    FUN_004a57a0(DAT_0052a57c, param_1, param_2, param_3, &local_4);
    return local_4;
  }
  *param_1 = 0;
  *param_2 = 0;
  *param_3 = 0;
  return 0;
}
