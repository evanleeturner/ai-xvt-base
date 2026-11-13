/* SPDX-License-Identifier: MIT */
/* z_bop 00427720 */
extern int DAT_009ecc34;
extern char DAT_00a08610[];
extern char DAT_009e9a77[];
extern char DAT_009e9a78[];

void __cdecl FUN_004a9b60(void);
void __cdecl FUN_004aba50(void);
void __cdecl FUN_00440da0(unsigned int param_1, int param_2);
void __cdecl FUN_004abe50(void);
int __cdecl FUN_004aba40(void);
void __cdecl FUN_004ac260(void);
void __cdecl FUN_004abee0(void);
void __cdecl FUN_004a9b70(void);

int __cdecl FUN_00427720(unsigned int param_1, int param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;

  iVar3 = DAT_009ecc34;
  if ((param_2 == DAT_009ecc34) && (DAT_00a08610[param_1 * 0x24] == '\0')) {
    return 0;
  }
  iVar1 = param_2 * 0x5bd;
  if ((unsigned char)DAT_009e9a77[iVar1] != param_1) {
    DAT_009e9a77[iVar1] = (char)param_1;
    if (param_2 == iVar3) {
      FUN_004a9b60();
      FUN_004aba50();
      FUN_00440da0(param_1, param_2);
      FUN_004abe50();
      iVar2 = FUN_004aba40();
      iVar3 = iVar2;
      if (0 < iVar2) {
        do {
          FUN_004abe50();
          iVar3 = iVar3 - 1;
        } while (iVar3 != 0);
      }
      FUN_004ac260();
      FUN_004abee0();
      FUN_004ac260();
      if (0 < iVar2) {
        do {
          FUN_004aba50();
          iVar2 = iVar2 - 1;
        } while (iVar2 != 0);
      }
      FUN_004a9b70();
    }
    DAT_009e9a78[iVar1] = (char)param_1;
    return 1;
  }
  return 1;
}
