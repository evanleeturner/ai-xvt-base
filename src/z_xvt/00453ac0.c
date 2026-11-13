/* SPDX-License-Identifier: MIT */
/* z_xvt 00453ac0 */
extern uint DAT_005f0928;
extern uint DAT_005f0f00;
extern uint DAT_005f09c4;
extern uint DAT_005f0e24;
extern uint DAT_005f09bc;
extern uint DAT_005f0e28;
extern uint DAT_005f09c0;
extern uint DAT_005f0e2c;
extern uint DAT_005f0e10;
extern uint _DAT_005254e0;
extern uint DAT_005f0920;
extern uint DAT_005f0e32;
extern uint DAT_005f0924;
extern uint DAT_005f0e30;
extern uint DAT_005f0e48;
extern uint DAT_005f0e34;
extern uint DAT_005f09d4;
extern uint DAT_00703c6c;
extern uint DAT_005f09d0;
extern uint DAT_00703c70;
extern uint DAT_005f09b8;
extern uint DAT_00703c84;
extern uint DAT_005f0e44;
extern uint DAT_00525e08;

void FUN_00453ac0(void)
{
  char cVar1;
  uint uVar2;
  uint uVar3;
  char *pcVar4;
  char *pcVar5;
  
  uVar2 = 0xffffffff;
  pcVar4 = (char *)&DAT_005f0928;
  do {
    pcVar5 = pcVar4;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar5 = pcVar4 + 1;
    cVar1 = *pcVar4;
    pcVar4 = pcVar5;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  pcVar4 = pcVar5 + -uVar2;
  pcVar5 = (char *)&DAT_005f0f00;
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(uint *)pcVar5 = *(uint *)pcVar4;
    pcVar4 = pcVar4 + 4;
    pcVar5 = pcVar5 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar5 = *pcVar4;
    pcVar4 = pcVar4 + 1;
    pcVar5 = pcVar5 + 1;
  }
  DAT_005f09c4 = DAT_005f0e24;
  DAT_005f09bc = DAT_005f0e28;
  DAT_005f09c0 = DAT_005f0e2c;
  DAT_005f0e10 = _DAT_005254e0;
  DAT_005f0920 = DAT_005f0e32;
  DAT_005f0924 = DAT_005f0e30;
  DAT_005f0e48 = DAT_005f0e34;
  DAT_005f09d4 = (unsigned short)DAT_00703c6c;
  DAT_005f09d0 = (unsigned short)DAT_00703c70;
  DAT_005f09b8 = (unsigned short)DAT_00703c84;
  DAT_005f0e44 = DAT_00525e08;
  return;
}
