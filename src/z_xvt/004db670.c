/* SPDX-License-Identifier: MIT */
/* z_xvt 004db670 */
extern undefined4 DAT_0077e0a2;
extern undefined4 DAT_0077dc66;
extern undefined4 DAT_0077e0a6;
extern undefined4 DAT_007806f2;
extern undefined4 DAT_0077dc6a;

void __cdecl FUN_004db670(undefined4 param_1, undefined4 param_2)
{
  *(undefined4 *)(&DAT_0077e0a2 + DAT_0077dc66 * 0x440) = param_1;
  *(undefined4 *)(&DAT_0077e0a6 + DAT_0077dc66 * 0x440) = param_2;
  DAT_007806f2 = 0xffffffff;
  DAT_0077dc6a = 1;
  return;
}
