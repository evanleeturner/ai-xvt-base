/* SPDX-License-Identifier: MIT */
/* z_bop 0046acb0 */
extern char DAT_009d1320;
extern char DAT_009d6820;

int __stdcall FUN_0046acb0(void)
{
  int iVar1;
  char *pcVar2;
  
  iVar1 = 0;
  pcVar2 = (char *)&DAT_009d1320;
  do {
    if (*pcVar2 == '\0') {
      return iVar1;
    }
    pcVar2 = pcVar2 + 0x55;
    iVar1 = iVar1 + 1;
  } while (pcVar2 < &DAT_009d6820);
  return iVar1;
}
