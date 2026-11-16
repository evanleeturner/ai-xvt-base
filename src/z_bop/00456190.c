/* SPDX-License-Identifier: MIT */
/* z_bop 00456190 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

extern unsigned char DAT_009d8ccb[];
extern unsigned char DAT_009d8cca[];
extern unsigned char DAT_009ed22c;
extern unsigned char DAT_009ed22d;
extern unsigned char DAT_009ed228[];
extern unsigned char DAT_009d6940[];
extern unsigned short DAT_009a1834;
extern unsigned char DAT_0099f98a;
extern unsigned char DAT_0099f98c;
extern int DAT_009a1084;
extern int DAT_009a1088;
extern int DAT_009a108c;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern char DAT_009a2062;
extern unsigned char DAT_009d8c94[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d8c83[];
extern unsigned char DAT_009d8c80[];
extern unsigned char DAT_009d8c82[];
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d8c87[];
extern unsigned char DAT_009d8c89[];
extern unsigned char DAT_009d8c8d[];
extern unsigned char DAT_009d8c8e[];
extern unsigned char DAT_009d9155[];
extern short DAT_009a2000;
extern unsigned char DAT_009a20c8[];
extern unsigned char DAT_009d9192[];
extern unsigned char DAT_009d911c[];
extern unsigned char DAT_009d9122[];
extern int DAT_00a08294;
extern short DAT_009a8070;
extern short DAT_00a080f0;
extern short DAT_0099f988;
extern short DAT_0099f98e;
extern int DAT_009a20b0;
extern int DAT_009cd26c;
extern int DAT_009fd438;
extern unsigned char DAT_009a1832;
extern unsigned char DAT_0099f985;
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern unsigned int DAT_009a1fe8;
extern int DAT_00a08104;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern unsigned char DAT_0051c641[];
extern unsigned char DAT_0051c645[];
extern unsigned char DAT_0051c649[];
extern unsigned char DAT_0051c64d[];
extern unsigned char DAT_0051c651[];
extern unsigned char DAT_0051c655[];
extern unsigned char DAT_0099f984;
extern unsigned char DAT_0099f983;
extern unsigned char DAT_0099f98b;
extern unsigned char DAT_009a1081;
extern unsigned char DAT_009a1080;
extern unsigned char DAT_009a1830;
extern unsigned char DAT_00521f1b[];
extern unsigned char DAT_00524170[];
extern unsigned char DAT_0099f987;
extern unsigned char DAT_0099f990;
extern unsigned char DAT_0099f982;
extern unsigned char DAT_0099f992;
extern unsigned short DAT_009a20ca[];
extern unsigned char DAT_009a20cc[];
extern unsigned char DAT_009a2103[];

void __cdecl FUN_00458e20(int param_1, int param_2);
unsigned int __cdecl FUN_00425cb0(void);
void __cdecl FUN_0046a8d0(int param_1, int param_2, int param_3);
void __cdecl FUN_0046a870(int param_1, int param_2, int param_3);
void __cdecl FUN_004037b0(unsigned int param_1, int param_2, int param_3, int param_4);
unsigned int __stdcall FUN_00456ac0(void);
unsigned int __cdecl FUN_004269e0(unsigned int param_1);
void __cdecl FUN_00451940(unsigned int param_1, unsigned int param_2);

unsigned long long __cdecl FUN_00456190(unsigned short param_1)
{
  int bVar1;
  unsigned char cVar2, cVar3, cVar4;
  unsigned char bVar5;
  short sVar6;
  unsigned int uVar7;
  unsigned int uVar8;
  unsigned short uVar9;
  /* Unresolved: values left in registers by earlier calls (Ghidra extraout_*).
     The evidence does not identify the source registers; zero placeholders. */
  unsigned short extraout_var = 0;
  unsigned short extraout_var_00 = 0;
  unsigned int extraout_EDX = 0;
  unsigned int extraout_EDX_00 = 0;
  int extraout_EDX_01 = 0;
  int iVar10;
  int iVar11;
  unsigned char bVar12;
  unsigned int uVar13;
  int iVar14;
  unsigned short uVar15;
  unsigned char local_10[10];
  unsigned int uVar16;
  unsigned int base;
  int tA, tB, tC;

  cVar4 = DAT_009ed22d;
  cVar3 = DAT_009ed22c;
  cVar2 = DAT_009ed228[3];
  bVar1 = 0;
  uVar13 = 0;
  DAT_0099f98a = 0;
  DAT_0099f98c = 0;
  iVar11 = (int)((unsigned int)DAT_009a1834 * 0x566u);
  if (((((DAT_009d8ccb[iVar11] == 0) ||
         ((DAT_009ed22c == 0 && DAT_009ed22d == 0) && DAT_009ed228[3] == 0)) ||
        (DAT_009d6940[1] != 0)) || (param_1 != 0xffff))) {
    FUN_00458e20(0x8000, (unsigned int)DAT_009a1834);
    DAT_009a1084 = DAT_00a07cdc;
    DAT_009a1088 = DAT_00a07cd8;
    DAT_009a108c = DAT_00a07cd4;
    if ((cVar3 != 0 || cVar4 != 0) || cVar2 != 0) {
      if (DAT_009a2062 == 2) {
        uVar13 = (unsigned int)DAT_009a1834;
        iVar11 = (int)(uVar13 * 0x566u);
        if (DAT_009d8c94[iVar11] != 0) {
          local_10[0] = 0; local_10[1] = 0; local_10[2] = 0; local_10[3] = 0;
          local_10[4] = 0; local_10[5] = 0; local_10[6] = 0; local_10[7] = 0;
          local_10[8] = 0; local_10[9] = 0;
          bVar12 = 0;
          if (0 < DAT_009a2000) {
            do {
              if (DAT_009d8c94[(unsigned int)bVar12 * 0x566u] != 0) {
                local_10[DAT_009d8c88[(unsigned int)bVar12 * 0x566u]] =
                    (unsigned char)(local_10[DAT_009d8c88[(unsigned int)bVar12 * 0x566u]] + 1);
              }
              bVar12 = (unsigned char)(bVar12 + 1);
            } while ((short)(unsigned short)bVar12 < DAT_009a2000);
          }
          if (local_10[0] == 1) {
            if (DAT_009d8c83[iVar11] == 0) {
              RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            bVar12 = (unsigned char)(DAT_009a20c8[uVar13 * 0x126u] / DAT_009d8c83[iVar11]);
            if ((DAT_009d8c94[iVar11] & 1) == 0) {
              bVar12 = (unsigned char)(-bVar12);
            }
            bVar5 = 0;
            if (0 < DAT_009a2000) {
              do {
                if (DAT_009d8c94[(unsigned int)bVar5 * 0x566u] ==
                    (unsigned char)((((unsigned int)DAT_009d8c94[iVar11] +
                                      ((unsigned int)bVar12 - 3u) * 3u) & 7u) + 1u)) {
                  FUN_00458e20(0x8000, (unsigned int)bVar5);
                  break;
                }
                bVar5 = (unsigned char)(bVar5 + 1);
              } while ((short)(unsigned short)bVar5 < DAT_009a2000);
            }
          }
        }
        DAT_009a1084 = DAT_00a07cdc;
        DAT_009a1088 = DAT_00a07cd8;
        DAT_009a108c = DAT_00a07cd4;
        __builtin_memcpy(&tA, &DAT_009d9192[(unsigned int)DAT_009a1834 * 0x566u], 4);
        if (tA != -1) {
          uVar15 = (unsigned short)FUN_00425cb0();
          uVar7 = FUN_00425cb0();
          uVar13 = uVar15 & 0x7fff;
          if ((uVar7 & 1) == 0) {
            uVar13 = (unsigned int)(-(int)uVar13);
          }
          DAT_009a1084 = (int)((unsigned int)DAT_009a1084 + uVar13);
          uVar15 = (unsigned short)FUN_00425cb0();
          uVar13 = FUN_00425cb0();
          if ((uVar13 & 1) == 0) {
            DAT_009a1088 = (int)((unsigned int)DAT_009a1088 - (unsigned int)(uVar15 & 0x7fff));
          } else {
            DAT_009a1088 = (int)((unsigned int)DAT_009a1088 + (unsigned int)(uVar15 & 0x7fff));
          }
        }
      }
      else if (DAT_00a08294 == 0xe) {
        iVar11 = (int)((unsigned int)DAT_009a1834 * 0x566u);
        if ((DAT_009d8c94[iVar11] != 0) && (*(short *)(&DAT_009d911c[iVar11]) != 0)) {
          FUN_00458e20(0x8001, (unsigned int)DAT_009a1834);
          DAT_009a1084 = DAT_00a07cdc;
          DAT_009a1088 = DAT_00a07cd8;
          DAT_009a108c = DAT_00a07cd4;
        }
      }
    }
    if (*(short *)(&DAT_009d9122[(unsigned int)DAT_009a1834 * 0x566u]) == 0) {
      DAT_009a8070 = 0;
      DAT_00a080f0 = 0x4000;
    } else {
      FUN_00458e20(0x8004, (unsigned int)DAT_009a1834);
      FUN_0046a8d0((int)((unsigned int)DAT_00a07cdc - (unsigned int)DAT_009a1084),
                   (int)((unsigned int)DAT_00a07cd8 - (unsigned int)DAT_009a1088),
                   (int)((unsigned int)DAT_00a07cd4 - (unsigned int)DAT_009a108c));
    }
    DAT_0099f988 = DAT_009a8070;
    DAT_0099f98e = DAT_00a080f0;
    __builtin_memcpy(&tA, &DAT_009d9192[(unsigned int)DAT_009a1834 * 0x566u], 4);
    if ((((DAT_009d6940[1] == 0) && (DAT_009d8ccb[(unsigned int)DAT_009a1834 * 0x566u] == 0)) &&
         ((cVar3 != 0 || cVar4 != 0) || cVar2 != 0)) &&
        ((param_1 == 0xffff && (tA == -1)))) {
      DAT_009a8070 = (short)(DAT_009a8070 - 0x8000);
      sVar6 = (short)(-0x8000 - (int)DAT_00a080f0);
      DAT_00a080f0 = sVar6;
      FUN_0046a870(0xffff, DAT_009a8070, sVar6);
      DAT_009a20b0 = (int)((unsigned int)DAT_009a20b0 * 8u);
      DAT_009cd26c = (int)((unsigned int)DAT_009cd26c * 8u);
      DAT_009fd438 = (int)((unsigned int)DAT_009fd438 * 8u);
      DAT_009a1084 = (int)((unsigned int)DAT_009a1084 + (unsigned int)DAT_009a20b0);
      DAT_009a1088 = (int)((unsigned int)DAT_009a1088 + (unsigned int)DAT_009cd26c);
      DAT_009a108c = (int)((unsigned int)DAT_009a108c + (unsigned int)DAT_009fd438);
      DAT_0099f98a = 1;
    }
    DAT_009a1832 = DAT_009d8c8d[(unsigned int)DAT_009a1834 * 0x566u];
    DAT_0099f985 = DAT_009d8c8e[(unsigned int)DAT_009a1834 * 0x566u];
  }
  else {
    uVar13 = (unsigned int)DAT_00a07cec;
    if ((int)uVar13 < DAT_009a1fe0) {
      uVar13 = 0;
      uVar15 = DAT_00a07cec;
      uVar16 = 0;
      do {
        base = uVar15 * 0x23u + DAT_009a1fe8;
        if (*(unsigned char *)(base + 3) != 0) {
          __builtin_memcpy(&tA, (unsigned char *)(base + 0x1f), 4);
          __builtin_memcpy(&DAT_00a08104, (unsigned char *)(unsigned int)tA + 0xa9, 4);
          if (*(unsigned char *)(base + 0x16) == DAT_009d8cca[iVar11]) {
            if (*(unsigned char *)((unsigned int)DAT_00a08104 + 5) == 0xff) {
              bVar1 = 1;
              uVar16 = uVar15;
              goto LAB_004562ae;
            }
          }
        }
        uVar15 = (unsigned short)(uVar15 + 1);
      } while ((int)(unsigned int)uVar15 < DAT_009a1fe0);
      bVar1 = 0;
    }
LAB_004562ae:
    if (!bVar1) {
      return (((unsigned long long)uVar13 << 32) | (unsigned int)iVar11) & 0xffffffffffff0000ULL;
    }
    iVar11 = (int)((uVar16 & 0xffff) * 0x23u);
    __builtin_memcpy(&tA, (unsigned char *)(DAT_009a1fe8 + iVar11 + 0x1f), 4);
    __builtin_memcpy(&DAT_00a08104, (unsigned char *)(unsigned int)tA + 0xa9, 4);
    iVar14 = (int)((unsigned int)*(unsigned char *)((unsigned int)DAT_00a08104 + 4) * 0x101u);
    __builtin_memcpy(&tA, &DAT_0051c641[iVar14], 4);
    __builtin_memcpy(&tB, &DAT_0051c645[iVar14], 4);
    __builtin_memcpy(&tC, &DAT_0051c649[iVar14], 4);
    FUN_004037b0(DAT_009a1fe8 + iVar11, tA, tB, tC);
    iVar10 = (int)(DAT_009a1fe8 + iVar11);
    DAT_009a1084 = (int)((unsigned int)*(int *)(iVar10 + 4) + (unsigned int)DAT_009d8a58);
    DAT_009a1088 = (int)((unsigned int)*(int *)(iVar10 + 8) + (unsigned int)DAT_009d8a54);
    DAT_009a108c = (int)((unsigned int)*(int *)(iVar10 + 0xc) + (unsigned int)DAT_009d8b60);
    __builtin_memcpy(&tA, &DAT_0051c64d[iVar14], 4);
    __builtin_memcpy(&tB, &DAT_0051c651[iVar14], 4);
    __builtin_memcpy(&tC, &DAT_0051c655[iVar14], 4);
    FUN_004037b0((unsigned int)iVar10, tA, tB, tC);
    iVar11 = (int)(DAT_009a1fe8 + iVar11);
    DAT_00a07cdc = (int)((unsigned int)*(int *)(iVar11 + 4) + (unsigned int)DAT_009d8a58);
    DAT_00a07cd8 = (int)((unsigned int)*(int *)(iVar11 + 8) + (unsigned int)DAT_009d8a54);
    DAT_00a07cd4 = (int)((unsigned int)*(int *)(iVar11 + 0xc) + (unsigned int)DAT_009d8b60);
    FUN_0046a8d0((int)((unsigned int)DAT_00a07cdc - (unsigned int)DAT_009a1084),
                 (int)((unsigned int)DAT_00a07cd8 - (unsigned int)DAT_009a1088),
                 (int)((unsigned int)DAT_00a07cd4 - (unsigned int)DAT_009a108c));
    DAT_0099f98c = 1;
    DAT_0099f988 = DAT_009a8070;
    DAT_0099f98e = DAT_00a080f0;
    DAT_009a1832 = 0;
    if (3 < DAT_009d8c83[(unsigned int)DAT_009a1834 * 0x566u]) {
      DAT_009a1832 = 6;
    }
    DAT_0099f985 = 0;
  }
  uVar7 = (unsigned int)DAT_009a1834;
  uVar13 = uVar7 * 0x566u;
  DAT_0099f984 = DAT_009d8c87[uVar13];
  DAT_0099f983 = DAT_009d8c88[uVar13];
  DAT_0099f98b = DAT_009d8c84[uVar13];
  DAT_009a1081 = DAT_009d9155[uVar13];
  iVar14 = 0;
  DAT_009a1080 = 0;
  DAT_009a1830 = DAT_00521f1b[(unsigned int)DAT_00524170[DAT_009d8c82[uVar13]] * 0x1au];
  DAT_0099f987 = DAT_009d8c89[uVar13];
  iVar11 = (int)(((((unsigned int)DAT_00524170[DAT_009d8c82[uVar13]] * 0x19u) >> 8) << 8) |
                 (unsigned int)DAT_0099f987);
  if (param_1 == 0xffff) {
    DAT_0099f990 = 0;
    DAT_0099f982 = 0xff;
    if (DAT_009d8c83[uVar13] != 0) {
      do {
        if (*(unsigned short *)(&DAT_009a20cc[(unsigned int)DAT_009a1834 * 0x126u]) <
            DAT_009a20ca[(unsigned int)DAT_009a1834 * 0x93u]) {
          uVar8 = FUN_00456ac0();
          if ((short)uVar8 == -1) {
            return (((unsigned long long)extraout_EDX << 32) | uVar8) & 0xffffffffffff0000ULL;
          }
          iVar14 = iVar14 + 1;
          uVar13 = (unsigned int)DAT_009a1834;
          *(short *)(&DAT_009a20cc[uVar13 * 0x126u]) =
              (short)(*(short *)(&DAT_009a20cc[uVar13 * 0x126u]) + 1);
          if (DAT_009d8c80[uVar13 * 0x566u] == DAT_0099f990) {
            DAT_009a2103[uVar13 * 0x126u] = (unsigned char)(DAT_009a2103[uVar13 * 0x126u] + 1);
          }
        }
        DAT_0099f990 = (unsigned char)(DAT_0099f990 + 1);
        iVar11 = (int)((unsigned int)DAT_009a1834 * 0x2b3u);
        uVar13 = (unsigned int)DAT_0099f990;
      } while (uVar13 < DAT_009d8c83[(unsigned int)DAT_009a1834 * 0x566u]);
    }
  }
  else {
    uVar13 = uVar7 * 0x126u;
    if (*(unsigned short *)(&DAT_009a20cc[uVar13]) < DAT_009a20ca[uVar7 * 0x93u]) {
      DAT_0099f990 = (unsigned char)param_1;
      uVar8 = FUN_00456ac0();
      if ((short)uVar8 == -1) {
        return (((unsigned long long)extraout_EDX_00 << 32) | uVar8) & 0xffffffffffff0000ULL;
      }
      iVar14 = 1;
      uVar7 = (unsigned int)DAT_009a1834;
      *(short *)(&DAT_009a20cc[uVar7 * 0x126u]) =
          (short)(*(short *)(&DAT_009a20cc[uVar7 * 0x126u]) + 1);
      iVar11 = (int)(uVar7 * 0x2b3u);
      uVar13 = (unsigned int)DAT_0099f990;
      if (DAT_009d8c80[uVar7 * 0x566u] == uVar13) {
        DAT_009a2103[uVar7 * 0x126u] = (unsigned char)(DAT_009a2103[uVar7 * 0x126u] + 1);
      }
    }
  }
  uVar9 = (unsigned short)(uVar13 >> 16);
  if ((((cVar3 != 0 || cVar4 != 0) || cVar2 != 0) && (param_1 == 0xffff)) && (iVar14 != 0)) {
    uVar13 = FUN_004269e0((((uVar13 >> 8) << 8) | (unsigned int)DAT_0099f992) & 0xffff00ffu);
    FUN_00451940(((unsigned int)extraout_var_00 << 16) | (unsigned int)DAT_009a1834, uVar13);
    iVar11 = extraout_EDX_01;
    uVar9 = extraout_var;
  }
  return ((unsigned long long)(unsigned int)iVar11 << 32) |
         (((unsigned int)uVar9 << 16) | 1u);
}
