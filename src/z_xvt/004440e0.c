/* SPDX-License-Identifier: MIT */
/* z_xvt 004440e0 */
extern uint DAT_006d4980;
extern uint DAT_006d4988;

uint __cdecl FUN_004440e0(ushort param_1)
{
  int iVar1;
  uint uVar2;
  uint uVar3;
  int local_1b;
  int local_1f;
  int local_a7;

  if (param_1 == 0xffff) {
    return 0;
  }
  if (param_1 >= DAT_006d4980) {
    return DAT_006d4980 & 0xffffff00;
  }
  uVar2 = (uint)param_1 * 0x23 + DAT_006d4988;
  __builtin_memcpy(&local_1b, (void *)(uVar2 + 0x1b), 4);
  if (local_1b == -1) {
    return uVar2 & 0xffffff00;
  }
  __builtin_memcpy(&local_1f, (void *)(uVar2 + 0x1f), 4);
  __builtin_memcpy(&local_a7, (void *)(local_1f + 0xa7), 4);
  iVar1 = local_a7;
  if (iVar1 == 0) {
    return 0;
  }
  uVar3 = (uint)iVar1 >> 8;
  if ((((((*(unsigned char *)(iVar1 + 0x1e0)) & 1) != 0) && (*(char *)(iVar1 + 0x237) != '\0')) &&
      (*(char *)(iVar1 + 0x233) == '\x03')) && (*(short *)(iVar1 + 0x238) != 0)) {
    return (uVar3 << 8) | 1;
  }
  return uVar3 << 8;
}
