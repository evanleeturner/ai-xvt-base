/* SPDX-License-Identifier: MIT */
/* z_bop 004ab890 */
extern int DAT_00527ea8;
extern void **DAT_0066ddd8;

void __cdecl FUN_004ac7e0(unsigned char *param_1, char param_2, short param_3);

void __cdecl FUN_004ab890(unsigned char *param_1, int param_2, int param_3)
{
  int iVar1;
  unsigned char *pcVar2;
  unsigned char *pcVar3;
  unsigned char acStack_400[1024];

  if (DAT_00527ea8 == 0) {
    FUN_004ac7e0(param_1, (char)param_2, (short)param_3);
    return;
  }
  if (DAT_0066ddd8 != (void **)0) {
    if (param_2 < param_2 + param_3) {
      iVar1 = param_3;
      pcVar2 = acStack_400 + param_2 * 4;
      pcVar3 = param_1 + param_2 * 3;
      do {
        *pcVar2 = *pcVar3 << 2;
        pcVar2[1] = pcVar3[1] << 2;
        iVar1 = iVar1 + -1;
        pcVar2[2] = pcVar3[2] << 2;
        pcVar2 = pcVar2 + 4;
        pcVar3 = pcVar3 + 3;
      } while (iVar1 != 0);
    }
    (*(void (__stdcall **)(void *, int, int, int, unsigned char *))(*DAT_0066ddd8 + 0x18))(DAT_0066ddd8, 0, param_2, param_3, acStack_400);
    return;
  }
  FUN_004ac7e0(param_1, (char)param_2, (short)param_3);
  return;
}
