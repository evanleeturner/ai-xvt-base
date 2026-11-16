/* SPDX-License-Identifier: MIT */
/* z_xvt 00481340 */
extern char DAT_00703ca0;
extern char DAT_007091a0;
extern int DAT_00732a40[];

int _strncmp(const char *s1, const char *s2, unsigned int n);

int __cdecl FUN_00481340(char *param_1)
{
  int iVar1;
  char *_Str1;
  int iVar2;
  
  iVar2 = 0;
  _Str1 = (char *)&DAT_00703ca0;
  do {
    iVar1 = _strncmp(_Str1, param_1, 0x50);
    if (iVar1 == 0) break;
    _Str1 = _Str1 + 0x55;
    iVar2 = iVar2 + 1;
  } while (_Str1 < &DAT_007091a0);
  return DAT_00732a40[iVar2];
}
