/* SPDX-License-Identifier: MIT */
/* z_xvt 004a68c0 */
extern int DAT_0052ae2c;
extern int *DAT_006572cc;

void __cdecl FUN_0048f5c0(char *param_1, char param_2, short param_3);

void __cdecl FUN_004a68c0(char *param_1, int param_2, int param_3)
{
  int iVar1;
  char *pcVar2;
  char *pcVar3;
  char acStack_400[1024];

  if (DAT_0052ae2c == 0) {
    FUN_0048f5c0(param_1, (char)param_2, (short)param_3);
    return;
  }
  if (DAT_006572cc != 0) {
    if (param_2 < param_2 + param_3) {
      iVar1 = param_3;
      pcVar2 = acStack_400 + param_2 * 4;
      pcVar3 = param_1 + param_2 * 3;
      do {
        *pcVar2 = *pcVar3 << 2;
        pcVar2[1] = pcVar3[1] << 2;
        iVar1 = iVar1 - 1;
        pcVar2[2] = pcVar3[2] << 2;
        pcVar2 = pcVar2 + 4;
        pcVar3 = pcVar3 + 3;
      } while (iVar1 != 0);
    }
    (*(void (__stdcall **)(int *, int, int, int, char *))(*DAT_006572cc + 0x18))(DAT_006572cc, 0, param_2, param_3, acStack_400);
    return;
  }
  FUN_0048f5c0(param_1, (char)param_2, (short)param_3);
  return;
}
