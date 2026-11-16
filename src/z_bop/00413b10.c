/* SPDX-License-Identifier: MIT */
/* z_bop 00413b10 */
extern const char DAT_0051be2c[];

long __cdecl _ftell(HFILE *stream);
int __cdecl _fscanf(HFILE *stream, const char *format, ...);
int __cdecl _fseek(HFILE *stream, long offset, int origin);

int __cdecl FUN_00413b10(HFILE *param_1)
{
  long _Offset;
  int iVar1;
  char local_1;
  
  _Offset = _ftell(param_1);
  iVar1 = _fscanf(param_1, DAT_0051be2c, &local_1);
  if (iVar1 != 1) {
    _fseek(param_1, _Offset, 0);
    return 0;
  }
  if (local_1 == '{') {
    _fseek(param_1, _Offset, 0);
    return 0;
  }
  _fseek(param_1, _Offset, 0);
  return 1;
}
