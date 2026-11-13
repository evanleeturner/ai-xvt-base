/* SPDX-License-Identifier: MIT */
/* z_xvt 00482d90 */
extern int DAT_0071b730;
extern int DAT_0071ecf4;
extern int DAT_006d4988;

uint __cdecl FUN_00482d90(ushort param_1, short param_2)
{
  byte bVar1;
  int iVar2;
  int iVar3;
  ushort uVar5;
  uint uVar4_val;
  int iVar6;
  ushort uVar7;
  byte abStack_c[12];
  
  iVar3 = 0;
  iVar2 = *(int *)(*(int *)(*(int *)((int)&DAT_0071b730 + DAT_0071ecf4 * 0x5bd) * 0x23 + 0x1f + DAT_006d4988) + 0xa7);
  do {
    abStack_c[*(byte *)(iVar3 + 0x293 + iVar2)] = (byte)iVar3;
    iVar3 = iVar3 + 1;
  } while (iVar3 < 10);
  
  uVar7 = 0xffff;
  iVar6 = 0;
  uVar5 = (ushort)((uint)iVar3 >> 0x10);
  
  do {
    bVar1 = abStack_c[iVar6];
    if (*(short *)(iVar2 + 0x29e + (uint)bVar1 * 2) == 0) {
      if ((uint)param_1 == (uint)bVar1) {
        if (param_2 != -1) {
          do {
            uVar4_val = ((uint)uVar5 << 16) | (uint)param_1;
            uVar4_val >>= 8;
            if (9 < iVar6 + 1) {
              iVar3 = 0;
              do {
                if (*(short *)(iVar2 + 0x29e + (uint)abStack_c[iVar3] * 2) != 0) {
                  return ((uVar4_val << 8) | abStack_c[iVar3]) & 0xffff00ff;
                }
                iVar3 = iVar3 + 1;
              } while (iVar3 < 10);
              iVar3 = 0;
              do {
                if (*(short *)(iVar2 + 0x29e + (uint)abStack_c[iVar3] * 2) == 0) {
                  return ((uVar4_val << 8) | abStack_c[iVar3]) & 0xffff00ff;
                }
                iVar3 = iVar3 + 1;
              } while (iVar3 < 10);
              return ((uint)uVar5 << 16) | (uint)param_1;
            }
            iVar3 = iVar6 + 1;
            iVar6 = iVar6 + 1;
          } while (*(short *)(iVar2 + 0x29e + (uint)abStack_c[iVar3] * 2) != 0);
          return ((uVar4_val << 8) | abStack_c[iVar3]) & 0xffff00ff;
        }
        if (uVar7 != 0xffff) {
          return ((uint)uVar5 << 16) | (uint)uVar7;
        }
        iVar3 = 9;
        do {
          if (*(short *)(iVar2 + 0x29e + (uint)abStack_c[iVar3] * 2) != 0) {
            return ((((uint)iVar3 >> 8) << 8) | abStack_c[iVar3]) & 0xffff00ff;
          }
          iVar3 = iVar3 + -1;
        } while (-1 < iVar3);
        return ((((uint)iVar3 >> 8) << 8) | abStack_c[9]) & 0xffff00ff;
      }
      uVar7 = (ushort)bVar1;
    }
    iVar6 = iVar6 + 1;
    if (9 < iVar6) {
      return (uint)uVar5 << 0x10;
    }
  } while (1);
}
