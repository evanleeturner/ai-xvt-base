/* SPDX-License-Identifier: MIT */
/* z_xvt 004b6bba */
extern uint DAT_0052f1c8;
extern int DAT_00742830;
extern void FUN_0045bbe0(void);

uint __cdecl FUN_004b6bba(int *param_1)
{
  uint uVar1;
  uint local_18;
  uint local_14;
  uint local_10;
  int *local_c;
  uint local_8;
  
  if (DAT_0052f1c8 == 0) {
    local_10 = 0;
  }
  else {
    local_8 = 0;
    local_10 = 0;
    local_c = &DAT_00742830;
    for (local_18 = 0; local_18 < DAT_0052f1c8; local_18 = local_18 + 1) {
      local_14 = 0;
      uVar1 = local_14;
      if ((local_c[1] == param_1[1]) && (uVar1 = 1, *local_c == *param_1)) {
        uVar1 = 2;
        if (*param_1 == 1) {
          if (((local_c[2] == param_1[2]) && (local_c[3] == param_1[3])) &&
             (local_c[4] == param_1[4])) {
            FUN_0045bbe0();
            return local_18;
          }
        }
        else {
          if (*param_1 != 2) {
            FUN_0045bbe0();
            return local_18;
          }
          if ((uint)param_1[0xb] <= (uint)local_c[0xb]) {
            local_14 = 3;
            uVar1 = 3;
            if (((local_c[2] == param_1[2]) && (local_c[3] == param_1[3])) &&
               (uVar1 = local_14, local_c[4] == param_1[4])) {
              FUN_0045bbe0();
              return local_18;
            }
          }
        }
      }
      local_14 = uVar1;
      if (local_8 < local_14) {
        local_10 = local_18;
        local_8 = local_14;
      }
      local_c = local_c + 0x29;
    }
    FUN_0045bbe0();
  }
  return local_10;
}
