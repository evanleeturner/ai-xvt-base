/* SPDX-License-Identifier: MIT */
/* z_bop 004b19e0 */
void __cdecl FUN_004b1810(int *param_1);
void __cdecl FUN_004b1890(int *param_1);

void __cdecl FUN_004b19e0(int *param_1, int *param_2, int param_3, int param_4)
{
  unsigned int uVar1;
  unsigned int uVar2;
  unsigned int *puVar3;
  unsigned int uVar4;
  unsigned int *puVar5;
  unsigned int *puVar6;
  unsigned int *local_4;
  
  FUN_004b1810(param_1);
  FUN_004b1810(param_2);
  local_4 = (unsigned int *)param_2[0x17];
  uVar4 = 0;
  puVar3 = (unsigned int *)(((unsigned int)param_1[9] >> 3) * (unsigned int)param_3 + (unsigned int)param_1[6] * (unsigned int)param_4 + (unsigned int)param_1[0x17]);
  uVar1 = ((unsigned int)param_2[9] >> 3) * (unsigned int)param_2[3];
  if (param_2[4] != 0) {
    do {
      uVar4 = uVar4 + 1;
      puVar5 = local_4;
      puVar6 = puVar3;
      for (uVar2 = uVar1 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
      }
      for (uVar2 = uVar1 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
        *(unsigned char *)puVar6 = *(unsigned char *)puVar5;
        puVar5 = (unsigned int *)((int)puVar5 + 1);
        puVar6 = (unsigned int *)((int)puVar6 + 1);
      }
      puVar3 = (unsigned int *)((int)puVar3 + (unsigned int)param_1[6]);
      local_4 = (unsigned int *)((int)local_4 + (unsigned int)param_2[6]);
    } while (uVar4 < (unsigned int)param_2[4]);
  }
  FUN_004b1890(param_1);
  FUN_004b1890(param_2);
  return;
}
