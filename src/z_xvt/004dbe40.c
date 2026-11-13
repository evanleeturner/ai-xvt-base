/* SPDX-License-Identifier: MIT */
/* z_xvt 004dbe40 */
extern uint DAT_007595d0;
extern uint DAT_007595d4;
extern unsigned char DAT_007591d0[];

uint __stdcall FUN_004dbe40(void)
{
  uint uVar2;
  uint iVar3;
  unsigned char *puVar1;

  if (DAT_007595d0 == DAT_007595d4) {
    return DAT_007595d4 & 0xffffff00;
  }
  iVar3 = DAT_007595d4 + 1;
  puVar1 = DAT_007591d0 + DAT_007595d4;
  uVar2 = DAT_007595d4 >> 8;
  DAT_007595d4 = iVar3;
  if (iVar3 == 0x400) {
    DAT_007595d4 = 0;
  }
  return (uVar2 << 8) | (uint)*puVar1;
}
