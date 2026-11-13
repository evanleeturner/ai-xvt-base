/* SPDX-License-Identifier: MIT */
/* z_xvt 00461db0 */
extern uint DAT_006db594;
extern uint DAT_006fede0;
extern uint DAT_0070acc8;
extern uint *DAT_00525534;
extern uint DAT_0052550c;
extern uint DAT_0052552c;
extern void *PTR_DAT_00527d18;
extern uint _DAT_00527d1c;
extern void *DAT_005f1d24;
extern void *DAT_005f1d20;
extern uint DAT_00527ce8;
extern uint DAT_00527d00;
extern void *FUN_0048f570(void);

void __stdcall FUN_00461db0(void)
{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;

  uVar2 = DAT_006db594;
  uVar1 = 0;
  if (DAT_006db594 != 0) {
    do {
      (&DAT_006fede0)[uVar1] = DAT_0070acc8 * uVar1;
      uVar1 = uVar1 + 1;
    } while (uVar1 < uVar2);
  }
  DAT_00525534 = (uint *)FUN_0048f570();
  uVar1 = DAT_006db594;
  uVar2 = DAT_0052550c;
  if (DAT_0052552c == 0x13) {
    uVar1 = DAT_0070acc8 * DAT_006db594;
    puVar5 = DAT_00525534;
    for (uVar2 = uVar1 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
      *puVar5 = 0;
      puVar5 = puVar5 + 1;
    }
    for (uVar1 = uVar1 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
      *(unsigned char *)puVar5 = 0;
      puVar5 = (uint *)((int)puVar5 + 1);
    }
    PTR_DAT_00527d18 = &DAT_00527ce8;
    _DAT_00527d1c = 10;
    goto LAB_00461eff;
  }
  if (DAT_0052552c == 0x101) {
    uVar4 = 0;
    if ((DAT_0070acc8 * DAT_006db594) / DAT_0052550c != 0) {
      do {
        uVar4 = uVar4 + 1;
        puVar5 = DAT_00525534;
        for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
          *puVar5 = 0;
          puVar5 = puVar5 + 1;
        }
        for (uVar3 = uVar2 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *(unsigned char *)puVar5 = 0;
          puVar5 = (uint *)((int)puVar5 + 1);
        }
      } while (uVar4 < (DAT_0070acc8 * uVar1) / uVar2);
    }
    uVar2 = (DAT_0070acc8 * uVar1) % uVar2;
    if (uVar2 != 0) goto LAB_00461ed5;
  }
  else {
    if (DAT_0052552c != 0x112) {
      uVar1 = DAT_0070acc8 * DAT_006db594;
      puVar5 = DAT_00525534;
      for (uVar2 = uVar1 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
        *puVar5 = 0;
        puVar5 = puVar5 + 1;
      }
      for (uVar1 = uVar1 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
        *(unsigned char *)puVar5 = 0;
        puVar5 = (uint *)((int)puVar5 + 1);
      }
      PTR_DAT_00527d18 = &DAT_00527ce8;
      _DAT_00527d1c = 10;
      goto LAB_00461eff;
    }
    uVar2 = DAT_0070acc8 * DAT_006db594;
LAB_00461ed5:
    puVar5 = DAT_00525534;
    for (uVar1 = uVar2 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
      *puVar5 = 0;
      puVar5 = puVar5 + 1;
    }
    for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
      *(unsigned char *)puVar5 = 0;
      puVar5 = (uint *)((int)puVar5 + 1);
    }
  }
  PTR_DAT_00527d18 = &DAT_00527d00;
  _DAT_00527d1c = 0xc;
LAB_00461eff:
  DAT_005f1d24 = &DAT_0070acc8;
  DAT_005f1d20 = &DAT_006fede0;
  return;
}
