/* SPDX-License-Identifier: MIT */
/* z_xvt 004918c0 */
extern int DAT_0065865c;
extern int DAT_006a8814;
extern int DAT_006587a0;

int _strncmp(const char *, const char *, int);

undefined1 __cdecl FUN_004918c0(char *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  char *local_4;
  
  local_4 = (char *)&DAT_0065865c;
  iVar3 = DAT_006a8814 + -1;
  iVar2 = 0;
  while (-1 < iVar3) {
    iVar4 = iVar3 >> 1;
    iVar1 = _strncmp(local_4 + iVar4 * 0x145, param_1, 0x40);
    if (iVar1 == 0) {
      iVar4 = iVar4 + iVar2;
      goto LAB_0049192f;
    }
    if (iVar3 < 1) break;
    if (iVar1 < 0) {
      iVar2 = iVar4 + 1 + iVar2;
      iVar3 = iVar3 - (iVar4 + 1);
      local_4 = local_4 + iVar4 * 0x145 + 0x145;
    }
    else {
      iVar3 = iVar4 + -1;
    }
  }
  iVar4 = -1;
LAB_0049192f:
  if (iVar4 == -1) {
    return 0;
  }
  return (&DAT_006587a0)[iVar4 * 0x145];
}
