/* SPDX-License-Identifier: MIT */
/* z_bop 004a9820 */
extern char DAT_009a1ed0;

void __cdecl FUN_004a9820(char *param_1)
{
  char *pcVar3;
  char cVar2;

  pcVar3 = &DAT_009a1ed0;
  cVar2 = DAT_009a1ed0;
  while (cVar2 != '\0') {
    pcVar3 = pcVar3 + 1;
    cVar2 = *pcVar3;
  }
  if (param_1 != (char *)0x0) {
    cVar2 = *param_1;
    while (cVar2 != '\0') {
      *pcVar3 = cVar2;
      pcVar3 = pcVar3 + 1;
      param_1 = param_1 + 1;
      cVar2 = *param_1;
    }
  }
  *pcVar3 = '\0';
  return;
}
