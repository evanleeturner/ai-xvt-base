/* SPDX-License-Identifier: MIT */
/* z_xvt 00408140 */
extern short DAT_0071b716;
extern int DAT_006db598;
extern int DAT_00730884;
extern short DAT_00732a20;
extern short DAT_0071e518;
extern short DAT_006d9d96;
extern short DAT_006da53c;
extern short DAT_00703ad4;
extern short DAT_00703c94;

int __stdcall FUN_00408140(void)
{
  DAT_0071b716 = DAT_0071b716 + 1;
  DAT_006db598 = DAT_006db598 - *(int *)(DAT_00730884 + 0x3eb8);
  if (DAT_0071b716 == 0) {
    DAT_0071e518 = 0;
  }
  else {
    if (DAT_00732a20 <= DAT_0071b716) {
      return 0;
    }
    if (DAT_0071b716 < 0) {
      DAT_006d9d96 = DAT_006d9d96 + -1;
      DAT_0071e518 = DAT_0071e518 - *(short *)(DAT_00730884 + 0x1934 + DAT_006d9d96 * 2);
    }
  }
  DAT_006da53c = DAT_006da53c + 1;
  if (DAT_006da53c == 0) {
    DAT_00703ad4 = *(short *)(DAT_00730884 + 0xe) + -1;
    DAT_006d9d96 = *(short *)(DAT_00730884 + 0x1932) + -1;
    DAT_0071e518 = (DAT_00703ad4 - *(short *)(DAT_00730884 + 0x1934 + DAT_006d9d96 * 2)) + 1;
    if (DAT_0071e518 < 0) {
      DAT_0071e518 = 0;
      return 1;
    }
  }
  else if (DAT_00732a20 <= DAT_006da53c) {
    if (DAT_006da53c == DAT_00732a20) {
      DAT_00703c94 = *(short *)(DAT_00730884 + 0x1932);
    }
    DAT_00703c94 = DAT_00703c94 + -1;
    DAT_00703ad4 = DAT_00703ad4 - *(short *)(DAT_00730884 + 0x1934 + DAT_00703c94 * 2);
  }
  return 1;
}
