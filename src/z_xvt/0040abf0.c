/* SPDX-License-Identifier: MIT */
/* z_xvt 0040abf0 */
extern unsigned int FUN_00439830(int, int);
extern void FUN_004398b0(int);
extern void FUN_00490a50(char *, int, int, int, unsigned int, int);
extern int DAT_0070a189;
extern int DAT_0085e792;
extern int DAT_0085e79b;
extern int DAT_0073ba30;

void __cdecl FUN_0040abf0(int param_1)
{
  unsigned int uVar1;
  unsigned int uVar2;
  int iVar3;
  
  if (((DAT_0070a189 != '\0') && (DAT_0085e792 != '\0')) && (DAT_0085e79b != 0)) {
    if (param_1 == 0) {
      uVar1 = FUN_00439830(0x27, 0x100);
      if (uVar1 != 0) {
        FUN_004398b0(0x27);
      }
      uVar1 = FUN_00439830(0x28, 0x100);
      if (uVar1 != 0) {
        FUN_004398b0(0x28);
        return;
      }
    }
    else {
      uVar1 = 0x7f;
      if (DAT_0085e79b < 10) {
        uVar1 = (unsigned int)DAT_0085e79b * 0xd;
      }
      if (param_1 == 1) {
        iVar3 = 0x28;
        uVar1 = uVar1 / 3;
      }
      else {
        iVar3 = 0x27;
        uVar1 = uVar1 / 2;
      }
      uVar2 = FUN_00439830(iVar3, 0x100);
      if (uVar2 == 0) {
        FUN_00490a50((char *)(&DAT_0073ba30 + iVar3 * 6), 1, 1, 0x7d, uVar1, 0x40);
      }
    }
  }
  return;
}
