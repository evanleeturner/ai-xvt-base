/* SPDX-License-Identifier: MIT */
/* z_bop 004fa420 */
void __stdcall FUN_004d1400(void);

extern int DAT_00b69e30;
extern int DAT_00aa5df0;
extern int DAT_00aa615e;
extern int DAT_00aa62ae;
extern int DAT_00aa5e68;
extern int DAT_00aa5cb0;
extern int DAT_00aa5cb4;
extern int DAT_00aa5ac0;
extern int DAT_00aa5ac4;
extern int DAT_00aa5ccc;
extern int DAT_00aa5adc;

void __stdcall FUN_004fa420(void)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int *local_8;
  int local_4;

  if (DAT_00b69e30 != 2) {
    FUN_004d1400();
  }
  local_8 = &DAT_00aa5df0;
  do {
    iVar5 = 0;
    piVar2 = &DAT_00aa615e;
    iVar1 = *local_8;
    do {
      if (*piVar2 == iVar1) {
        break;
      }
      piVar2 = (int *)((int)piVar2 + 0x2a);
      iVar5 = iVar5 + 1;
    } while (piVar2 < &DAT_00aa62ae);

    if (iVar5 == 8) {
      if (0 < DAT_00aa5e68) {
        iVar5 = 0;
        local_4 = DAT_00aa5e68;
        do {
          iVar4 = 0;
          iVar6 = iVar5;
          do {
            if (*(int *)((int)&DAT_00aa5cb0 + iVar6) == iVar1) {
              if (iVar4 < 7) {
                iVar7 = 7 - iVar4;
                iVar3 = iVar6;
                do {
                  iVar7 = iVar7 - 1;
                  *(int *)((int)&DAT_00aa5cb0 + iVar3) = *(int *)((int)&DAT_00aa5cb4 + iVar3);
                  *(int *)((int)&DAT_00aa5ac0 + iVar3) = *(int *)((int)&DAT_00aa5ac4 + iVar3);
                  iVar3 = iVar3 + 4;
                } while (iVar7 != 0);
              }
              *(int *)((int)&DAT_00aa5ccc + iVar5) = 0;
              *(int *)((int)&DAT_00aa5adc + iVar5) = 0xffffffff;
            }
            iVar6 = iVar6 + 4;
            iVar4 = iVar4 + 1;
          } while (iVar4 < 8);
          iVar5 = iVar5 + 0x20;
          local_4 = local_4 - 1;
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
