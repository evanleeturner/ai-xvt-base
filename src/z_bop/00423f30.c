/* SPDX-License-Identifier: MIT */
/* z_bop 00423f30 */
extern short DAT_009a7bb2;
extern short DAT_009e9654;
extern int DAT_009a8c18;
extern int DAT_005233d8;
extern short DAT_009d77f0;
extern short DAT_009d8c2c;
extern short DAT_009d8c2a;
extern short DAT_009ec458;
extern int DAT_009fe7d4;
extern short DAT_009d114c;
extern short DAT_009d8c14;

undefined4 __stdcall FUN_00423f30(void)
{
  DAT_009a7bb2 = DAT_009a7bb2 - 1;
  DAT_009e9654 = DAT_009e9654 - 1;
  DAT_009a8c18 = DAT_009a8c18 - DAT_005233d8;
  if (DAT_009e9654 < 0) {
    return 0;
  }
  if (DAT_009e9654 != DAT_009d77f0) {
    if ((DAT_009e9654 < DAT_009d8c2c) || (DAT_009a7bb2 < DAT_009d8c2c)) goto LAB_00423fb1;
    DAT_009d8c2a = DAT_009d8c2a - 1;
    DAT_009ec458 = DAT_009ec458 - *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009d8c2a * 2);
    if (-1 < DAT_009ec458) goto LAB_00423fb1;
  }
  DAT_009ec458 = 0;
LAB_00423fb1:
  if (DAT_009d77f0 == DAT_009a7bb2) {
    DAT_009d114c = *(short *)(DAT_009fe7d4 + 0xe) - 1;
    DAT_009d8c2a = *(short *)(DAT_009fe7d4 + 0x1932) - 1;
    DAT_009ec458 = (DAT_009d114c - *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009d8c2a * 2)) + 1;
    if (DAT_009ec458 < 0) {
      DAT_009ec458 = 0;
      return 1;
    }
  }
  else if (DAT_009a7bb2 < 0) {
    DAT_009d8c14 = DAT_009d8c14 - 1;
    DAT_009d114c = DAT_009d114c - *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009d8c14 * 2);
    if (DAT_009d114c < 0) {
      DAT_009d114c = 0;
    }
  }
  return 1;
}
