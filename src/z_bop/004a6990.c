/* SPDX-License-Identifier: MIT */
/* z_bop 004a6990 */
/* Prototypes for every callee (types inferred from the decompile; no struct evidence was supplied). */
int       FUN_004ae2a0(uint, uint);
int       FUN_004adc40(uint);
uint      FUN_004aeec0(uint, uint);
byte      FUN_00484e10(uint, uint);
ushort    FUN_00459750(uint);
uint      FUN_004ae5a0(uint, uint);
int       FUN_004ae640(uint, uint);
int       FUN_004ae6e0(uint, uint);
long long FUN_0046a3c0(short);
long long FUN_0046a6d0(short);
void      FUN_004037b0(byte *, uint, int, int);
uint      FUN_00425cb0(void);
void      FUN_0042e4d0(uint, uint, int);
int       FUN_004aedf0(uint, uint);
void      FUN_0042fb70(int, uint, uint, uint, uint, uint);

/* Globals (placeholders kept). */
extern byte  *DAT_00a08104;
extern byte  *DAT_009a1fe8;
extern ushort DAT_00a08294;
extern int    iRam00a05da8;
extern int    DAT_00a00870[];
extern int    DAT_00a00874[];
extern char   DAT_009d6954;
extern byte   DAT_009d6941;
extern short  DAT_009d6950;
extern int    DAT_009d6944;
extern byte   DAT_009d8b74;
extern byte   DAT_009d8b75;
extern uint   DAT_009d8a58;
extern int    DAT_009d8a54;
extern int    DAT_009d8b60;
extern int    DAT_009ecc34;
extern byte   DAT_009e9699;
extern byte   DAT_009e9670;
extern byte   DAT_009ec480;

uint FUN_004a6990(ushort param_1, short param_2, uint param_3, uint param_4)
{
  char cVar2;
  byte bVar3;
  ushort uVar4;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  short sVar13;
  uint uVar14;
  ushort uVar15;
  int iVar16;
  byte *puVar17;
  long long lVar18;
  long long lVar19;
  int local_34;
  uint local_2c;
  int local_28;
  byte *puVar1;
  byte *e1;
  byte *e2;
  byte *pp;
  byte *pq;
  uint t4;
  uint t32;
  ushort t16;
  ushort t16b;

  uVar15 = (ushort)(param_2 - 1);
  uVar6 = (uint)uVar15;
  cVar2 = *(char *)(DAT_00a08104 + uVar6 + 0x33c);
  if (cVar2 == '\0') {
    return param_3;
  }
  if (cVar2 != -1) {
    goto LAB_004a6a53;
  }
  if (*(char *)(DAT_009a1fe8 + (uint)param_1 * 0x23 + 3) == '6') {
    if ((int)uVar6 < 0) {
      iVar7 = 0;
    }
    else {
      uVar14 = uVar6;
      if (iRam00a05da8 <= (int)uVar6) {
        uVar14 = (uint)(iRam00a05da8 - 1);
      }
      iVar7 = DAT_00a00874[uVar14 + 0x154e];
    }
    if (iVar7 == 7) goto LAB_004a6a53;
  }
  return param_3;

LAB_004a6a53:
  if (param_3 == 0) {
    param_3 = 1;
  }
  iVar7 = (int)((uint)param_1 * 0x23);
  if ((DAT_00a08294 == 0xe) && (*(char *)(DAT_009a1fe8 + 3 + iVar7) == '6')) {
    if ((int)uVar6 < 0) {
      iVar8 = 0;
    }
    else {
      uVar14 = uVar6;
      if (iRam00a05da8 <= (int)uVar6) {
        uVar14 = (uint)(iRam00a05da8 - 1);
      }
      iVar8 = DAT_00a00874[uVar14 + 0x154e];
    }
    if (iVar8 == 7) {
      bVar3 = *(byte *)(DAT_009a1fe8 + 3 + iVar7);
      if (bVar3 < 0x49) {
        local_34 = DAT_00a00870[(uint)bVar3 * 0x65];
      }
      else {
        local_34 = FUN_004adc40((uint)bVar3);
      }
      iVar8 = 0;
      iVar16 = 0;
      if (0 < local_34) {
        do {
          bVar3 = *(byte *)(DAT_009a1fe8 + 3 + iVar7);
          uVar14 = (uint)bVar3;
          if (bVar3 < 0x49) {
            if (iVar8 < 0) {
              iVar9 = 0;
            }
            else {
              iVar9 = iVar8;
              if (DAT_00a00870[uVar14 * 0x65] <= iVar8) {
                iVar9 = DAT_00a00870[uVar14 * 0x65] - 1;
              }
              iVar9 = DAT_00a00874[(uint)iVar9 + uVar14 * 0x65];
            }
          }
          else {
            iVar9 = FUN_004ae2a0(uVar14, (uint)iVar8);
          }
          if ((iVar9 == 8) && (*(char *)(DAT_00a08104 + 0x33c + iVar8) != '\0')) {
            iVar16 = iVar16 + 1;
          }
          iVar8 = iVar8 + 1;
        } while (iVar8 < local_34);
      }
      if (iVar16 != 0) {
        return param_3;
      }
      __builtin_memcpy(&t32, DAT_00a08104 + 0x211, 4);
      if (t32 != 0) {
        return param_3;
      }
      __builtin_memcpy(&t32, DAT_00a08104 + 0x215, 4);
      if (t32 != 0) {
        return param_3;
      }
      e1 = DAT_009a1fe8 + (param_4 & 0xffff) * 0x23;
      cVar2 = *(char *)(e1 + 3);
      if (cVar2 == '(') {
        __builtin_memcpy(&uVar14, DAT_00a08104 + 0xcb, 4);
        __builtin_memcpy(&t4, DAT_00a08104 + 0xc3, 4);
        t32 = (((uint)*(byte *)(DAT_00a08104 + uVar6 + 0x33c) * 0x10 +
                (uVar14 / 100) * (uint)-5) - t4);
      }
      else {
        if (cVar2 != '\x03') {
          return param_3;
        }
        __builtin_memcpy(&pp, e1 + 0x1f, 4);
        __builtin_memcpy(&pq, pp + 0xa9, 4);
        if (*(char *)(pq + 7) != '\x03') {
          return param_3;
        }
        __builtin_memcpy(&uVar14, DAT_00a08104 + 0xcb, 4);
        __builtin_memcpy(&t4, DAT_00a08104 + 0xc3, 4);
        t32 = ((uint)*(byte *)(DAT_00a08104 + uVar6 + 0x33c) * 0x10 - t4);
      }
      param_3 = t32 + uVar14;
    }
  }
  uVar14 = (uint)*(byte *)(DAT_00a08104 + uVar6 + 0x33c);
  puVar1 = DAT_00a08104 + uVar6 + 0x33c;
  if (param_3 < uVar14 * 0x10) {
    iVar7 = (int)(uVar14 * 0x10 - param_3) >> 4;
    if (iVar7 == 0) {
      iVar7 = 1;
    }
    param_3 = 0;
    *puVar1 = (byte)iVar7;
  }
  else {
    *puVar1 = 0;
    param_3 = param_3 - uVar14 * 0x10;
    if ((DAT_00a08294 == 0xe) && (DAT_009d6954 == '\0')) {
      iVar7 = (int)((uint)param_1 * 0x23);
      bVar3 = *(byte *)(DAT_009a1fe8 + 3 + iVar7);
      uVar14 = (uint)bVar3;
      if (bVar3 < 0x49) {
        if ((int)uVar6 < 0) {
          iVar8 = 0;
        }
        else {
          uVar10 = uVar6;
          if (DAT_00a00870[uVar14 * 0x65] <= (int)uVar6) {
            uVar10 = (uint)(DAT_00a00870[uVar14 * 0x65] - 1);
          }
          iVar8 = DAT_00a00874[uVar14 * 0x65 + uVar10];
        }
      }
      else {
        iVar8 = FUN_004ae2a0(uVar14, uVar6);
      }
      if (iVar8 == 8) {
        bVar3 = *(byte *)(DAT_009a1fe8 + 3 + iVar7);
        if (bVar3 < 0x49) {
          local_34 = DAT_00a00870[(uint)bVar3 * 0x65];
        }
        else {
          local_34 = FUN_004adc40((uint)bVar3);
        }
        uVar14 = 0;
        iVar8 = 0;
        if (0 < local_34) {
          do {
            if (uVar6 != uVar14) {
              bVar3 = *(byte *)(DAT_009a1fe8 + 3 + iVar7);
              uVar10 = (uint)bVar3;
              if (bVar3 < 0x49) {
                if ((int)uVar14 < 0) {
                  iVar16 = 0;
                }
                else {
                  uVar11 = uVar14;
                  if (DAT_00a00870[uVar10 * 0x65] <= (int)uVar14) {
                    uVar11 = (uint)(DAT_00a00870[uVar10 * 0x65] - 1);
                  }
                  iVar16 = DAT_00a00874[uVar11 + uVar10 * 0x65];
                }
              }
              else {
                iVar16 = FUN_004ae2a0(uVar10, uVar14);
              }
              if ((iVar16 == 8) && (*(char *)(DAT_00a08104 + 0x33c + uVar14) != '\0')) {
                iVar8 = iVar8 + 1;
              }
            }
            uVar14 = uVar14 + 1;
          } while ((int)uVar14 < local_34);
        }
        if (iVar8 == 0) {
          t32 = 0;
          __builtin_memcpy(DAT_00a08104 + 0x215, &t32, 4);
          __builtin_memcpy(&t32, DAT_00a08104 + 0x215, 4);
          __builtin_memcpy(DAT_00a08104 + 0x211, &t32, 4);
        }
      }
    }
    uVar14 = (uint)param_1;
    iVar7 = (int)(uVar14 * 0x23);
    uVar10 = FUN_004aeec0((uint)*(byte *)(DAT_009a1fe8 + 3 + iVar7), uVar6);
    if (uVar10 != 0) {
      puVar17 = &DAT_009e9699;
      *(byte *)(DAT_00a08104 + uVar6 + 0x2d8) = 2;
      do {
        __builtin_memcpy(&t16, puVar17 - 0xf, 2);
        __builtin_memcpy(&t16b, puVar17, 2);
        if (((*(char *)(puVar17 - 0x19) != '\0') &&
             (t16 == param_1)) && (t16b == uVar15)) {
          bVar3 = *(byte *)(DAT_009a1fe8 + 3 + iVar7);
          if (bVar3 < 0x49) {
            iVar8 = DAT_00a00870[(uint)bVar3 * 0x65];
          }
          else {
            iVar8 = FUN_004adc40((uint)bVar3);
          }
          do {
            __builtin_memcpy(&uVar4, puVar17, 2);
            t16 = (ushort)(uVar4 + 1);
            __builtin_memcpy(puVar17, &t16, 2);
            if (iVar8 <= (int)(uint)(ushort)(uVar4 + 1)) {
              t16 = 0;
              __builtin_memcpy(puVar17, &t16, 2);
            }
            __builtin_memcpy(&t16, puVar17, 2);
            if (*(char *)(DAT_00a08104 + (uint)t16 + 0x2d8) == '\0') {
              if (FUN_00484e10((uint)*(byte *)(DAT_009a1fe8 + 3 + iVar7), (uint)t16) != 0) {
                break;
              }
            }
            __builtin_memcpy(&t16, puVar17, 2);
          } while (t16 != uVar15);
        }
        puVar17 = puVar17 + 0x5bd;
      } while (puVar17 < &DAT_009ec480 + 1);
      bVar3 = DAT_009d8b75;
      if (DAT_009d6941 != '\0') {
        DAT_009d6950 = (short)(DAT_009d6950 + 1);
        iVar8 = (int)((uint)DAT_009d6944 + 0x32);
        if (*(char *)(DAT_00a08104 + uVar6 + 0x30a) != '\0') {
          iVar8 = (int)((uint)DAT_009d6944 + 100);
        }
        DAT_009d6944 = iVar8;
        bVar3 = (byte)(DAT_009d8b75 + 2);
        if (0x3b < (byte)(DAT_009d8b75 + 2)) {
          DAT_009d8b74 = (byte)(DAT_009d8b74 + 1);
          bVar3 = (byte)(DAT_009d8b75 - 0x3a);
        }
      }
      DAT_009d8b75 = bVar3;
      uVar15 = FUN_00459750(0xd);
      if (uVar15 != 0xffff) {
        uVar10 = (uint)uVar15;
        iVar8 = (int)(uVar10 * 0x23);
        e1 = DAT_009a1fe8 + iVar7;
        e2 = DAT_009a1fe8 + iVar8;
        __builtin_memcpy(&t32, e1 + 4, 4);
        __builtin_memcpy(e2 + 4, &t32, 4);
        __builtin_memcpy(&t32, e1 + 8, 4);
        __builtin_memcpy(e2 + 8, &t32, 4);
        __builtin_memcpy(&t32, e1 + 0xc, 4);
        __builtin_memcpy(e2 + 0xc, &t32, 4);
        local_2c = FUN_004ae5a0((uint)*(byte *)(DAT_009a1fe8 + 3 + iVar7), uVar6);
        iVar16 = FUN_004ae640((uint)*(byte *)(DAT_009a1fe8 + 3 + iVar7), uVar6);
        local_28 = FUN_004ae6e0((uint)*(byte *)(DAT_009a1fe8 + 3 + iVar7), uVar6);
        if ((DAT_009d6941 != '\0') &&
            (bVar3 = *(byte *)(DAT_00a08104 + uVar6 + 0x30a), bVar3 != 0)) {
          sVar13 = (short)((uint)bVar3 * (uint)-0x100);
          lVar18 = FUN_0046a3c0(sVar13);
          lVar19 = FUN_0046a6d0(sVar13);
          iVar9 = (int)((uint)(int)(short)lVar19 * local_2c +
                        (0u - (uint)(int)(short)lVar18) * (uint)local_28);
          if (0x3fffffff < iVar9) {
            iVar9 = 0x3fffffff;
          }
          if (iVar9 < -0x3fffffff) {
            iVar9 = -0x3fff0000;
          }
          local_28 = (int)((uint)(int)(short)lVar18 * local_2c +
                           (uint)(int)(short)lVar19 * (uint)local_28);
          if (0x3fffffff < local_28) {
            local_28 = 0x3fffffff;
          }
          if (local_28 < -0x3fffffff) {
            local_28 = -0x3fff0000;
          }
          local_28 = local_28 >> 0xf;
          local_2c = (uint)(iVar9 >> 0xf) & 0xffff;
        }
        DAT_009d8a58 = local_2c;
        DAT_009d8a54 = local_28;
        DAT_009d8b60 = (int)(0u - (uint)iVar16);
        FUN_004037b0(DAT_009a1fe8 + iVar7, local_2c, local_28, DAT_009d8b60);
        e2 = DAT_009a1fe8 + iVar8;
        __builtin_memcpy(&t32, e2 + 4, 4);
        t32 = t32 + DAT_009d8a58;
        __builtin_memcpy(e2 + 4, &t32, 4);
        __builtin_memcpy(&t32, e2 + 8, 4);
        t32 = t32 + (uint)DAT_009d8a54;
        __builtin_memcpy(e2 + 8, &t32, 4);
        __builtin_memcpy(&t32, e2 + 0xc, 4);
        t32 = t32 + (uint)DAT_009d8b60;
        __builtin_memcpy(e2 + 0xc, &t32, 4);
        *(byte *)(DAT_009a1fe8 + 3 + iVar8) = 0x81;
        *(byte *)(DAT_009a1fe8 + 2 + iVar8) = 0xd;
        __builtin_memcpy(&pp, e2 + 0x1f, 4);
        *pp = 5;
        *(byte *)(DAT_009a1fe8 + 0x19 + iVar8) = 2;
        t16 = 0;
        __builtin_memcpy(&pp, e2 + 0x1f, 4);
        __builtin_memcpy(pp + 0x83, &t16, 2);
        __builtin_memcpy(&pp, e2 + 0x1f, 4);
        __builtin_memcpy(pp + 0x81, &t16, 2);
        __builtin_memcpy(&pp, e2 + 0x1f, 4);
        e1 = DAT_009a1fe8 + iVar7;
        __builtin_memcpy(&pq, e1 + 0x1f, 4);
        __builtin_memcpy(&t16, pq + 0x79, 2);
        __builtin_memcpy(pp + 0x79, &t16, 2);
        __builtin_memcpy(&t16, e1 + 0x12, 2);
        __builtin_memcpy(e2 + 0x12, &t16, 2);
        __builtin_memcpy(&t16, e1 + 0x10, 2);
        __builtin_memcpy(e2 + 0x10, &t16, 2);
        t16 = 0;
        __builtin_memcpy(e2 + 0x14, &t16, 2);
        __builtin_memcpy(&pp, e2 + 0x1f, 4);
        *(byte *)(pp + 0x92) = 1;
        __builtin_memcpy(&pp, e2 + 0x1f, 4);
        *(byte *)(pp + 0x8b) = *(byte *)(pp + 0x92);
        iVar16 = DAT_009ecc34;
        uVar11 = FUN_00425cb0();
        FUN_0042e4d0((uVar11 & 3) + 0x16, uVar10, iVar16);
        iVar16 = FUN_004aedf0((uint)*(byte *)(DAT_009a1fe8 + 3 + iVar7), uVar6);
        iVar16 = iVar16 >> 9;
        if (0xff < iVar16) {
          iVar16 = 0xff;
        }
        __builtin_memcpy(&pp, e2 + 0x1f, 4);
        *(char *)(pp + 1) = (char)iVar16;
      }
      __builtin_memcpy(&t32, &DAT_009e9670 + (uint)DAT_009ecc34 * 0x5bd, 4);
      if (t32 == (param_4 & 0xffff)) {
        uVar10 = (uint)*(byte *)(DAT_009a1fe8 + 3 + iVar7);
        if (uVar10 < 0x49) {
          if ((int)uVar6 < 0) {
            uVar12 = 0;
          }
          else {
            if (DAT_00a00870[uVar10 * 0x65] <= (int)uVar6) {
              uVar6 = (uint)(DAT_00a00870[uVar10 * 0x65] - 1);
            }
            uVar12 = (uint)DAT_00a00874[uVar6 + uVar10 * 0x65];
          }
        }
        else {
          uVar12 = (uint)FUN_004ae2a0(uVar10, uVar6);
        }
        switch (uVar12) {
        case 4:
        case 5:
        case 0x15:
          FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 2, uVar14, 0x4000);
          break;
        case 8:
          FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 3, uVar14, 0xffff);
          break;
        case 10:
        case 0x16:
          FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 1, uVar14, 0xffff);
          break;
        case 0xb:
        case 0xc:
        case 0x17:
        case 0x18:
          FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 4, uVar14, 0xffff);
        }
      }
    }
  }
  return param_3;
}
