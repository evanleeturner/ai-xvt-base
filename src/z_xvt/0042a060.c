/* SPDX-License-Identifier: MIT */
/* z_xvt 0042a060 */
extern uint DAT_006d9d56;
extern ushort DAT_006d9d50;
extern uint DAT_006d4988;
extern uint DAT_00732a2c;
extern uint DAT_00732a28;
extern uint DAT_00732a24;

void __cdecl FUN_00421c70(uint param_1, uint param_2);

void __stdcall FUN_0042a060(void)
{
  FUN_00421c70((uint)*(ushort *)(DAT_006d9d56 + 0x18),
               (uint)*(byte *)((uint)DAT_006d9d50 * 0x23 + 0x16 + DAT_006d4988));
  uint tmp_1f = DAT_00732a2c;
  __builtin_memcpy((void *)(DAT_006d9d56 + 0x1f), &tmp_1f, 4);
  uint tmp_23 = DAT_00732a28;
  __builtin_memcpy((void *)(DAT_006d9d56 + 0x23), &tmp_23, 4);
  uint tmp_27 = DAT_00732a24;
  __builtin_memcpy((void *)(DAT_006d9d56 + 0x27), &tmp_27, 4);
  return;
}
