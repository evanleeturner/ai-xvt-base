/* SPDX-License-Identifier: MIT */
/* z_bop 004d1400 */
extern int DAT_00acec49;
extern int DAT_00acf149;

int FUN_004d1400(void)
{
  int iVar1;
  char *piVar2;
  int local_val;
  
  iVar1 = 0;
  piVar2 = (char *)&DAT_00acec49;
  do {
    __builtin_memcpy(&local_val, piVar2, 4);
    if (local_val == 1) {
      iVar1 = iVar1 + 1;
    }
    piVar2 = piVar2 + 10;
  } while (piVar2 < (char *)&DAT_00acf149);
  return iVar1;
}
