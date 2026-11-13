/* SPDX-License-Identifier: MIT */
/* z_xvt 00421ff0 */
extern ushort DAT_006ccabe;
extern ushort DAT_006ccabc;
extern void *DAT_006d4988;

extern void FUN_0040f050(ushort param_1);
extern void FUN_00422f60(uint param_1, uint param_2);
extern uint FUN_00478470(void);

uint FUN_00421ff0(uint param_1, char param_2)
{
  ushort uVar1;
  ushort uVar4;
  int iVar2;
  uint uVar3;
  ushort uVar5;

  uVar1 = DAT_006ccabe;
  uVar4 = 0;
  uVar5 = DAT_006ccabc;
  if (DAT_006ccabc < DAT_006ccabe) {
    do {
      if (*(char *)((uint)uVar5 * 0x23 + 3 + (uint)DAT_006d4988) == '\0') {
        int *ptr = (int *)((char *)DAT_006d4988 + 0x1f + (uint)uVar5 * 0x23);
        *(ushort *)(*ptr + 0x83) = 0;
        iVar2 = (int)DAT_006d4988;
        *(char *)(*ptr + 1) = 0;
        break;
      }
      uVar5 = uVar5 + 1;
      iVar2 = (int)DAT_006d4988;
    } while (uVar5 < DAT_006ccabe);
    uVar4 = (ushort)((uint)iVar2 >> 16);
    if (uVar5 < uVar1) {
      FUN_0040f050(uVar5);
      goto LAB_00422095;
    }
  }
  uVar5 = 0xffff;
LAB_00422095:
  if (uVar5 != 0xffff) {
    FUN_00422f60((uint)uVar5, param_1 & 0xffff);
    iVar2 = (uint)uVar5 * 0x23;
    int *ptr = (int *)((char *)DAT_006d4988 + 0x1f + iVar2);
    *(char *)(*ptr + 0) = 3;
    *(char *)((char *)DAT_006d4988 + 2 + iVar2) = 0xb;
    *(char *)(*ptr + 1) = 0;
    *(char *)((char *)DAT_006d4988 + 3 + iVar2) = 0x59;
    *(char *)(*ptr + 0x85) = *(char *)((param_1 & 0xffff) * 0x23 + 3 + (uint)DAT_006d4988);
    *(uint *)((char *)DAT_006d4988 + 0x1b + iVar2) = 0xffffffff;
    *(ushort *)(*ptr + 0x81) = 0;
    uVar3 = FUN_00478470();
    *(short *)(*ptr + 0x7f) = ((short)(uVar3 & 0xffff0007) + 4) * 0xec;
    *(char *)((char *)DAT_006d4988 + 0x19 + iVar2) = param_2 * 2;
    *(char *)((char *)DAT_006d4988 + 0x1a + iVar2) = 0;
    return ((uint)((uVar3 & 0xffff0007) >> 16) << 16) | uVar5;
  }
  return ((uint)uVar4 << 16) | 0xffff;
}
