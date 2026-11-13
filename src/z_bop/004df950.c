/* SPDX-License-Identifier: MIT */
/* z_bop 004df950 */
extern void * __cdecl FUN_004cc4a0(char *param_1, char *param_2);
extern int __cdecl FUN_004cc590(void *param_1);
extern char * __cdecl _fgets(char *ptr, int num, void *stream);
extern int __cdecl _fscanf(void *stream, const char *format, ...);
extern void __cdecl FUN_004b4bf0(char *param_1);
extern char DAT_0051c4bc;
extern char s__s__d_0052c178[];

int __cdecl FUN_004df950(char *param_1)
{
  void *_File;
  char *pcVar1;
  int iVar2;
  int local_204;
  char local_200[256];
  char local_100[256];

  _File = FUN_004cc4a0(param_1, &DAT_0051c4bc);
  if (_File == (void *)0x0) {
    return 0;
  }
  pcVar1 = _fgets(local_200, 0xff, _File);
  if (pcVar1 == (char *)0x0) {
    FUN_004cc590(_File);
    return 1;
  }
  while (1) {
    iVar2 = _fscanf(_File, s__s__d_0052c178, local_200, local_100, &local_204);
    if (iVar2 == -1) {
      FUN_004cc590(_File);
      return 1;
    }
    if (iVar2 == 3) break;
    FUN_004b4bf0(local_100);
  }
  FUN_004cc590(_File);
  return 0;
}
