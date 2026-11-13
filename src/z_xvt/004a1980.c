/* SPDX-License-Identifier: MIT */
/* z_xvt 004a1980 */
extern uint FUN_004a7aa0(uint param_1);
extern int DAT_006d4988;
extern unsigned char DAT_0052b07e[];
extern unsigned char DAT_0052b0b3[];
extern unsigned char DAT_0052b0b6[];
extern int DAT_0052ad38[];
extern int DAT_0052ad20[];
extern unsigned short s_w1pln_00525b86[];

int __cdecl FUN_004a1980(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  uint local_8;

  iVar5 = param_1 * 0x23 + DAT_006d4988;
  __builtin_memcpy(&iVar1, (void *)(iVar5 + 0x1f), 4);
  __builtin_memcpy(&iVar2, (void *)(iVar1 + 0xa7), 4);
  uVar4 = FUN_004a7aa0(((iVar1 & 0xFFFFFF00) | *(unsigned char *)(iVar5 + 3)) & 0xFFFF00FF);
  uVar6 = 0;
  iVar1 = (uVar4 & 0xffff) * 0xef;
  iVar5 = (uint)(unsigned char)DAT_0052b07e[iVar1] * 0x28;
  if (*(unsigned char *)(iVar2 + 0x228) != 0) {
    do {
      iVar3 = uVar6 + iVar1;
      uVar6 = uVar6 + 1;
      local_8 = (uint)(unsigned char)*(unsigned char *)(iVar2 + 0x346 + DAT_0052b0b3[uVar6 + iVar1] * 4);
      iVar5 = iVar5 + (*(unsigned char *)(iVar2 + 0x346 + DAT_0052b0b6[iVar3] * 4) + local_8)
                      * (uint)*(unsigned short *)(s_w1pln_00525b86 + *(unsigned char *)(iVar2 + 0x228 + uVar6) * 2);
    } while (uVar6 < *(unsigned char *)(iVar2 + 0x228));
  }
  return iVar5 + *(int *)(DAT_0052ad38 + *(unsigned char *)(iVar2 + 0x23c) * 4) +
         *(int *)(DAT_0052ad20 + *(unsigned char *)(iVar2 + 0x233) * 4);
}
