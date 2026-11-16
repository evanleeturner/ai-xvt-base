/* SPDX-License-Identifier: MIT */
/* z_bop 004f3ea0 */
extern int DAT_00b69e30;
extern int DAT_00aa5df0;
extern int DAT_00aa615e;
extern int DAT_00aa62ae;
extern int DAT_00aa5e68;
extern int DAT_00aa5ccc;
extern int DAT_00aa5cb0;

void __stdcall FUN_004d1400(void);

void __stdcall FUN_004f3ea0(void)
{
  int *local_8;
  int iVar2;
  int *piVar4;
  int iVar3;
  int *puVar5;
  int iVar6;
  int iVar7;
  int *puVar1;
  int local_4;

  if (DAT_00b69e30 != 2) {
    FUN_004d1400();
  }
  local_8 = &DAT_00aa5df0;
  do {
    iVar2 = 0;
    piVar4 = &DAT_00aa615e;
    do {
      if ((*piVar4 != 0) && (*local_8 == *piVar4)) break;
      piVar4 = (int *)((int)piVar4 + 0x2a);
      iVar2 = iVar2 + 1;
    } while (piVar4 < &DAT_00aa62ae);
    if (iVar2 == 8) {
      if (0 < DAT_00aa5e68) {
        puVar5 = &DAT_00aa5ccc;
        iVar3 = 0;
        iVar2 = *local_8;
        local_4 = DAT_00aa5e68;
        do {
          iVar7 = 0;
          do {
            if ((&DAT_00aa5cb0)[iVar3 + iVar7] == iVar2) {
              if (iVar7 < 7) {
                iVar6 = 7 - iVar7;
                puVar1 = &DAT_00aa5cb0 + iVar3 + iVar7;
                do {
                  iVar6 = iVar6 + -1;
                  *puVar1 = puVar1[1];
                  puVar1 = puVar1 + 1;
                } while (iVar6 != 0);
              }
              *puVar5 = 0;
            }
            iVar7 = iVar7 + 1;
          } while (iVar7 < 8);
          puVar5 = puVar5 + 8;
          iVar3 = iVar3 + 8;
          local_4 = local_4 + -1;
        } while (local_4 != 0);
      }
      *local_8 = 0;
    }
    local_8 = local_8 + 1;
    if ((int *)0xaa5e0f < local_8) {
      return;
    }
  } while (1);
}
