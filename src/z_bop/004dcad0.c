/* SPDX-License-Identifier: MIT */
/* z_bop 004dcad0 */
extern uint DAT_00aa7a98;
extern uint DAT_00aa7a9c;
extern unsigned char DAT_00aa7698[];

uint __stdcall FUN_004dcad0(void)
{
  unsigned char *puVar1;
  uint uVar2;
  
  if (DAT_00aa7a98 == DAT_00aa7a9c) {
    return DAT_00aa7a9c & 0xffffff00;
  }
  puVar1 = DAT_00aa7698 + DAT_00aa7a9c;
  uVar2 = DAT_00aa7a9c >> 8;
  DAT_00aa7a9c = DAT_00aa7a9c + 1;
  if (DAT_00aa7a9c == 0x400) {
    DAT_00aa7a9c = 0;
  }
  return (uVar2 << 8) | (uint)*puVar1;
}
