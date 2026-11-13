/* SPDX-License-Identifier: MIT */
/* z_bop 00403070 */
extern uint FUN_00425b70(uint param_1, ushort param_2);
extern bool FUN_00403360(uint param_1, uint param_2);
extern ushort DAT_009a73d0;
extern ushort DAT_00524100;

bool __cdecl FUN_00403070(uint param_1)
{
  uint uVar2;
  
  uVar2 = FUN_00425b70(0x500, *(ushort *)(&DAT_00524100 + (uint)DAT_009a73d0 * 2));
  return FUN_00403360(param_1 & 0xffff, ((uVar2 & 0xffff) + 0xa00) * 0x100) == 1;
}
