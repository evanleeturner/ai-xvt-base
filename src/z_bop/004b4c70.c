/* SPDX-License-Identifier: MIT */
/* z_bop 004b4c70 */
extern int DAT_00aa7201;
void FUN_004b4bf0(char *param_1);

void __stdcall FUN_004b4c70(void)
{
  int iVar1;
  
  if (DAT_00aa7201 != 0) {
    iVar1 = 0x87bc;
    do {
      FUN_004b4bf0((char *)(DAT_00aa7201 + iVar1));
      iVar1 = iVar1 + -0x44;
    } while (-1 < iVar1);
  }
  return;
}
