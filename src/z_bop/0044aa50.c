/* SPDX-License-Identifier: MIT */
/* z_bop 0044aa50 */
extern short DAT_00a08108;
extern short DAT_009d8c00;
extern short DAT_0055698c;
extern short DAT_00556990;
extern unsigned char DAT_009a20ae;
extern unsigned short DAT_00a08102;
extern short DAT_009a6fe0;
extern short DAT_00556994;
extern short DAT_00a0813c;
extern unsigned short DAT_00556988;
extern short DAT_00a07ce4;

void __cdecl FUN_0044a5b0(int);

void __stdcall FUN_0044aa50(void)
{
  unsigned int uVar1;
  
  if ((short)DAT_00a08108 < DAT_009d8c00) {
    DAT_0055698c = DAT_009d8c00;
    DAT_00556990 = DAT_00a08108;
    uVar1 = (unsigned int)DAT_009a20ae + (unsigned int)DAT_00a08102;
    if ((int)(unsigned int)DAT_00a08108 < (int)(short)DAT_009a6fe0) {
      DAT_00556990 = DAT_009a6fe0;
    }
    DAT_00556994 = DAT_00a08102;
    if ((int)(unsigned int)DAT_00a08102 < (int)(short)DAT_00a0813c) {
      DAT_00556994 = DAT_00a0813c;
    }
    DAT_00556988 = (unsigned short)uVar1;
    if ((int)(short)DAT_00a07ce4 < (int)(uVar1 & 0xffff)) {
      DAT_00556988 = DAT_00a07ce4;
    }
    if (DAT_00556994 < DAT_00556988) {
      FUN_0044a5b0(0);
    }
  }
  return;
}
