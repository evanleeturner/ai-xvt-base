/* SPDX-License-Identifier: MIT */
/* z_xvt 0049af60 */
extern int DAT_00525568;
extern char DAT_0071b740[];
extern int DAT_0071ecf4;
extern int DAT_0071f720[];
extern int DAT_006db730[];

void __cdecl FUN_0049af60(int param_1)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  int *piVar6;

  if (DAT_00525568 == 0 && DAT_0071b740[param_1 * 0x5bd] != '\0' && param_1 != DAT_0071ecf4) {
    piVar1 = (int *)&DAT_0071f720 + param_1 * 0x7e9;
    iVar3 = 0;
    piVar6 = piVar1;
    if (DAT_006db730[param_1] > 0) {
      do {
        if (*piVar6 == 0 && piVar6[1] == 2) {
          if (DAT_006db730[param_1] != 0 && piVar1 <= piVar6) {
            iVar2 = DAT_006db730[param_1] - 1;
            iVar5 = 0;
            DAT_006db730[param_1] = iVar2;
            piVar4 = piVar1;
            if (iVar2 > 0) {
              do {
                if (piVar6 <= piVar4) {
                  *piVar4 = *(int *)((int)piVar4 + 0x12);
                  piVar4[1] = *(int *)((int)piVar4 + 0x16);
                  piVar4[2] = *(int *)((int)piVar4 + 0x1a);
                  piVar4[3] = *(int *)((int)piVar4 + 0x1e);
                  *(unsigned short *)(piVar4 + 4) = *(unsigned short *)((int)piVar4 + 0x22);
                }
                iVar5 = iVar5 + 1;
                piVar4 = (int *)((int)piVar4 + 0x12);
              } while (iVar5 < DAT_006db730[param_1]);
            }
          }
        } else {
          piVar6 = (int *)((int)piVar6 + 0x12);
          iVar3 = iVar3 + 1;
        }
      } while (iVar3 < DAT_006db730[param_1]);
    }
  }
  return;
}
