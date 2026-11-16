/* SPDX-License-Identifier: MIT */
/* z_bop 00415c90 */
extern short DAT_009ec5e0[];
extern unsigned char DAT_009ec5f6;
extern unsigned short DAT_009ec5fe;
extern int DAT_00523410;
extern short DAT_009d8b80[];
extern unsigned char DAT_009d8b90;
extern char DAT_009e9680;
extern unsigned char DAT_009ec468;
extern unsigned char DAT_009e9c1d;
extern unsigned char DAT_009eca05;
extern int _DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_00a08104;
extern int DAT_009d1144;
extern int DAT_009fe7dc;
extern short _DAT_009a8064;
extern unsigned short _DAT_009d77bc;
extern int DAT_009ecc34;
extern unsigned char DAT_009e968a;
extern unsigned char DAT_009e9670;
extern unsigned char DAT_009e9699;
extern int DAT_009a8c00;
extern unsigned char DAT_00521f1c;
extern unsigned short DAT_009d682e;
extern unsigned short DAT_009cc450;
extern unsigned short _DAT_00a0810a;
extern short _DAT_009ed22e;
extern unsigned char DAT_009ed22d;
extern unsigned char DAT_009ed22c;
extern unsigned char DAT_009ed22b;
extern unsigned char DAT_009d8b74;
extern unsigned char DAT_009d8b75;
extern unsigned char DAT_009d695c;
extern unsigned char DAT_009d6940;
extern int DAT_009d6964;
extern unsigned char DAT_009d695d;
extern unsigned char DAT_009d695e;
extern unsigned char DAT_009a2062;
extern unsigned char DAT_009d8c94[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d738a[];
extern unsigned char DAT_009d738b[];
extern unsigned char DAT_009d738d;
extern unsigned short DAT_0051bf60[];
extern unsigned char DAT_0051bf74;
extern unsigned char DAT_0051bf78[];
extern unsigned char DAT_009ec458;
extern unsigned short _DAT_00a07be0;
extern unsigned short _DAT_00a07be2;
extern int DAT_00a07ce8;

void __cdecl FUN_004034e0(unsigned int, unsigned int);
void __cdecl FUN_0042e4d0(int, int, int);
void __cdecl FUN_00450650(int, int);
void __cdecl FUN_00451590(void);

void __stdcall FUN_00415c90(void)
{
  int iVar1;
  char cVar2;
  int bVar3;
  short sVar4;
  char *pcVar5;
  int iVar6;
  int iVar7;
  short *psVar10;
  short *psVar11;
  unsigned short uVar9;
  unsigned short uVar12;
  unsigned char *piVar13;
  int iVar14;
  unsigned short *puVar15;
  int iVar16;
  unsigned char *pbVar17;
  unsigned int uVar18;
  unsigned short local_12;
  int local_c[3];
  int ti;
  int ti2;
  unsigned int tu;
  short ts;
  unsigned short th;
  unsigned short th2;
  int zero32;
  short zero16;
  unsigned char *rp;

  psVar10 = (short *)&DAT_009ec5e0;
  do {
    if ((*psVar10 != 0) &&
        (sVar4 = (short)(*psVar10 - DAT_009ec5fe), *psVar10 = sVar4, sVar4 < 0)) {
      *psVar10 = 0;
    }
    psVar10 = psVar10 + 1;
  } while (psVar10 < (short *)&DAT_009ec5f6);

  if (DAT_00523410 == 0) {
    psVar10 = DAT_009d8b80;
    do {
      pcVar5 = &DAT_009e9680;
      psVar11 = psVar10;
      do {
        if (((*pcVar5 != '\0') && (*psVar11 != 0)) &&
            (sVar4 = (short)(*psVar11 - DAT_009ec5fe), *psVar11 = sVar4, sVar4 < 0)) {
          *psVar11 = 0;
        }
        psVar11 = psVar11 + 8;
        pcVar5 = pcVar5 + 0x5bd;
      } while ((unsigned char *)pcVar5 < &DAT_009ec468);
      psVar10 = psVar10 + 1;
    } while (psVar10 < (short *)&DAT_009d8b90);
  }

  piVar13 = &DAT_009e9c1d;
  do {
    __builtin_memcpy(&ti, piVar13, 4);
    if (ti != 0) {
      iVar6 = (int)((unsigned int)ti - (unsigned int)DAT_009ec5fe);
      __builtin_memcpy(piVar13, &iVar6, 4);
      if (iVar6 < 0) {
        zero32 = 0;
        __builtin_memcpy(piVar13, &zero32, 4);
      }
    }
    __builtin_memcpy(&ti, piVar13 + 4, 4);
    if (ti != 0) {
      iVar6 = (int)((unsigned int)ti - (unsigned int)DAT_009ec5fe);
      __builtin_memcpy(piVar13 + 4, &iVar6, 4);
      if (iVar6 < 0) {
        zero32 = 0;
        __builtin_memcpy(piVar13 + 4, &zero32, 4);
      }
    }
    piVar13 = piVar13 + 0x5bd;
  } while (piVar13 < &DAT_009eca05);

  if (_DAT_00a07cec < DAT_009a1fe0) {
    iVar6 = (int)((unsigned int)_DAT_00a07cec * 0x23u);
    iVar14 = _DAT_00a07cec;
    do {
      if (*(char *)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar6 + 3u) != '\0') {
        __builtin_memcpy(&ti,
                         (unsigned char *)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar6 + 0x1fu),
                         4);
        __builtin_memcpy(&iVar16, (unsigned char *)((unsigned int)ti + 0xa9u), 4);
        DAT_00a08104 = iVar16;
        rp = (unsigned char *)iVar16;
        if (*(int *)(rp + 0x3c) != 0) {
          *(unsigned int *)(rp + 0x3c) =
              *(unsigned int *)(rp + 0x3c) - (unsigned int)DAT_009ec5fe;
        }
        if ((*(int *)(rp + 0x60) != 0) &&
            (iVar7 = (int)(*(unsigned int *)(rp + 0x60) - (unsigned int)DAT_009ec5fe),
             *(int *)(rp + 0x60) = iVar7, iVar7 < 0)) {
          *(int *)(rp + 0x60) = 0;
        }
        if ((*(short *)(rp + 100) != 0) &&
            (sVar4 = (short)(*(short *)(rp + 100) - DAT_009ec5fe),
             *(short *)(rp + 100) = sVar4, sVar4 < 0)) {
          *(short *)(rp + 100) = 0;
        }
        __builtin_memcpy(&uVar12, (unsigned char *)DAT_00a08104 + 0x1e7, 2);
        if (uVar12 != 0) {
          th = (unsigned short)(uVar12 - DAT_009ec5fe);
          __builtin_memcpy((unsigned char *)DAT_00a08104 + 0x1e7, &th, 2);
          if (uVar12 < th) {
            th2 = 0;
            __builtin_memcpy((unsigned char *)DAT_00a08104 + 0x1e7, &th2, 2);
          }
        }
        if (*(short *)((unsigned char *)DAT_00a08104 + 0x24a) != 0) {
          *(short *)((unsigned char *)DAT_00a08104 + 0x24a) =
              (short)(*(short *)((unsigned char *)DAT_00a08104 + 0x24a) - DAT_009ec5fe);
          if (*(short *)((unsigned char *)DAT_00a08104 + 0x24a) < 0) {
            *(short *)((unsigned char *)DAT_00a08104 + 0x24a) = 0;
          }
        }
        iVar16 = 0;
        if (*((unsigned char *)DAT_00a08104 + 0x21d) != 0) {
          iVar7 = 0;
          do {
            iVar1 = (int)((unsigned int)DAT_00a08104 + 0x3b2u + (unsigned int)iVar7);
            sVar4 = *(short *)((unsigned char *)iVar1 + 2);
            if (0 < sVar4) {
              *(unsigned short *)((unsigned char *)iVar1 + 2) =
                  (unsigned short)(sVar4 - DAT_009ec5fe);
            }
            iVar7 = (int)((unsigned int)iVar7 + 4u);
            iVar16 = (int)((unsigned int)iVar16 + 1u);
          } while (iVar16 < (int)(unsigned int)*((unsigned char *)DAT_00a08104 + 0x21d));
        }
      }
      iVar6 = (int)((unsigned int)iVar6 + 0x23u);
      iVar14 = (int)((unsigned int)iVar14 + 1u);
    } while (iVar14 < DAT_009a1fe0);
  }

  if (DAT_009d1144 < DAT_009fe7dc) {
    iVar6 = (int)((unsigned int)DAT_009d1144 * 0x23u);
    iVar14 = DAT_009d1144;
    do {
      if (*(char *)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar6 + 3u) != '\0') {
        __builtin_memcpy(&ti,
                         (unsigned char *)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar6 + 0x1fu),
                         4);
        __builtin_memcpy(&iVar16, (unsigned char *)((unsigned int)ti + 0xadu), 4);
        rp = (unsigned char *)iVar16;
        __builtin_memcpy(&ti2, rp + 0x16, 4);
        if (ti2 != 0) {
          tu = (unsigned int)ti2 - (unsigned int)DAT_009ec5fe;
          __builtin_memcpy(rp + 0x16, &tu, 4);
        }
        __builtin_memcpy(&ti2, rp + 0x3a, 4);
        if (ti2 != 0) {
          iVar7 = (int)((unsigned int)ti2 - (unsigned int)DAT_009ec5fe);
          __builtin_memcpy(rp + 0x3a, &iVar7, 4);
          if (iVar7 < 0) {
            zero32 = 0;
            __builtin_memcpy(rp + 0x3a, &zero32, 4);
          }
        }
        if ((*(short *)(rp + 0x3e) != 0) &&
            (sVar4 = (short)(*(short *)(rp + 0x3e) - DAT_009ec5fe),
             *(short *)(rp + 0x3e) = sVar4, sVar4 < 0)) {
          *(short *)(rp + 0x3e) = 0;
        }
      }
      iVar6 = (int)((unsigned int)iVar6 + 0x23u);
      iVar14 = (int)((unsigned int)iVar14 + 1u);
    } while (iVar14 < DAT_009fe7dc);
  }

  _DAT_009a8064 = (short)(_DAT_009a8064 - DAT_009ec5fe);
  if (_DAT_009a8064 < 0) {
    _DAT_009d77bc = _DAT_009d77bc ^ 0x400;
    __builtin_memcpy(&uVar12, &DAT_009e968a + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
    if (uVar12 != 0xffff) {
      __builtin_memcpy(&uVar18, &DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
      if (uVar18 != 0xffffffff) {
        FUN_004034e0((unsigned int)uVar12, uVar18);
        DAT_009a8c00 = DAT_009a8c00 >> 5;
        __builtin_memcpy(&th, &DAT_009e968a + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
        __builtin_memcpy(&local_c[0],
                         &DAT_00521f1c +
                             (unsigned int)*(unsigned char *)((unsigned int)th * 0x23u + 3u +
                                                              (unsigned int)DAT_009a1fe8) * 0x1au,
                         4);
      }
    }
    if ((_DAT_009d77bc & 0x400) == 0) {
      _DAT_009a8064 = 0xe;
      if (local_c[0] <= DAT_009a8c00) goto LAB_00416013;
    }
    else if (DAT_009a8c00 < local_c[0]) {
LAB_00416013:
      _DAT_009a8064 = 0x76;
    }
    else {
      _DAT_009a8064 = 0xe;
    }
  }

  __builtin_memcpy(&DAT_009d682e, &DAT_009e9699 + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
  __builtin_memcpy(&th, &DAT_009e968a + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
  DAT_009cc450 = th | _DAT_00a0810a | _DAT_009d77bc;

  _DAT_009ed22e = (short)(_DAT_009ed22e - DAT_009ec5fe);
  if (0 < _DAT_009ed22e) {
    return;
  }
  _DAT_009ed22e = (short)(_DAT_009ed22e + 0xec);
  DAT_009ed22d = (unsigned char)(DAT_009ed22d + 1);
  if (0x3b < DAT_009ed22d) {
    DAT_009ed22d = 0;
    DAT_009ed22c = (unsigned char)(DAT_009ed22c + 1);
    if (0x3b < DAT_009ed22c) {
      DAT_009ed22c = 0;
      DAT_009ed22b = (unsigned char)(DAT_009ed22b + 1);
      if (0x17 < DAT_009ed22b) {
        DAT_009ed22b = 0;
      }
    }
  }

  if ((DAT_009d8b74 == 0) && (DAT_009d8b75 == 0)) {
LAB_00416158:
    if (DAT_009d695c != 0) {
      if ((DAT_009d8b74 == 2) && (DAT_009d8b75 == 0)) {
        FUN_0042e4d0(0x23, -1, DAT_009ecc34);
        FUN_00450650(0xc9, DAT_009ecc34);
      }
      if ((DAT_009d8b74 == 1) && (DAT_009d8b75 == 0)) {
        FUN_0042e4d0(0x5c, -1, DAT_009ecc34);
        FUN_00450650(0xca, DAT_009ecc34);
      }
      if (DAT_009d8b74 == 0) {
        if (DAT_009d8b75 == 0x0f) {
          __builtin_memcpy(&iVar6, &DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
          if ((iVar6 == -1) ||
              ((((cVar2 = *(char *)((unsigned int)iVar6 * 0x23u + 3u +
                                    (unsigned int)DAT_009a1fe8),
                  cVar2 != '\x01') &&
                 (cVar2 != '\x02')) && (cVar2 != '\x03')) &&
               ((cVar2 != '\x0e' && (cVar2 != '\x04'))))) {
            bVar3 = 0;
          }
          else {
            bVar3 = 1;
          }
          if (bVar3) {
            uVar18 = 0x57;
          }
          else {
            FUN_0042e4d0(0x5c, -1, DAT_009ecc34);
            uVar18 = 0x5c;
          }
          FUN_0042e4d0((int)uVar18, -1, DAT_009ecc34);
        }
        if ((DAT_009d8b74 == 0) && (DAT_009d8b75 == 2)) {
          FUN_00450650(0xcb, DAT_009ecc34);
        }
      }
    }
  }
  else {
    DAT_009d8b75 = (unsigned char)(DAT_009d8b75 - 1);
    if (DAT_009d8b75 == 0xff) {
      DAT_009d8b75 = 0x3b;
      DAT_009d8b74 = (unsigned char)(DAT_009d8b74 - 1);
      if (DAT_009d8b74 == 0xff) {
        DAT_009d8b75 = 0;
        DAT_009d8b74 = 0;
      }
    }
    if (DAT_009d695c != 0) {
      if (((DAT_009d8b74 == 0) && (DAT_009d8b75 == 0)) && (DAT_00523410 == 0)) {
        pcVar5 = &DAT_009e9680;
        do {
          *pcVar5 = 2;
          pcVar5 = pcVar5 + 0x5bd;
        } while ((unsigned char *)pcVar5 < &DAT_009ec468);
        DAT_009d6940 = 1;
      }
      goto LAB_00416158;
    }
  }

  if (((DAT_009d6964 < 2) || (DAT_009d695d == 0)) || (DAT_009d695e != 0)) goto LAB_0041643e;
  if (((DAT_009a2062 == 2) || (DAT_009a2062 == 4)) &&
      ((DAT_009d695d < DAT_009d8b74) || ((DAT_009d8b74 == 0 && (DAT_009d8b75 == 0))))) {
    local_c[0] = 0;
    local_c[1] = 0;
    zero16 = 0;
    __builtin_memcpy((unsigned char *)local_c + 8, &zero16, 2);
    pcVar5 = &DAT_009e9680;
    do {
      if (*pcVar5 == '\x01') {
        __builtin_memcpy(&th, pcVar5 - 4, 2);
        *((unsigned char *)local_c + (unsigned int)th) = 1;
      }
      pcVar5 = pcVar5 + 0x5bd;
    } while ((unsigned char *)pcVar5 < &DAT_009ec468);
    if ((DAT_009a2062 == 2) && (_DAT_00a07cec < DAT_009a1fe0)) {
      iVar6 = (int)((unsigned int)DAT_009a1fe0 - (unsigned int)_DAT_00a07cec);
      pbVar17 = (unsigned char *)((unsigned int)_DAT_00a07cec * 0x23u + 0x16u +
                                  (unsigned int)DAT_009a1fe8);
      do {
        if ((pbVar17[-0x13] != 0) && (DAT_009d8c94[(unsigned int)*pbVar17 * 0x566u] != 0)) {
          *((unsigned char *)local_c +
            (unsigned int)DAT_009d8c88[(unsigned int)*pbVar17 * 0x566u]) = 1;
        }
        pbVar17 = pbVar17 + 0x23;
        iVar6 = (int)((unsigned int)iVar6 - 1u);
      } while (iVar6 != 0);
    }
    iVar14 = 0;
    iVar6 = 0;
    do {
      if (*((char *)local_c + iVar6) != '\0') {
        iVar14 = (int)((unsigned int)iVar14 + 1u);
        local_c[0] = iVar6;
      }
      iVar6 = (int)((unsigned int)iVar6 + 1u);
    } while (iVar6 < 10);
    if (iVar14 == 1) {
      if (DAT_009a2062 != 2) {
        if (((DAT_009d738a[(unsigned int)local_c[0] * 3u] != 1) &&
             (DAT_009d738a[(unsigned int)local_c[0] * 3u] != 2)) &&
            (DAT_009d738b[(unsigned int)local_c[0] * 3u] != 1)) goto LAB_004163fd;
      }
      DAT_009d8b75 = 0;
      DAT_009d695e = 1;
      DAT_009d8b74 = DAT_009d695d;
      DAT_009d695c = DAT_009d695d;
    }
  }
LAB_004163fd:
  if (((DAT_009d695e == 0) && (DAT_009a2062 == 4)) &&
      ((DAT_009d738a[0] == 1) || (DAT_009d738d == 1))) {
    DAT_009d8b75 = 0;
    DAT_009d695e = 1;
    DAT_009d8b74 = DAT_009d695d;
    DAT_009d695c = DAT_009d695d;
  }

LAB_0041643e:
  local_c[0] = 0;
  piVar13 = &DAT_009e9670;
  do {
    __builtin_memcpy(&ti, piVar13, 4);
    if ((piVar13[0x10] != 0) && (ti != -1)) {
      uVar12 = 0xffff;
      local_12 = 0xffff;
      __builtin_memcpy(&ti2,
                       (unsigned char *)((unsigned int)ti * 0x23u + 0x1fu +
                                         (unsigned int)DAT_009a1fe8),
                       4);
      __builtin_memcpy(&iVar6, (unsigned char *)((unsigned int)ti2 + 0xa9u), 4);
      rp = (unsigned char *)iVar6;
      __builtin_memcpy(&th, rp + 0x1e5, 2);
      if (th != 0) {
        iVar14 = 0;
        do {
          if ((*(short *)(rp + 0x2ac + (unsigned int)iVar14 * 2u) == 0) &&
              (uVar9 = (unsigned short)*(rp + 0x2a1 + (unsigned int)iVar14), uVar9 < uVar12)) {
            local_12 = (unsigned short)iVar14;
            uVar12 = uVar9;
          }
          iVar14 = (int)((unsigned int)iVar14 + 1u);
        } while (iVar14 < 10);
        uVar18 = 0;
        puVar15 = DAT_0051bf60;
        psVar10 = (short *)(rp + 0x2c2);
        do {
          if ((psVar10[-0xb] == 0) && (local_12 == uVar18)) {
            if (*psVar10 == 0) {
              psVar10[-0xb] = 100;
              __builtin_memcpy(&th, rp + 0x1e5, 2);
              th = (unsigned short)(th | *puVar15);
              __builtin_memcpy(rp + 0x1e5, &th, 2);
              _DAT_00a07be2 = 0x58;
              _DAT_00a07be0 = (unsigned short)DAT_0051bf78[uVar18];
              FUN_00450650(0x56, local_c[0]);
            }
            else {
              *psVar10 = (short)(*psVar10 + -1);
            }
          }
          puVar15 = puVar15 + 1;
          psVar10 = psVar10 + 1;
          uVar18 = uVar18 + 1;
        } while ((unsigned char *)puVar15 < &DAT_0051bf74);
      }
    }
    piVar13 = piVar13 + 0x5bd;
    local_c[0] = (int)((unsigned int)local_c[0] + 1u);
  } while (piVar13 < &DAT_009ec458);

  iVar6 = 0;
  if (0 < DAT_00a07ce8) {
    iVar14 = 0;
    do {
      if (*(char *)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar14 + 3u) != '\0') {
        __builtin_memcpy(&ti,
                         (unsigned char *)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar14 + 0x1fu),
                         4);
        __builtin_memcpy(&ts, (unsigned char *)((unsigned int)ti + 0x83u), 2);
        ts = (short)(ts + 1);
        __builtin_memcpy((unsigned char *)((unsigned int)ti + 0x83u), &ts, 2);
      }
      iVar14 = (int)((unsigned int)iVar14 + 0x23u);
      iVar6 = (int)((unsigned int)iVar6 + 1u);
    } while (iVar6 < DAT_00a07ce8);
  }
  FUN_00451590();
  return;
}
