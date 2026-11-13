/* SPDX-License-Identifier: MIT */
/* z_xvt 0044bfb0 */
extern ushort DAT_006d49a0;
extern ushort DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern char DAT_00703ca0[];
extern char s_escortldr1pln_00524904[];
extern char DAT_00732e54[];
extern byte DAT_006d9d7a;
extern uint DAT_006d9d50;
extern uint DAT_006d499c;
extern void *DAT_006d9d56;

extern ushort __cdecl FUN_0049ec30(ushort, undefined2, ushort);
extern int __cdecl FUN_0042b150(int);
extern bool __cdecl FUN_00429a20(uint);
extern bool __cdecl FUN_00429c40(uint);
extern bool __cdecl FUN_0044c560(ushort, short);
extern void __cdecl FUN_0042a1a0(uint, uint);

ushort __cdecl FUN_0044bfb0(undefined2 param_1, ushort param_2, short param_3, undefined2 param_4, ushort param_5)
{
  ushort uVar1;
  ushort uVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  uint unaff_EBP;
  char *pcVar6;
  char *pcVar7;
  bool bVar8;
  ushort local_8;
  ushort uStack_6;
  undefined2 local_4;
  undefined2 uStack_2;

  uStack_6 = 0xffff;
  local_4 = 0xffff;
  uStack_2 = 0xffff;
  local_8 = 0;
  unaff_EBP = 0;
  if (0 < DAT_006d49a0) {
    do {
      uVar1 = FUN_0049ec30(local_8, param_1, param_2);
      uVar2 = FUN_0049ec30(local_8, param_4, param_5);
      if (param_3 == 1) {
        uVar1 = uVar1 | uVar2;
      }
      else {
        uVar1 = uVar1 & uVar2;
      }
      if ((uVar1 != 0) &&
         (unaff_EBP = (unaff_EBP & 0xFFFF0000) | DAT_00732a3c,
         (int)(uint)DAT_00732a3c < DAT_006d4980)) {
        do {
          uVar5 = unaff_EBP & 0xffff;
          iVar3 = uVar5 * 0x23 + DAT_006d4988;
          uVar1 = (ushort)unaff_EBP;
          if (*(char *)(iVar3 + 3) != '\0') {
            iVar3 = FUN_0042b150(*(int *)(*(int *)(iVar3 + 0x1f) + 0xa7));
            bVar8 = 0;
            iVar4 = 0xe;
            pcVar6 = (char *)((int)&DAT_00703ca0 + (uint)*(byte *)(iVar3 + 0xb) * 0x55);
            pcVar7 = s_escortldr1pln_00524904;
            do {
              if (iVar4 == 0) break;
              iVar4 = iVar4 - 1;
              bVar8 = *pcVar6 == *pcVar7;
              pcVar6 = pcVar6 + 1;
              pcVar7 = pcVar7 + 1;
            } while (bVar8);
            if (((bVar8) && (*(byte *)(iVar3 + 0x2d) == local_8)) &&
               (*(ushort *)((char *)DAT_00732e54 + 0x342) != uVar1)) {
              bVar8 = FUN_00429a20(uVar5);
              if (bVar8 != 0) {
                if ((DAT_006d9d7a & 4) != 0) {
                  bVar8 = FUN_00429c40(unaff_EBP);
                  if (bVar8 == 0) goto LAB_0044c12b;
                }
                if ((DAT_006d9d7a & 1) != 0) {
                  bVar8 = FUN_0044c560(uVar1, 0xff);
                  if (bVar8 == 0) goto LAB_0044c12b;
                }
                FUN_0042a1a0((uint)DAT_006d9d50, uVar5);
                if (DAT_006d499c < (((uint)uStack_2 << 16) | local_4)) {
                  local_4 = (undefined2)DAT_006d499c;
                  uStack_2 = (undefined2)(DAT_006d499c >> 0x10);
                  uStack_6 = uVar1;
                }
              }
            }
          }
LAB_0044c12b:
          unaff_EBP = (unaff_EBP & 0xFFFF0000) | (uVar1 + 1);
        } while ((int)(uint)(ushort)(uVar1 + 1) < DAT_006d4980);
      }
      local_8 = local_8 + 1;
    } while ((int)(uint)local_8 < (int)DAT_006d49a0);
  }
  if (uStack_6 != 0xffff) {
    *(ushort *)(DAT_006d9d56 + 0x18) = uStack_6;
    *(undefined2 *)(DAT_006d9d56 + 0x1a) = *(undefined2 *)((uint)uStack_6 * 0x23 + DAT_006d4988);
    *(undefined1 *)(DAT_006d9d56 + 0x1e) = 1;
  }
  return uStack_6;
}
