/* SPDX-License-Identifier: MIT */
/* z_bop 00423b90 */
extern short DAT_009e9654;
extern int DAT_009a8c18;
extern int DAT_005233d8;
extern short DAT_009d8c2c;
extern short DAT_009d8c2a;
extern short DAT_009ec458;
extern int DAT_009fe7d4;
extern short DAT_009a7bb2;
extern short DAT_009d114c;
extern short DAT_00a07cd0;
extern short DAT_009ed230;
extern short DAT_009d8c14;

int __stdcall FUN_00423b90(void)
{
  DAT_009e9654 = DAT_009e9654 + 1;
  DAT_009a8c18 = DAT_009a8c18 + DAT_005233d8;
  if (DAT_009e9654 != 0) {
    if (-1 < DAT_009e9654) {
      if (DAT_009d8c2c <= DAT_009e9654) {
        return 0;
      }
      goto LAB_00423bb5;
    }
    DAT_009d8c2a = DAT_009d8c2a + -1;
    DAT_009ec458 = DAT_009ec458 - *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009d8c2a * 2);
    if (-1 < DAT_009ec458) goto LAB_00423bb5;
  }
  DAT_009ec458 = 0;
LAB_00423bb5:
  DAT_009a7bb2 = DAT_009a7bb2 + 1;
  if (DAT_009a7bb2 == 0) {
    DAT_009d114c = *(short *)(DAT_009fe7d4 + 0xe) + -1;
    DAT_009d8c2a = *(short *)(DAT_009fe7d4 + 0x1932) + -1;
    DAT_009ec458 = DAT_00a07cd0 - *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009d8c2a * 2);
    if (DAT_009ec458 < 0) {
      DAT_009ec458 = 0;
      return 1;
    }
  }
  else if (DAT_009d8c2c <= DAT_009a7bb2) {
    if (DAT_009d8c2c == DAT_009a7bb2) {
      DAT_009d114c = DAT_009ed230;
      DAT_009d8c14 = *(short *)(DAT_009fe7d4 + 0x1932);
    }
    DAT_009d8c14 = DAT_009d8c14 + -1;
    DAT_009d114c = DAT_009d114c - *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009d8c14 * 2);
  }
  return 1;
}
