/* SPDX-License-Identifier: MIT */
/* z_xvt 0040a840 */
extern unsigned int __cdecl FUN_00439830(int param_1, int param_2);
extern int __cdecl FUN_004398b0(int param_1);
extern void __cdecl FUN_00490a50(void *param_1, int param_2, int param_3, int param_4, int param_5, int param_6);

extern char DAT_0070a189;
extern char DAT_0085e792;
extern char DAT_0085e79b;
extern char DAT_0073bef8;
extern char DAT_0073bee0;

int __cdecl FUN_0040a840(int param_1)
{
  unsigned int uVar1;
  int iVar2;
  
  if (DAT_0070a189 == '\0') {
    return 0;
  }
  if (DAT_0085e792 == '\0') {
    return 0;
  }
  if (DAT_0085e79b == 0) {
    return 0;
  }
  iVar2 = 0x7f;
  if (DAT_0085e79b < 10) {
    iVar2 = (unsigned int)DAT_0085e79b * 0xd;
  }
  if ((param_1 == 0) || (param_1 == 1)) {
    uVar1 = FUN_00439830(0x33, 0x100);
    if (uVar1 != 0) {
      FUN_004398b0(0x33);
      return 1;
    }
    uVar1 = FUN_00439830(0x32, 0x100);
    if (uVar1 != 0) {
      FUN_004398b0(0x32);
    }
  }
  else if (param_1 == 3) {
    uVar1 = FUN_00439830(0x32, 0x100);
    if (uVar1 != 0) {
      FUN_004398b0(0x32);
    }
    uVar1 = FUN_00439830(0x33, 0x100);
    if (uVar1 == 0) {
      FUN_00490a50(&DAT_0073bef8, 1, 1, 0x7d, iVar2, 0x40);
      return 1;
    }
  }
  else {
    uVar1 = FUN_00439830(0x33, 0x100);
    if (uVar1 != 0) {
      FUN_004398b0(0x33);
    }
    uVar1 = FUN_00439830(0x32, 0x100);
    if (uVar1 == 0) {
      FUN_00490a50(&DAT_0073bee0, 1, 1, 0x7d, iVar2, 0x40);
      return 1;
    }
  }
  return 1;
}
