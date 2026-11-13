/* SPDX-License-Identifier: MIT */
/* z_bop 004b1a90 */
void __cdecl FUN_004b1810(int *param_1);
void __cdecl FUN_004b1890(int *param_1);

void __cdecl FUN_004b1a90(int *param_1, unsigned int param_2)
{
  int iVar1;
  unsigned int uVar2;
  unsigned int uVar4;
  unsigned int uVar6;
  void *puVar3;
  void *puVar5;

  FUN_004b1810(param_1);
  puVar5 = (void *)param_1[0x17];
  iVar1 = param_1[9];
  if (iVar1 == 8) {
    uVar4 = 0;
    if (param_1[4] != 0) {
      do {
        uVar6 = param_1[3];
        uVar4 = uVar4 + 1;
        puVar3 = puVar5;
        for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
          *(unsigned int *)puVar3 = param_2;
          puVar3 = (void *)((unsigned int)puVar3 + 4);
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
          *(unsigned char *)puVar3 = (unsigned char)param_2;
          puVar3 = (void *)((unsigned int)puVar3 + 1);
        }
        puVar5 = (void *)((unsigned int)puVar5 + param_1[6]);
      } while (uVar4 < (unsigned int)param_1[4]);
    }
  }
  else if (iVar1 == 0x10) {
    uVar4 = 0;
    if (param_1[4] != 0) {
      do {
        uVar6 = 0;
        puVar3 = puVar5;
        if (param_1[3] != 0) {
          do {
            *(unsigned short *)puVar3 = (unsigned short)param_2;
            uVar6 = uVar6 + 1;
            puVar3 = (void *)((unsigned int)puVar3 + 2);
          } while (uVar6 < (unsigned int)param_1[3]);
        }
        puVar5 = (void *)((unsigned int)puVar5 + param_1[6]);
        uVar4 = uVar4 + 1;
      } while (uVar4 < (unsigned int)param_1[4]);
    }
  }
  else if ((iVar1 == 0x20) && (uVar4 = 0, param_1[4] != 0)) {
    do {
      uVar6 = 0;
      puVar3 = puVar5;
      if (param_1[3] != 0) {
        do {
          *(unsigned int *)puVar3 = param_2;
          uVar6 = uVar6 + 1;
          puVar3 = (void *)((unsigned int)puVar3 + 4);
        } while (uVar6 < (unsigned int)param_1[3]);
      }
      puVar5 = (void *)((unsigned int)puVar5 + param_1[6]);
      uVar4 = uVar4 + 1;
    } while (uVar4 < (unsigned int)param_1[4]);
  }
  FUN_004b1890(param_1);
  return;
}
