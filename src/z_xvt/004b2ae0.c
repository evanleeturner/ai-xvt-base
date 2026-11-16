/* SPDX-License-Identifier: MIT */
/* z_xvt 004b2ae0 */
#ifndef CONCAT22
#define CONCAT22(a, b) ((uint)(((uint)(ushort)(a) << 16) | (uint)(ushort)(b)))
#endif
#ifndef CONCAT31
#define CONCAT31(a, b) ((uint)(((uint)(a) << 8) | (uint)(byte)(b)))
#endif

extern short DAT_006559b0[];
extern uint DAT_00525518;
extern byte DAT_00525508;
extern ushort DAT_00655810;
extern ushort DAT_00655414;
extern ushort DAT_00655416;
extern ushort DAT_00655418;
extern ushort DAT_00655184;
extern ushort DAT_00655186;
extern ushort DAT_00655188;
extern ushort DAT_00655176;
extern ushort DAT_0065517a;
extern ushort DAT_0065517c;
extern ushort DAT_0065517e;
extern ushort DAT_006551c8;
extern ushort DAT_006551cc;
extern ushort DAT_00655a90;
extern ushort DAT_00655a92;
extern ushort DAT_00655a94;
extern ushort DAT_00655a9a;
extern ushort DAT_00655a9c;
extern ushort DAT_00655a9e;
extern ushort DAT_00655aa0;
extern ushort DAT_00655aa2;
extern ushort DAT_006da538;
extern uint DAT_006da9ec;
extern int DAT_0071ecf4;
extern byte DAT_0071b743[];
extern byte DAT_0071b730[];
extern int DAT_006d4988;
extern byte DAT_006d4a4e;
extern byte DAT_0071f2e2;
extern byte DAT_0052b0b8[];
extern byte DAT_0052b0b9[];
extern byte DAT_00655fda[];
extern byte DAT_00655fd8[];
extern byte DAT_00655fc2[];
extern byte DAT_00655fc0[];
extern byte DAT_00655fce[];
extern byte DAT_00655fcc[];
extern byte DAT_00655faa[];
extern byte DAT_00655fa8[];
extern byte DAT_00655fb6[];
extern byte DAT_00655fb4[];
extern byte DAT_00655b68[];
extern byte DAT_00655b64[];
extern byte DAT_00655b66[];

void FUN_00426a50(uint, ushort, uint, uint, uint, uint, ushort, ushort, uint);
void FUN_0048ee20(char);

void __stdcall FUN_004b2ae0(void)
{
  byte bVar1;
  byte bVar2;
  short sVar3;
  int iVar4;
  short sVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  ushort uVar9;
  uint uVar10;
  uint uVar11;
  ushort uVar12;
  ushort uVar13;
  ushort uVar14;

  uVar12 = 0;
  do {
    sVar3 = DAT_006559b0[uVar12];
    sVar5 = (short)DAT_00525518;
    uVar9 = (ushort)(DAT_00525518 >> 0x10);
    uVar13 = DAT_00655810;
    uVar14 = DAT_00655414;
    switch ((uint)uVar12) {
    case 0:
      if (sVar3 != 0) {
        if (DAT_0071b743[(uint)DAT_0071ecf4 * 0x5bd] != '\0') goto LAB_004b2d85;
        uVar6 = CONCAT22(uVar9, sVar5 * (short)DAT_006da538);
        iVar4 = (uint)DAT_00655184 * 0xc;
        uVar8 = CONCAT22(uVar9, *(ushort *)(DAT_00655fda + iVar4));
        uVar10 = CONCAT22((short)((uint)iVar4 >> 0x10), *(ushort *)(DAT_00655fd8 + iVar4));
        uVar11 = CONCAT22((short)(((uint)DAT_00655184 * 3) >> 0x10), DAT_0065517a);
        uVar7 = CONCAT22(uVar9, DAT_0065517e);
        uVar13 = DAT_00655176;
        uVar14 = DAT_0065517c;
LAB_004b2dd3:
        FUN_00426a50(DAT_006da9ec, (ushort)DAT_00525508, uVar7, uVar11, uVar10, uVar8, uVar13, uVar14, uVar6);
      }
      break;
    case 1:
      if (sVar3 != 0) {
        if (DAT_0071b743[(uint)DAT_0071ecf4 * 0x5bd] != '\0') goto LAB_004b2d85;
        uVar6 = CONCAT22(uVar9, sVar5 * (short)DAT_006da538);
        iVar4 = (uint)DAT_00655184 * 0xc;
        uVar8 = CONCAT22(uVar9, *(ushort *)(DAT_00655fc2 + iVar4));
        uVar10 = CONCAT22((short)((uint)iVar4 >> 0x10), *(ushort *)(DAT_00655fc0 + iVar4));
        uVar11 = CONCAT22((short)(((uint)DAT_00655184 * 3) >> 0x10), DAT_00655aa2);
        uVar7 = CONCAT22(uVar9, DAT_00655a94);
        uVar13 = DAT_00655a9c;
        uVar14 = DAT_00655a90;
        goto LAB_004b2dd3;
      }
      break;
    case 3:
      if ((sVar3 != 0) && (DAT_0071b743[(uint)DAT_0071ecf4 * 0x5bd] == '\0')) {
        uVar6 = CONCAT22(uVar9, sVar5 * (short)DAT_006da538);
        iVar4 = (uint)DAT_00655184 * 0xc;
        uVar8 = CONCAT22(uVar9, *(ushort *)(DAT_00655fce + iVar4));
        uVar10 = CONCAT22((short)((uint)iVar4 >> 0x10), *(ushort *)(DAT_00655fcc + iVar4));
        uVar11 = CONCAT22((short)(((uint)DAT_00655184 * 3) >> 0x10), DAT_00655aa0);
        uVar7 = CONCAT22(uVar9, DAT_00655a92);
        uVar13 = DAT_00655a9e;
        uVar14 = DAT_00655a9a;
        goto LAB_004b2dd3;
      }
      break;
    case 4:
      if (sVar3 != 0) {
        if (DAT_0071b743[(uint)DAT_0071ecf4 * 0x5bd] != '\0') goto LAB_004b2d85;
        uVar6 = CONCAT22(uVar9, sVar5 * (short)DAT_006da538);
        iVar4 = (uint)DAT_00655184 * 0xc;
        uVar8 = CONCAT22(uVar9, *(ushort *)(DAT_00655faa + iVar4));
        uVar10 = CONCAT22((short)((uint)iVar4 >> 0x10), *(ushort *)(DAT_00655fa8 + iVar4));
        uVar11 = CONCAT22((short)(((uint)DAT_00655184 * 3) >> 0x10), DAT_00655418);
        uVar7 = CONCAT22(uVar9, DAT_00655416);
        goto LAB_004b2dd3;
      }
      break;
    case 5:
      if (sVar3 != 0) {
        uVar6 = CONCAT22(uVar9, sVar5 * (short)DAT_006da538);
        iVar4 = (uint)DAT_00655184 * 0xc;
        uVar8 = CONCAT22(uVar9, *(ushort *)(DAT_00655faa + iVar4));
        uVar10 = CONCAT22((short)((uint)iVar4 >> 0x10), *(ushort *)(DAT_00655fa8 + iVar4));
        uVar11 = CONCAT22((short)(((uint)DAT_00655184 * 3) >> 0x10), DAT_00655418);
        uVar7 = CONCAT22(uVar9, DAT_00655416);
        goto LAB_004b2dd3;
      }
      break;
    case 6:
      if ((sVar3 != 0) && (DAT_0071b743[(uint)DAT_0071ecf4 * 0x5bd] != '\0')) {
LAB_004b2d85:
        uVar6 = CONCAT22(uVar9, sVar5 * (short)DAT_006da538);
        iVar4 = (uint)DAT_00655184 * 0xc;
        uVar8 = CONCAT22(uVar9, *(ushort *)(DAT_00655fb6 + iVar4));
        uVar10 = CONCAT22((short)((uint)iVar4 >> 0x10), *(ushort *)(DAT_00655fb4 + iVar4));
        uVar11 = CONCAT22((short)(((uint)DAT_00655184 * 3) >> 0x10), DAT_00655186);
        uVar7 = CONCAT22(uVar9, DAT_006551cc);
        uVar13 = DAT_00655188;
        uVar14 = DAT_006551c8;
        goto LAB_004b2dd3;
      }
    }
    uVar12 = uVar12 + 1;
    if (7 < uVar12) {
      if (DAT_00655184 == 0x78) {
        int iVar_a;
        int iVar_b;
        int iVar_c;
        __builtin_memcpy(&iVar_a, DAT_0071b730 + (uint)DAT_0071ecf4 * 0x5bd, 4);
        __builtin_memcpy(&iVar_b, (byte *)(uint)((uint)iVar_a * 0x23 + 0x1f + (uint)DAT_006d4988), 4);
        __builtin_memcpy(&iVar_c, (byte *)(uint)((uint)iVar_b + 0xa7), 4);
        iVar4 = iVar_c;
        if ((*(byte *)(iVar4 + 0x1db) & 8) != 0) {
          bVar1 = *(byte *)(iVar4 + 4);
          FUN_0048ee20('\0');
          uVar12 = 0;
          iVar4 = (uint)bVar1 * 0xef;
          bVar1 = DAT_0052b0b9[iVar4];
          bVar2 = DAT_0052b0b8[iVar4];
          if ((ushort)((ushort)bVar1 + (ushort)bVar2) != 0) {
            do {
              iVar4 = ((uint)uVar12 + (uint)DAT_00655184) * 0xc;
              if (*(short *)(DAT_00655b68 + iVar4) != 0) {
                uVar6 = CONCAT22((short)(DAT_00525518 >> 0x10),
                                 (short)DAT_00525518 * (short)DAT_006da538);
                uVar8 = CONCAT31(uVar6 >> 8, DAT_006d4a4e) & 0xffff00ff;
                uVar9 = (ushort)(uVar8 >> 0x10);
                FUN_00426a50(DAT_006da9ec, 0,
                             CONCAT22(uVar9, uVar12 * (DAT_0071f2e2 + 1) + 2), 2,
                             CONCAT22((short)((uint)iVar4 >> 0x10),
                                      *(ushort *)(DAT_00655b64 + iVar4)),
                             CONCAT22(uVar9, *(ushort *)(DAT_00655b66 + iVar4)),
                             *(short *)(DAT_00655b68 + iVar4) * (DAT_0071f2e2 + 1),
                             (ushort)uVar8, uVar6);
              }
              uVar12 = uVar12 + 1;
            } while (uVar12 < (ushort)((ushort)bVar1 + (ushort)bVar2));
          }
        }
      }
      return;
    }
  } while (1);
}
