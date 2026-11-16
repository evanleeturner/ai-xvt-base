/* SPDX-License-Identifier: MIT */
/* z_bop 00463160 */
/* Prototypes for every callee. */
int   __cdecl FUN_0046f700(void);
int   __cdecl FUN_0046e040(void);
int   __cdecl FUN_0046fa30(void);
void  __cdecl FUN_00448da0(void);
void  __cdecl FUN_00448f90(int param_1, void *param_2, int param_3);
void  __cdecl FUN_00448ed0(void);
int * __cdecl FUN_0046f5e0(int *param_1, int *param_2, int param_3);
int   __cdecl FUN_0046f660(int param_1);
char *__cdecl FUN_0046f720(void *param_1);
void  __cdecl FUN_0046d300(unsigned int *param_1, int param_2);
int   __cdecl FUN_0046d350(int param_1, unsigned int *param_2, unsigned int param_3);
DWORD __stdcall timeGetTime(void);

/* Globals. Unaligned/strided ones are byte arrays accessed via memcpy. */
extern int DAT_009fd394;
extern int DAT_005233ec;
extern unsigned char DAT_00b6bc32[];    /* unaligned 4-byte value */
extern unsigned char DAT_00bb73a9[];    /* unaligned 0x118-byte block */
extern int DAT_009d7800[];
extern unsigned char DAT_009e9bf6[];    /* short, stride 0x5bd */
extern unsigned char DAT_009e9678[];    /* short, stride 0x5bd */
extern unsigned int DAT_00557360;
extern unsigned int DAT_00557364;
extern unsigned int DAT_00557368[0x46];
extern void *DAT_00a60a28;
extern char *DAT_00a60a2c;
extern char *DAT_00a60a30;
extern char *DAT_00a60a34;
extern int DAT_00a08138;
extern int DAT_009ecc34;

unsigned int __stdcall FUN_00463160(void)
{
  char cVar1;
  unsigned char bVar2;
  int bVar3;
  int bVar4;
  int bVar5;
  unsigned int uVar6;
  int iVar7;
  int iVar8;
  int *piVar10;
  DWORD DVar11;
  char *pcVar13;
  char *pcVar22;
  unsigned int uVar14;
  unsigned int uVar15;
  unsigned char *pbVar19;
  unsigned char *pbVar23;
  unsigned char *pb;
  unsigned char *pbSrc;
  unsigned int *puDst;
  unsigned int *puVar16;
  int *piVar20;
  int *piDst;
  unsigned short us;
  int local_114;
  DWORD local_110;
  int local_10c;
  int local_108;
  DWORD local_104;
  unsigned char local_100[256];

  bVar4 = 1;
  bVar3 = 1;
  bVar5 = 1;
  local_110 = 0;
  iVar7 = FUN_0046f700();
  if (DAT_009fd394 < 2) {
    __builtin_memcpy(&uVar6, DAT_00b6bc32, 4);
    us = (unsigned short)DAT_005233ec;
    __builtin_memcpy(DAT_009e9bf6, &us, 2);
    us = (unsigned short)uVar6;
    __builtin_memcpy(DAT_009e9678, &us, 2);
    pbSrc = DAT_00bb73a9;
    puDst = (unsigned int *)DAT_009d7800;
    for (iVar7 = 0x46; iVar7 != 0; iVar7 = iVar7 + -1) {
      __builtin_memcpy(puDst, pbSrc, 4);
      pbSrc = pbSrc + 4;
      puDst = puDst + 1;
    }
  }
  else {
    iVar8 = FUN_0046e040();
    if (iVar8 == 0) {
      DAT_00557364 = (unsigned int)DAT_005233ec;
      __builtin_memcpy(&DAT_00557368[0], DAT_00b6bc32, 4);
      DAT_00557360 = 0x11;
      FUN_0046d350(iVar7, &DAT_00557360, 0xc);
      FUN_00448da0();
      FUN_00448f90(1, DAT_00a60a28, 0x30);
      do {
        piVar10 = FUN_0046f5e0(&local_114, &local_108, 0x3c);
        if (piVar10 == (int *)0) {
          return 0;
        }
        if ((*piVar10 == 0x1d) && (DVar11 = timeGetTime(), 200 < (int)(DVar11 - local_110))) {
          local_110 = DVar11;
          pcVar13 = FUN_0046f720((void *)FUN_0046f660(local_114));
          if (pcVar13 == (char *)0) {
            uVar14 = 0xffffffff;
            pcVar13 = DAT_00a60a2c;
            do {
              pcVar22 = pcVar13;
              if (uVar14 == 0) break;
              uVar14 = uVar14 - 1;
              pcVar22 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar22;
            } while (cVar1 != '\0');
            uVar14 = ~uVar14;
            pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
            pbVar23 = local_100;
            for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
              __builtin_memcpy(pbVar23, pbVar19, 4);
              pbVar19 = pbVar19 + 4;
              pbVar23 = pbVar23 + 4;
            }
          }
          else {
            uVar14 = 0xffffffff;
            do {
              pcVar22 = pcVar13;
              if (uVar14 == 0) break;
              uVar14 = uVar14 - 1;
              pcVar22 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar22;
            } while (cVar1 != '\0');
            uVar14 = ~uVar14;
            pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
            pbVar23 = local_100;
            for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
              __builtin_memcpy(pbVar23, pbVar19, 4);
              pbVar19 = pbVar19 + 4;
              pbVar23 = pbVar23 + 4;
            }
            for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
              *pbVar23 = *pbVar19;
              pbVar19 = pbVar19 + 1;
              pbVar23 = pbVar23 + 1;
            }
            bVar3 = !bVar4;
            pcVar13 = DAT_00a60a34;
            if (bVar3) {
              pcVar13 = DAT_00a60a30;
            }
            uVar14 = 0xffffffff;
            do {
              pcVar22 = pcVar13;
              if (uVar14 == 0) break;
              uVar14 = uVar14 - 1;
              pcVar22 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar22;
            } while (cVar1 != '\0');
            uVar14 = ~uVar14;
            iVar7 = -1;
            pbVar19 = local_100;
            do {
              pbVar23 = pbVar19;
              if (iVar7 == 0) break;
              iVar7 = iVar7 + -1;
              pbVar23 = pbVar19 + 1;
              bVar2 = *pbVar19;
              pbVar19 = pbVar23;
            } while (bVar2 != 0);
            pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
            pbVar23 = pbVar23 + -1;
            for (uVar15 = uVar14 >> 2; bVar4 = bVar3, uVar15 != 0; uVar15 = uVar15 - 1) {
              __builtin_memcpy(pbVar23, pbVar19, 4);
              pbVar19 = pbVar19 + 4;
              pbVar23 = pbVar23 + 4;
            }
          }
          for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
            *pbVar23 = *pbVar19;
            pbVar19 = pbVar19 + 1;
            pbVar23 = pbVar23 + 1;
          }
          FUN_00448f90(3, local_100, 0x30);
        }
      } while (*piVar10 != 0x12);
      FUN_00448ed0();
      DAT_00a08138 = piVar10[1];
      if (0 < DAT_009fd394) {
        iVar7 = DAT_009fd394;
        pb = DAT_009e9bf6;
        do {
          us = (unsigned short)piVar10[2];
          __builtin_memcpy(pb, &us, 2);
          iVar7 = iVar7 + -1;
          us = (unsigned short)piVar10[3];
          __builtin_memcpy(pb - 0x57e, &us, 2);
          pb = pb + 0x5bd;
          piVar10 = piVar10 + 2;
        } while (iVar7 != 0);
      }
    }
    else {
      if (0 < DAT_009fd394) {
        pb = DAT_009e9bf6;
        iVar7 = DAT_009fd394;
        do {
          us = 0x13;
          __builtin_memcpy(pb, &us, 2);
          us = 0;
          __builtin_memcpy(pb - 0x57e, &us, 2);
          iVar7 = iVar7 + -1;
          pb = pb + 0x5bd;
        } while (iVar7 != 0);
      }
      FUN_0046fa30();
      __builtin_memcpy(&uVar6, DAT_00b6bc32, 4);
      iVar7 = (int)((unsigned int)DAT_009ecc34 * 0x5bdU);
      us = (unsigned short)DAT_005233ec;
      __builtin_memcpy(DAT_009e9bf6 + iVar7, &us, 2);
      us = (unsigned short)uVar6;
      __builtin_memcpy(DAT_009e9678 + iVar7, &us, 2);
      FUN_00448da0();
      FUN_00448f90(1, DAT_00a60a28, 0x30);
      local_10c = 0;
      local_104 = timeGetTime();
      while (iVar7 = FUN_0046fa30(), local_10c < iVar7 + -1) {
        piVar10 = FUN_0046f5e0(&local_114, &local_108, 0x3c);
        DVar11 = timeGetTime();
        if (piVar10 == (int *)0) {
          if (60000 < DVar11 - local_104) {
            return 0;
          }
        }
        else {
          local_104 = DVar11;
          if ((*piVar10 == 0x1d) && (DVar11 = timeGetTime(), 200 < (int)(DVar11 - local_110))) {
            local_110 = DVar11;
            pcVar13 = FUN_0046f720((void *)FUN_0046f660(local_114));
            if (pcVar13 == (char *)0) {
              uVar14 = 0xffffffff;
              pcVar13 = DAT_00a60a2c;
              do {
                pcVar22 = pcVar13;
                if (uVar14 == 0) break;
                uVar14 = uVar14 - 1;
                pcVar22 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar22;
              } while (cVar1 != '\0');
              uVar14 = ~uVar14;
              pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
              pbVar23 = local_100;
              for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
                __builtin_memcpy(pbVar23, pbVar19, 4);
                pbVar19 = pbVar19 + 4;
                pbVar23 = pbVar23 + 4;
              }
            }
            else {
              uVar14 = 0xffffffff;
              do {
                pcVar22 = pcVar13;
                if (uVar14 == 0) break;
                uVar14 = uVar14 - 1;
                pcVar22 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar22;
              } while (cVar1 != '\0');
              uVar14 = ~uVar14;
              pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
              pbVar23 = local_100;
              for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
                __builtin_memcpy(pbVar23, pbVar19, 4);
                pbVar19 = pbVar19 + 4;
                pbVar23 = pbVar23 + 4;
              }
              for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
                *pbVar23 = *pbVar19;
                pbVar19 = pbVar19 + 1;
                pbVar23 = pbVar23 + 1;
              }
              bVar3 = !bVar5;
              pcVar13 = DAT_00a60a34;
              if (bVar3) {
                pcVar13 = DAT_00a60a30;
              }
              uVar14 = 0xffffffff;
              do {
                pcVar22 = pcVar13;
                if (uVar14 == 0) break;
                uVar14 = uVar14 - 1;
                pcVar22 = pcVar13 + 1;
                cVar1 = *pcVar13;
                pcVar13 = pcVar22;
              } while (cVar1 != '\0');
              uVar14 = ~uVar14;
              iVar7 = -1;
              pbVar19 = local_100;
              do {
                pbVar23 = pbVar19;
                if (iVar7 == 0) break;
                iVar7 = iVar7 + -1;
                pbVar23 = pbVar19 + 1;
                bVar2 = *pbVar19;
                pbVar19 = pbVar23;
              } while (bVar2 != 0);
              pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
              pbVar23 = pbVar23 + -1;
              for (uVar15 = uVar14 >> 2; bVar5 = bVar3, uVar15 != 0; uVar15 = uVar15 - 1) {
                __builtin_memcpy(pbVar23, pbVar19, 4);
                pbVar19 = pbVar19 + 4;
                pbVar23 = pbVar23 + 4;
              }
            }
            for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
              *pbVar23 = *pbVar19;
              pbVar19 = pbVar19 + 1;
              pbVar23 = pbVar23 + 1;
            }
            FUN_00448f90(3, local_100, 0x30);
          }
          if (*piVar10 == 0x11) {
            iVar7 = FUN_0046f660(local_114);
            us = (unsigned short)piVar10[1];
            __builtin_memcpy(DAT_009e9bf6 + (int)((unsigned int)iVar7 * 0x5bdU), &us, 2);
            us = (unsigned short)piVar10[2];
            __builtin_memcpy(DAT_009e9678 + (int)((unsigned int)iVar7 * 0x5bdU), &us, 2);
            local_10c = local_10c + 1;
          }
        }
      }
      FUN_00448ed0();
      DAT_00557364 = (unsigned int)DAT_00a08138;
      DAT_00557360 = 0x12;
      if (0 < DAT_009fd394) {
        iVar7 = DAT_009fd394;
        puVar16 = &DAT_00557368[0];
        pb = DAT_009e9bf6;
        do {
          __builtin_memcpy(&us, pb, 2);
          *puVar16 = (unsigned int)us;
          iVar7 = iVar7 + -1;
          __builtin_memcpy(&us, pb - 0x57e, 2);
          puVar16[1] = (unsigned int)us;
          puVar16 = puVar16 + 2;
          pb = pb + 0x5bd;
        } while (iVar7 != 0);
      }
      FUN_0046d300(&DAT_00557360, (int)((unsigned int)DAT_009fd394 * 8U + 8U));
      do {
        piVar10 = FUN_0046f5e0(&local_114, &local_108, 0x3c);
        if (piVar10 == (int *)0) {
          return 0;
        }
      } while (*piVar10 != 0x12);
    }
    DAT_00557364 = (unsigned int)DAT_009ecc34;
    DAT_00557360 = 0x20;
    pbSrc = DAT_00bb73a9;
    puDst = &DAT_00557368[0];
    for (iVar7 = 0x46; iVar7 != 0; iVar7 = iVar7 + -1) {
      __builtin_memcpy(puDst, pbSrc, 4);
      pbSrc = pbSrc + 4;
      puDst = puDst + 1;
    }
    iVar7 = 0;
    FUN_0046d350(0, &DAT_00557360, 0x120);
    FUN_0046fa30();
    while ((iVar8 = FUN_0046fa30(), iVar7 < iVar8 &&
            (piVar10 = FUN_0046f5e0(&local_114, &local_108, 0x1e), piVar10 != (int *)0))) {
      if (*piVar10 == 0x1d) {
        DVar11 = timeGetTime();
        if (200 < (int)(DVar11 - local_110)) {
          local_110 = DVar11;
          pcVar13 = FUN_0046f720((void *)FUN_0046f660(local_114));
          if (pcVar13 == (char *)0) {
            uVar14 = 0xffffffff;
            pcVar13 = DAT_00a60a2c;
            do {
              pcVar22 = pcVar13;
              if (uVar14 == 0) break;
              uVar14 = uVar14 - 1;
              pcVar22 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar22;
            } while (cVar1 != '\0');
            uVar14 = ~uVar14;
            pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
            pbVar23 = local_100;
            for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
              __builtin_memcpy(pbVar23, pbVar19, 4);
              pbVar19 = pbVar19 + 4;
              pbVar23 = pbVar23 + 4;
            }
          }
          else {
            uVar14 = 0xffffffff;
            do {
              pcVar22 = pcVar13;
              if (uVar14 == 0) break;
              uVar14 = uVar14 - 1;
              pcVar22 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar22;
            } while (cVar1 != '\0');
            uVar14 = ~uVar14;
            pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
            pbVar23 = local_100;
            for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
              __builtin_memcpy(pbVar23, pbVar19, 4);
              pbVar19 = pbVar19 + 4;
              pbVar23 = pbVar23 + 4;
            }
            for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
              *pbVar23 = *pbVar19;
              pbVar19 = pbVar19 + 1;
              pbVar23 = pbVar23 + 1;
            }
            bVar3 = !bVar3;
            pcVar13 = DAT_00a60a34;
            if (bVar3) {
              pcVar13 = DAT_00a60a30;
            }
            uVar14 = 0xffffffff;
            do {
              pcVar22 = pcVar13;
              if (uVar14 == 0) break;
              uVar14 = uVar14 - 1;
              pcVar22 = pcVar13 + 1;
              cVar1 = *pcVar13;
              pcVar13 = pcVar22;
            } while (cVar1 != '\0');
            uVar14 = ~uVar14;
            iVar8 = -1;
            pbVar19 = local_100;
            do {
              pbVar23 = pbVar19;
              if (iVar8 == 0) break;
              iVar8 = iVar8 + -1;
              pbVar23 = pbVar19 + 1;
              bVar2 = *pbVar19;
              pbVar19 = pbVar23;
            } while (bVar2 != 0);
            pbVar19 = (unsigned char *)(pcVar22 + -uVar14);
            pbVar23 = pbVar23 + -1;
            for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
              __builtin_memcpy(pbVar23, pbVar19, 4);
              pbVar19 = pbVar19 + 4;
              pbVar23 = pbVar23 + 4;
            }
          }
          for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
            *pbVar23 = *pbVar19;
            pbVar19 = pbVar19 + 1;
            pbVar23 = pbVar23 + 1;
          }
          FUN_00448f90(3, local_100, 0x30);
        }
      }
      else if (*piVar10 == 0x20) {
        iVar7 = iVar7 + 1;
        piVar20 = piVar10 + 2;
        piDst = &DAT_009d7800[(int)((unsigned int)piVar10[1] * 0x46U)];
        for (iVar8 = 0x46; iVar8 != 0; iVar8 = iVar8 + -1) {
          *piDst = *piVar20;
          piVar20 = piVar20 + 1;
          piDst = piDst + 1;
        }
      }
    }
    FUN_00448ed0();
  }
  return 1;
}
