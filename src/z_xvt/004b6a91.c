/* SPDX-License-Identifier: MIT */
/* z_xvt 004b6a91 */
extern unsigned int DAT_0052f1c0;
extern int DAT_00743270[];
void FUN_0045bbe0(void);

unsigned int __cdecl FUN_004b6a91(int *param_1)
{
  int *local_18;
  unsigned int local_14;
  unsigned int local_10;
  unsigned int local_c;
  unsigned int local_8;
  
  if (DAT_0052f1c0 == 0) {
    local_10 = 0;
  }
  else {
    local_8 = 0;
    local_10 = 0;
    local_18 = DAT_00743270;
    for (local_c = 0; local_c < DAT_0052f1c0; local_c = local_c + 1) {
      local_14 = 0;
      if (param_1[1] == 0 || param_1[1] == local_18[1]) {
        local_14 = 1;
        if (param_1[2] == 0 || param_1[2] == local_18[2]) {
          local_14 = 2;
          if ((param_1[4] & local_18[4]) != 0) {
            local_14 = 3;
            if (*param_1 == *local_18) {
              FUN_0045bbe0();
              return local_c;
            }
          }
        }
      }
      if (local_8 < local_14) {
        local_10 = local_c;
        local_8 = local_14;
      }
      local_18 = local_18 + 0x5e;
    }
    FUN_0045bbe0();
  }
  return local_10;
}
