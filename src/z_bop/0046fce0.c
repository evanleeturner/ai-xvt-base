/* SPDX-License-Identifier: MIT */
/* z_bop 0046fce0 */
extern uint DAT_00527028;
extern int DAT_0052702c;
extern char DAT_005599b7[];
extern char DAT_005599b6[];
extern uint DAT_0099f6e0;
extern int DAT_00999be8[];
extern int DAT_005599a8[];
extern char DAT_005599b5[];

uint __cdecl FUN_0046fce0(uint param_1, uint param_2, int param_3, int param_4, uint param_5)
{
  uint uVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  
  uVar1 = DAT_00527028;
  iVar4 = DAT_0052702c;
  do {
    if (iVar4 == 0) {
      return 0xffffffff;
    }
    iVar2 = uVar1 * 0x210;
    if (DAT_005599b7[iVar2] != '\0') {
      uVar5 = (uint)(unsigned char)DAT_005599b6[iVar2];
      uVar3 = 0;
      if (DAT_0099f6e0 != 0) {
        piVar6 = DAT_00999be8;
        do {
          if (*piVar6 == DAT_005599a8[uVar1 * 0x84]) break;
          piVar6 = piVar6 + 0x8e;
          uVar3 = uVar3 + 1;
        } while (uVar3 < DAT_0099f6e0);
      }
      if (uVar3 == param_5) {
        if (param_3 == 0) {
          if (param_4 == 0) {
            if (((DAT_005599b5[iVar2] != '\0') && (DAT_005599b5[iVar2] != '\x02')) &&
               (uVar5 == param_2)) {
              return uVar1;
            }
          }
          else if ((DAT_005599b5[iVar2] == '\x02') && (uVar5 == param_2)) {
            return uVar1;
          }
        }
        else if ((DAT_005599b5[iVar2] == '\0') && (uVar5 == param_2)) {
          return uVar1;
        }
      }
    }
    uVar1 = uVar1 + 1;
    if (0x3ff < uVar1) {
      uVar1 = 0;
    }
    iVar4 = iVar4 + -1;
  } while (1);
}
