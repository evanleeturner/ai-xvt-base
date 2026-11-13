/* SPDX-License-Identifier: MIT */
/* z_xvt 0040acd0 */
extern int DAT_00525550;
extern char DAT_0070a189;
extern char DAT_0085e792;
extern unsigned char DAT_0085e79b;
extern int DAT_0071b730;
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern char DAT_0071b741;
extern int DAT_0073bbf8;

extern unsigned int __cdecl FUN_00439830(int, int);
extern void __cdecl FUN_004398b0(int);
extern void __cdecl FUN_00490a50(void *, int, int, int, int, int);

void __stdcall FUN_0040acd0(void)
{
  int iVar1;
  unsigned int uVar2;
  int iVar3;
  
  if ((((DAT_00525550 == 0) && (DAT_0070a189 != '\0')) && (DAT_0085e792 != '\0')) &&
      (DAT_0085e79b != 0)) {
    iVar1 = *(int *)((int)&DAT_0071b730 + DAT_0071ecf4 * 0x5bd);
    if (iVar1 == -1) {
      uVar2 = FUN_00439830(0x13, 0x100);
      if (uVar2 != 0) {
        FUN_004398b0(0x13);
        return;
      }
    }
    else {
      iVar1 = *(int *)(*(int *)(iVar1 * 0x23 + 0x1f + DAT_006d4988) + 0xa7);
      if (*(char *)(iVar1 + 0x23c) == '\x01') {
        if ((&DAT_0071b741)[DAT_0071ecf4 * 0x5bd] == '\x01') {
          uVar2 = FUN_00439830(0x13, 0x100);
          if (uVar2 != 0) {
            FUN_004398b0(0x13);
            return;
          }
        }
        else {
          iVar3 = 0x7f;
          if (DAT_0085e79b < 10) {
            iVar3 = (unsigned int)DAT_0085e79b * 0xd;
          }
          if (*(short *)(iVar1 + 0x23e) == 0) {
            uVar2 = FUN_00439830(0x13, 0x100);
            if (uVar2 != 0) {
              FUN_004398b0(0x13);
            }
          }
          else {
            uVar2 = FUN_00439830(0x13, 0x100);
            if (uVar2 == 0) {
              FUN_00490a50(&DAT_0073bbf8, 1, 1, 0x7d, iVar3 >> 2, 0x40);
              return;
            }
          }
        }
      }
    }
  }
  return;
}
