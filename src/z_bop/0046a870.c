/* SPDX-License-Identifier: MIT */
/* z_bop 0046a870 */
extern short DAT_00a07c5c;
extern int DAT_009d6824;
extern short DAT_009d12b0;
extern int DAT_009a20b0;
extern int DAT_009a8e14;
extern int DAT_009cd26c;
extern int DAT_009e9648;
extern int DAT_009fd438;
extern int DAT_00a00520;

void __cdecl FUN_0046a7c0(void);

void __cdecl FUN_0046a870(uint param_1, short param_2, undefined2 param_3)
{
  DAT_00a07c5c = 0x4000 - param_2;
  DAT_009d6824 = param_1 & 0xffff;
  DAT_009d12b0 = param_3;
  FUN_0046a7c0();
  DAT_009a20b0 = DAT_009a8e14;
  DAT_009cd26c = DAT_009e9648;
  DAT_009fd438 = DAT_00a00520;
  return;
}
