/* SPDX-License-Identifier: MIT */
/* z_bop 004b7a10 */
extern int DAT_00aa7205;
extern int DAT_00aa7201;

void __cdecl FUN_004b7a10(int param_1)
{
  int iVar1;
  int iVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  
  if ((-1 < param_1) && (param_1 < DAT_00aa7205)) {
    if (param_1 < DAT_00aa7205 + -1) {
      iVar2 = param_1 * 0x44;
      do {
        puVar4 = (undefined4 *)(DAT_00aa7201 + iVar2);
        param_1 = param_1 + 1;
        iVar2 = iVar2 + 0x44;
        puVar3 = puVar4 + 0x11;
        for (iVar1 = 0x11; iVar1 != 0; iVar1 = iVar1 + -1) {
          *puVar4 = *puVar3;
          puVar3 = puVar3 + 1;
          puVar4 = puVar4 + 1;
        }
      } while (param_1 < DAT_00aa7205 + -1);
    }
    DAT_00aa7205 = DAT_00aa7205 + -1;
  }
  return;
}
