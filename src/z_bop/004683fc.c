/* SPDX-License-Identifier: MIT */
/* z_bop 004683fc */
extern uint DAT_009a1fe8;
extern uint DAT_00a08104;
extern uint DAT_009a73b6;
extern void __stdcall FUN_00403400(void);

uint __fastcall FUN_004683fc(uint param_1, short param_2)
{
  byte bVar1;
  uint in_EAX;
  uint uVar2;
  int iVar3;
  short *psVar4;

  bVar1 = *(byte *)(param_1 + 0x35);
  if (bVar1 > 2) {
    uVar2 = 0;
    goto LAB_00468478;
  }
  uVar2 = *(ushort *)(param_1 + 0x18);
  psVar4 = (short *)(uVar2 * 0x23 + DAT_009a1fe8);
  if (*(char **)((char *)psVar4 + 0x1f) == 0) {
    if (*(char *)((char *)psVar4 + 3) != '\0') {
      param_2 = 1;
    }
  }
  else {
    if (*(char *)((char *)psVar4 + 3) != (char)param_2) {
      param_2 = 1;
    }
    if (**(char **)((char *)psVar4 + 0x1f) != '\x05') {
      param_2 = 1;
    }
  }
  if (*(short *)(DAT_00a08104 + 0x1e5) != 0) {
    param_2 = 1;
  }
  in_EAX = ((uint)DAT_00a08104 & 0xFFFF0000) | (uint)(ushort)*psVar4;
  if (*(short *)(DAT_009a73b6 + 0x1a) != *psVar4) {
    param_2 = 1;
  }
LAB_00468478:
  if ((((bVar1 == 1) || (bVar1 == 2)) &&
      (iVar3 = uVar2 * 0x23 + DAT_009a1fe8, in_EAX = DAT_009a1fe8, *(int *)(iVar3 + 0x1b) == -1)) &&
     (in_EAX = *(uint *)(iVar3 + 0x1f), *(short *)(in_EAX + 0x79) == 0)) {
    param_2 = 1;
  }
  if (param_2 == 0) {
    return in_EAX & 0xffff0000;
  }
  *(uint *)(DAT_00a08104 + 0xb9) = 0;
  *(uint *)(DAT_00a08104 + 0xb5) = *(uint *)(DAT_00a08104 + 0xb9);
  *(uint *)(DAT_00a08104 + 0xb1) = *(uint *)(DAT_00a08104 + 0xb5);
  *(ushort *)(DAT_009a73b6 + 0x18) = 0x8000;
  *(ushort *)(DAT_009a73b6 + 0x1a) = 0;
  *(byte *)(DAT_009a73b6 + 0x1e) = 0;
  FUN_00403400();
  return 1;
}
