/* SPDX-License-Identifier: MIT */
/* z_bop 0040b180 */
extern unsigned int DAT_0054f9a0;
extern unsigned int DAT_0054f994;
extern unsigned int _DAT_0051a550;
extern unsigned int DAT_0066ddc4;
extern unsigned int DAT_005233e0;
extern unsigned int DAT_009a8e3c;
extern float _DAT_0054f9a4;
extern unsigned int DAT_0066dde0;
extern unsigned int DAT_005233e8;
extern unsigned int DAT_009a7b60;
extern float _DAT_0053f974;
extern unsigned int DAT_0051a54c;
extern unsigned int DAT_00999426;
extern unsigned int DAT_0054f990;
extern unsigned int DAT_0052f868;
extern unsigned int DAT_00528c5c;
extern unsigned int DAT_0053f970;
extern unsigned int DAT_00999420;
extern unsigned int DAT_0054f988;

void __stdcall FUN_0040b180(void)
{
  unsigned int uVar1;
  
  DAT_0054f9a0 = 0;
  DAT_0054f994 = 0;
  _DAT_0051a550 = 0;
  _DAT_0054f9a4 = (float)(((unsigned int)(DAT_0066ddc4 - DAT_005233e0) >> 1) + DAT_009a8e3c);
  _DAT_0053f974 = (float)(((unsigned int)(DAT_0066dde0 - DAT_005233e8) >> 1) + DAT_009a7b60);
  DAT_0051a54c = 1;
  DAT_0054f990 = (DAT_00999426 * 3 & 0x1fffffffU) >> 4;
  DAT_0052f868 = (unsigned int)(DAT_00999426 * 0x18) / 0x14 >> 2;
  if (*(unsigned int *)(DAT_00528c5c + 0x44) < DAT_0054f990) {
    DAT_0054f990 = *(unsigned int *)(DAT_00528c5c + 0x44);
  }
  if (0x100 < (int)DAT_0054f990) {
    DAT_0054f990 = 0x100;
  }
  if (0x100 < DAT_0052f868) {
    DAT_0052f868 = 0x100;
  }
  uVar1 = (*(int *)(DAT_00528c5c + 0x40) + DAT_0054f990 * -0x40) / 0x18;
  if (uVar1 < DAT_0052f868) {
    DAT_0052f868 = uVar1;
  }
  DAT_0053f970 = DAT_00999420;
  DAT_0054f988 = DAT_00999420 + (DAT_00999426 / 2) * 0x18;
  return;
}
