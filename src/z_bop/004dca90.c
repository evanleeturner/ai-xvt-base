/* SPDX-License-Identifier: MIT */
/* z_bop 004dca90 */
extern byte DAT_00aa7498[];

bool __cdecl FUN_004dca90(byte param_1)
{
  return (DAT_00aa7498[param_1] & 0x80) != 0;
}
