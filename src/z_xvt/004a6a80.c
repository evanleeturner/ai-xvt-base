/* SPDX-License-Identifier: MIT */
/* z_xvt 004a6a80 */
extern int DAT_0052ae28;
extern int DAT_00525534;
extern int DAT_0052ae50;
extern int DAT_0070acc8;
extern int DAT_0052aef8;
extern int DAT_0052ae2c;
extern int DAT_0052ae30;
extern int DAT_0052ae54;
extern int DAT_00657c0c;
extern int DAT_006572b8;
extern int DAT_006572d4;
extern int DAT_00525520;
extern int DAT_00525528;
extern int DAT_00525518;
extern unsigned char DAT_0071f2eb;
extern int *DAT_006572c8;
extern int *DAT_00657c00;
extern int *DAT_006572cc;
extern int *DAT_006572c0;
extern int *DAT_00658020;
extern char DAT_00657700[];
extern char s____CleanupAndExit_err____d_0052af38[];
extern char s_Game_could_not_start_0052af20[];
extern char s_ERROR_00524f00[];

int FUN_004d58c0(void);
int FUN_004d5510(void);
void FUN_0042fda0(void);
void FUN_0048f560(int param_1);
void FUN_0048f550(int param_1);
void FUN_00461f20(int param_1, int param_2, int param_3, int param_4);

void __stdcall FUN_004a6a80(void)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  int *piVar4;
  int aiStack_6c[27];

  if (DAT_0052ae28 == 1) {
    DAT_00525534 = FUN_004d58c0();
    DAT_0052ae50 = DAT_00525534;
    DAT_0070acc8 = FUN_004d5510();
    return;
  }
  if (DAT_0052aef8 != 0) {
    DAT_0052aef8 = DAT_0052aef8 + 1;
    return;
  }
  DAT_0052aef8 = 1;
  if (DAT_0052ae2c == 0) {
    piVar4 = aiStack_6c;
    for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
      *piVar4 = 0;
      piVar4 = piVar4 + 1;
    }
    aiStack_6c[0] = 0x6c;
    aiStack_6c[1] = 1;
    aiStack_6c[26] = 0x200;
    iVar1 = (*(int (__stdcall **)(int *, int *, int **, int))(*DAT_006572c8 + 0x18))(DAT_006572c8, aiStack_6c, &DAT_00657c00, 0);
    if (iVar1 != 0) {
      wsprintfA(DAT_00657700, s____CleanupAndExit_err____d_0052af38, 4);
      OutputDebugStringA(DAT_00657700);
      if (DAT_00657c00 != (int *)0) {
        (*(int (__stdcall **)(int *))(*DAT_00657c00 + 8))(DAT_00657c00);
        DAT_00657c00 = (int *)0;
      }
      if (DAT_006572cc != (int *)0) {
        (*(int (__stdcall **)(int *))(*DAT_006572cc + 8))(DAT_006572cc);
        DAT_006572cc = (int *)0;
      }
      if ((DAT_0052ae30 != 0) && (DAT_006572c0 != (int *)0)) {
        (*(int (__stdcall **)(int *))(*DAT_006572c0 + 8))(DAT_006572c0);
        DAT_006572c0 = (int *)0;
      }
      FUN_0042fda0();
      MessageBoxA((HWND)0, s_Game_could_not_start_0052af20, s_ERROR_00524f00, 0);
      return;
    }
    piVar4 = aiStack_6c;
    for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
      *piVar4 = 0;
      piVar4 = piVar4 + 1;
    }
    (*(int (__stdcall **)(int *, int *))(*DAT_00657c00 + 0x58))(DAT_00657c00, aiStack_6c);
    DAT_00657c0c = aiStack_6c[4];
  }
  if (DAT_0052ae30 == 0) {
    piVar4 = aiStack_6c;
    for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
      *piVar4 = 0;
      piVar4 = piVar4 + 1;
    }
    aiStack_6c[0] = 0x6c;
    while (1) {
      iVar1 = (*(int (__stdcall **)(int *, int, int *, int, int))(*DAT_00657c00 + 100))(DAT_00657c00, 0, aiStack_6c, 0, 0);
      if (iVar1 == 0) break;
      if (iVar1 != -0x7789fde4) {
        return;
      }
    }
    FUN_0048f560(aiStack_6c[9]);
    FUN_0048f550((int)((unsigned int)DAT_00657c0c * 0x1e0u));
    DAT_00525534 = aiStack_6c[9];
    DAT_0052ae50 = aiStack_6c[9];
  }
  else {
    if (DAT_0071f2eb == 0) {
      piVar4 = aiStack_6c;
      for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *piVar4 = 0;
        piVar4 = piVar4 + 1;
      }
      aiStack_6c[0] = 0x6c;
      do {
        iVar1 = (*(int (__stdcall **)(int *, int, int *, int, int))(*DAT_00657c00 + 100))(DAT_00657c00, 0, aiStack_6c, 0, 0);
        if (iVar1 == 0) {
          FUN_0048f560(aiStack_6c[9]);
          DAT_00525534 = aiStack_6c[9];
          DAT_0052ae50 = aiStack_6c[9];
          piVar4 = aiStack_6c;
          for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
            *piVar4 = 0;
            piVar4 = piVar4 + 1;
          }
          aiStack_6c[0] = 0x6c;
          (*(int (__stdcall **)(int *, int *))(*DAT_00657c00 + 0x58))(DAT_00657c00, aiStack_6c);
          DAT_00657c0c = aiStack_6c[2];
          uVar3 = ((unsigned int)DAT_006572b8 - (unsigned int)DAT_00525520) >> 1;
          iVar1 = (int)(uVar3 * (unsigned int)DAT_00525518);
          uVar3 = ((unsigned int)DAT_006572d4 - (unsigned int)DAT_00525528) >> 1;
          iVar2 = (int)(uVar3 * (unsigned int)aiStack_6c[2]);
          DAT_00525534 = (int)((unsigned int)DAT_00525534 + (unsigned int)iVar1 + (unsigned int)iVar2);
          DAT_0052ae50 = (int)((unsigned int)DAT_0052ae50 + (unsigned int)iVar1 + (unsigned int)iVar2);
          FUN_0048f550((int)((unsigned int)aiStack_6c[2] * 0x1e0u));
          return;
        }
      } while (iVar1 == -0x7789fde4);
      return;
    }
    if (DAT_0052ae54 == 0) {
      piVar4 = aiStack_6c;
      for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *piVar4 = 0;
        piVar4 = piVar4 + 1;
      }
      aiStack_6c[0] = 0x6c;
      while (1) {
        iVar1 = (*(int (__stdcall **)(int *, int, int *, int, int))(*DAT_00658020 + 100))(DAT_00658020, 0, aiStack_6c, 0, 0);
        if (iVar1 == 0) break;
        if (iVar1 != -0x7789fde4) {
          return;
        }
      }
      FUN_0048f560(aiStack_6c[9]);
      DAT_00525534 = aiStack_6c[9];
      DAT_0052ae50 = aiStack_6c[9];
      piVar4 = aiStack_6c;
      for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *piVar4 = 0;
        piVar4 = piVar4 + 1;
      }
      aiStack_6c[0] = 0x6c;
      (*(int (__stdcall **)(int *, int *))(*DAT_00658020 + 0x58))(DAT_00658020, aiStack_6c);
      DAT_00657c0c = aiStack_6c[2];
      uVar3 = ((unsigned int)DAT_006572b8 - (unsigned int)DAT_00525520) >> 1;
      iVar2 = (int)(uVar3 * (unsigned int)DAT_00525518);
      uVar3 = ((unsigned int)DAT_006572d4 - (unsigned int)DAT_00525528) >> 1;
      iVar1 = (int)(uVar3 * (unsigned int)aiStack_6c[2]);
      DAT_00525534 = (int)((unsigned int)DAT_00525534 + (unsigned int)iVar2 + (unsigned int)iVar1);
      DAT_0052ae50 = (int)((unsigned int)DAT_0052ae50 + (unsigned int)iVar2 + (unsigned int)iVar1);
      FUN_0048f550((int)((unsigned int)aiStack_6c[2] * 0x1e0u));
      if (DAT_0070acc8 != DAT_00657c0c) {
        DAT_0070acc8 = DAT_00657c0c;
        FUN_00461f20(DAT_00525534, DAT_00657c0c, 0x1e0, -1);
        return;
      }
    }
    else {
      piVar4 = aiStack_6c;
      for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *piVar4 = 0;
        piVar4 = piVar4 + 1;
      }
      aiStack_6c[0] = 0x6c;
      while (1) {
        iVar1 = (*(int (__stdcall **)(int *, int, int *, int, int))(*DAT_006572c0 + 100))(DAT_006572c0, 0, aiStack_6c, 0, 0);
        if (iVar1 == 0) break;
        if (iVar1 != -0x7789fde4) {
          return;
        }
      }
      FUN_0048f560(aiStack_6c[9]);
      DAT_00525534 = aiStack_6c[9];
      DAT_0052ae50 = aiStack_6c[9];
      piVar4 = aiStack_6c;
      for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *piVar4 = 0;
        piVar4 = piVar4 + 1;
      }
      aiStack_6c[0] = 0x6c;
      (*(int (__stdcall **)(int *, int *))(*DAT_006572c0 + 0x58))(DAT_006572c0, aiStack_6c);
      DAT_00657c0c = aiStack_6c[2];
      FUN_0048f550((int)((unsigned int)aiStack_6c[2] * 0x1e0u));
      if (DAT_0070acc8 != DAT_00657c0c) {
        DAT_0070acc8 = DAT_00657c0c;
        FUN_00461f20(DAT_00525534, DAT_00657c0c, 0x1e0, -1);
        return;
      }
    }
  }
  return;
}
