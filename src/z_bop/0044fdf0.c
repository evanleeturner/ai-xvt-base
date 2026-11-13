/* SPDX-License-Identifier: MIT */
/* z_bop 0044fdf0 */
extern uint DAT_005235f0;
extern uint DAT_005233b8;
extern uint DAT_009d1308;
extern uint DAT_00a07ce8;
extern uint DAT_009a1fe8;

void __cdecl FUN_0044f880(uint param_1);

void __cdecl FUN_0044fdf0(uint param_1)
{
  DAT_005235f0 = 0;
  if ((DAT_005233b8 != 0) && (param_1 < (uint)(DAT_009d1308 + DAT_00a07ce8))) {
    FUN_0044f880(param_1 * 0x23 + DAT_009a1fe8);
  }
  return;
}
