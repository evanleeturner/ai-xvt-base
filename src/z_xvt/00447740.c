/* SPDX-License-Identifier: MIT */
/* z_xvt 00447740 */
extern int DAT_00732e54;
extern void *DAT_006d4988;
extern byte DAT_0052b0b4[];
extern int DAT_0071ecf4;
extern byte DAT_0071b757[];

uint __cdecl FUN_004479a0(int param_1, uint param_2, uint param_3, uint param_4);
int __cdecl FUN_00443ee0(uint param_1);
void __cdecl FUN_004a1d70(uint param_1, int param_2);

void __cdecl FUN_00447740(int param_1, uint param_2)
{
  int iVar1;
  short *psVar2;
  byte bVar3;
  byte bVar4;
  char cVar5;
  int bVar6;
  uint uVar7;
  uint uVar8;
  short sVar9;

  DAT_00732e54 = *(int *)(*(int *)((char *)DAT_006d4988 + 0x1f + param_1 * 0x23) + 0xa7);
  bVar3 = *(byte *)(param_2 + 0x22b + DAT_00732e54);
  iVar1 = param_2 + DAT_00732e54;
  sVar9 = 0;
  bVar6 = 0;
  bVar4 = DAT_0052b0b4[param_2 + (uint)*(byte *)(DAT_00732e54 + 4) * 0xef];
  uVar8 = (uint)bVar4;

  if ((bVar3 & 0x7f) == 3) {
    uVar7 = FUN_004479a0(param_1, uVar8, (uint)*(byte *)(iVar1 + 0x229), param_2);
    if (uVar7 == 0xffffffff) {
      if (*(char *)(DAT_00732e54 + 0x346 + uVar8 * 4) != '\0') {
        bVar6 = 1;
      }
    } else {
      sVar9 = 1;
    }
    uVar7 = (uint)(unsigned short)(bVar4 + 1);
    uVar8 = FUN_004479a0(param_1, uVar7, (uint)*(byte *)(DAT_00732e54 + 0x229 + param_2), param_2);
    if (uVar8 != 0xffffffff) {
      sVar9 = sVar9 + 1;
      goto LAB_004478bf;
    }
    cVar5 = *(char *)(DAT_00732e54 + 0x346 + uVar7 * 4);
  } else if ((bVar3 & 0x80) == 0) {
    uVar7 = FUN_004479a0(param_1, uVar8, (uint)*(byte *)(iVar1 + 0x229), param_2);
    if (uVar7 != 0xffffffff) {
      sVar9 = 1;
      goto LAB_004478bf;
    }
    cVar5 = *(char *)(DAT_00732e54 + 0x346 + uVar8 * 4);
  } else {
    uVar7 = (uint)(unsigned short)(bVar4 + 1);
    uVar8 = FUN_004479a0(param_1, uVar7, (uint)*(byte *)(iVar1 + 0x229), param_2);
    if (uVar8 != 0xffffffff) {
      sVar9 = 1;
      goto LAB_004478bf;
    }
    cVar5 = *(char *)(DAT_00732e54 + 0x346 + uVar7 * 4);
  }

  if (cVar5 != '\0') {
    bVar6 = 1;
  }

LAB_004478bf:
  psVar2 = (short *)(DAT_00732e54 + 0x22d + param_2 * 2);
  *psVar2 = *psVar2 + 0x1d8;

  if ((*(int *)((char *)DAT_006d4988 + 0x1b + param_1 * 0x23) == DAT_0071ecf4) && (!bVar6)) {
    uVar8 = FUN_00443ee0((DAT_00732e54 & 0xFFFF0000) | *(byte *)(DAT_00732e54 + 0x229 + DAT_0071b757[DAT_0071ecf4 * 0x5bd]));
    if (sVar9 == 0) {
      FUN_004a1d70((uVar8 & 0xffff) + 0x26, DAT_0071ecf4);
      return;
    }
    if (sVar9 == 1) {
      FUN_004a1d70((uVar8 & 0xffff) + 0x30, DAT_0071ecf4);
      return;
    }
    if (sVar9 == 2) {
      FUN_004a1d70((uVar8 & 0xffff) + 0x3a, DAT_0071ecf4);
    }
  }
  return;
}
