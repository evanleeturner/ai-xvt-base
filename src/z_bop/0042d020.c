/* SPDX-License-Identifier: MIT */
/* z_bop 0042d020 */
void __cdecl FUN_0042d040(char *);
extern char DAT_00a1051c;
extern char DAT_00a5faa4;

void __stdcall FUN_0042d020(void)
{
  char *pcVar1;
  
  pcVar1 = &DAT_00a1051c;
  do {
    FUN_0042d040(pcVar1);
    pcVar1 = pcVar1 + 0x145;
  } while (pcVar1 < &DAT_00a5faa4);
  return;
}
