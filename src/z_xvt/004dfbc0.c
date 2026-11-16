/* SPDX-License-Identifier: MIT */
/* z_xvt 004dfbc0 */
HFILE *FUN_004be160(char *param_1, void *param_2);
char *_fgets(char *buffer, int size, HFILE *stream);
void FUN_004be1f0(HFILE *stream);
int _fscanf(HFILE *stream, const char *format, ...);
void FUN_004d0430(char *param_1);
extern void *DAT_0052a330;

int __cdecl FUN_004dfbc0(char *param_1)
{
  HFILE *_File;
  char *pcVar1;
  int iVar2;
  int local_204;
  char local_200[256];
  char local_100[256];
  
  _File = FUN_004be160(param_1, &DAT_0052a330);
  if (_File == (HFILE *)0x0) {
    return 0;
  }
  pcVar1 = _fgets(local_200, 0xff, _File);
  if (pcVar1 == (char *)0x0) {
    FUN_004be1f0(_File);
    return 1;
  }
  while (1) {
    iVar2 = _fscanf(_File, "%s %s %d\n", local_200, local_100, &local_204);
    if (iVar2 == -1) {
      FUN_004be1f0(_File);
      return 1;
    }
    if (iVar2 != 3) break;
    FUN_004d0430(local_100);
  }
  FUN_004be1f0(_File);
  return 0;
}
