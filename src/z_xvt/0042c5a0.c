/* SPDX-License-Identifier: MIT */
/* z_xvt 0042c5a0 */
extern uint _DAT_005373d0;

int __stdcall FUN_00432ca0(void);
void __cdecl FUN_00431110(int param_1, uint *param_2, uint param_3);

void __stdcall FUN_0042c5a0(void)
{
  int iVar1;
  uint *puVar2;
  uint uVar3;
  
  _DAT_005373d0 = 0x1d;
  uVar3 = 4;
  puVar2 = &_DAT_005373d0;
  iVar1 = FUN_00432ca0();
  FUN_00431110(iVar1, puVar2, uVar3);
  return;
}
