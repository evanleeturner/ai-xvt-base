/* SPDX-License-Identifier: MIT */
/* z_bop 0049f4e0 */
extern unsigned int __cdecl FUN_00404620(unsigned int);
extern void __cdecl FUN_004a4cb0(unsigned short);
extern unsigned int DAT_00a08104;
extern unsigned int DAT_009a73b6;
extern unsigned int DAT_009a1fe8;
extern int DAT_00527760;
extern unsigned int DAT_009a73d0;

void __cdecl FUN_0049f4e0(unsigned int param_1)
{
  unsigned short uVar1;
  short sVar2;
  unsigned int uVar3;
  int iVar4;

  uVar1 = *(unsigned short *)(DAT_00a08104 + 0x6a);
  if (uVar1 == 0xffff) {
    iVar4 = param_1 * 8;
    if ((*(unsigned char *)(DAT_009a73b6 + 0x35) & 1) != 0) {
      sVar2 = *(short *)(param_1 * 0x23 + 0x10 + DAT_009a1fe8) + -0x4000;
      goto LAB_0049f565;
    }
  }
  else {
    if ((*(unsigned char *)(DAT_009a73b6 + 0x35) & 1) != 0) {
      sVar2 = *(short *)((unsigned int)uVar1 * 0x23 + 0x10 + DAT_009a1fe8) + -0x4000;
      goto LAB_0049f565;
    }
    param_1 = (unsigned int)uVar1;
    iVar4 = param_1 << 3;
  }
  sVar2 = *(short *)((iVar4 - param_1) * 5 + 0x10 + DAT_009a1fe8) + 0x4000;
LAB_0049f565:
  *(short *)(DAT_009a73b6 + 0x32) = sVar2;
  uVar3 = FUN_00404620(DAT_00a08104);
  FUN_004a4cb0(((unsigned short)(uVar3 >> 1) & 0x7fff) + 0x8000);
  *(short *)(DAT_009a73b6 + 0x3a) = *(short *)(&DAT_00527760 + (unsigned int)DAT_009a73d0 * 2) * 0xec;
  return;
}
