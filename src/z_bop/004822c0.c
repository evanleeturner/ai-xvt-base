/* SPDX-License-Identifier: MIT */
/* z_bop 004822c0 */
extern uint DAT_009d1308;
extern uint DAT_00a07ce8;
extern uint DAT_009e9670;
extern uint DAT_009a1fe8;
extern uint DAT_00521f2e[];
extern uint DAT_00a08104;

uint __cdecl FUN_00484f80(ushort param_1);

uint __cdecl FUN_004822c0(ushort param_1, short param_2, int param_3)
{
  char cVar1;
  uint in_EAX;
  uint uVar2;
  int iVar3;
  short sVar4;
  short sVar5;
  ushort uVar6;
  uint uVar7;

  uVar7 = param_1;
  sVar4 = (short)DAT_009d1308 + (short)DAT_00a07ce8;
  do {
    do {
      do {
        do {
          do {
            sVar5 = sVar4 - 1;
            if (sVar4 == 0) {
              return (in_EAX & 0xFFFF0000) | 0xFFFF;
            }
            uVar6 = (short)uVar7 + param_2;
            uVar7 = uVar6;
            if (uVar6 < 0x8000) {
              if (DAT_009d1308 + DAT_00a07ce8 == uVar6) {
                uVar7 = 0;
              }
            } else {
              uVar7 = DAT_009d1308 - 1 + (DAT_00a07ce8 & 0xFFFF);
            }
            uVar2 = uVar7 & 0xFFFF;
            in_EAX = param_3 * 0x16F;
            sVar4 = sVar5;
          } while (*(uint *)((int)&DAT_009e9670 + param_3 * 0x5BD) == uVar2);
          iVar3 = DAT_009a1fe8 + uVar2 * 0x23;
          in_EAX = (((uVar2 * 7 >> 8) << 8) | *(unsigned char *)(iVar3 + 3)) & 0xFFFF00FF;
        } while ((short)in_EAX == 0 ||
                 ((in_EAX = 0), (DAT_00521f2e[*(unsigned char *)(iVar3 + 3) * 0x1A] & 1) == 0));
        in_EAX = 0;
        if (*(int *)(iVar3 + 0x1F) == 0) goto LAB_004823c8;
        in_EAX = (((uint)*(int *)(iVar3 + 0x1F) >> 8) << 8) | *(unsigned char *)(iVar3 + 2);
      } while (*(char *)(iVar3 + 2) == '\r');
      in_EAX = FUN_00484f80((ushort)uVar7);
    } while ((char)in_EAX != '\0');
    in_EAX = *(uint *)(DAT_009a1fe8 + 0x1F + uVar2 * 0x23);
    if (*(char *)in_EAX != '\0') break;
    DAT_00a08104 = *(uint *)(in_EAX + 0xA9);
    cVar1 = *(char *)(DAT_00a08104 + 7);
    in_EAX = ((uint)DAT_00a08104 >> 8) << 8 | cVar1;
  } while (cVar1 == '\x03' || cVar1 == '\x04');
LAB_004823c8:
  return (in_EAX & 0xFFFF0000) | uVar7;
}
