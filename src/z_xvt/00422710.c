/* SPDX-License-Identifier: MIT */
/* z_xvt 00422710 */
extern ushort DAT_006cca90;
extern ushort DAT_006cca92;
extern int DAT_006d4988;
extern void FUN_0040f050(ushort param_1);

ushort __cdecl FUN_00422710(uint param_1)
{
  ushort uVar1;
  ushort uVar2;
  
  uVar1 = *(ushort *)((int)&DAT_006cca92 + (param_1 & 0xffff) * 4);
  uVar2 = *(ushort *)((int)&DAT_006cca90 + (param_1 & 0xffff) * 4);
  if (uVar2 < uVar1) {
    do {
      if (*(char *)((uint)uVar2 * 0x23 + 3 + DAT_006d4988) == '\0') {
        int local_ptr;
        __builtin_memcpy(&local_ptr, (void *)(DAT_006d4988 + 0x1f + (uint)uVar2 * 0x23), 4);
        ushort zero_val = 0;
        __builtin_memcpy((void *)(local_ptr + 0x83), &zero_val, 2);
        *(char *)(local_ptr + 1) = 0;
        break;
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < uVar1);
    if (uVar2 < uVar1) {
      FUN_0040f050(uVar2);
      return uVar2;
    }
  }
  return 0xffff;
}
