/* SPDX-License-Identifier: MIT */
/* z_xvt 004cee50 */
extern char DAT_00759645;
extern unsigned int DAT_007806fa;
extern unsigned int DAT_00780732;
extern unsigned int DAT_00758830;

void __stdcall FUN_004d4d90(void);
void __stdcall FUN_004ca450(void);
unsigned int __stdcall FUN_004d4d00(void);

unsigned int __stdcall FUN_004cee50(void)
{
  char cVar1;
  
  cVar1 = DAT_00759645;
  if (DAT_007806fa == 0) {
    return 0;
  }
  FUN_004d4d90();
  DAT_00780732 = 1;
  FUN_004ca450();
  if (cVar1 != '\0') {
    DAT_00758830 = FUN_004d4d00();
  }
  return 1;
}
