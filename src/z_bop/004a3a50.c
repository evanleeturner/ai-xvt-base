/* SPDX-License-Identifier: MIT */
/* z_bop 004a3a50 */
extern int DAT_00a08104;
extern int DAT_009a1fe8;
extern int DAT_009a73b6;
extern int DAT_00527760;
extern int DAT_009a73d0;

extern unsigned int __cdecl FUN_00404620(int param_1);
extern void __cdecl FUN_004a4cb0(int param_1);

void __cdecl FUN_004a3a50(int param_1)
{
  short sVar1;
  unsigned int uVar2;
  
  if (*(unsigned short *)(DAT_00a08104 + 0x6a) == 0xffff) {
    sVar1 = *(short *)(param_1 * 0x23 + 0x10 + DAT_009a1fe8) + 0x8000;
  }
  else {
    sVar1 = *(short *)((unsigned int)*(unsigned short *)(DAT_00a08104 + 0x6a) * 0x23 + 0x10 + DAT_009a1fe8);
  }
  *(short *)(DAT_009a73b6 + 0x32) = sVar1;
  uVar2 = FUN_00404620(DAT_00a08104);
  FUN_004a4cb0(((unsigned short)(uVar2 >> 1) & 0x7fff) + 0x8000);
  *(short *)(DAT_009a73b6 + 0x3a) = *(short *)(&DAT_00527760 + (unsigned int)DAT_009a73d0 * 2) * 0xec;
  return;
}
