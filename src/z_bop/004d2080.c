/* SPDX-License-Identifier: MIT */
/* z_bop 004d2080 */
extern uint DAT_00b53575;
extern int DAT_00b53579;
extern char DAT_00acf580[];
extern char DAT_00acf57f[];
extern char DAT_00acf57e[];
extern uint DAT_00b69a89;
extern int DAT_00b63f91[];
extern int DAT_00acf571[];

uint __cdecl FUN_004d2080(undefined4 param_1, uint param_2, int param_3, int param_4, uint param_5)
{
  int iVar1;
  char cVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  uint uVar7;
  
  uVar3 = DAT_00b53575;
  iVar1 = DAT_00b53579;
  do {
    if (iVar1 == 0) {
      return 0xffffffff;
    }
    iVar4 = uVar3 * 0x210;
    if (DAT_00acf580[iVar4] != '\0') {
      uVar5 = (uint)(unsigned char)DAT_00acf57f[iVar4];
      cVar2 = DAT_00acf57e[iVar4];
      uVar7 = 0;
      if (DAT_00b69a89 != 0) {
        piVar6 = DAT_00b63f91;
        do {
          if (*piVar6 == DAT_00acf571[uVar3 * 0x84]) break;
          piVar6 = piVar6 + 0x8e;
          uVar7 = uVar7 + 1;
        } while (uVar7 < DAT_00b69a89);
      }
      if (uVar7 == param_5) {
        if (param_3 == 0) {
          if (param_4 == 0) {
            if (((cVar2 != '\0') && (cVar2 != '\x02')) && (uVar5 == param_2)) {
              return uVar3;
            }
          }
          else if ((cVar2 == '\x02') && (uVar5 == param_2)) {
            return uVar3;
          }
        }
        else if ((cVar2 == '\0') && (uVar5 == param_2)) {
          return uVar3;
        }
      }
    }
    uVar3 = uVar3 + 1;
    if (0x3ff < uVar3) {
      uVar3 = 0;
    }
    iVar1 = iVar1 + -1;
  } while (1);
}
