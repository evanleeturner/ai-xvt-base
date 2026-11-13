/* SPDX-License-Identifier: MIT */
/* z_xvt 00433e00 */
extern undefined4 DAT_006da4f8;
extern undefined4 DAT_006db59c;
extern undefined4 DAT_0071e6b8;

void __cdecl FUN_00433e00(undefined4 param_1, undefined4 param_2)
{
  if (DAT_006da4f8 < 0x128) {
    *(undefined4 *)(DAT_0071e6b8 + DAT_006da4f8 * 0xc) = param_2;
    *(undefined4 *)(DAT_0071e6b8 + 4 + DAT_006da4f8 * 0xc) = param_1;
    *(int *)(DAT_0071e6b8 + 8 + DAT_006da4f8 * 0xc) = DAT_006db59c;
    DAT_006db59c = DAT_0071e6b8 + (DAT_006da4f8 + -2 + (DAT_006da4f8 + 1) * 2) * 4;
    DAT_006da4f8 = DAT_006da4f8 + 1;
  }
  return;
}
