/* SPDX-License-Identifier: MIT */
/* z_bop 00452830 */
void __cdecl FUN_00452830(char *param_1, char *param_2)
{
  char cVar2;

  cVar2 = *param_2;
  while (cVar2 != '\0') {
    param_2 = param_2 + 1;
    cVar2 = *param_2;
  }
  cVar2 = *param_1;
  while (cVar2 != '\0') {
    cVar2 = *param_1;
    param_1 = param_1 + 1;
    *param_2 = cVar2;
    param_2 = param_2 + 1;
    cVar2 = *param_1;
  }
  *param_2 = '\0';
  return;
}
