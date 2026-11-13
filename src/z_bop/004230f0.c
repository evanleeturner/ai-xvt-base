/* SPDX-License-Identifier: MIT */
/* z_bop 004230f0 */
extern short DAT_009e9652;
extern int DAT_009a8c18;
extern int DAT_009fe7d4;
extern short DAT_009ed230;
extern short DAT_009ec458;
extern short DAT_009d114c;
extern short DAT_009d1314;
extern short DAT_009a73f6;
extern short DAT_009a7bb0;
extern short DAT_00a07cd0;

int __stdcall FUN_004230f0(void)
{
  DAT_009e9652 = DAT_009e9652 + -1;
  DAT_009a8c18 = DAT_009a8c18 + *(int *)(DAT_009fe7d4 + 0x3eb8);
  if (DAT_009e9652 == DAT_009ed230) {
    DAT_009ec458 = 0;
    DAT_009d114c = -1;
    DAT_009d1314 = -1;
  }
  else if (DAT_009e9652 < 0) {
    if (DAT_009e9652 == -1) {
      DAT_009a73f6 = -1;
    }
    DAT_009a73f6 = DAT_009a73f6 + 1;
    DAT_009ec458 = DAT_009ec458 + *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009a73f6 * 2);
  }
  DAT_009a7bb0 = DAT_009a7bb0 + -1;
  if (DAT_009ed230 == DAT_009a7bb0) {
    DAT_009d114c = *(short *)(DAT_009fe7d4 + 0xe) + -1;
    return 1;
  }
  if (DAT_009a7bb0 < 0) {
    return 0;
  }
  if ((DAT_00a07cd0 <= DAT_009a7bb0) && (DAT_009e9652 < DAT_00a07cd0)) {
    DAT_009d1314 = DAT_009d1314 + 1;
    DAT_009d114c = DAT_009d114c + *(short *)(DAT_009fe7d4 + 0x1934 + DAT_009d1314 * 2);
  }
  return 1;
}
