/* SPDX-License-Identifier: MIT */
/* z_bop 0046f840 */
extern uint _DAT_005597a0;
extern int _DAT_005597a4;
extern int DAT_009994fc;
extern uint DAT_00999550;
extern uint DAT_005597a8;

void __cdecl FUN_0046d350(int param_1, uint *param_2, int param_3);
int * __cdecl FUN_0046f5e0(int *param_1, uint *param_2, int param_3);

uint __cdecl FUN_0046f840(int param_1)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  uint *puVar4;
  uint *puVar5;
  uint *local_c;
  uint local_8;
  int local_4;
  
  _DAT_005597a0 = 0xe;
  _DAT_005597a4 = DAT_009994fc;
  FUN_0046d350(param_1, (uint *)&_DAT_005597a0, 8);
  if (param_1 == 0) {
    do {
      piVar1 = FUN_0046f5e0(&local_4, &local_8, 0x3c);
      if (piVar1 == (int *)0x0) {
        return 0;
      }
    } while (*piVar1 != 0xe);
  }
  iVar3 = 0;
  if (0 < DAT_009994fc) {
    local_c = &DAT_00999550;
    do {
      _DAT_005597a0 = 0xf;
      puVar4 = local_c;
      puVar5 = &DAT_005597a8;
      _DAT_005597a4 = iVar3;
      for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 + -1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
      }
      FUN_0046d350(param_1, (uint *)&_DAT_005597a0, 0x30);
      if (param_1 == 0) {
        do {
          piVar1 = FUN_0046f5e0(&local_4, &local_8, 0x3c);
          if (piVar1 == (int *)0x0) {
            return 0;
          }
        } while (*piVar1 != 0xf);
      }
      iVar3 = iVar3 + 1;
      local_c = local_c + 10;
    } while (iVar3 < DAT_009994fc);
  }
  return 1;
}
