/* SPDX-License-Identifier: MIT */
/* z_xvt 00405160 */
extern char DAT_00523574[];

long __cdecl _ftell(HFILE stream);
int __cdecl _fscanf(HFILE stream, const char *format, ...);
int __cdecl _fseek(HFILE stream, long offset, int origin);

void __cdecl FUN_00405160(HFILE param_1)
{
  int bVar1;
  long _Offset;
  int iVar2;
  char local_2;
  char local_1;
  
  _Offset = _ftell(param_1);
  iVar2 = _fscanf(param_1, DAT_00523574, &local_2);
  if ((iVar2 == 1) && (local_2 == ']')) {
    _fseek(param_1, _Offset, 0);
    bVar1 = 1;
  }
  else {
    _fseek(param_1, _Offset, 0);
    bVar1 = 0;
  }
  if (!bVar1) {
    do {
      iVar2 = _fscanf(param_1, DAT_00523574, &local_1);
      if (iVar2 != 1) {
        return;
      }
    } while (local_1 != ',');
  }
  return;
}
