/* SPDX-License-Identifier: MIT */
/* z_bop 004d9bf0 */
extern int DAT_0052c004;
extern int DAT_00665470;
extern int DAT_00665474;
void * __cdecl FID_conflict__memcpy(void *, const void *, unsigned int);

int __cdecl FUN_004d9bf0(int param_1)
{
  int *piVar1;
  unsigned int uVar2;
  
  uVar2 = 0;
  if (DAT_0052c004 != 0) {
    piVar1 = &DAT_00665470;
    do {
      if (*piVar1 == param_1) {
        FID_conflict__memcpy(&DAT_00665470 + uVar2, &DAT_00665474 + uVar2, (DAT_0052c004 - uVar2) * 4 - 4);
        DAT_0052c004 = DAT_0052c004 - 1;
        return 1;
      }
      piVar1 = piVar1 + 1;
      uVar2 = uVar2 + 1;
    } while (uVar2 < (unsigned int)DAT_0052c004);
  }
  return 0;
}
