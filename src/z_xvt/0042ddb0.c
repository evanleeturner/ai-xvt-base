/* SPDX-License-Identifier: MIT */
/* z_xvt 0042ddb0 */
extern uint DAT_005373d4;
extern uint DAT_005373d0;
extern uint DAT_005373d8;
extern uint DAT_00732a18;

extern int FUN_00432ca0(void);
extern void FUN_00431110(int param_1, uint *param_2, uint param_3);

void __cdecl FUN_0042ddb0(uint *param_1, uint *param_2, uint param_3)
{
  uint uVar1;
  int iVar2;
  uint *puVar3;
  uint *puVar4;

  DAT_005373d4 = DAT_00732a18;
  DAT_005373d0 = 4;
  puVar3 = &DAT_005373d8;
  for (uVar1 = param_3 & 0x3fffffff; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar3 = *param_1;
    param_1 = param_1 + 1;
    puVar3 = puVar3 + 1;
  }
  for (iVar2 = 0; iVar2 != 0; iVar2 = iVar2 + -1) {
    *(unsigned char *)puVar3 = *(unsigned char *)param_1;
    param_1 = (uint *)((int)param_1 + 1);
    puVar3 = (uint *)((int)puVar3 + 1);
  }
  puVar3 = (uint *)((char *)&DAT_005373d8 + param_3 * 4);
  for (uVar1 = param_3 & 0x3fffffff; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar3 = *param_2;
    param_2 = param_2 + 1;
    puVar3 = puVar3 + 1;
  }
  for (iVar2 = 0; iVar2 != 0; iVar2 = iVar2 + -1) {
    *(unsigned char *)puVar3 = *(unsigned char *)param_2;
    param_2 = (uint *)((int)param_2 + 1);
    puVar3 = (uint *)((int)puVar3 + 1);
  }
  uVar1 = param_3 * 8 + 8;
  puVar4 = &DAT_005373d0;
  iVar2 = FUN_00432ca0();
  FUN_00431110(iVar2, puVar4, uVar1);
  return;
}
