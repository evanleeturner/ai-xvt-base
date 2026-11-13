/* SPDX-License-Identifier: MIT */
/* z_xvt 0049bfc0 */
extern uint DAT_00645a84;
extern uint DAT_0052acbc;
extern uint *DAT_00645778;
extern uint *DAT_00645a80;
extern uint DAT_00645ad0;
extern uint DAT_00732a18;
extern uint DAT_00645a90;
extern uint DAT_00645738;
extern uint DAT_0052555c;

void __cdecl FUN_00499ce0(void);
void __cdecl FUN_0042ddb0(uint *param_1, uint *param_2, uint param_3);
void __stdcall FUN_0049c210(void);

void __cdecl FUN_0049bfc0(uint param_1, uint param_2)
{
  uint uVar1;
  uint *puVar2;
  uint *puVar3;
  
  DAT_00645a84 = param_1;
  DAT_0052acbc = 1;
  puVar2 = DAT_00645778;
  puVar3 = DAT_00645a80;
  for (uVar1 = param_1 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar3 = *puVar2;
    puVar2 = puVar2 + 1;
    puVar3 = puVar3 + 1;
  }
  for (uVar1 = param_1 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(unsigned char *)puVar3 = *(unsigned char *)puVar2;
    puVar2 = (uint *)((int)puVar2 + 1);
    puVar3 = (uint *)((int)puVar3 + 1);
  }
  DAT_00645ad0 = DAT_00645a84;
  DAT_00732a18 = param_2;
  FUN_00499ce0();
  FUN_0042ddb0(&DAT_00645a90, &DAT_00645738, 0x10);
  DAT_00645a84 = DAT_00645ad0;
  puVar2 = DAT_00645a80;
  puVar3 = DAT_00645778;
  for (uVar1 = DAT_00645ad0 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar3 = *puVar2;
    puVar2 = puVar2 + 1;
    puVar3 = puVar3 + 1;
  }
  for (uVar1 = DAT_00645a84 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(unsigned char *)puVar3 = *(unsigned char *)puVar2;
    puVar2 = (uint *)((int)puVar2 + 1);
    puVar3 = (uint *)((int)puVar3 + 1);
  }
  DAT_0052555c = 0;
  FUN_0049c210();
  return;
}
