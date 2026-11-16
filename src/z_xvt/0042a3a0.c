/* SPDX-License-Identifier: MIT */
/* z_xvt 0042a3a0 */
extern unsigned short DAT_006d9d50;
extern unsigned int DAT_006d4988;
extern unsigned int DAT_006d9d56;

void __cdecl FUN_00418cd0(int param_1, int param_2, int param_3);

void __stdcall FUN_0042a3a0(void)
{
  unsigned int iVar1;
  int val_1f, val_23, val_27;
  int val_i4, val_i8, val_ic;

  iVar1 = (unsigned int)DAT_006d9d50 * 0x23 + DAT_006d4988;

  __builtin_memcpy(&val_1f, (void *)(DAT_006d9d56 + 0x1f), 4);
  __builtin_memcpy(&val_23, (void *)(DAT_006d9d56 + 0x23), 4);
  __builtin_memcpy(&val_27, (void *)(DAT_006d9d56 + 0x27), 4);

  val_i4 = *(int *)(iVar1 + 4);
  val_i8 = *(int *)(iVar1 + 8);
  val_ic = *(int *)(iVar1 + 0xc);

  FUN_00418cd0(val_1f - val_i4, val_23 - val_i8, val_27 - val_ic);
  return;
}
