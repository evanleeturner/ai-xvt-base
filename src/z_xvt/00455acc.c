/* SPDX-License-Identifier: MIT */
/* z_xvt 00455acc */
extern unsigned int DAT_006d4988;
extern unsigned int DAT_00732e54;
extern unsigned int DAT_006d9d56;
extern void __stdcall FUN_0042a060(void);

unsigned int __fastcall FUN_00455acc(unsigned int param_1, short param_2)
{
  unsigned char bVar1;
  unsigned int in_EAX;
  unsigned int uVar2;
  int iVar3;
  short *psVar4;

  bVar1 = *(unsigned char *)(param_1 + 0x35);
  if (bVar1 > 2) {
    uVar2 = 0;
    goto LAB_00455b48;
  }
  uVar2 = (unsigned int)*(unsigned short *)(param_1 + 0x18);
  psVar4 = (short *)(uVar2 * 0x23 + DAT_006d4988);
  if (*(char **)((int)psVar4 + 0x1f) == (char *)0x0) {
    if (*(char *)((int)psVar4 + 3) == '\0') {
      param_2 = 1;
    }
  }
  else {
    if (*(char *)((int)psVar4 + 3) == (char)param_2) {
      param_2 = 1;
    }
    if (**(char **)((int)psVar4 + 0x1f) == '\x05') {
      param_2 = 1;
    }
  }
  if (*(short *)(DAT_00732e54 + 0x1df) == 0) {
    param_2 = 1;
  }
  in_EAX = ((unsigned int)DAT_00732e54 & 0xFFFF0000) | (*psVar4 & 0xFFFF);
  if (*(short *)(DAT_006d9d56 + 0x1a) != *psVar4) {
    param_2 = 1;
  }
LAB_00455b48:
  if ((((bVar1 == 1) || (bVar1 == 2)) &&
      (iVar3 = uVar2 * 0x23 + DAT_006d4988, in_EAX = DAT_006d4988, *(int *)(iVar3 + 0x1b) != -1)) &&
     (in_EAX = *(unsigned int *)(iVar3 + 0x1f), *(short *)(in_EAX + 0x79) != 0)) {
    param_2 = 1;
  }
  if (param_2 == 0) {
    return in_EAX & 0xffff0000;
  }
  *(unsigned int *)(DAT_00732e54 + 0xb9) = 0;
  *(unsigned int *)(DAT_00732e54 + 0xb5) = *(unsigned int *)(DAT_00732e54 + 0xb9);
  *(unsigned int *)(DAT_00732e54 + 0xb1) = *(unsigned int *)(DAT_00732e54 + 0xb5);
  *(unsigned short *)(DAT_006d9d56 + 0x18) = 0x8000;
  *(unsigned short *)(DAT_006d9d56 + 0x1a) = 0;
  *(unsigned char *)(DAT_006d9d56 + 0x1e) = 0;
  FUN_0042a060();
  return 1;
}
