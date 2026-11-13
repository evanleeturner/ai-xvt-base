/* SPDX-License-Identifier: MIT */
/* z_xvt 0049ae70 */
extern int DAT_00525568;
extern int DAT_0071f720;
extern char DAT_0071b740;
extern int DAT_006db730;
extern int DAT_0071ecf4;
extern char DAT_0071e528;

void __stdcall FUN_0049ae70(void)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int *piVar5;
  int *piVar6;
  int *piStack_c;
  char *pcStack_8;
  int iStack_4;
  
  if (DAT_00525568 == 0) {
    iStack_4 = 0;
    piStack_c = &DAT_0071f720;
    pcStack_8 = &DAT_0071b740;
    piVar5 = &DAT_006db730;
    do {
      if (((*pcStack_8 != '\0') && (iStack_4 != DAT_0071ecf4)) &&
         (iVar3 = 0, piVar4 = piStack_c, 0 < *piVar5)) {
        do {
          if ((*piVar4 == 0) && (piVar4[1] == 2)) {
            if ((*piVar5 != 0) && (piStack_c <= piVar4)) {
              iVar1 = *piVar5 - 1;
              iVar2 = 0;
              *piVar5 = iVar1;
              piVar6 = piStack_c;
              if (0 < iVar1) {
                do {
                  if (piVar4 <= piVar6) {
                    *piVar6 = *(int *)((int)piVar6 + 0x12);
                    piVar6[1] = *(int *)((int)piVar6 + 0x16);
                    piVar6[2] = *(int *)((int)piVar6 + 0x1a);
                    piVar6[3] = *(int *)((int)piVar6 + 0x1e);
                    *(unsigned short *)(piVar6 + 4) = *(unsigned short *)((int)piVar6 + 0x22);
                  }
                  iVar2 = iVar2 + 1;
                  piVar6 = (int *)((int)piVar6 + 0x12);
                } while (iVar2 < *piVar5);
              }
            }
          }
          else {
            piVar4 = (int *)((int)piVar4 + 0x12);
            iVar3 = iVar3 + 1;
          }
        } while (iVar3 < *piVar5);
      }
      piVar5 = piVar5 + 1;
      piStack_c = piStack_c + 0x7e9;
      pcStack_8 = pcStack_8 + 0x5bd;
      iStack_4 = iStack_4 + 1;
    } while (pcStack_8 < &DAT_0071e528);
  }
  return;
}
