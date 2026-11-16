/* SPDX-License-Identifier: MIT */
/* z_xvt 0044cdb0 */
extern unsigned char *DAT_00732e54;
extern unsigned char *DAT_006d9d56;
extern unsigned char *DAT_006d4988;
extern int DAT_006d4980;
extern int DAT_00732a38;
extern int DAT_00732904;
extern ushort DAT_006d9d50;
extern ushort DAT_006d9d70;
extern ushort DAT_006da9f0;
extern ushort DAT_00732e40;
extern ushort DAT_0071b704;
extern uint DAT_006db580;
extern uint DAT_00525cd0[];
extern unsigned char DAT_00525cdc[];
extern char DAT_00525bf7[];
extern ushort DAT_00525a7e[];
extern unsigned char DAT_0052b0b4[];
extern char DAT_00703ca0[];
extern char s_disableldr1pln_00525028[];

/* Calling conventions and return widths of the callees are not confirmed. */
unsigned char FUN_00429a20(uint a);
void FUN_0042a140(uint a, uint b);
uint FUN_004440e0(uint a);
uint FUN_00478320(uint a, uint b);
int FUN_004490f0(uint a);
uint FUN_00449790(uint a, uint b);
uint FUN_00422ab0(uint a);
void FUN_00447740(uint a, uint b);

uint __stdcall FUN_0044cdb0(void)
{
  unsigned char bVar1;
  unsigned char bVar2;
  unsigned char bVar10;
  unsigned char bVar14;
  unsigned char bVar7;
  char cVar3;
  char cVar4;
  ushort uVar8;
  ushort uVar15;
  ushort uVar18;
  ushort uVar19;
  uint uVar13;
  uint uVar20;
  uint uVar9;
  int iVar11;
  int iVar12;
  int iVar16;
  int iVar17;
  int psVar5;
  unsigned char *psVar6;
  unsigned char *rec;
  unsigned char *pu;
  unsigned char *pq;
  char *pcVar22;
  char *pcVar24;
  unsigned char *pbVar21;
  unsigned char *pbVar23;
  unsigned char bVar25;
  ushort local_46;
  ushort local_44;
  ushort uStack_3c;
  ushort local_3a;
  uint local_38;
  uint local_2c;
  uint local_28[10];
  short s16;
  ushort u16;
  ushort u16b;
  uint u32;

  __builtin_memcpy(&s16, DAT_00732e54 + 0x1df, 2);
  if (s16 == 0) {
    return (uint)DAT_00732e54 & 0xffff0000;
  }
  __builtin_memcpy(&uVar19, DAT_006d9d56 + 0x18, 2);
  local_28[0] = local_28[0] & 0xffff0000;
  psVar5 = (int)(uint)uVar19;
  bVar7 = FUN_00429a20((uint)psVar5);
  if ((uint)bVar7 == 0) {
    local_38 = (uint)DAT_00732e54;
    uVar19 = 0;
    if (DAT_00732e54[0x211] != 0) {
      do {
        DAT_00732e54[(uint)uVar19 + 0x216] = 0;
        local_38 = (uint)(ushort)(uVar19 + 1);
        uVar19 = (ushort)(uVar19 + 1);
      } while (local_38 < (uint)DAT_00732e54[0x211]);
    }
  }
  else {
    uVar20 = DAT_00525cd0[DAT_006d9d70];
    if (psVar5 < DAT_00732a38) {
      rec = DAT_006d4988 + psVar5 * 0x23;
      __builtin_memcpy(&pu, rec + 0x1f, 4);
      if (pu != 0) {
        __builtin_memcpy(&u16, pu + 0x79, 2);
        if (0x19 < u16) {
          __builtin_memcpy(&u16, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x10, 2);
          __builtin_memcpy(&u16b, rec + 0x10, 2);
          uVar8 = (ushort)(u16 - u16b);
          if (0x7fff < uVar8) {
            uVar8 = (ushort)(0 - uVar8);
          }
          if (uVar8 < 0x2000) {
            uVar20 = uVar20 - 0x4000;
          }
          else if (uVar8 < 0x5000) {
            uVar20 = uVar20 - 0x2000;
          }
        }
        if ((rec[2] == 5) || (rec[2] == 4)) {
          uVar20 = uVar20 + 0x6000;
        }
      }
    }
    FUN_0042a140((uint)DAT_006d9d50, (uint)psVar5);
    __builtin_memcpy(&u16, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x10, 2);
    uStack_3c = (ushort)(DAT_006da9f0 - u16);
    if (0x7fff < uStack_3c) {
      uStack_3c = (ushort)(0 - uStack_3c);
    }
    __builtin_memcpy(&u16, DAT_00732e54 + 0xd, 2);
    local_3a = (ushort)(DAT_00732e40 - u16);
    if (0x7fff < local_3a) {
      local_3a = (ushort)(0 - local_3a);
    }
    if (((uStack_3c < 0x800) && (local_3a < 0x800)) && (DAT_006db580 < uVar20)) {
      bVar14 = 3;
      if (0x1fff < (int)DAT_006db580) {
        bVar14 = (unsigned char)(((int)DAT_006db580 < 0x4000) + 1);
      }
      local_28[0] = (local_28[0] & 0xffff0000) | (uint)(ushort)DAT_00525cdc[DAT_006d9d70];
    }
    else {
      bVar14 = 0;
    }
    bVar1 = DAT_00732e54[0x211];
    bVar2 = DAT_006d9d56[0xb];
    uVar15 = (ushort)bVar2;
    uVar8 = 0;
    if (bVar1 != 0) {
      do {
        if (bVar14 == 0) {
LAB_0044d037:
          bVar10 = 0;
        }
        else {
          bVar10 = bVar14;
          if ((char)DAT_00732e54[(uint)uVar8 + 0x214] == -0x73) {
            bVar7 = (uVar15 == 0);
            iVar16 = 0xf;
            pcVar22 = DAT_00703ca0 + (uint)uVar15 * 0x55;
            pcVar24 = s_disableldr1pln_00525028;
            do {
              if (iVar16 == 0) break;
              iVar16 = iVar16 + -1;
              bVar7 = (*pcVar22 == *pcVar24);
              pcVar22 = pcVar22 + 1;
              pcVar24 = pcVar24 + 1;
            } while (bVar7);
            if ((!bVar7) || (DAT_006d9d56[0x1e] != 1)) goto LAB_0044d037;
          }
          else if (DAT_006d9d56[0x1e] != 0) {
            iVar11 = 0xf;
            bVar7 = 0;
            iVar16 = 0;
            bVar25 = 1;
            pbVar21 = (unsigned char *)DAT_00703ca0 + (uint)uVar15 * 0x55;
            pbVar23 = (unsigned char *)s_disableldr1pln_00525028;
            do {
              if (iVar11 == 0) break;
              iVar11 = iVar11 + -1;
              bVar7 = (*pbVar21 < *pbVar23);
              bVar25 = (*pbVar21 == *pbVar23);
              pbVar21 = pbVar21 + 1;
              pbVar23 = pbVar23 + 1;
            } while (bVar25);
            if (!bVar25) {
              iVar16 = (1 - (uint)bVar7) - (uint)(bVar7 != 0);
            }
            bVar10 = (unsigned char)((((uint)(iVar16 == 0)) - 1U) & (uint)bVar14);
          }
        }
        uVar18 = (ushort)(uVar8 + 1);
        DAT_00732e54[(uint)uVar8 + 0x216] = bVar10;
        DAT_00732e54[(uint)uVar8 + 0x218] = (unsigned char)local_28[0];
        uVar8 = uVar18;
      } while (uVar18 < bVar1);
    }
    local_38 = (uint)psVar5;
    if ((psVar5 < DAT_006d4980) &&
        (local_38 = FUN_004440e0((uint)uVar19), psVar6 = DAT_00732e54, (char)local_38 == 0)) {
      iVar11 = psVar5 * 0x23;
      rec = DAT_006d4988 + iVar11;
      cVar3 = (char)DAT_006d4988[2 + iVar11];
      if ((cVar3 == 4) || ((cVar3 == 5) || (cVar3 == 3))) {
        cVar3 = 2;
        local_44 = 6;
        local_46 = 0x10;
        local_28[0] = ((uint)(-(int)(DAT_006d9d70 == 2)) & 0x9f12) + 0x31b5a;
      }
      else {
        local_28[0] = 0x18dad;
        local_46 = 2;
        cVar3 = 1;
        local_44 = 1;
      }
      local_38 = 0;
      __builtin_memcpy(&pu, rec + 0x1f, 4);
      __builtin_memcpy(&pq, pu + 0xa7, 4);
      __builtin_memcpy(&s16, pq + 0x20b, 2);
      local_2c = (uint)(int)s16;
      bVar7 = (uVar15 == 0);
      iVar12 = 0xf;
      pcVar22 = DAT_00703ca0 + (uint)uVar15 * 0x55;
      pcVar24 = s_disableldr1pln_00525028;
      do {
        if (iVar12 == 0) break;
        iVar12 = iVar12 + -1;
        bVar7 = (*pcVar22 == *pcVar24);
        pcVar22 = pcVar22 + 1;
        pcVar24 = pcVar24 + 1;
      } while (bVar7);
      if (bVar7) {
        __builtin_memcpy(&u16, pq + 199, 2);
        uVar20 = (uint)u16 / 10;
        __builtin_memcpy(&u16b, pq + 0xc3, 2);
        if ((ushort)uVar20 < u16b) {
          local_2c = local_2c + uVar20;
        }
        uVar20 = 0;
        if (DAT_00732e54[0x228] != 0) {
          do {
            uVar13 = (uint)DAT_00732e54[uVar20 + 0x229];
            if (DAT_00525bf7[uVar13] == cVar3) {
              uVar8 = DAT_00525a7e[uVar13];
              cVar4 = (char)rec[2];
              if ((cVar4 == 4) || (cVar4 == 5)) {
                uVar8 = (ushort)(uVar8 >> 4);
              }
              if (cVar4 == 3) {
                uVar8 = (ushort)(uVar8 >> 2);
              }
              local_38 = local_38 + (uint)uVar8;
            }
            uVar20 = (uint)(ushort)((short)uVar20 + 1);
          } while (uVar20 < (uint)DAT_00732e54[0x228]);
        }
      }
      else {
        __builtin_memcpy(&u16, pq + 199, 2);
        __builtin_memcpy(&u16b, pq + 0xc3, 2);
        local_2c = local_2c + ((uint)u16 - (uint)u16b);
      }
      uVar13 = (uint)DAT_0071b704;
      uVar8 = DAT_0071b704;
      uVar20 = uVar13;
      while ((int)uVar20 < DAT_00732904) {
        iVar17 = (int)(DAT_006d4988 + (uint)uVar8 * 0x23);
        bVar14 = *(unsigned char *)(iVar17 + 3);
        if (0x88 < bVar14) {
          __builtin_memcpy(&pu, (unsigned char *)iVar17 + 0x1f, 4);
          __builtin_memcpy(&pq, pu + 0xa3, 4);
          iVar17 = (int)pq;
          __builtin_memcpy(&u16, (unsigned char *)iVar17 + 4, 2);
          if ((*(char *)(iVar17 + 1) != 0) && (u16 == uVar19)) {
            cVar4 = (char)rec[2];
            uVar15 = DAT_00525a7e[bVar14];
            if ((cVar4 == 4) || (cVar4 == 5)) {
              uVar15 = (ushort)(uVar15 >> 4);
            }
            if (cVar4 == 3) {
              uVar15 = (ushort)(uVar15 >> 2);
            }
            local_38 = local_38 + (uint)uVar15;
          }
        }
        uVar8 = (ushort)(uVar8 + 1);
        uVar20 = (uint)uVar8;
      }
      if (local_38 < local_2c) {
        local_38 = local_38 & 0xffff0000;
        uVar8 = DAT_0071b704;
        while ((int)uVar13 < DAT_00732904) {
          iVar16 = (int)(DAT_006d4988 + (uint)uVar8 * 0x23);
          bVar14 = *(unsigned char *)(iVar16 + 3);
          if ((bVar14 != 0) && (DAT_00525bf7[bVar14] == cVar3)) {
            __builtin_memcpy(&pu, (unsigned char *)iVar16 + 0x1f, 4);
            __builtin_memcpy(&pq, pu + 0xa3, 4);
            iVar16 = (int)pq;
            __builtin_memcpy(&u16, (unsigned char *)iVar16 + 4, 2);
            if ((*(char *)(iVar16 + 1) != 0) && (u16 == uVar19)) {
              local_38 = (local_38 & 0xffff0000) | (uint)(ushort)((ushort)local_38 + 1);
            }
          }
          uVar8 = (ushort)(uVar8 + 1);
          uVar13 = (uint)uVar8;
        }
        if ((((ushort)local_38 < local_46) &&
             (local_38 = (uint)DAT_006d9d56, (char)DAT_006d9d56[0x34] == 0x17)) &&
            (local_38 = (uint)DAT_00732e54, (__builtin_memcpy(&s16, DAT_00732e54 + 0x1e1, 2), s16 == 0))) {
          __builtin_memcpy(&u32, DAT_00732e54 + 0x1d, 4);
          if (u32 == 0) {
            local_38 = (uint)local_44;
            if ((uint)DAT_00732e54[0x78] < local_38) {
              if ((uStack_3c < 0x300) && ((local_3a < 0x300) && (DAT_006db580 < local_28[0]))) {
                uVar19 = (ushort)((DAT_006d9d70 + 1) * 0x1d8);
                if (DAT_006d9d70 == 2) {
                  __builtin_memcpy(&u32, DAT_006d9d56 + 0xe, 4);
                  __builtin_memcpy(&u16, DAT_00732e54 + 0x231, 2);
                  u16 = (ushort)(u16 + (ushort)u32);
                  __builtin_memcpy(DAT_00732e54 + 0x231, &u16, 2);
                  __builtin_memcpy(&u16, DAT_00732e54 + 0xc5, 2);
                  __builtin_memcpy(&u16b, DAT_00732e54 + 0xc3, 2);
                  if (u16 <= u16b) {
                    uVar19 = 0x2c4;
                  }
                }
                else {
                  __builtin_memcpy(&u32, DAT_006d9d56 + 0xe, 4);
                  uVar20 = FUN_00478320(u32, 0xc000);
                  __builtin_memcpy(&u16, psVar6 + 0x231, 2);
                  u16 = (ushort)(u16 + (ushort)uVar20);
                  __builtin_memcpy(psVar6 + 0x231, &u16, 2);
                }
                local_38 = (uint)DAT_00732e54;
                __builtin_memcpy(&s16, DAT_00732e54 + 0x231, 2);
                if (((int)(uint)uVar19 <= (int)s16) &&
                    (uVar19 = 0, DAT_00732e54[0x228] != 0)) {
                  do {
                    psVar6 = DAT_00732e54;
                    uVar20 = (uint)uVar19;
                    __builtin_memcpy(&s16, DAT_00732e54 + uVar20 * 2 + 0x22d, 2);
                    if ((s16 == 0) &&
                        (DAT_00525bf7[DAT_00732e54[uVar20 + 0x229]] == cVar3)) {
                      __builtin_memcpy(&u16, DAT_00732e54 + 0xc5, 2);
                      __builtin_memcpy(&u16b, DAT_00732e54 + 0xc3, 2);
                      if ((DAT_006d9d70 == 2) && (u16 <= u16b)) {
                        DAT_00732e54[uVar20 + 0x22b] = 3;
                      }
                      else {
                        uVar13 = (uint)DAT_0052b0b4[uVar20 + (uint)DAT_00732e54[4] * 0xef];
                        bVar14 = DAT_00732e54[uVar13 * 4 + 0x346];
                        bVar1 = DAT_00732e54[uVar13 * 4 + 0x34a];
                        DAT_00732e54[uVar20 + 0x22b] = 1;
                        if (bVar14 < bVar1) {
                          psVar6[uVar20 + 0x22b] = 0x81;
                        }
                      }
                      uVar8 = 0;
                      local_28[0] = local_28[0] & 0xffffff00;
                      if (psVar5 < DAT_006d4980) {
                        uVar13 = 0;
                        iVar16 = FUN_004490f0((uint)DAT_006d4988[3 + iVar11]);
                        if (0 < iVar16) {
                          do {
                            uVar9 = FUN_00449790((uint)DAT_006d4988[3 + iVar11], uVar13);
                            if (((short)uVar9 == 1) || ((short)uVar9 == 3)) {
                              ((unsigned char *)local_28)[uVar8] = (unsigned char)uVar13;
                              uVar8 = (ushort)(uVar8 + 1);
                            }
                            uVar13 = (uint)(ushort)((short)uVar13 + 1);
                          } while ((int)uVar13 < iVar16);
                        }
                        uVar8 = (ushort)FUN_00422ab0((uint)uVar8);
                        u16 = (ushort)((unsigned char *)local_28)[uVar8];
                        __builtin_memcpy(DAT_006d9d56 + 0x1c, &u16, 2);
                      }
                      else {
                        u16 = 0;
                        __builtin_memcpy(DAT_006d9d56 + 0x1c, &u16, 2);
                      }
                      FUN_00447740((uint)DAT_006d9d50, uVar20);
                      DAT_00732e54[0x78] = (unsigned char)(DAT_00732e54[0x78] + 1);
                      cVar4 = (char)DAT_006d4988[2 + iVar11];
                      if ((cVar4 == 0) || (cVar4 == 1)) {
                        u16 = 0;
                        __builtin_memcpy(DAT_00732e54 + 0x231, &u16, 2);
                      }
                    }
                    uVar19 = (ushort)(uVar19 + 1);
                  } while (uVar19 < DAT_00732e54[0x228]);
                  return 0;
                }
              }
              else {
                __builtin_memcpy(&u32, DAT_006d9d56 + 0xe, 4);
                __builtin_memcpy(&u16, DAT_00732e54 + 0x231, 2);
                u16 = (ushort)(u16 - (ushort)u32);
                __builtin_memcpy(DAT_00732e54 + 0x231, &u16, 2);
                local_38 = (uint)(DAT_00732e54 + 0x231);
                if ((short)u16 < 0) {
                  u16 = 0;
                  __builtin_memcpy(DAT_00732e54 + 0x231, &u16, 2);
                  return local_38 & 0xffff0000;
                }
              }
            }
          }
        }
      }
    }
  }
  return local_38 & 0xffff0000;
}
