/* SPDX-License-Identifier: MIT */
/* z_bop 004daba0 */
extern char DAT_00b69d20[];
extern const char s_active_u_0052c05c[];
extern const char s__ulita_u_0052c050[];
extern const char s__ulitb_u_0052c044[];

int _sprintf(char *dst, const char *fmt, ...);
unsigned int FUN_004b5570(char *param_1, int param_2, int param_3);

void __cdecl FUN_004daba0(int *param_1)
{
  int iVar1;
  int iVar2;
  int *piVar3;
  int local_20[8];

  piVar3 = local_20;
  for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 + -1) {
    *piVar3 = *param_1;
    param_1 = param_1 + 1;
    piVar3 = piVar3 + 1;
  }
  iVar2 = 0;
  do {
    if (local_20[iVar2] == 1) {
      _sprintf((char *)&DAT_00b69d20, s_active_u_0052c05c, iVar2 + 1);
      FUN_004b5570((char *)&DAT_00b69d20, 0, 0);
    }
    iVar1 = local_20[7];
    iVar2 = iVar2 + 1;
  } while (iVar2 < 8);
  iVar2 = 0;
  piVar3 = local_20;
  local_20[7] = local_20[6];
  local_20[6] = local_20[5];
  local_20[5] = iVar1;
  do {
    if (*piVar3 == 2) {
      if ((piVar3 != local_20) && (piVar3 != local_20 + 6)) {
        _sprintf((char *)&DAT_00b69d20, s__ulita_u_0052c050, iVar2 + 1, piVar3[-1]);
        FUN_004b5570((char *)&DAT_00b69d20, 0, 0);
      }
      if ((piVar3 != local_20 + 7) && (piVar3 != local_20 + 5)) {
        _sprintf((char *)&DAT_00b69d20, s__ulitb_u_0052c044, iVar2 + 1, piVar3[1]);
        FUN_004b5570((char *)&DAT_00b69d20, 0, 0);
      }
    }
    piVar3 = piVar3 + 1;
    iVar2 = iVar2 + 1;
  } while (piVar3 < local_20 + 8);
  return;
}
