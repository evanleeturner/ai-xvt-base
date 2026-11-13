/* SPDX-License-Identifier: MIT */
/* z_bop 0044af70 */
extern uint DAT_005233cc;
extern uint DAT_005233ec;
extern uint DAT_005233f4;
extern byte DAT_00523928[];
extern byte DAT_00523929[];
extern ushort DAT_005569c0[];
extern ushort DAT_00a00530[];

int __cdecl FUN_0040e190(int param_1);
void __stdcall FUN_00410ff0(void);

void __cdecl FUN_0044af70(uint param_1, uint param_2, byte param_3)
{
  int iVar1;
  uint uVar2;
  ushort *puVar3;
  uint uVar4;
  ushort uVar5;
  short local_e;
  
  local_e = 7;
  uVar4 = 0;
  uVar5 = 0;
  do {
    iVar1 = FUN_0040e190((int)(char)DAT_00523929[uVar4] + (param_2 & 0xffff));
    uVar2 = (uint)(iVar1 + ((int)(char)DAT_00523928[uVar4] + (param_1 & 0xffff)) * 2);
    if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
      uVar2 = uVar2 % DAT_005233cc;
      FUN_00410ff0();
      FUN_00410ff0();
    }
    uVar4 = (uint)(ushort)(uVar4 + 2);
    puVar3 = (ushort *)(uVar2 + (DAT_005233f4 & 0xfffffffe));
    DAT_005569c0[uVar5] = *puVar3;
    *puVar3 = DAT_00a00530[(uint)param_3];
    local_e = local_e - 1;
    uVar5 = uVar5 + 1;
  } while (local_e != 0);
  return;
}
