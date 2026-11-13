/* SPDX-License-Identifier: MIT */
/* z_xvt 004f0890 */
extern int DAT_0081b850;
extern int DAT_007579b0;
extern int DAT_00757cee;
extern int DAT_00757e3e;
extern int DAT_00757a28;
extern int DAT_0075788c;
extern int DAT_00757870;

void __stdcall FUN_004cde00(void);

void __stdcall FUN_004f0890(void)
{
  int *piStack_8;
  int iVar6;
  int *piVar2;
  int iVar3;
  int iStack_4;
  int *puVar4;
  int iVar7;
  int iVar5;
  int *puVar1;

  if (DAT_0081b850 != 2) {
    FUN_004cde00();
  }
  piStack_8 = &DAT_007579b0;
  do {
    iVar6 = 0;
    piVar2 = &DAT_00757cee;
    do {
      if ((*piVar2 != 0) && (*piStack_8 == *piVar2)) {
        break;
      }
      piVar2 = (int *)((int)piVar2 + 0x2a);
      iVar6 = iVar6 + 1;
    } while (piVar2 < &DAT_00757e3e);
    
    if (iVar6 == 8) {
      if (DAT_00757a28 > 0) {
        iVar3 = 0;
        iStack_4 = DAT_00757a28;
        iVar6 = *piStack_8;
        puVar4 = &DAT_0075788c;
        do {
          iVar7 = 0;
          do {
            if (((int *)&DAT_00757870)[iVar3 + iVar7] == iVar6) {
              if (iVar7 < 7) {
                iVar5 = 7 - iVar7;
                puVar1 = &((int *)&DAT_00757870)[iVar3 + iVar7];
                do {
                  iVar5 = iVar5 - 1;
                  *puVar1 = puVar1[1];
                  puVar1 = puVar1 + 1;
                } while (iVar5 != 0);
              }
              *puVar4 = 0;
            }
            iVar7 = iVar7 + 1;
          } while (iVar7 < 8);
          puVar4 = puVar4 + 8;
          iVar3 = iVar3 + 8;
          iStack_4 = iStack_4 - 1;
        } while (iStack_4 != 0);
      }
      *piStack_8 = 0;
    }
    piStack_8 = piStack_8 + 1;
    if ((int *)0x7579cf < piStack_8) {
      return;
    }
  } while (1);
}
