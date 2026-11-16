/* SPDX-License-Identifier: MIT */
/* z_xvt 004d9ea0 */
extern int DAT_0081b740;
extern char s_active_u_00532b34[];
extern char s__ulita_u_00532b28[];
extern char s__ulitb_u_00532b1c[];
int __cdecl _sprintf(char *param_1, const char *param_2, ...);
void __cdecl FUN_004d0e60(char *param_1, int param_2, int param_3);

void __cdecl FUN_004d9ea0(int *param_1)
{
  int iVar1;
  int *piVar2;
  int local_1c[7];
  
  piVar2 = local_1c;
  for (iVar1 = 7; iVar1 != 0; iVar1 = iVar1 - 1) {
    *piVar2 = *param_1;
    param_1 = param_1 + 1;
    piVar2 = piVar2 + 1;
  }
  iVar1 = 0;
  do {
    if (local_1c[iVar1] == 1) {
      _sprintf((char *)&DAT_0081b740, s_active_u_00532b34, iVar1 + 1);
      FUN_004d0e60((char *)&DAT_0081b740, 0, 0);
    }
    iVar1 = iVar1 + 1;
  } while (iVar1 < 7);
  iVar1 = 0;
  piVar2 = local_1c;
  do {
    if (*piVar2 == 2) {
      if ((piVar2 != local_1c) && (piVar2 != local_1c + 5)) {
        _sprintf((char *)&DAT_0081b740, s__ulita_u_00532b28, iVar1 + 1, piVar2[-1]);
        FUN_004d0e60((char *)&DAT_0081b740, 0, 0);
      }
      if ((piVar2 != local_1c + 4) && (piVar2 != local_1c + 6)) {
        _sprintf((char *)&DAT_0081b740, s__ulitb_u_00532b1c, iVar1 + 1, piVar2[1]);
        FUN_004d0e60((char *)&DAT_0081b740, 0, 0);
      }
    }
    piVar2 = piVar2 + 1;
    iVar1 = iVar1 + 1;
  } while (piVar2 < local_1c + 7);
  return;
}
