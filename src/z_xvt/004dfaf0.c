/* SPDX-License-Identifier: MIT */
/* z_xvt 004dfaf0 */
extern char DAT_0052a330;
extern char s__s__d_00533080[];

void * __cdecl FUN_004be160(char *param_1, char *param_2);
int __cdecl FUN_004be1f0(void *param_1);
char * __cdecl _fgets(char *buffer, int count, void *stream);
int __cdecl _fscanf(void *stream, const char *format, ...);
void __cdecl FUN_004d0360(char *param_1, char *param_2, int param_3, int param_4);

int __cdecl FUN_004dfaf0(char *param_1)
{
  void *_File;
  char *pcVar1;
  int iVar2;
  int local_204;
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
    iVar2 = _fscanf(_File, s__s__d_00533080, local_200, local_100, &local_204);
    if (iVar2 == -1) {
      FUN_004be1f0(_File);
      return 1;
    }
    if (iVar2 != 3) break;
    FUN_004d0360(local_200, local_100, 0, local_204);
  }
  FUN_004be1f0(_File);
  return 0;
}
