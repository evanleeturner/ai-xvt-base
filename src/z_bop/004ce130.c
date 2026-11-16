/* SPDX-License-Identifier: MIT */
/* z_bop 004ce130 */
/* xvt.h is always included */

DWORD __stdcall GetTickCount(void);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void FUN_004d4970(void);
void FUN_004d1ea0(void);
void FUN_004d25d0(void);
/* old-style declaration: the call at 0x004ced55 pushes one 4-byte argument, the others none */
unsigned int FUN_0046fad0();
void FUN_004cef70(int, unsigned int *, int);
unsigned int FUN_004d23b0(int);
void FUN_004cf980(int, unsigned int, unsigned int, unsigned int *, unsigned int);
/* __cdecl; Ghidra shows 3 params but the original call at 0x004ce764 pushes 16 bytes */
int FUN_004cf830(int, unsigned int *, int, ...);
int FUN_004d1fa0(int, unsigned int, unsigned int, unsigned int);
unsigned int FUN_004d48e0(void);

extern unsigned char DAT_00aa7b0d;
extern unsigned int DAT_00aa6cfc;
extern int *DAT_00acebc5;
extern unsigned char DAT_00acebf1[];
extern unsigned char DAT_00acf145[];
extern unsigned char DAT_00acf14d[];
extern unsigned char DAT_00acf359[];
extern unsigned int DAT_00b53571;
extern unsigned int DAT_00b53579;
extern unsigned int DAT_00b63d7d;
extern unsigned int DAT_00b69a89;
extern unsigned int DAT_00b69a8d;
extern unsigned int DAT_00b69a91;

extern unsigned char DAT_00acf571[];
extern unsigned char DAT_00b5357d[];
extern unsigned char DAT_00b53581[];
extern unsigned char DAT_00b5358a[];
extern unsigned char DAT_00b5358b[];
extern unsigned char DAT_00b5358d[];
extern unsigned char DAT_00b63f99[];
extern unsigned char DAT_00b63fa1[];
extern unsigned char DAT_00b63fa9[];
extern unsigned char DAT_00b63fad[];
extern unsigned char DAT_00b641b9[];
extern unsigned char DAT_00b641bd[];
extern unsigned char DAT_00b641c1[];
extern unsigned char DAT_00b641c5[];

extern int DAT_00665050[];
extern unsigned int DAT_00665054[];
extern unsigned int DAT_00665058[];
extern unsigned int DAT_0066505c[];
extern unsigned int DAT_00665060[];
extern int DAT_00665064[];
extern int DAT_00665410;

#define LD32(a) ({ unsigned int t_; __builtin_memcpy(&t_, (const void *)(a), 4); t_; })
#define ST32(a, v) do { unsigned int v_ = (v); __builtin_memcpy((void *)(a), &v_, 4); } while (0)

#define OUTP(p) do { (p) = DAT_00acf571 + LD32(&DAT_00b53571) * 0x210; } while (0)

#define ADVANCE_OUT() do { \
    uTmp = LD32(&DAT_00b53571) + 1; \
    ST32(&DAT_00b53579, LD32(&DAT_00b53579) + 1); \
    if (0x3ff < uTmp) { uTmp = 0; } \
    ST32(&DAT_00b53571, uTmp); \
  } while (0)

#define BUMP_BD(i) do { \
    uTmp = LD32(DAT_00b641b9 + (i) * 0x238); \
    if (0x14 < uTmp) { \
      ST32(DAT_00b641bd + (i) * 0x238, LD32(DAT_00b641bd + (i) * 0x238) + 1); \
    } \
  } while (0)

#define EXIT_PATH() do { \
    if (local_420 != 0) { DAT_00aa6cfc = FUN_004d48e0(); } \
    return 0; \
  } while (0)

/* dword loop followed by 0..3 byte tail, as the original's rep movsd/movsb */
#define COPY_BLOCK(dst, src, n) do { \
    pd = (unsigned char *)(dst); \
    ps = (const unsigned char *)(src); \
    for (uCnt = (n) >> 2; uCnt != 0; uCnt = uCnt - 1) { \
      __builtin_memcpy(pd, ps, 4); \
      ps = ps + 4; \
      pd = pd + 4; \
    } \
    __builtin_memcpy(pd, ps, (n) & 3); \
  } while (0)

unsigned int __stdcall FUN_004ce130(void)
{
  unsigned char bVar1;
  unsigned int uVar2;
  int iVar3;
  DWORD DVar4;
  int *piVar5;
  unsigned int uVar6;
  unsigned int uVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  unsigned char *puVar11;
  unsigned char *pbVar12;
  unsigned char *puVar13;
  unsigned int uVar14;
  unsigned char *pbVar15;
  unsigned char *puVar16;
  int bVar17;
  int local_43c;
  unsigned int uStack_438;
  unsigned int uStack_434;
  unsigned int uStack_430;
  unsigned int uStack_42c;
  unsigned int local_428;
  int iStack_424;
  unsigned int local_420;
  unsigned int uStack_41c[5];
  int local_404;
  unsigned char local_400[0x400];
  unsigned short uStack_hdr;
  int *pObj;
  unsigned char *pbOut;
  unsigned int uTmp;
  unsigned int auPkt[4];
  unsigned int uDat;
  int iPeerId;
  unsigned char *pd;
  const unsigned char *ps;
  unsigned int uCnt;

  local_420 = (unsigned int)DAT_00aa7b0d;
  FUN_004d4970();
  __builtin_memcpy(&pObj, &DAT_00acebc5, 4);
  if (pObj == (int *)0) {
    if (local_420 != 0) {
      DAT_00aa6cfc = FUN_004d48e0();
    }
    return 0;
  }
  FUN_004d1ea0();
  FUN_004d25d0();
LAB_004ce161:
  do {
    while (1) {
      do {
        while (1) {
          if (0x3fe < LD32(&DAT_00b53579)) {
            EXIT_PATH();
          }
          local_428 = 0x400;
          __builtin_memcpy(&pObj, &DAT_00acebc5, 4);
          iVar3 = ((int (__stdcall *)(int *, int *, int *, int, void *, unsigned int *))
                   *(unsigned int *)(*pObj + 100))
                  (pObj, &local_43c, &local_404, 1, local_400, &local_428);
          if (iVar3 != 0) {
            EXIT_PATH();
          }
          if (local_43c != 0) break;
          if (0x200 < local_428) {
            local_428 = 0x200;
          }
          OUTP(pbOut);
          ST32(pbOut, 0);
          ST32(pbOut + 4, local_428);
          pbOut[0xf] = 0;
          COPY_BLOCK(pbOut + 0x10, local_400, local_428);
          ADVANCE_OUT();
        }
      } while ((int)LD32(DAT_00acf145) != local_404);
      __builtin_memcpy(&uStack_hdr, local_400, 2);
      puVar11 = local_400 + 2;
      uVar9 = uStack_hdr & 0x7f;
      uStack_42c = (unsigned int)((uStack_hdr & 0x80) != 0);
      uVar7 = (unsigned int)((uStack_hdr & 0x8000) == 0);
      uStack_430 = ((unsigned int)uStack_hdr & 0xffff7f00U) >> 8;
      if ((uVar9 < 0x3c) || (0x3f < uVar9)) {
        iStack_424 = 1;
      } else {
        iStack_424 = 0;
      }
      if ((iStack_424 != 0) && (uStack_438 = FUN_0046fad0(), uStack_438 == 0)) {
        unsigned short us_;
        puVar11 = local_400 + 4;
        __builtin_memcpy(&us_, local_400 + 2, 2);
        uStack_438 = (unsigned int)us_;
      }
      if (uVar9 != 0x35) break;
      uStack_41c[0] = 0x34;
      FUN_004cef70(local_43c, uStack_41c, 4);
    }
    if (uVar9 == 0x36) {
      __builtin_memcpy(auPkt, puVar11, 16);
      uVar7 = FUN_004d23b0(local_43c);
      DVar4 = GetTickCount();
      ST32(DAT_00b641c5 + uVar7 * 0x238, DVar4);
      uVar7 = auPkt[0];
      DVar4 = GetTickCount();
      if (uVar7 < DVar4 - 0x28) {
        uVar7 = (DVar4 - 0x28) - uVar7;
      } else {
        uVar7 = 1;
      }
      uVar9 = 0;
      piVar5 = DAT_00665050;
      do {
        if (*piVar5 == local_43c) {
          if (uVar7 < 0x2ee) {
            if (DAT_00665064[uVar9 * 6] != 0) {
              uVar6 = DAT_00665054[uVar9 * 6];
              uStack_434 = uVar6 / (unsigned int)DAT_00665064[uVar9 * 6];
              if (uStack_434 < uVar7) {
                if (uStack_434 == 0) {
                  RaiseException(0xC0000094UL, 0, 0, NULL);
                }
                if (((uVar7 - uStack_434) * 100) / uStack_434 < 0x33) {
                  DAT_00665054[uVar9 * 6] = uVar6 + uVar7;
                  DAT_00665058[uVar9 * 6] = auPkt[1];
                  DAT_0066505c[uVar9 * 6] = auPkt[2];
                  DAT_00665060[uVar9 * 6] = auPkt[3];
                  DAT_00665064[uVar9 * 6] = DAT_00665064[uVar9 * 6] + 1;
                } else {
                  DAT_00665058[uVar9 * 6] = auPkt[1];
                  DAT_0066505c[uVar9 * 6] = auPkt[2];
                  DAT_00665060[uVar9 * 6] = auPkt[3];
                }
              } else {
                DAT_00665054[uVar9 * 6] = uVar6 + uVar7;
                DAT_00665058[uVar9 * 6] = auPkt[1];
                DAT_0066505c[uVar9 * 6] = auPkt[2];
                DAT_00665060[uVar9 * 6] = auPkt[3];
                DAT_00665064[uVar9 * 6] = DAT_00665064[uVar9 * 6] + 1;
              }
            }
          } else {
            DAT_00665058[uVar9 * 6] = auPkt[1];
            DAT_0066505c[uVar9 * 6] = auPkt[2];
            DAT_00665060[uVar9 * 6] = auPkt[3];
          }
          break;
        }
        piVar5 = piVar5 + 6;
        uVar9 = uVar9 + 1;
      } while (piVar5 < &DAT_00665410);
      if (0x27 < uVar9) {
        iVar3 = 0;
        piVar5 = DAT_00665050;
        do {
          if (*piVar5 == 0) {
            if (0x2ee < uVar7) {
              uVar7 = 0x2ee;
            }
            DAT_00665050[iVar3 * 6] = local_43c;
            DAT_00665054[iVar3 * 6] = uVar7;
            DAT_00665058[iVar3 * 6] = auPkt[1];
            DAT_0066505c[iVar3 * 6] = auPkt[2];
            DAT_00665060[iVar3 * 6] = auPkt[3];
            DAT_00665064[iVar3 * 6] = 1;
            break;
          }
          piVar5 = piVar5 + 6;
          iVar3 = iVar3 + 1;
        } while (piVar5 < &DAT_00665410);
      }
      goto LAB_004ce161;
    }
    if (uVar9 == 0x33) {
      if (LD32(&DAT_00b69a8d) != 0) {
        __builtin_memcpy(auPkt, puVar11, 16);
        uVar7 = auPkt[0];
        uVar9 = auPkt[1];
        uVar6 = FUN_004d23b0(local_43c);
        BUMP_BD(uVar6);
        uVar8 = 0;
        uVar6 = LD32(&DAT_00b69a91);
        do {
          iVar3 = (int)(uVar6 * 0x210 + LD32(&DAT_00b69a8d));
          if ((*(int *)(iVar3 + 4) != 0) && ((*(unsigned int *)(iVar3 + 0x14) & 0x7fffffff) == uVar7)) {
            FUN_004cf980(local_43c, 0, (unsigned int)*(unsigned char *)(iVar3 + 0xe),
                         (unsigned int *)(iVar3 + 0x10), *(unsigned int *)(iVar3 + 4));
            break;
          }
          uVar8 = uVar8 + 1;
          uVar6 = uVar6 + 1;
          if (0xff < uVar6) {
            uVar6 = 0;
          }
        } while (uVar8 < 0x100);
        if (0xff < uVar8) {
          uStack_41c[0] = 0x39;
          FUN_004cf980(local_43c, 0, uVar9, uStack_41c, 4);
        }
      }
    }
    else if (uVar9 == 0x38) {
      __builtin_memcpy(auPkt, puVar11, 16);
      uVar7 = auPkt[0];
      uVar9 = auPkt[1];
      uVar6 = FUN_004d23b0(local_43c);
      DVar4 = GetTickCount();
      ST32(DAT_00b641c5 + uVar6 * 0x238, DVar4);
      BUMP_BD(uVar6);
      uVar8 = 0;
      uVar6 = LD32(&DAT_00b63d7d);
      do {
        iVar3 = (int)(uVar6 * 0x210);
        uDat = LD32(DAT_00b53581 + uVar6 * 0x210);
        if (uDat != 0) {
          if ((uVar9 == 0) || (uVar9 == 2)) {
            if ((unsigned char)DAT_00b5358b[iVar3] == uVar7) {
              bVar1 = DAT_00b5358a[iVar3];
              goto joined_r0x004ce671;
            }
          }
          else if ((LD32(DAT_00b5357d + uVar6 * 0x210) == (unsigned int)local_43c) &&
                   ((unsigned char)DAT_00b5358b[iVar3] == uVar7)) {
            bVar1 = DAT_00b5358a[iVar3];
joined_r0x004ce671:
            if (bVar1 == uVar9) {
              FUN_004cf980(local_43c, uVar9, uVar7, (unsigned int *)(DAT_00b5358d + uVar6 * 0x210),
                           uDat);
              break;
            }
          }
        }
        uVar8 = uVar8 + 1;
        uVar6 = uVar6 + 1;
        if (0x7f < uVar6) {
          uVar6 = 0;
        }
      } while (uVar8 < 0x80);
      if (0x7f < uVar8) {
        uStack_41c[0] = 0x39;
        FUN_004cf980(local_43c, uVar9, uVar7, uStack_41c, 4);
      }
    }
    else if (uVar9 == 0x37) {
      __builtin_memcpy(auPkt, puVar11, 16);
      uVar7 = auPkt[0];
      uVar9 = auPkt[1];
      uVar6 = auPkt[2];
      iPeerId = (int)LD32(DAT_00acebf1);
      if (iPeerId == local_43c) {
        uStack_434 = FUN_004d23b0(local_43c);
        uStack_434 = uStack_434 * 0x238;
        DVar4 = GetTickCount();
        uStack_41c[0] = 0x36;
        ST32(DAT_00b641c5 + uStack_434, DVar4);
        uStack_41c[2] = LD32(DAT_00b641b9 + uStack_434);
        uStack_41c[1] = auPkt[3];
        uStack_41c[3] = LD32(DAT_00b641bd + uStack_434);
        uStack_41c[4] = LD32(DAT_00b641c1 + uStack_434);
        iPeerId = (int)LD32(DAT_00acebf1);
        FUN_004cf830(iPeerId, uStack_41c, 0x14, 0);
      }
      if (LD32(DAT_00acf14d) == uVar7) {
        uVar7 = 0x80;
      }
      if (LD32(DAT_00acf359) == uVar9) {
        uVar9 = 0x80;
      }
      uVar8 = FUN_004d23b0(local_43c);
      if ((LD32(&DAT_00b69a89) == uVar8) || (LD32(DAT_00b63fad + uVar8 * 0x238) == uVar6)) {
        uVar6 = 0x80;
      }
      uStack_434 = 0x80;
      uVar8 = LD32(&DAT_00b63d7d);
      do {
        if (((uVar7 == 0x80) && (uVar9 == 0x80)) && (uVar6 == 0x80)) break;
        iVar3 = (int)(uVar8 * 0x210);
        uVar2 = LD32(DAT_00b53581 + uVar8 * 0x210);
        uVar14 = uVar6;
        if (uVar2 != 0) {
          if (DAT_00b5358a[iVar3] == 0) {
            if ((unsigned char)DAT_00b5358b[iVar3] == uVar7) {
              FUN_004cf980(local_43c, 0, uVar7, (unsigned int *)(DAT_00b5358d + iVar3), uVar2);
              uVar7 = 0x80;
            }
          }
          else if (DAT_00b5358a[iVar3] == 2) {
            if ((unsigned char)DAT_00b5358b[iVar3] == uVar9) {
              FUN_004cf980(local_43c, 2, uVar9, (unsigned int *)(DAT_00b5358d + iVar3), uVar2);
              uVar9 = 0x80;
            }
          }
          else if ((LD32(DAT_00b5357d + uVar8 * 0x210) == (unsigned int)local_43c) &&
                   ((unsigned char)DAT_00b5358b[iVar3] == uVar6)) {
            uVar14 = 0x80;
            FUN_004cf980(local_43c, 1, uVar6, (unsigned int *)(DAT_00b5358d + iVar3), uVar2);
          }
        }
        uVar8 = uVar8 + 1;
        if (0x7f < uVar8) {
          uVar8 = 0;
        }
        uStack_434 = uStack_434 - 1;
        uVar6 = uVar14;
      } while (uStack_434 != 0);
    }
    else {
      uVar6 = FUN_004d23b0(local_43c);
      DVar4 = GetTickCount();
      ST32(DAT_00b641c5 + uVar6 * 0x238, DVar4);
      if ((uVar7 != 0) && (uStack_42c != 0)) {
        /* laid out first in the original (0x004ce8e6..0x004ceb56) */
        bVar17 = (local_400[2] != 0);
        if (bVar17) {
          uStack_42c = (unsigned int)(local_400[2] == 2);
        } else {
          uStack_42c = 0;
        }
        puVar13 = local_400 + 3;
        if (iStack_424 == 0) {
          uStack_438 = local_428 - 2;
        } else {
          uStack_438 = FUN_0046fad0();
          if (uStack_438 == 0) {
            unsigned short us_;
            puVar13 = local_400 + 5;
            __builtin_memcpy(&us_, local_400 + 3, 2);
            uStack_438 = (unsigned int)us_;
          }
        }
        if (0x1fc < uStack_438) {
          uStack_438 = 0x1fc;
        }
        OUTP(pbOut);
        ST32(pbOut + 0x10, uVar9);
        puVar16 = pbOut + 0x14;
        COPY_BLOCK(puVar16, puVar13, uStack_438);
        ST32(pbOut, (unsigned int)local_43c);
        ST32(pbOut + 4, uStack_438 + 4);
        ST32(pbOut + 8, 0);
        pbOut[0xc] = 0;
        pbOut[0xf] = 1;
        uVar7 = FUN_004d23b0(local_43c);
        if (bVar17) {
          if (uStack_42c == 0) {
            pbOut[0xd] = 1;
            if ((uVar7 < LD32(&DAT_00b69a89)) && (uVar7 < 0x28)) {
              puVar11 = DAT_00b63f99 + uVar7 * 0x238;
              uVar7 = LD32(puVar11) + 1;
              if (0x7f < uVar7) {
                uVar7 = 0;
              }
              goto joined_r0x004ceb0d;
            }
          }
          else {
            pbOut[0xd] = 2;
            if ((uVar7 < LD32(&DAT_00b69a89)) && (uVar7 < 0x28)) {
              puVar11 = DAT_00b63fa9 + uVar7 * 0x238;
              uVar7 = LD32(puVar11) + 1;
              if (0x7f < uVar7) {
                uVar7 = 0;
              }
joined_r0x004ceb0d:
              if (uVar7 == uStack_430) {
                ST32(puVar11, uVar7);
              }
            }
          }
        }
        else {
          pbOut[0xd] = 0;
          if ((uVar7 < LD32(&DAT_00b69a89)) && (uVar7 < 0x28)) {
            puVar11 = DAT_00b63fa1 + uVar7 * 0x238;
            uVar7 = LD32(puVar11) + 1;
            if (0x7f < uVar7) {
              uVar7 = 0;
            }
            goto joined_r0x004ceb0d;
          }
        }
        pbOut[0xe] = (unsigned char)uStack_430;
        ADVANCE_OUT();
      }
      else {
        /* original 0x004ceb56.. */
        if (iStack_424 != 0) {
          iVar3 = 0x7f;
          if (uStack_430 != 0) {
            iVar3 = (int)(uStack_430 - 1);
          }
          iVar3 = FUN_004d1fa0(local_43c, (unsigned int)iVar3, uVar7, uStack_42c);
          if (iVar3 == 0) {
            BUMP_BD(uVar6);
            uVar6 = (unsigned int)*(unsigned char *)(puVar11 + uStack_438);
            pbVar12 = (unsigned char *)(puVar11 + uStack_438) + 1;
            if (uVar6 != 0x39) {
              uVar8 = local_428 - ((unsigned int)(pbVar12 - local_400) & 0xffffU);
              if (0x1fc < uVar8) {
                uVar8 = 0x1fc;
              }
              OUTP(pbOut);
              ST32(pbOut + 0x10, uVar6);
              pbVar15 = pbOut + 0x14;
              COPY_BLOCK(pbVar15, pbVar12, uVar8);
              ST32(pbOut, (unsigned int)local_43c);
              ST32(pbOut + 4, uVar8 + 4);
              ST32(pbOut + 8, 0);
              pbOut[0xc] = 0;
              pbOut[0xf] = 0;
              if (uVar7 == 0) {
                if (uStack_42c == 0) {
                  pbOut[0xd] = 1;
                } else {
                  pbOut[0xd] = 2;
                }
              } else {
                pbOut[0xd] = 0;
              }
              if (uStack_430 == 0) {
                pbOut[0xe] = 0x7f;
              } else {
                pbOut[0xe] = (unsigned char)((char)uStack_430 + -1);
              }
              ADVANCE_OUT();
            }
          }
        }
        uStack_434 = (unsigned int)FUN_004d1fa0(local_43c, uStack_430, uVar7, uStack_42c);
        if (uStack_434 == 0) {
          puVar13 = local_400 + 2;
          if (iStack_424 == 0) {
            uStack_438 = local_428 - 2;
          } else {
            /* original call at 0x004ced55 pushes 4 argument bytes (cdecl); value unresolved */
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-non-prototype"
            uStack_438 = FUN_0046fad0(0);
#pragma clang diagnostic pop
            if (uStack_438 == 0) {
              unsigned short us_;
              puVar13 = local_400 + 4;
              __builtin_memcpy(&us_, local_400 + 2, 2);
              uStack_438 = (unsigned int)us_;
            }
          }
          if (0x1fc < uStack_438) {
            uStack_438 = 0x1fc;
          }
          OUTP(pbOut);
          ST32(pbOut + 0x10, uVar9);
          puVar16 = pbOut + 0x14;
          COPY_BLOCK(puVar16, puVar13, uStack_438);
          ST32(pbOut, (unsigned int)local_43c);
          ST32(pbOut + 4, uStack_438 + 4);
          ST32(pbOut + 8, 0);
          pbOut[0xc] = 0;
          pbOut[0xf] = 0;
          if (uStack_434 == 0) {
            pbOut[0xf] = 0;
          } else {
            pbOut[0xf] = 1;
          }
          if (uVar7 == 0) {
            if (uStack_42c == 0) {
              pbOut[0xd] = 1;
            } else {
              pbOut[0xd] = 2;
            }
          } else {
            pbOut[0xd] = 0;
          }
          pbOut[0xe] = (unsigned char)uStack_430;
          ADVANCE_OUT();
        }
      }
    }
  } while (1);
}
