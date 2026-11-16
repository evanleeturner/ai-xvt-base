/* SPDX-License-Identifier: MIT */
/* z_bop 0046a7c0 */
extern int DAT_00a00520;
extern int DAT_009a8e14;
extern short DAT_009d12b0;
extern int DAT_009d6824;
extern int DAT_009e9648;
extern short DAT_00a07c5c;

int __cdecl FUN_0046a650(int param_1, short param_2);
int __cdecl FUN_0046a760(int param_1, short param_2);

void __stdcall FUN_0046a7c0(void)
{
  DAT_00a00520 = FUN_0046a650(DAT_009d6824, DAT_009d12b0);
  DAT_009a8e14 = FUN_0046a760(DAT_00a00520, DAT_00a07c5c);
  DAT_009e9648 = FUN_0046a650(DAT_00a00520, DAT_00a07c5c);
  DAT_00a00520 = FUN_0046a760(DAT_009d6824, DAT_009d12b0);
  return;
}
