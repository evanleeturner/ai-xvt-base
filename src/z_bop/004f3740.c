/* SPDX-License-Identifier: MIT */
/* z_bop 004f3740 */
extern int   DAT_00aa5e68;
extern int   DAT_00aa5cb0[];
extern int   DAT_00aa5e10[];
extern int   DAT_00aa5df0[];
extern int   DAT_00aa5e38;
extern int   DAT_00aa5e40[];
extern int   DAT_00aa5e60;
extern char  DAT_00aa47ba[];
extern char  DAT_00aa4f4e[];
extern unsigned char DAT_00b6a5aa[4];   /* unaligned 32-bit global, read via memcpy */
extern unsigned char DAT_00b6a606[4];   /* unaligned 32-bit global, read via memcpy */
extern int   DAT_00aa6120;
extern int   DAT_00aa6124;
extern int   DAT_00aa612c;
extern int   DAT_00aa6130;
extern char  DAT_00b69d20[];
extern char  DAT_00528118[];
extern int   DAT_00b69e30;
extern unsigned int DAT_00b6a2c0;
extern unsigned char DAT_00b6a270[];
extern char  DAT_00bb73a7;
extern unsigned int DAT_00aa6af0;
extern int   DAT_00aa6af4;
extern int   DAT_00aa6af8;
extern int   DAT_00aa6afc;
/* Array of 0x2a-byte records starting at 0xaa6150: name at +0, id (int) at +0xe (0xaa615e),
   int at +0x12 (0xaa6162); the int fields are unaligned, so they are read via memcpy. */
extern unsigned char DAT_00aa6150[];

void FUN_004d6460(int *, int, int, int, int);
char *FUN_004ddc10(int);
int _sprintf(char *, const char *, ...);
void FUN_004db7d0(int, char *, int *, int, int, unsigned int);
void FUN_004d64a0(int *, int, int);
char FUN_004dc2e0(int);
unsigned char FUN_004dc2a0(int);
unsigned char FUN_004dc2c0(int);
void FUN_004de090(int *, int *);
int FUN_004d6b90(int *, int, int);
void FUN_004cef70(int, unsigned int *, int);
void FUN_004de5a0(const char *, int, int, int);
void FUN_004dc1d0(void);
int FUN_004d1400(void);
void FUN_004d4cf0(void *);
void FUN_004d4d20(void *);
int FUN_004d11e0(void);
int FUN_004cfee0(void);
unsigned char FUN_004dc1e0(void);
unsigned char FUN_004dc200(void);
int FUN_004d11d0(void);
void FUN_004dc1c0(int);

void FUN_004f3740(int param_1)
{
  char bVar1;
  char *pcVar3;
  int iVar4;
  int *piVar5;
  int iVar6;
  int *puVar7;
  int *piVar8;
  int iVar9;
  int iVar10;
  int *piVar11;
  int uVar12;
  char *puVar13;
  unsigned int uVar14;
  int local_44;
  char *local_40;
  int local_3c;
  int local_38;
  int local_34;
  int local_30;
  int *local_2c;
  int *local_28;
  int local_24[4];
  int local_14;
  int local_10[4];
  int tmp_a;
  int tmp_b;
  unsigned char *pb;

  FUN_004d6460(local_24, 0x5a, 0xcf, 0x100, 0xdd);
  local_38 = 0;
  if (0 < DAT_00aa5e68) {
    local_2c = DAT_00aa5cb0;
    local_3c = 0;
    local_28 = DAT_00aa5e10;
    local_40 = DAT_00aa47ba;
    do {
      if (((DAT_00aa5e68 == 8) || (DAT_00aa5e68 == 7)) && (local_40 == DAT_00aa4f4e)) {
        FUN_004d6460(local_24, 0x106, 0xcf, 0x1ac, 0xdd);
      }
      __builtin_memcpy(&tmp_a, DAT_00b6a5aa, 4);
      if (tmp_a == 3) {
        __builtin_memcpy(&tmp_b, DAT_00b6a606, 4);
        if (((DAT_00aa6120 == 0) || (DAT_00aa6124 == 0)) || (tmp_b != 1)) {
          _sprintf(DAT_00b69d20, DAT_00528118, local_40);
        }
        else if (local_40 == DAT_00aa47ba) {
          uVar12 = DAT_00aa6130;
          pcVar3 = FUN_004ddc10(0x308);
          _sprintf(DAT_00b69d20, "%s - %c%s: %d", DAT_00aa47ba, 4, pcVar3, uVar12);
        }
        else {
          uVar12 = DAT_00aa612c;
          pcVar3 = FUN_004ddc10(0x308);
          _sprintf(DAT_00b69d20, "%s - %c%s: %d", local_40, 4, pcVar3, uVar12);
        }
      }
      else {
        iVar6 = local_38 + 1;
        puVar13 = local_40;
        pcVar3 = FUN_004ddc10(0xd9);
        _sprintf(DAT_00b69d20, "%s %u: %s", pcVar3, iVar6, puVar13);
      }
      FUN_004db7d0(0xc, DAT_00b69d20, local_24, 0, 1, 0xffff);
      FUN_004d64a0(local_24, 0, 0xf);
      local_44 = 0;
      if (0 < *local_28) {
        do {
          bVar1 = FUN_004dc2e0(2);
          if ((bVar1 != 0) &&
              (FUN_004dc2a0(2) != 0 || FUN_004dc2c0(2) != 0)) {
            FUN_004de090(&local_30, &local_34);
            iVar4 = FUN_004d6b90(local_24, local_30, local_34);
            iVar10 = DAT_00b69e30;
            iVar6 = DAT_00aa5e38;
            if (iVar4 != 0) {
              iVar4 = DAT_00aa5cb0[local_3c + local_44];
              if (iVar4 == 0) {
LAB_004f39d5:
                iVar9 = 0;
                piVar5 = local_2c;
                if (0 < local_44) {
                  do {
                    if (*piVar5 == 0) break;
                    iVar9 = iVar9 + 1;
                    piVar5 = piVar5 + 1;
                  } while (iVar9 < local_44);
                }
              }
              else {
                piVar5 = DAT_00aa5df0;
                do {
                  if (*piVar5 == iVar4) {
                    *piVar5 = 0;
                  }
                  piVar5 = piVar5 + 1;
                } while (piVar5 < DAT_00aa5e10);
                if (iVar10 != 2) {
                  DAT_00aa6af0 = 0x4f;
                  DAT_00aa6af8 = 10;
                  DAT_00aa6af4 = iVar4;
                  FUN_004cef70(0, &DAT_00aa6af0, 0xc);
                  iVar10 = DAT_00b69e30;
                  iVar6 = DAT_00aa5e38;
                }
                iVar9 = local_44;
                if (iVar4 == 0) goto LAB_004f39d5;
              }
              iVar4 = 0;
              DAT_00aa5cb0[local_3c + iVar9] = iVar6;
              piVar5 = DAT_00aa5df0;
              do {
                if (*piVar5 == iVar6) break;
                piVar5 = piVar5 + 1;
                iVar4 = iVar4 + 1;
              } while (piVar5 < DAT_00aa5e10);
              if (iVar4 == 8) {
                iVar4 = 0;
                piVar5 = DAT_00aa5df0;
                do {
                  if (*piVar5 == 0) {
                    DAT_00aa5df0[iVar4] = iVar6;
                    break;
                  }
                  piVar5 = piVar5 + 1;
                  iVar4 = iVar4 + 1;
                } while (piVar5 < DAT_00aa5e10);
              }
              if (DAT_00bb73a7 != '\0') {
                FUN_004de5a0("slotsound", 1, 0, 0xff);
                iVar10 = DAT_00b69e30;
                iVar6 = DAT_00aa5e38;
              }
              if (iVar10 != 2) {
                DAT_00aa6af8 = local_38;
                DAT_00aa6af0 = 0x4f;
                DAT_00aa6af4 = iVar6;
                DAT_00aa6afc = iVar9;
                FUN_004cef70(0, &DAT_00aa6af0, 0x10);
                if (DAT_00b69e30 != 2) {
                  DAT_00aa6af4 = DAT_00aa5e38;
                  DAT_00aa6af0 = 0x67;
                  FUN_004cef70(0, &DAT_00aa6af0, 8);
                }
              }
              DAT_00aa5e38 = 0;
              FUN_004dc1d0();
            }
          }
          if (DAT_00aa5cb0[local_3c + local_44] != 0) {
            iVar6 = 1;
            if (DAT_00b69e30 != 2) {
              iVar6 = FUN_004d1400();
            }
            iVar10 = 0;
            if (0 < iVar6) {
              pb = DAT_00aa6150 + 0xe;
              do {
                __builtin_memcpy(&tmp_a, pb, 4);
                if (tmp_a == DAT_00aa5cb0[local_3c + local_44]) break;
                pb = pb + 0x2a;
                iVar10 = iVar10 + 1;
              } while (iVar10 < iVar6);
            }
            if (iVar10 < iVar6) {
              iVar6 = (int)DAT_00aa6150 + iVar10 * 0x2a;
              iVar10 = iVar10 * 0x2a;
              uVar12 = 1;
              __builtin_memcpy(&tmp_a, DAT_00aa6150 + 0x12 + iVar10, 4);
              pcVar3 = FUN_004ddc10(tmp_a + 0x9a);
              _sprintf(DAT_00b69d20, "%c%s %c%s", 6, pcVar3, uVar12, iVar6);
              FUN_004d4cf0(local_10);
              FUN_004d4d20(local_24);
              iVar6 = FUN_004d11e0();
              __builtin_memcpy(&tmp_a, DAT_00aa6150 + 0xe + iVar10, 4);
              if ((iVar6 == tmp_a) ||
                  (uVar14 = DAT_00b6a2c0, DAT_00b69e30 == 2)) {
                uVar14 = *(unsigned int *)(DAT_00b6a270 +
                                           (((unsigned int)(param_1 % 0x18)) & 0xfffffffeU) * 2);
              }
              FUN_004db7d0(0xc, DAT_00b69d20, local_24, 0, 1, uVar14);
              FUN_004d4d20(local_10);
              bVar1 = FUN_004dc2e0(2);
              if (((bVar1 == 0) &&
                   ((iVar6 = FUN_004cfee0(), iVar6 != 0 || (DAT_00b69e30 == 2)))) &&
                  (FUN_004dc1e0() != 0 || FUN_004dc200() != 0)) {
                FUN_004de090(&local_30, &local_34);
                iVar6 = FUN_004d6b90(local_24, local_30, local_34);
                if (iVar6 != 0) {
                  if (DAT_00b69e30 == 2) {
                    __builtin_memcpy(&DAT_00aa5e38, DAT_00aa6150 + 0xe + iVar10, 4);
                    FUN_004dc1c0(2);
                    iVar6 = 0;
                    piVar5 = DAT_00aa5df0;
LAB_004f3def:
                    if (*piVar5 != DAT_00aa5e38) goto code_r0x004f3df3;
                    DAT_00aa5df0[iVar6] = 0;
                    iVar6 = local_44 + 1;
                    if (iVar6 < 8) {
                      iVar10 = 8 - iVar6;
                      puVar7 = DAT_00aa5cb0 - 1 + (local_3c + iVar6);
                      do {
                        iVar10 = iVar10 + -1;
                        *puVar7 = puVar7[1];
                        puVar7 = puVar7 + 1;
                      } while (iVar10 != 0);
                    }
                    local_2c[7] = 0;
                    goto LAB_004f3e41;
                  }
                  iVar6 = 0;
                  if (0 < DAT_00aa5e60) {
                    piVar5 = DAT_00aa5e40;
                    do {
                      __builtin_memcpy(&tmp_a, DAT_00aa6150 + 0xe + iVar10, 4);
                      if (*piVar5 == tmp_a) break;
                      piVar5 = piVar5 + 1;
                      iVar6 = iVar6 + 1;
                    } while (iVar6 < DAT_00aa5e60);
                  }
                  if (iVar6 == DAT_00aa5e60) {
                    __builtin_memcpy(&DAT_00aa6af4, DAT_00aa6150 + 0xe + iVar10, 4);
                    DAT_00aa5e60 = DAT_00aa5e60 + 1;
                    DAT_00aa5e40[iVar6] = DAT_00aa6af4;
                    DAT_00aa6af0 = 0x68;
                    DAT_00aa6af8 = FUN_004d11d0();
                    FUN_004cef70(0, &DAT_00aa6af0, 0xc);
                    __builtin_memcpy(&DAT_00aa5e38, DAT_00aa6150 + 0xe + iVar10, 4);
                    FUN_004dc1c0(2);
                    DAT_00aa6af0 = 0x4f;
                    DAT_00aa6af4 = DAT_00aa5e38;
                    DAT_00aa6af8 = 10;
                    FUN_004cef70(0, &DAT_00aa6af0, 0xc);
                    iVar10 = DAT_00aa5e68;
                    iVar6 = DAT_00aa5e38;
                    iVar4 = 0;
                    piVar5 = DAT_00aa5df0;
                    do {
                      if (*piVar5 == DAT_00aa5e38) {
                        DAT_00aa5df0[iVar4] = 0;
                        break;
                      }
                      piVar5 = piVar5 + 1;
                      iVar4 = iVar4 + 1;
                    } while (piVar5 < DAT_00aa5e10);
                    if (0 < iVar10) {
                      piVar5 = DAT_00aa5cb0;
                      iVar4 = 0;
                      piVar11 = DAT_00aa5e10;
                      local_14 = iVar10;
                      do {
                        iVar10 = 0;
                        piVar8 = piVar5;
                        if (0 < *piVar11) {
LAB_004f3d83:
                          if (*piVar8 != iVar6) goto code_r0x004f3d88;
                          iVar10 = iVar10 + 1;
                          if (iVar10 < 8) {
                            iVar9 = 8 - iVar10;
                            puVar7 = DAT_00aa5cb0 - 1 + (iVar4 + iVar10);
                            do {
                              iVar9 = iVar9 + -1;
                              *puVar7 = puVar7[1];
                              puVar7 = puVar7 + 1;
                            } while (iVar9 != 0);
                          }
                          piVar5[7] = 0;
                        }
LAB_004f3dbc:
                        piVar5 = piVar5 + 8;
                        iVar4 = iVar4 + 8;
                        piVar11 = piVar11 + 1;
                        local_14 = local_14 + -1;
                      } while (local_14 != 0);
                      local_14 = 0;
                    }
                  }
                }
              }
            }
          }
LAB_004f3e41:
          FUN_004d64a0(local_24, 0, 0xf);
          local_44 = local_44 + 1;
        } while (local_44 < *local_28);
      }
      local_2c = local_2c + 8;
      local_3c = local_3c + 8;
      local_28 = local_28 + 1;
      local_40 = local_40 + 0x1e5;
      local_38 = local_38 + 1;
    } while (local_38 < DAT_00aa5e68);
  }
  return;
code_r0x004f3df3:
  piVar5 = piVar5 + 1;
  iVar6 = iVar6 + 1;
  if ((int *)0xaa5e0f < piVar5) goto LAB_004f3e41;
  goto LAB_004f3def;
code_r0x004f3d88:
  iVar10 = iVar10 + 1;
  piVar8 = piVar8 + 1;
  if (*piVar11 <= iVar10) goto LAB_004f3dbc;
  goto LAB_004f3d83;
}
