/* SPDX-License-Identifier: MIT */
/* z_bop 004d9680 */
extern uint DAT_00aa6cf4;
extern uint DAT_0052bf50;
extern int DAT_00aa62d0;
extern uint DAT_00aa62d4;
extern int DAT_00aa62d8;
extern int DAT_00aa62dc;
extern int DAT_00aa62b0;
extern uint DAT_00aa6ae8;
extern uint DAT_00665440;
extern uint *DAT_00aa6118;

int __cdecl FUN_004d9280(int, uint, int, int);
void __cdecl _qsort(void *, uint, uint, int (__cdecl *)(const void *, const void *));
void __stdcall FUN_004cd9d0(void);
int __cdecl LAB_004d95d0(const void *, const void *);

uint __stdcall FUN_004d9680(void)
{
  uint uVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  uint *puVar5;
  int local_4;
  
  if (DAT_00aa6cf4 == 0) {
    return 0xffffffff;
  }
  if (DAT_0052bf50 != 0xffffffff) {
    iVar2 = DAT_0052bf50 * 0x3e;
    local_4 = FUN_004d9280(*(int *)((int)&DAT_00aa62d0 + iVar2),
                           *(uint *)((int)&DAT_00aa62d4 + iVar2),
                           *(int *)((int)&DAT_00aa62d8 + iVar2),
                           *(int *)((int)&DAT_00aa62dc + iVar2));
  }
  _qsort(&DAT_00aa62b0, DAT_00aa6cf4, 0x3e, LAB_004d95d0);
  if (DAT_0052bf50 != 0xffffffff) {
    uVar4 = 0;
    if (DAT_00aa6cf4 != 0) {
      piVar3 = &DAT_00aa62d0;
      do {
        iVar2 = FUN_004d9280(*piVar3, piVar3[1], piVar3[2], piVar3[3]);
        uVar1 = uVar4;
        if (iVar2 == local_4) break;
        piVar3 = (int *)((int)piVar3 + 0x3e);
        uVar4 = uVar4 + 1;
        uVar1 = DAT_0052bf50;
      } while (uVar4 < DAT_00aa6cf4);
      DAT_0052bf50 = uVar1;
      if (uVar4 != DAT_00aa6cf4) {
        return DAT_0052bf50;
      }
    }
    DAT_0052bf50 = 0xffffffff;
    DAT_00aa6ae8 = 0xffffffff;
    puVar5 = &DAT_00665440;
    for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 - 1) {
      *puVar5 = 0;
      puVar5 = puVar5 + 1;
    }
    puVar5 = DAT_00aa6118;
    for (iVar2 = 0x400; iVar2 != 0; iVar2 = iVar2 - 1) {
      *puVar5 = 0;
      puVar5 = puVar5 + 1;
    }
    FUN_004cd9d0();
  }
  return DAT_0052bf50;
}
