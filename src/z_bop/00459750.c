/* SPDX-License-Identifier: MIT */
/* z_bop 00459750 */
extern ushort DAT_00a082c2[];
extern ushort DAT_00a082c0[];
extern void *DAT_009a1fe8;

void __cdecl FUN_0041bb00(ushort param_1);

ushort __cdecl FUN_00459750(uint param_1)
{
  ushort uVar1;
  ushort uVar2;
  
  uVar1 = *(ushort *)((char *)DAT_00a082c2 + (param_1 & 0xffff) * 4);
  uVar2 = *(ushort *)((char *)DAT_00a082c0 + (param_1 & 0xffff) * 4);
  if (uVar2 < uVar1) {
    do {
      if (*(char *)((char *)DAT_009a1fe8 + 3 + (uint)uVar2 * 0x23) == '\0') {
        ushort local_zero = 0;
        __builtin_memcpy((void *)(*(int *)((char *)DAT_009a1fe8 + 0x1f + (uint)uVar2 * 0x23) + 0x85), &local_zero, 2);
        *(char *)(*(int *)((char *)DAT_009a1fe8 + 0x1f + (uint)uVar2 * 0x23) + 1) = 0;
        break;
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < uVar1);
    if (uVar2 < uVar1) {
      FUN_0041bb00(uVar2);
      return uVar2;
    }
  }
  return 0xffff;
}
