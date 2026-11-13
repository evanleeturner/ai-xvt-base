/* SPDX-License-Identifier: MIT */
/* z_xvt 004df970 */
extern void * __cdecl FUN_004be160(char *param_1, char *param_2);
extern void __cdecl FUN_004be1f0(void *param_1);
extern void __cdecl FUN_004ddf50(char *param_1, char *param_2);
extern char * __cdecl _fgets(char *s, int n, void *stream);
extern int __cdecl _fscanf(void *stream, const char *format, ...);
extern char DAT_0052a330;
extern char s__s__s_00533078[];

int __cdecl FUN_004df970(char *param_1)
{
  void *_File;
  char *pcVar1;
  int iVar2;
  char local_200[256];
  char local_100[256];

  _File = FUN_004be160(param_1, &DAT_0052a330);
  if (_File == (void *)0x0) {
    return 0;
  }
  pcVar1 = _fgets(local_200, 0xff, _File);
  if (pcVar1 == (char *)0x0) {
    FUN_004be1f0(_File);
    return 1;
  }
  while (1) {
    iVar2 = _fscanf(_File, s__s__s_00533078, local_200, local_100);
    if (iVar2 == -1) {
      FUN_004be1f0(_File);
      return 1;
    }
    if (iVar2 != 2) {
      break;
    }
    FUN_004ddf50(local_200, local_100);
  }
  FUN_004be1f0(_File);
  return 0;
}
