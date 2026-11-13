/* SPDX-License-Identifier: MIT */
/* z_xvt 00409680 */
extern short DAT_006da53e;
extern short DAT_0071b718;
extern int DAT_006db598;
extern int DAT_00525518;
extern short DAT_0070a170;
extern short DAT_0070acec;
extern short DAT_0070acea;
extern short DAT_0071e518;
extern int DAT_00730884;
extern short DAT_00703ad4;
extern short DAT_0070acd4;

int __stdcall FUN_00409680(void)
{
  DAT_006da53e = DAT_006da53e - 1;
  DAT_0071b718 = DAT_0071b718 - 1;
  DAT_006db598 = DAT_006db598 - DAT_00525518;
  if (DAT_0071b718 < 0) {
    return 0;
  }
  if (DAT_0071b718 != DAT_0070a170) {
    if ((DAT_0071b718 < DAT_0070acec) || (DAT_0070acec <= DAT_006da53e)) goto LAB_00409701;
    DAT_0070acea = DAT_0070acea - 1;
    DAT_0071e518 = DAT_0071e518 - *(short *)(DAT_00730884 + 0x1934 + DAT_0070acea * 2);
    if (-1 < DAT_0071e518) goto LAB_00409701;
  }
  DAT_0071e518 = 0;
LAB_00409701:
  if (DAT_006da53e == DAT_0070a170) {
    DAT_00703ad4 = *(short *)(DAT_00730884 + 0xe) - 1;
    DAT_0070acea = *(short *)(DAT_00730884 + 0x1932) - 1;
    DAT_0071e518 = (DAT_00703ad4 - *(short *)(DAT_00730884 + 0x1934 + DAT_0070acea * 2)) + 1;
    if (DAT_0071e518 < 0) {
      DAT_0071e518 = 0;
      return 1;
    }
  }
  else if (DAT_006da53e < 0) {
    DAT_0070acd4 = DAT_0070acd4 - 1;
    DAT_00703ad4 = DAT_00703ad4 - *(short *)(DAT_00730884 + 0x1934 + DAT_0070acd4 * 2);
    if (DAT_00703ad4 < 0) {
      DAT_00703ad4 = 0;
    }
  }
  return 1;
}
