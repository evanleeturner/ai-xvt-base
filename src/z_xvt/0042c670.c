/* SPDX-License-Identifier: MIT */
/* z_xvt 0042c670 */
extern int DAT_0081f7de;
extern int DAT_0071bcb8;
extern int DAT_0081fa9e;
extern int DAT_0081f812;

void __cdecl FUN_0042c670(int param_1)
{
  char *piVar1;
  int iVar2;
  int local_val;
  int target_val;
  int write_val;
  
  piVar1 = (char *)&DAT_0081f7de;
  iVar2 = 0;
  do {
    __builtin_memcpy(&local_val, piVar1, 4);
    __builtin_memcpy(&target_val, (char *)&DAT_0071bcb8 + param_1 * 0x5bd, 4);
    if (local_val == target_val) goto LAB_0042c6a4;
    piVar1 = piVar1 + 0x58;
    iVar2 = iVar2 + 1;
  } while (piVar1 < (char *)&DAT_0081fa9e);
  iVar2 = 0;
LAB_0042c6a4:
  write_val = 1;
  __builtin_memcpy((char *)&DAT_0081f812 + iVar2 * 0x58, &write_val, 4);
  return;
}
