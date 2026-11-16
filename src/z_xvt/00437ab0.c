/* SPDX-License-Identifier: MIT */
/* z_xvt 00437ab0 */
/* Globals keep the decompiler's placeholder names. The types are inferred and are not confirmed by the evidence. */
extern int   DAT_005ef238;
extern int   DAT_0073290c;
extern int   DAT_00525554;
extern int   DAT_005ef23c;
extern int   DAT_005ef230;
extern int   DAT_005ef234;
extern int   DAT_00525790[20];   /* DAT_00525790..DAT_005257dc; DAT_005257dc is element 19 */
extern int   DAT_005257e0;
extern uint  DAT_006db5a8;       /* type inferred (uint); signedness of compares follows Ghidra's casts */
extern uint  DAT_0071b71c;       /* type inferred */
extern uint  DAT_00732a18;       /* type inferred */
extern int   DAT_006db5a4;
extern int   DAT_0071ecf4;
extern char  DAT_0071b740;
extern char  DAT_0071e528;
extern int   DAT_0072f444;
extern char  DAT_007092c0;
extern int   DAT_00732e48;
extern int   DAT_00525558;
extern int   DAT_00525550;
extern int   DAT_0052554c;
extern int   ram0x0071f2ec;
extern int   DAT_0070aaf0[];     /* element width unconfirmed */
extern char *DAT_006a89a8;
extern char *DAT_006a89c4;
extern byte *DAT_006a89b0;
extern byte *DAT_006a89c0;
extern ushort DAT_006d4a4e;
extern int   DAT_00525520;
extern int   DAT_0052551c;
extern int   DAT_00525524;
extern int   DAT_00525528;
extern int   DAT_0052ae54;
extern void (*DAT_00732a30)(int, uint, uint, uint, uint);
extern int   DAT_0071e51c;
extern int   DAT_006db57c;
extern char  DAT_00525720;
extern char  DAT_00732800[];
extern int   DAT_0071b704;
extern int   DAT_00732904;
extern int   DAT_006d4988;
extern char  s_Fell_Behind__tickcounter___7d_se_00525a0c[];
extern char  s_Too_far_Ahead__tickcounter___7d_s_005259d0[];
extern char  s_R___2d_U___2d_N___2d_O___2d_T____00525980[];
extern char  s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_00525934[];
extern char  s_Raw_10__2d_11__2d_12__2d_13__2d_1_005258e8[];
extern char  s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052589c[];
extern char  s_Pct_10__2d_11__2d_12__2d_13__2d_1_00525850[];
extern char  s________Long_Update___d_______War_00525804[];

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
int  __cdecl _sprintf(char *, const char *, ...);
int  FUN_0048f530(void);
void FUN_0042c710(void);
int  FUN_00431d30(void);
void FUN_0042d530(void);
void FUN_0049a600(uint);
void FUN_00490a00(void);
void FUN_0042c5c0(int);
void FUN_00438a40(void);
char *FUN_0042b340(void);
void FUN_00438c30(int, byte *, int);
int  FUN_004a53f0(void);
uint FUN_004a5470(void);
void FUN_0042c5a0(void);
void FUN_0048ee20(char);
void FUN_004a7040(void);
void FUN_004a6a80(void);
void FUN_004a6f90(void);
void FUN_0049adc0(int);
int  FUN_00432ca0(void);
int  FUN_00433680(int);
void FUN_0049b4e0(void);
/* FINDING: the callee is __fastcall with param_1 in ECX, but the caller passes whatever ECX holds
   after FUN_0049b4e0 (extraout_ECX), which C cannot express. Declared without the parameter so the
   call site leaves ECX untouched, as the original does. */
void __fastcall FUN_0045bb40(void);
void FUN_0049b2e0(void);
void FUN_0042c670(int);
void FUN_0042c600(int);

void __stdcall FUN_00437ab0(void)
{
  byte bVar1;
  char cVar2;
  int iVar3;
  char *pcVar4;
  int iVar5;
  int *piVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  byte *pbVar10;
  uint uVar11;
  int iVar12;
  uint *puVar13;
  char *pcVar14;
  byte *pbVar15;
  uint uVar16;
  int iVar17;
  uint uVar18;
  int local_378;
  uint uStack_374;
  byte abStack_36c[80];
  byte local_31c[256];
  char acStack_21c[180];
  char local_168[180];
  char local_b4[180];
  int iQuot;

  DAT_005ef238 = 0;
  DAT_0073290c = 0;
  DAT_00525554 = 0;
  DAT_005ef23c = 0;
  DAT_005ef230 = 0;
  puVar13 = (uint *)DAT_00525790;
  for (iVar7 = 0x14; iVar7 != 0; iVar7 = (int)((uint)iVar7 - 1u)) {
    *puVar13 = 0;
    puVar13 = puVar13 + 1;
  }
  do {
    while (1) {
      local_378 = 0;
      iVar7 = FUN_0048f530();
      DAT_006db5a8 = DAT_006db5a8 + iVar7;
      iVar17 = (int)DAT_006db5a8;
      iVar7 = FUN_0048f530();
      iVar7 = (int)(DAT_006db5a8 + (uint)iVar7);
      iVar3 = (int)((uint)iVar7 - DAT_0071b71c);
      while (DAT_006db5a8 = (uint)iVar7, iVar3 < 8) {
        iVar7 = FUN_0048f530();
        iVar7 = (int)(DAT_006db5a8 + (uint)iVar7);
        iVar3 = (int)((uint)iVar7 - DAT_0071b71c);
      }
      FUN_0042c710();
      iVar3 = FUN_00431d30();
      if ((iVar3 == 0) && (0x1ba8 < DAT_006db5a4)) {
        (&DAT_0071b740)[DAT_0071ecf4 * 0x5bd] = 0;
        FUN_0042c670(DAT_0071ecf4);
        return;
      }
      iVar3 = FUN_0048f530();
      DAT_0072f444 = 0;
      iVar3 = (int)(DAT_006db5a8 + (uint)iVar3);
      iVar8 = 0;
      pcVar4 = &DAT_0071b740;
      do {
        cVar2 = *pcVar4;
        if ((cVar2 == '\x01') || (cVar2 == '\x02')) {
          DAT_0072f444 = (int)((uint)DAT_0072f444 + 1u);
        }
        if (cVar2 == '\x01') {
          iVar8 = (int)((uint)iVar8 + 1u);
        }
        pcVar4 = pcVar4 + 0x5bd;
      } while (pcVar4 < &DAT_0071e528);
      if (iVar8 == 0) {
        DAT_007092c0 = '\x01';
      }
      if ((&DAT_0071b740)[DAT_0071ecf4 * 0x5bd] == '\0') {
        DAT_007092c0 = '\x01';
      }
      DAT_006db5a8 = (uint)iVar3;
      if (DAT_007092c0 == '\0') break;
      if (DAT_00732a18 == DAT_0071b71c) {
        return;
      }
      if (0x49c < (int)((uint)iVar3 - DAT_0071b71c)) {
        return;
      }
      iVar7 = FUN_0048f530();
      DAT_006db5a8 = DAT_006db5a8 + (uint)iVar7;
      iVar7 = (int)(DAT_006db5a8 - DAT_0071b71c);
      while (iVar7 < 8) {
        iVar7 = FUN_0048f530();
        DAT_006db5a8 = DAT_006db5a8 + (uint)iVar7;
        iVar7 = (int)(DAT_006db5a8 - DAT_0071b71c);
      }
    }
    if (DAT_00732e48 == 1) {
      iVar8 = FUN_0048f530();
      uVar9 = DAT_006db5a8 + (uint)iVar8;
      uVar18 = uVar9;
      if (DAT_005ef238 != 0) {
        uVar18 = (uint)DAT_005ef234 + (uint)DAT_005ef238;
        iVar8 = DAT_005ef234 >> 3;
        if (uVar18 < uVar9) {
          iVar5 = (int)(uVar9 - uVar18);
          if (iVar8 < (int)(uVar9 - uVar18)) {
            iVar5 = iVar8;
          }
          if (iVar5 == 0) {
            iVar5 = 1;
          }
          if (4 < iVar5) {
            iVar5 = (int)(uVar9 - uVar18);
          }
          uVar18 = uVar18 + (uint)iVar5;
        }
        else if (uVar9 < uVar18) {
          iVar5 = (int)(uVar18 - uVar9);
          if (iVar8 < (int)(uVar18 - uVar9)) {
            iVar5 = iVar8;
          }
          if (iVar5 == 0) {
            iVar5 = 1;
          }
          if (4 < iVar5) {
            iVar5 = (int)(uVar18 - uVar9);
          }
          uVar18 = uVar18 - (uint)iVar5;
        }
      }
      if ((int)(uVar18 - DAT_0071b71c) < 4) {
        uVar18 = DAT_0071b71c + 4u;
      }
      DAT_005ef234 = (int)(uVar18 - (uint)DAT_005ef238);
      DAT_006db5a8 = uVar18;
      FUN_0042d530();
      DAT_00525558 = (int)(DAT_006db5a8 - DAT_0071b71c);
      DAT_00525550 = 0;
      FUN_0049a600(DAT_006db5a8);
      DAT_0071b71c = DAT_006db5a8;
      DAT_00732a18 = DAT_006db5a8;
      DAT_005ef238 = (int)DAT_006db5a8;
      DAT_006db5a8 = (DAT_006db5a8 - uVar18) + uVar9;
      FUN_00490a00();
    }
    else {
      iVar8 = FUN_0048f530();
      DAT_006db5a8 = DAT_006db5a8 + (uint)iVar8;
      iVar8 = DAT_005ef234 >> 3;
      if (DAT_00732a18 + (uint)ram0x0071f2ec < DAT_006db5a8) {
        iVar12 = (int)((DAT_006db5a8 - (uint)ram0x0071f2ec) - DAT_00732a18) >> 4;
        if (iVar12 == 0) {
          iVar12 = 1;
        }
        if (iVar8 < iVar12) {
          iVar12 = iVar8;
        }
        iVar5 = (int)(0u - (uint)iVar12);
LAB_00437e60:
        DAT_006db5a8 = DAT_006db5a8 + (uint)iVar5;
        DAT_0052554c = (int)((uint)DAT_0052554c + (uint)iVar12);
      }
      else if (DAT_006db5a8 < DAT_00732a18 + (uint)ram0x0071f2ec) {
        iVar5 = (int)((DAT_00732a18 - DAT_006db5a8) + (uint)ram0x0071f2ec) >> 4;
        if (iVar5 == 0) {
          iVar5 = 1;
        }
        if (iVar8 < iVar5) {
          iVar5 = iVar8;
        }
        iVar12 = (int)(0u - (uint)iVar5);
        goto LAB_00437e60;
      }
      if ((int)DAT_006db5a8 < (int)DAT_00732a18) {
        _sprintf(local_b4, s_Fell_Behind__tickcounter___7d_se_00525a0c, DAT_006db5a8, DAT_00732a18,
                 (DAT_00732a18 - DAT_006db5a8) + (uint)ram0x0071f2ec);
        iVar8 = (int)((DAT_00732a18 - DAT_006db5a8) + (uint)ram0x0071f2ec);
        DAT_006db5a8 = DAT_006db5a8 + (uint)iVar8;
        DAT_0052554c = (int)((uint)DAT_0052554c - (uint)iVar8);
      }
      if (((int)((uint)ram0x0071f2ec + 0x674u) < (int)(DAT_006db5a8 - DAT_00732a18)) &&
          (iVar8 = FUN_00431d30(), iVar8 == 0)) {
        FUN_0042c5c0(DAT_0071ecf4);
        _sprintf(local_168, s_Too_far_Ahead__tickcounter___7d_s_005259d0, DAT_006db5a8, DAT_00732a18,
                 DAT_006db5a8 - DAT_00732a18);
        iVar5 = 0;
        iVar8 = FUN_0048f530();
        DAT_006db5a4 = 0;
        DAT_006db5a8 = DAT_006db5a8 + (uint)iVar8;
        FUN_00438a40();
        uVar18 = 0xffffffff;
        pcVar4 = DAT_006a89a8;
        do {
          pcVar14 = pcVar4;
          if (uVar18 == 0) break;
          uVar18 = uVar18 - 1;
          pcVar14 = pcVar4 + 1;
          cVar2 = *pcVar4;
          pcVar4 = pcVar14;
        } while (cVar2 != '\0');
        uVar18 = ~uVar18;
        pbVar10 = (byte *)(pcVar14 - uVar18);
        pbVar15 = local_31c;
        for (uVar9 = uVar18 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          *(uint *)pbVar15 = *(uint *)pbVar10;
          pbVar10 = pbVar10 + 4;
          pbVar15 = pbVar15 + 4;
        }
        for (uVar18 = uVar18 & 3; uVar18 != 0; uVar18 = uVar18 - 1) {
          *pbVar15 = *pbVar10;
          pbVar10 = pbVar10 + 1;
          pbVar15 = pbVar15 + 1;
        }
        pcVar4 = FUN_0042b340();
        if (pcVar4 != (char *)0x0) {
          uVar18 = 0xffffffff;
          do {
            pcVar14 = pcVar4;
            if (uVar18 == 0) break;
            uVar18 = uVar18 - 1;
            pcVar14 = pcVar4 + 1;
            cVar2 = *pcVar4;
            pcVar4 = pcVar14;
          } while (cVar2 != '\0');
          uVar18 = ~uVar18;
          iVar8 = -1;
          pbVar10 = local_31c;
          do {
            pbVar15 = pbVar10;
            if (iVar8 == 0) break;
            iVar8 = (int)((uint)iVar8 - 1u);
            pbVar15 = pbVar10 + 1;
            bVar1 = *pbVar10;
            pbVar10 = pbVar15;
          } while (bVar1 != 0);
          pbVar10 = (byte *)(pcVar14 - uVar18);
          pbVar15 = pbVar15 - 1;
          for (uVar9 = uVar18 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            *(uint *)pbVar15 = *(uint *)pbVar10;
            pbVar10 = pbVar10 + 4;
            pbVar15 = pbVar15 + 4;
          }
          for (uVar18 = uVar18 & 3; uVar18 != 0; uVar18 = uVar18 - 1) {
            *pbVar15 = *pbVar10;
            pbVar10 = pbVar10 + 1;
            pbVar15 = pbVar15 + 1;
          }
        }
        FUN_00438c30(1, local_31c, 0x34);
        do {
          if ((int)(DAT_006db5a8 - DAT_00732a18) <= ram0x0071f2ec) break;
          iVar8 = FUN_004a53f0();
          if ((iVar8 != 0) && (uVar18 = FUN_004a5470(), uVar18 == 0x1b)) {
            DAT_007092c0 = 1;
            FUN_0042c600(DAT_0071ecf4);
            iVar7 = DAT_0071ecf4;
            DAT_0070aaf0[DAT_0071ecf4] = 1;
            (&DAT_0071b740)[iVar7 * 0x5bd] = 0;
            FUN_0042c670(iVar7);
            return;
          }
          iVar8 = (int)DAT_006db5a8;
          FUN_0042c710();
          iVar12 = FUN_0048f530();
          DAT_006db5a4 = (int)((uint)DAT_006db5a4 + ((DAT_006db5a8 + (uint)iVar12) - (uint)iVar8));
          iVar5 = (int)(((uint)iVar5 - (uint)iVar8) + DAT_006db5a8 + (uint)iVar12);
          DAT_006db5a8 = (uint)iVar8;
          if (0x1ba8 < DAT_006db5a4) {
            DAT_007092c0 = 1;
            FUN_0042c600(DAT_0071ecf4);
            iVar7 = DAT_0071ecf4;
            DAT_0070aaf0[DAT_0071ecf4] = 1;
            (&DAT_0071b740)[iVar7 * 0x5bd] = 0;
            FUN_0042c670(DAT_0071ecf4);
            return;
          }
          if (0xec < iVar5) {
            iVar5 = 0;
            FUN_0042c5a0();
          }
          /* signed IDIV: Ghidra shows no (uint) cast on this divide */
          uVar18 = (uint)((int)(0x1ba8u - (uint)DAT_006db5a4) / 0x76);
          if (uVar18 != uStack_374) {
            if ((int)uVar18 < 0x32) {
              _sprintf((char *)abStack_36c, DAT_006a89c4, (int)uVar18 / 2, (uVar18 & 1) * 5);
              pbVar10 = abStack_36c;
            }
            else {
              pbVar10 = DAT_006a89b0;
              if ((uVar18 & 1) != 0) {
                pbVar10 = DAT_006a89c0;
              }
            }
            FUN_00438c30(3, pbVar10, 0x34);
            uStack_374 = uVar18;
          }
          iVar8 = 0;
          pcVar4 = &DAT_0071b740;
          DAT_0072f444 = 0;
          do {
            cVar2 = *pcVar4;
            if ((cVar2 == '\x01') || (cVar2 == '\x02')) {
              DAT_0072f444 = (int)((uint)DAT_0072f444 + 1u);
            }
            if (cVar2 == '\x01') {
              iVar8 = (int)((uint)iVar8 + 1u);
            }
            pcVar4 = pcVar4 + 0x5bd;
          } while (pcVar4 < &DAT_0071e528);
          if (iVar8 == 0) {
            DAT_007092c0 = '\x01';
          }
          if ((&DAT_0071b740)[DAT_0071ecf4 * 0x5bd] == '\0') {
            DAT_007092c0 = '\x01';
          }
        } while ((DAT_007092c0 == '\0') || (DAT_00732a18 != DAT_0071b71c));
        FUN_0048ee20('\0');
        uVar11 = ((uint)DAT_00525520 - (uint)DAT_0052551c) >> 1;
        uVar18 = (uint)DAT_006d4a4e;
        uVar16 = (uint)DAT_00525520 + (uint)DAT_0052551c;
        uVar9 = (uint)DAT_00525528 + (uint)DAT_00525524;
        if (DAT_005257e0 != 0) {
          FUN_004a7040();
          DAT_0052ae54 = 0;
          FUN_004a6a80();
          (*DAT_00732a30)(DAT_005257e0, ((uVar16 >> 1) - uVar11 / 2) - 1 & 0xffff,
                          ((uVar9 >> 1) - (uVar18 * 5) / 2) - 1 & 0xffff, uVar11 + 2 & 0xffff,
                          uVar18 * 5 + 2);
          FUN_004a6f90();
          DAT_0052ae54 = 1;
          FUN_004a7040();
        }
        FUN_0048f530();
        iVar8 = 0;
        pcVar4 = &DAT_0071b740;
        DAT_0072f444 = 0;
        do {
          cVar2 = *pcVar4;
          if ((cVar2 == '\x01') || (cVar2 == '\x02')) {
            DAT_0072f444 = (int)((uint)DAT_0072f444 + 1u);
          }
          if (cVar2 == '\x01') {
            iVar8 = (int)((uint)iVar8 + 1u);
          }
          pcVar4 = pcVar4 + 0x5bd;
        } while (pcVar4 < &DAT_0071e528);
        if (iVar8 == 0) {
          DAT_007092c0 = '\x01';
        }
        if ((&DAT_0071b740)[DAT_0071ecf4 * 0x5bd] == '\0') {
          DAT_007092c0 = '\x01';
        }
        if ((DAT_007092c0 != '\0') && (DAT_00732a18 == DAT_0071b71c)) {
          return;
        }
        DAT_006db5a8 = DAT_00732a18 + (uint)ram0x0071f2ec;
      }
      uVar18 = DAT_006db5a8;
      if (DAT_0071b71c < DAT_006db5a8) {
        uVar9 = DAT_006db5a8;
        if (DAT_005ef238 != 0) {
          uVar9 = (uint)DAT_005ef234 + (uint)DAT_005ef238;
          iVar8 = DAT_005ef234 >> 3;
          if (uVar9 < DAT_006db5a8) {
            iVar5 = (int)(DAT_006db5a8 - uVar9);
            if (iVar8 < (int)(DAT_006db5a8 - uVar9)) {
              iVar5 = iVar8;
            }
            if (iVar5 == 0) {
              iVar5 = 1;
            }
            if (4 < iVar5) {
              iVar5 = (int)(DAT_006db5a8 - uVar9);
            }
            uVar9 = uVar9 + (uint)iVar5;
          }
          else if (DAT_006db5a8 < uVar9) {
            iVar5 = (int)(uVar9 - DAT_006db5a8);
            if (iVar8 < (int)(uVar9 - DAT_006db5a8)) {
              iVar5 = iVar8;
            }
            if (iVar5 == 0) {
              iVar5 = 1;
            }
            if (4 < iVar5) {
              iVar5 = (int)(uVar9 - DAT_006db5a8);
            }
            uVar9 = uVar9 - (uint)iVar5;
          }
        }
        if ((int)(uVar9 - DAT_0071b71c) < 4) {
          uVar9 = DAT_0071b71c + 4u;
        }
        DAT_005ef234 = (int)(uVar9 - (uint)DAT_005ef238);
        DAT_006db5a8 = uVar9;
        FUN_0042d530();
        FUN_0049adc0(DAT_005ef234);
        DAT_00525550 = 1;
        local_378 = (int)(DAT_006db5a8 - DAT_0071b71c);
        FUN_0049a600(DAT_006db5a8);
        DAT_0071b71c = DAT_006db5a8;
        DAT_005ef238 = (int)DAT_006db5a8;
        DAT_006db5a8 = (DAT_006db5a8 - uVar9) + uVar18;
      }
    }
    iVar8 = FUN_0048f530();
    DAT_006db5a8 = DAT_006db5a8 + (uint)iVar8;
    iVar12 = (int)(DAT_006db5a8 - (uint)iVar3);
    iVar8 = FUN_0048f530();
    iVar8 = (int)(DAT_006db5a8 + (uint)iVar8);
    iVar5 = (int)(((uint)iVar8 - (uint)ram0x0071f2ec) - DAT_00732a18);
    if (iVar5 < 0x1d8) {
      DAT_0071e51c = 0;
    }
    else if (iVar5 < 0x3b0) {
      DAT_0071e51c = 1;
    }
    else {
      DAT_0071e51c = 2;
      if (0x587 < iVar5) {
        DAT_0071e51c = 3;
      }
    }
    DAT_006db5a8 = (uint)iVar8;
    if (DAT_005ef23c == 0) {
      DAT_006db57c = 0;
    }
    else {
      iVar5 = FUN_00432ca0();
      iVar5 = FUN_00433680(iVar5);
      DAT_005ef230 = (int)((uint)DAT_005ef230 + ((uint)iVar5 - (uint)DAT_005ef23c) * 10u);
      if (DAT_005ef230 == 0) {
        DAT_006db57c = 0;
      }
      else if (DAT_005ef230 < 10) {
        DAT_006db57c = 1;
      }
      else {
        DAT_006db57c = 2;
        if (0x13 < DAT_005ef230) {
          DAT_006db57c = 3;
        }
      }
      DAT_005ef23c = iVar5;
      if (DAT_005ef230 != 0) {
        DAT_005ef230 = (int)((uint)DAT_005ef230 - 1u);
      }
    }
    FUN_0049b4e0();
    if (1 < DAT_00732e48) {
      DAT_00525554 = 1;
    }
    FUN_0045bb40();
    DAT_00525554 = 0;
    FUN_00490a00();
    FUN_0049b2e0();
    iVar5 = FUN_0048f530();
    DAT_006db5a8 = DAT_006db5a8 + (uint)iVar5;
    iVar17 = (int)(DAT_006db5a8 - (uint)iVar17);
    if (iVar17 == 0) {
      iVar17 = 1;
    }
    if (DAT_00525720 != '\0') {
      if (iVar17 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      /* |0xec / iVar17| <= 236, so the quotient always fits in 32 bits */
      iQuot = (int)(0xecLL / iVar17);
      _sprintf(acStack_21c, s_R___2d_U___2d_N___2d_O___2d_T____00525980, DAT_006db5a8 - (uint)iVar8, iVar12,
               (int)((uint)iVar3 - (uint)iVar7),
               (int)((((uint)iVar17 - ((uint)iVar3 - (uint)iVar7)) - (uint)iVar12) -
                     (DAT_006db5a8 - (uint)iVar8)),
               iVar17, iQuot, DAT_006db5a8, DAT_0071b71c, DAT_00732a18,
               DAT_006db5a8 - DAT_00732a18, ram0x0071f2ec, DAT_0052554c);
      if (iVar12 < 0) {
        iVar12 = 0;
      }
      if (iVar12 < 0x14) {
        DAT_00525790[iVar12] = (int)((uint)DAT_00525790[iVar12] + 1u);
      }
      else {
        DAT_00525790[19] = (int)((uint)DAT_00525790[19] + 1u);
      }
      _sprintf(DAT_00732800, s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_00525934, DAT_00525790[0], DAT_00525790[1],
               DAT_00525790[2], DAT_00525790[3], DAT_00525790[4], DAT_00525790[5], DAT_00525790[6],
               DAT_00525790[7], DAT_00525790[8], DAT_00525790[9]);
      uVar18 = 0;
      _sprintf(DAT_00732800, s_Raw_10__2d_11__2d_12__2d_13__2d_1_005258e8, DAT_00525790[10], DAT_00525790[11],
               DAT_00525790[12], DAT_00525790[13], DAT_00525790[14], DAT_00525790[15], DAT_00525790[16],
               DAT_00525790[17], DAT_00525790[18], DAT_00525790[19]);
      piVar6 = DAT_00525790;
      do {
        iVar7 = *piVar6;
        piVar6 = piVar6 + 1;
        uVar18 = uVar18 + (uint)iVar7;
      } while (piVar6 < &DAT_005257e0);
      if (uVar18 != 0) {
        _sprintf(DAT_00732800, s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052589c,
                 ((uint)DAT_00525790[0] * 100u) / uVar18, ((uint)DAT_00525790[1] * 100u) / uVar18,
                 ((uint)DAT_00525790[2] * 100u) / uVar18, ((uint)DAT_00525790[3] * 100u) / uVar18,
                 ((uint)DAT_00525790[4] * 100u) / uVar18, ((uint)DAT_00525790[5] * 100u) / uVar18,
                 ((uint)DAT_00525790[6] * 100u) / uVar18, ((uint)DAT_00525790[7] * 100u) / uVar18,
                 ((uint)DAT_00525790[8] * 100u) / uVar18, ((uint)DAT_00525790[9] * 100u) / uVar18);
        _sprintf(DAT_00732800, s_Pct_10__2d_11__2d_12__2d_13__2d_1_00525850,
                 ((uint)DAT_00525790[10] * 100u) / uVar18, ((uint)DAT_00525790[11] * 100u) / uVar18,
                 ((uint)DAT_00525790[12] * 100u) / uVar18, ((uint)DAT_00525790[13] * 100u) / uVar18,
                 ((uint)DAT_00525790[14] * 100u) / uVar18, ((uint)DAT_00525790[15] * 100u) / uVar18,
                 ((uint)DAT_00525790[16] * 100u) / uVar18, ((uint)DAT_00525790[17] * 100u) / uVar18,
                 ((uint)DAT_00525790[18] * 100u) / uVar18, ((uint)DAT_00525790[19] * 100u) / uVar18);
      }
      iVar17 = 0;
      iVar7 = 0;
      if (DAT_0071b704 < DAT_00732904) {
        pcVar4 = (char *)((uint)DAT_0071b704 * 0x23u + 3u + (uint)DAT_006d4988);
        iVar3 = (int)((uint)DAT_00732904 - (uint)DAT_0071b704);
        do {
          if (*pcVar4 != '\0') {
            if (pcVar4[-1] == '\x06') {
              iVar7 = (int)((uint)iVar7 + 1u);
            }
            else {
              iVar17 = (int)((uint)iVar17 + 1u);
            }
          }
          pcVar4 = pcVar4 + 0x23;
          iVar3 = (int)((uint)iVar3 - 1u);
        } while (iVar3 != 0);
      }
      if (7 < iVar12) {
        _sprintf(DAT_00732800, s________Long_Update___d_______War_00525804, iVar12, local_378, iVar7,
                 iVar17);
      }
    }
  } while (1);
}
