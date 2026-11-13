/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f040 */
extern char DAT_006d4870;

void __cdecl FUN_0048f040(undefined1 param_1)
{
  undefined4 *puVar3;
  char cVar2;
  
  puVar3 = (undefined4 *)&DAT_006d4870;
  cVar2 = *(char *)puVar3;
  while (cVar2 != '\0') {
    puVar3 = (undefined4 *)((int)puVar3 + 1);
    cVar2 = *(char *)puVar3;
  }
  *(undefined1 *)puVar3 = param_1;
  *(undefined1 *)((int)puVar3 + 1) = 0;
  return;
}
