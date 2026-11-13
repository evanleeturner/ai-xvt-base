/* SPDX-License-Identifier: MIT */
/* z_bop 004ddf90 */
extern uint DAT_00aa6cfc;
extern uint DAT_00aa7356;
extern int DAT_00aa734e;
extern uint DAT_00aa7b1f;
extern int DAT_00aa7b13;
extern int DAT_00aa7215;
extern int DAT_00aa7211;
extern uint *DAT_00aa734a;
extern int DAT_00aa735a;

extern uint __stdcall FUN_004d48e0(void);
extern void __stdcall FUN_004d4970(void);

void __stdcall FUN_004ddf90(void)
{
  int iVar1;
  uint uVar2;
  uint *puVar3;
  uint uVar4;
  uint *puVar5;
  int iVar6;
  uint *puVar7;
  uint *puVar9;
  ushort *puVar8;
  ushort *puVar10;
  int local_4;
  
  DAT_00aa6cfc = FUN_004d48e0();
  uVar2 = DAT_00aa7356;
  iVar1 = DAT_00aa734e;
  if (DAT_00aa7b1f == 8) {
    puVar3 = (uint *)(DAT_00aa6cfc + DAT_00aa7b13 * DAT_00aa7215 + DAT_00aa7211);
    puVar5 = DAT_00aa734a;
    iVar6 = DAT_00aa735a;
    if (0 < DAT_00aa735a) {
      do {
        puVar7 = puVar5;
        puVar9 = puVar3;
        for (uVar4 = uVar2 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
          *puVar9 = *puVar7;
          puVar7 = puVar7 + 1;
          puVar9 = puVar9 + 1;
        }
        for (uVar4 = uVar2 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
          *(uchar *)puVar9 = *(uchar *)puVar7;
          puVar7 = (uint *)((int)puVar7 + 1);
          puVar9 = (uint *)((int)puVar9 + 1);
        }
        puVar3 = (uint *)((int)puVar3 + DAT_00aa7b13);
        iVar6 = iVar6 + -1;
        puVar5 = (uint *)((int)puVar5 + iVar1);
      } while (iVar6 != 0);
    }
  }
  else if ((DAT_00aa7b1f == 0x10) &&
          (puVar8 = (ushort *)(DAT_00aa7b13 * DAT_00aa7215 + DAT_00aa7211 * 2 + DAT_00aa6cfc),
          0 < DAT_00aa735a)) {
    local_4 = DAT_00aa735a;
    puVar5 = DAT_00aa734a;
    do {
      uVar4 = uVar2;
      puVar3 = puVar5;
      puVar10 = puVar8;
      if (0 < (int)uVar2) {
        do {
          *puVar10 = *(ushort *)puVar3;
          uVar4 = uVar4 - 1;
          puVar3 = (uint *)((int)puVar3 + 2);
          puVar10 = puVar10 + 1;
        } while (uVar4 != 0);
      }
      puVar5 = (uint *)((int)puVar5 + iVar1 * 2);
      puVar8 = (ushort *)((int)puVar8 + (DAT_00aa7b13 & 0xfffffffe));
      local_4 = local_4 + -1;
    } while (local_4 != 0);
  }
  FUN_004d4970();
  return;
}
