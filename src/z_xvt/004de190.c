/* SPDX-License-Identifier: MIT */
/* z_xvt 004de190 */
extern int *DAT_00759aec;
extern int DAT_00759af4;
extern int *DAT_00759af8;
extern int DAT_00759b00;
extern int DAT_00759b04;
extern unsigned char DAT_00759645;
extern int DAT_00758830;

int __cdecl FUN_004dee40(char *param_1);
void __cdecl FUN_004d4d90(void);
int __cdecl FUN_004d4d00(void);
int __cdecl FUN_00417c20(int *param_1, char *param_2);

int __cdecl FUN_004de190(char *param_1, int param_2, int param_3, uint param_4, int param_5, int param_6)
{
  int iVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int *piVar8;
  uint uStack_c;
  uint uStack_8;
  uint uStack_4;

  if (DAT_00759aec == (int *)0x0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar1 = FUN_004dee40(param_1);
  if (iVar1 == -1) {
    return 0;
  }
  uStack_8 = (uint)DAT_00759645;
  FUN_004d4d90();
  if (DAT_00759b00 == 0xc) {
    iVar5 = 0;
    iVar6 = 0;
    do {
      if ((*(int *)((int)DAT_00759af8 + iVar6) != -1) &&
         ((piVar3 = (int *)((int *)((int)DAT_00759af8 + iVar6))[2],
          iVar2 = (**(int (__stdcall **)(int *, uint *))(*piVar3 + 0x24))(piVar3, &uStack_4), iVar2 != 0 ||
          (((uStack_4 & 1) == 0 && ((uStack_4 & 4) == 0)))))) {
        (**(int (__stdcall **)(int *))(*(int *)DAT_00759af8[iVar5 * 3 + 2] + 8))((int *)DAT_00759af8[iVar5 * 3 + 2]);
        DAT_00759af8[iVar5 * 3] = -1;
        DAT_00759af8[iVar5 * 3 + 2] = 0;
        DAT_00759b00 = DAT_00759b00 + -1;
        break;
      }
      iVar6 = iVar6 + 0xc;
      iVar5 = iVar5 + 1;
    } while (iVar6 < 0x90);
    if (iVar5 == 0xc) {
      iVar5 = 0xc;
      uStack_c = param_4;
      iVar6 = 0;
      piVar3 = DAT_00759af8;
      do {
        uVar4 = (uint)*(unsigned char *)(*piVar3 * 0x145 + 0x144 + DAT_00759af4);
        if ((int)uVar4 < (int)uStack_c) {
          iVar5 = iVar6;
          uStack_c = uVar4;
        }
        piVar3 = piVar3 + 3;
        iVar6 = iVar6 + 1;
      } while (iVar6 < 0xc);
      if (iVar5 == 0xc) {
        if (param_2 != 0) {
          iVar5 = 0;
          piVar3 = DAT_00759af8;
          do {
            if (*piVar3 == iVar1) {
              (**(int (__stdcall **)(int *, int))(*(int *)DAT_00759af8[iVar5 * 3 + 2] + 0x34))
                        ((int *)DAT_00759af8[iVar5 * 3 + 2], 0);
              DAT_00759af8[iVar5 * 3 + 1] = DAT_00759b04;
              DAT_00759b04 = DAT_00759b04 + 1;
              if (uStack_8 != 0) {
                DAT_00758830 = FUN_004d4d00();
              }
              return 1;
            }
            piVar3 = piVar3 + 3;
            iVar5 = iVar5 + 1;
          } while (iVar5 < 0xc);
        }
        if (uStack_8 != 0) {
          DAT_00758830 = FUN_004d4d00();
        }
        return 0;
      }
      (**(int (__stdcall **)(int *))(*(int *)DAT_00759af8[iVar5 * 3 + 2] + 0x48))((int *)DAT_00759af8[iVar5 * 3 + 2]);
      (**(int (__stdcall **)(int *))(*(int *)DAT_00759af8[iVar5 * 3 + 2] + 8))((int *)DAT_00759af8[iVar5 * 3 + 2]);
      DAT_00759af8[iVar5 * 3] = -1;
      DAT_00759af8[iVar5 * 3 + 2] = 0;
      DAT_00759b00 = DAT_00759b00 + -1;
    }
  }
  else {
    iVar5 = 0;
    piVar3 = DAT_00759af8;
    do {
      if (*piVar3 == -1) break;
      piVar3 = piVar3 + 3;
      iVar5 = iVar5 + 1;
    } while (iVar5 < 0xc);
  }
  piVar8 = (int *)0;
  (**(int (__stdcall **)(int *, int, int **))(*DAT_00759aec + 0x14))
            (DAT_00759aec, *(int *)(DAT_00759af4 + 0x140 + iVar1 * 0x145), &piVar8);
  if (piVar8 != (int *)0x0) {
    (**(int (__stdcall **)(int *, int))(*piVar8 + 0x34))(piVar8, 0);
    if (param_3 < 0x80) {
      if (param_3 < 0) {
        param_3 = 0;
      }
    }
    else {
      param_3 = 0x7f;
    }
    (**(int (__stdcall **)(int *, int))(*piVar8 + 0x3c))
              (piVar8, ((int)((uint)param_3 * 5u - 0x27bu) * 400) / 0x7f);
    uStack_4 = (uint)param_5;
    if (0x7f < (int)uStack_4) {
      uStack_4 = 0x7f;
    }
    if ((int)uStack_4 < 0) {
      uStack_4 = 0;
    }
    (**(int (__stdcall **)(int *, int))(*piVar8 + 0x40))
              (piVar8, ((int)(uStack_4 * 5u - 0x13bu) * 400) / 0x3f);
    uVar4 = (uint)(param_6 == 1);
    iVar6 = (**(int (__stdcall **)(int *, int, int, uint))(*piVar8 + 0x30))(piVar8, 0, 0, uVar4);
    if (iVar6 == -0x7787ff6a) {
      iVar6 = DAT_00759af4 + iVar1 * 0x145;
      iVar6 = FUN_00417c20(*(int **)(iVar6 + 0x140), (char *)(iVar6 + 0x40));
      if (iVar6 == 1) {
        (**(int (__stdcall **)(int *, int))(*piVar8 + 0x34))(piVar8, 0);
        iVar6 = (**(int (__stdcall **)(int *, int, int, uint))(*piVar8 + 0x30))(piVar8, 0, 0, uVar4);
        if (iVar6 == 0) {
          DAT_00759af8[iVar5 * 3] = iVar1;
          DAT_00759af8[iVar5 * 3 + 2] = (int)piVar8;
          DAT_00759af8[iVar5 * 3 + 1] = DAT_00759b04;
          DAT_00759b04 = DAT_00759b04 + 1;
          DAT_00759b00 = DAT_00759b00 + 1;
          iVar6 = 0;
        }
        else {
          iVar6 = 0;
        }
      }
    }
    else if (iVar6 == 0) {
      DAT_00759af8[iVar5 * 3] = iVar1;
      DAT_00759af8[iVar5 * 3 + 2] = (int)piVar8;
      DAT_00759af8[iVar5 * 3 + 1] = DAT_00759b04;
      DAT_00759b04 = DAT_00759b04 + 1;
      DAT_00759b00 = DAT_00759b00 + 1;
      iVar6 = 1;
    }
    if (uStack_8 != 0) {
      DAT_00758830 = FUN_004d4d00();
    }
    return iVar6;
  }
  if (uStack_8 != 0) {
    DAT_00758830 = FUN_004d4d00();
  }
  return 0;
}
