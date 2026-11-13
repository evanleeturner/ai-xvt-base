/* SPDX-License-Identifier: MIT */
/* z_bop 00425a70 */
char *_fgets(char *s, int n, void *stream);

int __cdecl FUN_00425a70(void *param_1, char *param_2)
{
  char cVar1;
  char *pcVar2;
  int iVar3;
  unsigned int uVar4;

  do {
    pcVar2 = _fgets(param_2, 0x400, param_1);
    if (pcVar2 == (char *)0x0) {
      return -1;
    }
    param_2[0x3ff] = '\0';
  } while ((*param_2 == '/') && (param_2[1] == '/'));

  uVar4 = 0xffffffff;
  pcVar2 = param_2;
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar2;
    pcVar2 = pcVar2 + 1;
  } while (cVar1 != '\0');

  uVar4 = ~uVar4;
  iVar3 = uVar4 - 1;
  if (param_2[uVar4 - 2] == '\n') {
    param_2[uVar4 - 2] = '\0';
    iVar3 = uVar4 - 2;
  }
  return iVar3;
}
