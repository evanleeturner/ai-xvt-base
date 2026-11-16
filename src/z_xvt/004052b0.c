/* SPDX-License-Identifier: MIT */
/* z_xvt 004052b0 */
extern char DAT_00523574[];

long _ftell(HFILE stream);
int _fscanf(HFILE stream, const char *format, ...);
int _fseek(HFILE stream, long offset, int origin);

int __cdecl FUN_004052b0(HFILE param_1)
{
  long _Offset;
  int iVar1;
  char local_1;
  
  _Offset = _ftell(param_1);
  iVar1 = _fscanf(param_1, DAT_00523574, &local_1);
  if (iVar1 != 1) {
    _fseek(param_1, _Offset, 0);
    return 0;
  }
  if (local_1 == '"') {
    _fseek(param_1, _Offset, 0);
    return 0;
  }
  _fseek(param_1, _Offset, 0);
  return 1;
}
