/* SPDX-License-Identifier: MIT */
/* z_bop 004b5100 */
extern uint __cdecl FUN_004d64e0(int *param_1);
extern void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
extern void __cdecl FUN_004d6480(int *param_1, int *param_2);
extern int DAT_00aa7b1f;
extern int DAT_00aa7b13;
extern int DAT_00aa6cfc;

uint __cdecl FUN_004b5100(int *param_1, int *param_2, int param_3, int param_4)
{
  uint uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int local_2c = 0;
  int local_20 = 0;
  int local_1c = 0;
  int local_18 = 0;
  int local_14 = 0;
  int local_10 = 0;
  int local_c = 0;
  int local_8 = 0;
  int local_4 = 0;

  if (param_1 == (int *)0x0) {
    return 0;
  }
  FUN_004d6480(&local_20, param_2);
  FUN_004d64a0(&local_20, param_3 - *param_2, param_4 - param_2[1]);
  FUN_004d6480(&local_10, &local_20);
  uVar1 = FUN_004d64e0(&local_20);
  if ((local_20 <= local_18) && (local_1c <= local_14)) {
    local_20 = local_20 - local_10;
    local_1c = local_1c - local_c;
    iVar4 = *param_2;
    local_18 = (((param_2[2] - iVar4) - local_20) - local_8) + 1 + local_18;
    iVar2 = param_2[1];
    local_2c = (((param_2[3] - iVar2) - local_1c) - local_4) + 1 + local_14;
    if (DAT_00aa7b1f == 8) {
      iVar4 = (iVar2 + local_1c) * *param_1 + param_1[4] + local_20 + iVar4;
      iVar2 = (local_1c + param_4) * DAT_00aa7b13 + local_20 + param_3 + DAT_00aa6cfc;
      if (0 < local_2c) {
        do {
          iVar3 = 0;
          if (0 < local_18) {
            do {
              if (*(char *)(iVar4 + iVar3) != '\0') {
                *(char *)(iVar2 + iVar3) = *(char *)(iVar4 + iVar3);
              }
              iVar3 = iVar3 + 1;
            } while (iVar3 < local_18);
          }
          iVar4 = iVar4 + *param_1;
          iVar2 = iVar2 + DAT_00aa7b13;
          local_2c = local_2c + -1;
        } while (local_2c != 0);
        return uVar1;
      }
    }
    else {
      if (DAT_00aa7b1f != 0x10) {
        return uVar1;
      }
      iVar4 = (iVar2 + local_1c) * *param_1 + param_1[4] + local_20 + iVar4;
      iVar2 = (local_1c + param_4) * DAT_00aa7b13 + (local_20 + param_3) * 2 + DAT_00aa6cfc;
      if (0 < local_2c) {
        do {
          iVar3 = 0;
          if (0 < local_18) {
            do {
              if (*(char *)(iVar4 + iVar3) != '\0') {
                *(short *)(iVar2 + iVar3 * 2) = (short)param_1[*(unsigned char *)(iVar4 + iVar3) + 5];
              }
              iVar3 = iVar3 + 1;
            } while (iVar3 < local_18);
          }
          iVar4 = iVar4 + *param_1;
          iVar2 = iVar2 + (DAT_00aa7b13 & 0xfffffffe);
          local_2c = local_2c + -1;
        } while (local_2c != 0);
      }
    }
  }
  return uVar1;
}
