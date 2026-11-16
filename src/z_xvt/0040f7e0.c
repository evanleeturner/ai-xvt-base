/* SPDX-License-Identifier: MIT */
/* z_xvt 0040f7e0 */
/* Prototypes for callees */
uint __cdecl FUN_00419190(ushort param_1, ushort param_2);
uint __cdecl FUN_0040fb70(int param_1);

/* Globals (types taken from the decompile) */
extern uint DAT_006d9d38;
extern int  DAT_006d4974;
extern int  DAT_0073294c;
extern int  DAT_006d4978;
extern int  DAT_007329b0;
extern int  DAT_006d4970;
extern int  DAT_007329b4;
extern int  DAT_0071e6c4;
extern int  DAT_00732f38;
extern int  DAT_00732fe8;
extern int  DAT_0070a180;
extern int  DAT_0070acd8;
extern int  DAT_00703ad8;
extern int  DAT_006d4988;
extern int  DAT_00525568;
extern int  DAT_00523b68;
extern byte DAT_0052686c[];
extern char DAT_00525bf7[];

uint __cdecl FUN_0040f7e0(ushort param_1, ushort param_2)
{
  byte bVar1;
  char cVar2;
  int iVar3;
  int bVar4;
  uint in_EAX;   /* incoming EAX, intentionally not initialized */
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  int iVar11;
  uint uVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  uint uVar16;
  uint local_18;
  int tmp_i;
  short tmp_s0;
  short tmp_s1;
  ushort tmp_us;

  (void)&in_EAX;
  DAT_006d9d38 = 0x20000;
  uVar10 = (uint)DAT_006d4974 - (uint)DAT_0073294c;
  bVar4 = 1;
  if ((int)uVar10 < 0) {
    uVar10 = 0u - uVar10;
  }
  if (0x20000 < (int)uVar10) {
    return in_EAX & 0xffff0000;
  }
  uVar12 = (uint)DAT_006d4978 - (uint)DAT_007329b0;
  if ((int)uVar12 < 0) {
    uVar12 = 0u - uVar12;
  }
  if ((int)uVar12 < 0x20001) {
    uVar5 = (uint)DAT_006d4970 - (uint)DAT_007329b4;
    if ((int)uVar5 < 0) {
      uVar5 = 0u - uVar5;
    }
    if (0x20000 < (int)uVar5) {
      return uVar5 & 0xffff0000;
    }
    if ((uVar12 < uVar10) && (uVar5 < uVar10)) {
      uVar5 = uVar5 >> 2;
      DAT_006d9d38 = (uVar12 >> 2) + uVar5 + uVar10;
    }
    else if ((uVar10 < uVar12) && (uVar5 < uVar12)) {
      uVar5 = uVar5 >> 2;
      DAT_006d9d38 = (uVar10 >> 2) + uVar5 + uVar12;
    }
    else {
      DAT_006d9d38 = (uVar10 >> 2) + (uVar12 >> 2) + uVar5;
    }
    if ((int)DAT_006d9d38 < 0x20001) {
      iVar11 = (int)((uint)DAT_006d4974 - (uint)DAT_0071e6c4);
      if (iVar11 < 0) {
        iVar11 = (int)(0u - (uint)iVar11);
      }
      iVar14 = (int)((uint)DAT_006d4978 - (uint)DAT_00732f38);
      if (iVar14 < 0) {
        iVar14 = (int)(0u - (uint)iVar14);
      }
      iVar15 = (int)((uint)DAT_006d4970 - (uint)DAT_00732fe8);
      if (iVar15 < 0) {
        iVar15 = (int)(0u - (uint)iVar15);
      }
      iVar6 = (int)((uint)DAT_0073294c - (uint)DAT_0070a180);
      if (iVar6 < 0) {
        iVar6 = (int)(0u - (uint)iVar6);
      }
      iVar7 = (int)((uint)DAT_007329b0 - (uint)DAT_0070acd8);
      if (iVar7 < 0) {
        iVar7 = (int)(0u - (uint)iVar7);
      }
      iVar8 = (int)((uint)DAT_007329b4 - (uint)DAT_00703ad8);
      if (iVar8 < 0) {
        iVar8 = (int)(0u - (uint)iVar8);
      }
      iVar13 = (int)((uint)param_2 * 0x23u + (uint)DAT_006d4988);
      bVar1 = *(byte *)(iVar13 + 3);
      uVar10 = (uint)*(ushort *)(DAT_0052686c + (uint)bVar1 * 0x16u);
      cVar2 = *(char *)(iVar13 + 2);
      if ((cVar2 == '\a') || (cVar2 == '\x06')) {
        if (bVar1 != 0x9b) {
          uVar10 = (uint)((int)uVar10 >> 1);
        }
      }
      else if ((cVar2 == '\0') &&
              ((iVar9 = (int)((uint)param_1 * 0x23u + (uint)DAT_006d4988),
                *(char *)(iVar9 + 2) == '\a' || (*(char *)(iVar9 + 2) == '\x06')))) {
        __builtin_memcpy(&tmp_i, (byte *)(iVar13 + 0x1f), 4);
        __builtin_memcpy(&tmp_i, (byte *)((uint)tmp_i + 0xa7), 4);
        iVar3 = tmp_i;
        __builtin_memcpy(&tmp_s0, (byte *)((uint)iVar3 + 0x20b), 2);
        __builtin_memcpy(&tmp_s1, (byte *)((uint)iVar3 + 0x20d), 2);
        if (((int)tmp_s0 + (int)tmp_s1 != 0) &&
           (uVar10 = uVar10 + (uint)((int)uVar10 >> 1), 0x447 < uVar10)) {
          uVar10 = 0x446;
        }
        __builtin_memcpy(&tmp_i, (byte *)(iVar13 + 0x1b), 4);
        if ((((DAT_00525568 != 0) && (tmp_i != -1)) &&
            (*(char *)(iVar9 + 2) == '\x06')) &&
           (local_18 = (uint)*(byte *)(iVar9 + 3), DAT_00525bf7[local_18] == '\0')) {
          switch ((uint)bVar1) {
          case 2:
          case 4:
            break;
          case 3:
          case 6:
          case 8:
            uVar10 = uVar10 * 2u;
            break;
          case 5:
            uVar10 = uVar10 + uVar10 / 2u + (uint)((int)uVar10 >> 2);
            break;
          default:
            uVar10 = uVar10 + uVar10 / 2u;
          }
          bVar4 = 0;
        }
      }
      uVar12 = (uint)iVar11 + (uint)iVar6 + uVar10;
      uVar5 = (uint)iVar14 + (uint)iVar7 + uVar10;
      uVar16 = (uint)iVar15 + (uint)iVar8 + uVar10;
      if ((uVar5 < uVar12) && (uVar16 < uVar12)) {
        uVar12 = (uVar5 >> 2) + (uVar16 >> 2) + uVar12;
      }
      else if ((uVar12 < uVar5) && (uVar16 < uVar5)) {
        uVar12 = (uVar12 >> 2) + (uVar16 >> 2) + uVar5;
      }
      else {
        uVar12 = (uVar12 >> 2) + (uVar5 >> 2) + uVar16;
      }
      if (uVar12 < DAT_006d9d38) {
        return uVar12 & 0xffff0000;
      }
      if (((bVar4) && (0x446 < uVar10)) || (bVar1 == 0x3a)) {
        if (DAT_00523b68 == 0) {
          uVar10 = FUN_00419190(param_1, param_2);
          return uVar10;
        }
        DAT_00523b68 = 0;
        __builtin_memcpy(&tmp_i, (byte *)(iVar13 + 0x1f), 4);
        __builtin_memcpy(&tmp_us, (byte *)((uint)tmp_i + 0x79), 2);
        if (tmp_us < 0x28) {
          uVar10 = FUN_00419190(param_1, param_2);
          return uVar10;
        }
      }
      uVar10 = FUN_0040fb70((int)(((int)uVar10 >> 3) + ((int)uVar10 >> 2)));
      return uVar10;
    }
    return uVar5 & 0xffff0000;
  }
  return (uint)DAT_007329b0 & 0xffff0000;
}
