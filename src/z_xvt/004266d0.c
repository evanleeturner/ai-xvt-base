/* SPDX-License-Identifier: MIT */
/* z_xvt 004266d0 */
extern int _DAT_006db7bc;
extern int DAT_00525518;
extern int _DAT_006da4fc;
extern int DAT_0070acc8;
extern int DAT_00525534;
extern int DAT_006d4984;
extern int DAT_007091ae;

void __cdecl FUN_004266d0(int *param_1, int *param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int *piVar5;
  int *piVar6;
  int local_18;
  unsigned int local_14;
  int *local_10;
  int local_c;
  int *local_8;

  local_18 = 0;
  piVar4 = (int *)(_DAT_006db7bc * DAT_00525518 + _DAT_006da4fc * DAT_0070acc8 + DAT_00525534);
  if (DAT_006d4984 != 0) {
    do {
      local_14 = (unsigned int)DAT_007091ae;
      while (((unsigned int)param_1 & 3) != 0 && local_14 != 0) {
        local_14--;
        iVar3 = *param_2;
        param_2++;
        *(char *)piVar4 = (char)iVar3;
        piVar4++;
        param_1++;
      }
      iVar3 = (int)(local_14 + ((int)local_14 >> 31 & 3)) >> 2;
      while (iVar3 != 0) {
        local_c = iVar3;
        piVar5 = param_1;
        piVar6 = param_2;
        do {
          if (local_c == 0) break;
          local_c--;
          iVar2 = *piVar6;
          iVar1 = *piVar5;
          piVar5++;
          piVar6++;
        } while (iVar1 == iVar2);
        iVar3 = iVar3 - (local_c + 1);
        piVar5 = piVar4 + iVar3;
        piVar4 = piVar5 + 1;
        *piVar5 = param_2[iVar3];
        local_8 = param_2 + iVar3 + 1;
        local_10 = param_1 + iVar3 + 1;
        iVar3 = local_c;
        param_1 = local_10;
        param_2 = local_8;
        if (local_c != 0) {
          do {
            *piVar4 = *local_8;
            piVar4++;
            local_8++;
            local_10++;
            local_c--;
            iVar3 = local_c;
            param_1 = local_10;
            param_2 = local_8;
            if (local_c == 0) break;
          } while (*local_8 != *local_10);
        }
      }
      for (; (local_14 & 3) != 0; local_14--) {
        iVar3 = *param_2;
        param_2++;
        *(char *)piVar4 = (char)iVar3;
        piVar4++;
        param_1++;
      }
      local_18++;
      piVar4 = (int *)((int)piVar4 + (DAT_0070acc8 - (unsigned int)DAT_007091ae));
    } while (local_18 < (int)(unsigned int)DAT_006d4984);
  }
  return;
}
