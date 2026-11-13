/* SPDX-License-Identifier: MIT */
/* z_xvt 0048efd0 */
extern char DAT_006d4870;

void __cdecl FUN_0048efd0(char *param_1)
{
  char cVar1;
  char *pcVar2;
  
  pcVar2 = (char *)&DAT_006d4870;
  if (param_1 != (char *)0x0) {
    cVar1 = *param_1;
    while (cVar1 != '\0') {
      cVar1 = *param_1;
      param_1 = param_1 + 1;
      *pcVar2 = cVar1;
      pcVar2 = pcVar2 + 1;
      cVar1 = *param_1;
    }
  }
  *pcVar2 = '\0';
  return;
}
