/* SPDX-License-Identifier: MIT */
/* z_xvt 004d2190 */
extern int DAT_00758830;
extern int DAT_0075964b;   /* unaligned global (0x75964b), read via memcpy */
extern int DAT_00759657;   /* unaligned global (0x759657), read via memcpy */
extern int DAT_0075dc46;   /* unaligned global (0x75dc46), read via memcpy */

void __cdecl FUN_004dd000(int *param_1, int *param_2);
uint __cdecl FUN_004dd0b0(int *param_1);
uint __cdecl FUN_004df890(uint param_1);
void __cdecl FUN_004d23e0(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7, char param_8);
void __cdecl FUN_004d2700(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7, short param_8);

uint __cdecl FUN_004d2190(int *param_1, int param_2, int param_3, uint param_4, int param_5)
{
  uint uVar2;
  int iVar1;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  /* local_20..local_14: rect {left, top, right(local_18), bottom(local_14)} */
  int local_20[4];
  /* local_10..local_4: result of FUN_004dd000 {local_10, local_c, local_8, local_4} */
  int local_10[4];
  int g_59657;
  int g_5964b;
  int g_5dc46;

  if (param_1 == (int *)0x0) {
    return 0;
  }
  local_20[0] = param_2;
  local_20[1] = param_3;
  local_20[2] = (int)((unsigned int)*param_1 + (unsigned int)param_2 - 1u);
  local_20[3] = (int)((unsigned int)param_1[1] + (unsigned int)param_3 - 1u);
  FUN_004dd000(local_10, local_20);
  uVar2 = FUN_004dd0b0(local_20);
  if (local_20[0] <= local_20[2]) {
    if (local_20[3] < local_20[1]) {
      return uVar2;
    }
    local_10[0] = (int)((unsigned int)local_20[0] - (unsigned int)local_10[0]);
    local_10[1] = (int)((unsigned int)local_20[1] - (unsigned int)local_10[1]);
    iVar1 = (int)((unsigned int)(int)((unsigned int)(int)((unsigned int)local_20[2] - (unsigned int)local_10[0]) - (unsigned int)local_10[2]) + (unsigned int)*param_1);
    iVar5 = (int)((unsigned int)(int)((unsigned int)(int)((unsigned int)param_1[1] - (unsigned int)local_10[1]) - (unsigned int)local_10[3]) + (unsigned int)local_20[3]);
    __builtin_memcpy(&g_59657, &DAT_00759657, 4);
    __builtin_memcpy(&g_5964b, &DAT_0075964b, 4);
    if (param_1[2] == 0) {
      if (g_59657 == 8) {
        iVar3 = (int)((unsigned int)(int)((unsigned int)local_10[1] * (unsigned int)*param_1) + (unsigned int)param_1[4] + (unsigned int)local_10[0]);
        iVar4 = (int)((unsigned int)(int)((unsigned int)(int)((unsigned int)param_3 + (unsigned int)local_10[1]) * (unsigned int)g_5964b) + (unsigned int)DAT_00758830 + (unsigned int)local_10[0] + (unsigned int)param_2);
        if (0 < iVar5) {
          do {
            iVar6 = 0;
            if (0 < iVar1) {
              do {
                if (*(char *)(iVar3 + iVar6) != '\0') {
                  *(char *)(iVar4 + iVar6) = (char)param_4;
                }
                iVar6 = iVar6 + 1;
              } while (iVar6 < iVar1);
            }
            iVar3 = (int)((unsigned int)iVar3 + (unsigned int)*param_1);
            iVar4 = (int)((unsigned int)iVar4 + (unsigned int)g_5964b);
            iVar5 = iVar5 - 1;
          } while (iVar5 != 0);
        }
      }
      else if (g_59657 == 0x10) {
        __builtin_memcpy(&g_5dc46, &DAT_0075dc46, 4);
        if ((param_5 != 0) && (g_5dc46 != 0)) {
          param_4 = FUN_004df890(param_4);
        }
        iVar3 = (int)((unsigned int)(int)((unsigned int)*param_1 * (unsigned int)local_10[1]) + (unsigned int)param_1[4] + (unsigned int)local_10[0]);
        iVar4 = (int)((unsigned int)(int)((unsigned int)(int)((unsigned int)param_3 + (unsigned int)local_10[1]) * (unsigned int)g_5964b) + (unsigned int)(int)((unsigned int)(int)((unsigned int)local_10[0] + (unsigned int)param_2) * 2u) + (unsigned int)DAT_00758830);
        if (0 < iVar5) {
          do {
            iVar6 = 0;
            if (0 < iVar1) {
              do {
                if (*(char *)(iVar3 + iVar6) != '\0') {
                  *(short *)(iVar4 + iVar6 * 2) = (short)param_4;
                }
                iVar6 = iVar6 + 1;
              } while (iVar6 < iVar1);
            }
            iVar3 = (int)((unsigned int)iVar3 + (unsigned int)*param_1);
            iVar4 = (int)((unsigned int)iVar4 + ((unsigned int)g_5964b & 0xfffffffeu));
            iVar5 = iVar5 - 1;
          } while (iVar5 != 0);
        }
      }
    }
    else if (g_59657 == 8) {
      FUN_004d23e0(param_1, (int)((unsigned int)local_10[0] + (unsigned int)param_2),
                   (int)((unsigned int)param_3 + (unsigned int)local_10[1]), local_10[0], local_10[1],
                   iVar1, iVar5, (char)param_4);
    }
    else if (g_59657 == 0x10) {
      __builtin_memcpy(&g_5dc46, &DAT_0075dc46, 4);
      if ((param_5 != 0) && (g_5dc46 != 0)) {
        param_4 = FUN_004df890(param_4);
      }
      FUN_004d2700(param_1, (int)((unsigned int)local_10[0] + (unsigned int)param_2),
                   (int)((unsigned int)param_3 + (unsigned int)local_10[1]), local_10[0], local_10[1],
                   iVar1, iVar5, (short)param_4);
    }
  }
  return uVar2;
}
