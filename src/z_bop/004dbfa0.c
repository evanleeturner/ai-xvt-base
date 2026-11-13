/* SPDX-License-Identifier: MIT */
/* z_bop 004dbfa0 */
extern int DAT_00aabd11;

int __cdecl FUN_004dbfa0(char *param_1, int param_2)
{
  char cVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  
  if (param_1 == (char *)0x0) {
    return 0;
  }
  if ((-1 < param_2) && (param_2 < 0x100)) {
    iVar2 = (&DAT_00aabd11)[param_2];
    if (iVar2 != 0) {
      iVar3 = 0;
      iVar4 = 0;
      cVar1 = *param_1;
      while (cVar1 != '\0') {
        if (6 < (unsigned char)param_1[iVar4]) {
          iVar3 = iVar3 + (unsigned int)*(unsigned char *)((unsigned char)param_1[iVar4] + 0x504 + iVar2) +
                  (unsigned int)*(unsigned char *)(iVar2 + 0x609);
        }
        iVar4 = iVar4 + 1;
        cVar1 = param_1[iVar4];
      }
      return iVar3 - (unsigned int)*(unsigned char *)(iVar2 + 0x609);
    }
    return 0;
  }
  return 0;
}
