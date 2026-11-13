/* SPDX-License-Identifier: MIT */
/* z_bop 0042cbd0 */
extern int DAT_00520ef0[];
extern void (*DAT_009a6fe4)(unsigned int);

void __cdecl FUN_0042cbd0(int param_1, unsigned int param_2, unsigned int param_3)
{
  _Bool bVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  unsigned int uVar4;

  bVar1 = 0;
  for (; param_2 != 0; param_2 = param_2 - 1) {
    int divisor = DAT_00520ef0[param_2];
    uVar2 = (unsigned int)(param_1 / divisor);
    param_1 = param_1 - (int)((uVar2 & 0xffff) * divisor);
    uVar4 = uVar2 >> 0x10;
    if (bVar1 || param_2 > param_3 || (unsigned short)uVar2 != 0) {
      bVar1 = 1;
      if ((unsigned short)uVar2 > 9) {
        uVar2 = (uVar2 & 0xffff0000) | 9;
      }
      uVar3 = ((unsigned int)(unsigned short)(uVar2 >> 0x10) << 16) | (unsigned int)(unsigned short)(uVar2 + 0x30);
    }
    else {
      uVar3 = (uVar4 << 16) | 0x20;
      bVar1 = 0;
    }
    DAT_009a6fe4(uVar3);
  }
  return;
}
