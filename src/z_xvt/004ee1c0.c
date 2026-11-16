/* SPDX-License-Identifier: MIT */
/* z_xvt 004ee1c0 */
extern int DAT_00757870;
extern int DAT_00757874;
extern int DAT_00757680;
extern int DAT_00757684;
extern int DAT_0075788c;
extern int DAT_0075769c;
extern int DAT_007579b0;
extern int DAT_007579d0;
extern int DAT_00757a28;
extern int DAT_0081b850;
extern int DAT_0081bc64;
/* Unaligned globals (address % 4 == 2): accessed via memcpy */
extern unsigned char DAT_00757cee;
extern unsigned char DAT_0081f7de;
extern unsigned char DAT_0081fa9e;

int * FUN_004cc7b0(int *param_1);
int FUN_004cdbc0(void);
int FUN_004cde00(void);

unsigned int __stdcall FUN_004ee1c0(void)
{
  int *piVar1;
  int *piVar4;
  int iVar2;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int bVar10;
  int local_c;
  int *local_8;
  int *local_4;
  unsigned char *pbVar1;
  unsigned char *pbVar3;
  int tmp;
  int zero;
  int val;

  iVar7 = 10;
  local_8 = FUN_004cc7b0(&local_c);
  piVar1 = &DAT_00757870;
  do {
    iVar9 = 8;
    do {
      iVar6 = 0;
      bVar10 = local_c == 0;
      if (0 < local_c) {
        piVar4 = local_8 + 9;
        do {
          if ((*piVar4 != 0) && (piVar4[-1] == *piVar1)) break;
          piVar4 = piVar4 + 10;
          iVar6 = iVar6 + 1;
        } while (iVar6 < local_c);
        bVar10 = local_c == iVar6;
      }
      if (bVar10) {
        *piVar1 = 0;
      }
      piVar1 = piVar1 + 1;
      iVar9 = iVar9 + -1;
    } while (iVar9 != 0);
    iVar7 = iVar7 + -1;
  } while (iVar7 != 0);

  pbVar3 = &DAT_00757cee;
  piVar1 = &DAT_007579b0;
  do {
    iVar7 = 0;
    if (0 < local_c) {
      piVar4 = local_8 + 9;
      do {
        if ((*piVar4 != 0) && (piVar4[-1] == *piVar1)) break;
        piVar4 = piVar4 + 10;
        iVar7 = iVar7 + 1;
      } while (iVar7 < local_c);
    }
    if (iVar7 == 8) {
      *piVar1 = 0;
      zero = 0;
      __builtin_memcpy(pbVar3, &zero, 4);
    }
    pbVar3 = pbVar3 + 0x2a;
    piVar1 = piVar1 + 1;
  } while (piVar1 < &DAT_007579d0);

  pbVar1 = &DAT_0081f7de;
  do {
    iVar7 = 0;
    bVar10 = local_c == 0;
    __builtin_memcpy(&val, pbVar1, 4);
    if (0 < local_c) {
      piVar4 = local_8 + 9;
      do {
        if ((*piVar4 != 0) && (piVar4[-1] == val)) break;
        piVar4 = piVar4 + 10;
        iVar7 = iVar7 + 1;
      } while (iVar7 < local_c);
      bVar10 = local_c == iVar7;
    }
    if (bVar10) {
      zero = 0;
      __builtin_memcpy(pbVar1, &zero, 4);
    }
    pbVar1 = pbVar1 + 0x58;
  } while (pbVar1 < &DAT_0081fa9e);

  iVar7 = 0;
  pbVar1 = &DAT_0081f7de;
  do {
    iVar6 = FUN_004cdbc0();
    iVar9 = iVar7;
    __builtin_memcpy(&val, pbVar1, 4);
    if (iVar6 == val) break;
    pbVar1 = pbVar1 + 0x58;
    iVar7 = iVar7 + 1;
    iVar9 = DAT_0081bc64;
  } while (pbVar1 < &DAT_0081fa9e);
  DAT_0081bc64 = iVar9;

  if (DAT_0081b850 == 2) {
    local_c = 1;
  }
  else {
    local_c = FUN_004cde00();
  }

  local_4 = &DAT_007579b0;
  do {
    iVar7 = 0;
    bVar10 = local_c == 0;
    if (0 < local_c) {
      do {
        if (*local_4 == 0) break;
        iVar7 = iVar7 + 1;
      } while (iVar7 < local_c);
      bVar10 = local_c == iVar7;
    }
    if ((!bVar10) && (0 < DAT_00757a28)) {
      iVar9 = 0;
      iVar7 = *local_4;
      tmp = DAT_00757a28;
      do {
        iVar8 = 0;
        iVar6 = iVar9;
        do {
          if (*(int *)((unsigned char *)&DAT_00757870 + iVar6) == iVar7) {
            if (iVar8 < 7) {
              iVar2 = 7 - iVar8;
              iVar5 = iVar6;
              do {
                iVar2 = iVar2 + -1;
                *(int *)((unsigned char *)&DAT_00757870 + iVar5) =
                     *(int *)((unsigned char *)&DAT_00757874 + iVar5);
                *(int *)((unsigned char *)&DAT_00757680 + iVar5) =
                     *(int *)((unsigned char *)&DAT_00757684 + iVar5);
                iVar5 = iVar5 + 4;
              } while (iVar2 != 0);
            }
            *(int *)((unsigned char *)&DAT_0075788c + iVar9) = 0;
            *(int *)((unsigned char *)&DAT_0075769c + iVar9) = -1;
          }
          iVar6 = iVar6 + 4;
          iVar8 = iVar8 + 1;
        } while (iVar8 < 8);
        iVar9 = iVar9 + 0x20;
        tmp = tmp + -1;
      } while (tmp != 0);
    }
    local_4 = local_4 + 1;
    if ((int *)0x7579cf < local_4) {
      return 1;
    }
  } while (1);
}
