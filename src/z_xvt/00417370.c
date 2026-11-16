/* SPDX-License-Identifier: MIT */
/* z_xvt 00417370 */
extern int __cdecl FUN_00461ff0(int param_1);
extern void __cdecl FUN_004651f0(uint arg1, uint arg2);

extern uint DAT_0052550c;
extern uint DAT_0052552c;
extern uint DAT_00525534;
extern byte DAT_00523c50[];
extern byte DAT_00523c51[];
extern undefined2 DAT_00536a08[];
extern undefined2 DAT_007325e0[];

void __cdecl FUN_00417370(uint param_1, uint param_2, byte param_3)
{
  int iVar1;
  uint uVar2;
  undefined2 *puVar3;
  uint uVar4;
  ushort uVar5;
  short local_e;
  
  local_e = 7;
  uVar4 = 0;
  uVar5 = 0;
  do {
    iVar1 = FUN_00461ff0((int)(char)DAT_00523c51[uVar4] + (param_2 & 0xffff));
    uVar2 = iVar1 + ((int)(char)DAT_00523c50[uVar4] + (param_1 & 0xffff)) * 2;
    if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
      uVar2 = uVar2 % DAT_0052550c;
      FUN_004651f0(0, 0);
      FUN_004651f0(0, 0);
    }
    uVar4 = (uint)(ushort)((short)uVar4 + 2);
    puVar3 = (undefined2 *)(uVar2 + (DAT_00525534 & 0xfffffffe));
    DAT_00536a08[uVar5] = *puVar3;
    *puVar3 = DAT_007325e0[param_3];
    local_e = local_e + -1;
    uVar5 = uVar5 + 1;
  } while (local_e != 0);
  return;
}
