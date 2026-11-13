/* SPDX-License-Identifier: MIT */
/* z_xvt 00480890 */
extern int _strncmp(const char *, const char *, unsigned int);
extern char DAT_00703ca0;
extern char DAT_007091a0;

int __cdecl FUN_00480890(char *param_1)
{
  int iVar1;
  char *_Str1;
  int iVar2;
  
  _Str1 = (char *)&DAT_00703ca0;
  iVar2 = 0;
  do {
    iVar1 = _strncmp(_Str1, param_1, 0x50);
    if (iVar1 == 0) {
      return iVar2;
    }
    _Str1 = _Str1 + 0x55;
    iVar2 = iVar2 + 1;
  } while (_Str1 < &DAT_007091a0);
  return iVar2;
}
