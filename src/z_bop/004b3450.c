/* SPDX-License-Identifier: MIT */
/* z_bop 004b3450 */
extern void FUN_004079e0(void);

uint FUN_004b3450(int *param_1, int *param_2, uint param_3)
{
  int iVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint local_4;

  if (param_3 == 0) {
    return 0;
  }
  uVar3 = 0;
  uVar2 = 0;
  local_4 = 0;
  if (param_3 != 0) {
    iVar1 = *param_1;
    do {
      uVar4 = 0;
      if ((*param_2 == iVar1) && (uVar4 = 1, param_2[1] == param_1[1])) {
        uVar4 = 2;
        if (iVar1 == 1) {
          if (((param_1[2] == param_2[2]) && (param_2[3] == param_1[3])) &&
             (param_2[4] == param_1[4])) {
            FUN_004079e0();
            return uVar3;
          }
        }
        else {
          if (iVar1 != 2) {
            FUN_004079e0();
            return uVar3;
          }
          if (*param_2 == 2) {
            uVar4 = 3;
          }
          if (((param_1[2] == param_2[2]) && (param_2[3] == param_1[3])) &&
             ((param_2[4] == param_1[4] && (param_2[0xb] == param_1[0xb])))) {
            FUN_004079e0();
            return uVar3;
          }
        }
      }
      if (uVar2 < uVar4) {
        uVar2 = uVar4;
        local_4 = uVar3;
      }
      param_2 = param_2 + 0x29;
      uVar3 = uVar3 + 1;
    } while (uVar3 < param_3);
  }
  FUN_004079e0();
  return local_4;
}
