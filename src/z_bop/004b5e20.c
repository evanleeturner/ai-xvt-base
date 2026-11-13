/* SPDX-License-Identifier: MIT */
/* z_bop 004b5e20 */
extern void __cdecl FUN_004d6480(int *param_1, int *param_2);
extern uint __cdecl FUN_004d64e0(int *param_1);
extern void __cdecl FUN_004b5fe0(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7);
extern void __cdecl FUN_004b6340(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7);
extern int DAT_00aa7b1f;
extern int DAT_00aa7b13;
extern int DAT_00aa6cfc;

uint __cdecl FUN_004b5e20(int *param_1, int param_2, int param_3)
{
  int local_24;
  int local_20;
  int local_1c;
  int local_18;
  int local_10;
  int local_c;
  int local_8 = 0;
  int local_4 = 0;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  uint uVar2;
  char *puVar1;

  if (param_1 == (int *)0x0) {
    return 0;
  }
  local_24 = param_2;
  local_20 = param_3;
  local_1c = *param_1 + param_2 - 1;
  local_18 = param_1[1] + param_3 - 1;
  FUN_004d6480(&local_10, &local_24);
  uVar2 = FUN_004d64e0(&local_24);
  if (local_24 <= local_1c) {
    if (local_18 < local_20) {
      return uVar2;
    }
    local_10 = local_24 - local_10;
    local_c = local_20 - local_1c;
    iVar3 = *param_1;
    iVar4 = ((local_1c - local_10) - local_8) + iVar3;
    iVar7 = ((param_1[1] - local_c) - local_4) + local_18;
    if (param_1[2] == 0) {
      if (DAT_00aa7b1f == 8) {
        iVar3 = iVar3 * local_c + param_1[4] + local_10;
        local_10 = (local_c + param_3) * DAT_00aa7b13 + param_2 + DAT_00aa6cfc + local_10;
        if (0 < iVar7) {
          do {
            iVar5 = 0;
            if (0 < iVar4) {
              do {
                puVar1 = (char *)(iVar3 + iVar5);
                iVar5 = iVar5 + 1;
                *(char *)(local_10 - 1 + iVar5) = *puVar1;
              } while (iVar5 < iVar4);
            }
            iVar3 = iVar3 + *param_1;
            local_10 = local_10 + DAT_00aa7b13;
            iVar7 = iVar7 - 1;
          } while (iVar7 != 0);
        }
      }
      else if (DAT_00aa7b1f == 0x10) {
        iVar5 = iVar3 * local_c + param_1[4] + local_10;
        iVar3 = (local_c + param_3) * DAT_00aa7b13 + (local_10 + param_2) * 2 + DAT_00aa6cfc;
        if (0 < iVar7) {
          do {
            iVar6 = 0;
            if (0 < iVar4) {
              do {
                iVar6 = iVar6 + 1;
                *(short *)(iVar3 - 2 + iVar6 * 2) = (short)param_1[*(unsigned char *)(iVar5 - 1 + iVar6) + 5];
              } while (iVar6 < iVar4);
            }
            iVar5 = iVar5 + *param_1;
            iVar3 = iVar3 + (DAT_00aa7b13 & 0xfffffffe);
            iVar7 = iVar7 - 1;
          } while (iVar7 != 0);
        }
      }
    }
    else if (DAT_00aa7b1f == 8) {
      FUN_004b5fe0(param_1, local_10 + param_2, local_c + param_3, local_10, local_c, iVar4, iVar7);
    }
    else if (DAT_00aa7b1f == 0x10) {
      FUN_004b6340(param_1, local_10 + param_2, local_c + param_3, local_10, local_c, iVar4, iVar7);
    }
  }
  return uVar2;
}
