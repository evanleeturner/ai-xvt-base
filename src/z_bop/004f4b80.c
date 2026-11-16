/* SPDX-License-Identifier: MIT */
/* z_bop 004f4b80 */
extern int DAT_00b69e30;
extern unsigned char DAT_00b6a5aa[4];    /* int at misaligned 0xb6a5aa */
extern unsigned char DAT_00b6a5ae[];     /* int array at misaligned 0xb6a5ae */
extern int DAT_00aa6114;
extern int DAT_00aa6134;
extern int DAT_00aa6138;
extern unsigned char DAT_00b6a2f6[4];    /* int at misaligned 0xb6a2f6 */
extern unsigned char DAT_00bb00ea[4];    /* int at misaligned 0xbb00ea */
extern unsigned char DAT_00bb00fa[4];    /* int at misaligned 0xbb00fa */
extern unsigned char DAT_00bb00e6[24];   /* 6 dwords at misaligned 0xbb00e6 */
extern char DAT_00bb73a6;
extern int DAT_00aa6af0;
extern int DAT_00aa6af4;
extern int DAT_00aa6af8;
extern unsigned int DAT_00aa6afc[6];
extern unsigned char DAT_00bb260e[];
extern unsigned char DAT_00bb2616[];
extern unsigned char DAT_00bb260a[];
extern unsigned char DAT_00bb242e[];
extern unsigned char DAT_00bb242a[];
extern unsigned char DAT_00bb2436[];
extern unsigned char DAT_00bb2046[];
extern unsigned char DAT_00bb204e[];

int FUN_004cfee0(void);
int *FUN_004d0070(int *param_1, unsigned int *param_2);
void FUN_004e5000(int param_1);
int FUN_004f1700(void);
void FUN_004cef70(int param_1, unsigned int *param_2, int param_3);
DWORD __stdcall GetTickCount(void);

int __stdcall FUN_004f4b80(void)
{
  int iVar1;
  DWORD DVar2;
  DWORD DVar3;
  int *piVar5;
  int *piVar6;
  int local_20;
  unsigned int local_1c;
  unsigned int local_18[6]; /* local_18[5] aliases local_4 */
  int tmp;
  int sel;
  int cur;

  if (DAT_00b69e30 != 2) {
    iVar1 = FUN_004cfee0();
    if (iVar1 == 0) {
      DVar2 = GetTickCount();
      while ((piVar5 = FUN_004d0070(&local_20, &local_1c), piVar5 == (int *)0) ||
             (*piVar5 != 0x7b)) {
        DVar3 = GetTickCount();
        if (30000 < DVar3 - DVar2) {
          tmp = 0;
          __builtin_memcpy(DAT_00b6a5aa, &tmp, 4);
          return 0;
        }
      }
      tmp = 5;
      __builtin_memcpy(DAT_00b6a5aa, &tmp, 4);
      FUN_004e5000(5);
      if (DAT_00aa6114 != 0) {
        DAT_00aa6134 = 0;
        if (DAT_00aa6138 != 0) {
          piVar6 = (int *)(DAT_00aa6114 + 0x140);
          do {
            __builtin_memcpy(&sel, DAT_00b6a5aa, 4);
            __builtin_memcpy(&cur, DAT_00b6a5ae + (unsigned int)sel * 4u, 4);
            if (*piVar6 == cur) break;
            piVar6 = piVar6 + 0x52;
            DAT_00aa6134 = (int)((unsigned int)DAT_00aa6134 + 1u);
          } while (DAT_00aa6134 < DAT_00aa6138);
        }
      }
      if (piVar5[1] == 0) {
        tmp = 0;
        __builtin_memcpy(DAT_00b6a5aa, &tmp, 4);
        iVar1 = *(int *)(DAT_00aa6114 + 0x140 + (int)((unsigned int)DAT_00aa6134 * 0x148u));
        __builtin_memcpy(DAT_00bb260e + (unsigned int)iVar1 * 0x28u, &tmp, 4);
        return 0;
      }
      piVar6 = (int *)(DAT_00aa6114 + 0x140 + (int)((unsigned int)DAT_00aa6134 * 0x148u));
      __builtin_memcpy(local_18, DAT_00bb2616 + (unsigned int)*piVar6 * 0x28u, 24);
      __builtin_memcpy(&iVar1, DAT_00bb260a + (unsigned int)*piVar6 * 0x28u, 4);
      __builtin_memcpy(DAT_00bb00e6, piVar5 + 3, 24);
      if (piVar5[2] == iVar1) {
        __builtin_memcpy(DAT_00bb00fa, &local_18[5], 4);
      }
      else {
        tmp = 0;
        __builtin_memcpy(DAT_00bb00fa, &tmp, 4);
      }
    }
    else {
      tmp = 5;
      __builtin_memcpy(DAT_00b6a5aa, &tmp, 4);
      FUN_004e5000(5);
      if (DAT_00aa6114 != 0) {
        DAT_00aa6134 = 0;
        if (DAT_00aa6138 != 0) {
          piVar5 = (int *)(DAT_00aa6114 + 0x140);
          do {
            __builtin_memcpy(&sel, DAT_00b6a5aa, 4);
            __builtin_memcpy(&cur, DAT_00b6a5ae + (unsigned int)sel * 4u, 4);
            if (*piVar5 == cur) break;
            piVar5 = piVar5 + 0x52;
            DAT_00aa6134 = (int)((unsigned int)DAT_00aa6134 + 1u);
          } while (DAT_00aa6134 < DAT_00aa6138);
        }
      }
      if ((DAT_00bb73a6 == '\0') ||
          (piVar5 = (int *)(DAT_00aa6114 + 0x140 + (int)((unsigned int)DAT_00aa6134 * 0x148u)),
           __builtin_memcpy(&tmp, DAT_00bb242e + (unsigned int)*piVar5 * 0x28u, 4),
           tmp == 0)) {
        tmp = 0;
        __builtin_memcpy(DAT_00b6a5aa, &tmp, 4);
        DAT_00aa6af4 = 0;
        DAT_00aa6af0 = 0x7b;
        iVar1 = *(int *)(DAT_00aa6114 + 0x140 + (int)((unsigned int)DAT_00aa6134 * 0x148u));
        __builtin_memcpy(DAT_00bb242e + (unsigned int)iVar1 * 0x28u, &tmp, 4);
        FUN_004cef70(0, (unsigned int *)&DAT_00aa6af0, 8);
        return 0;
      }
      DAT_00aa6af0 = 0x7b;
      DAT_00aa6af4 = 1;
      __builtin_memcpy(&DAT_00aa6af8, DAT_00bb242a + (unsigned int)*piVar5 * 0x28u, 4);
      __builtin_memcpy(DAT_00aa6afc, DAT_00bb2436 + (unsigned int)*piVar5 * 0x28u, 24);
      FUN_004cef70(0, (unsigned int *)&DAT_00aa6af0, 0x24);
      __builtin_memcpy(DAT_00bb00e6,
                       DAT_00bb2436 +
                       (unsigned int)*(int *)(DAT_00aa6114 + 0x140 +
                                              (int)((unsigned int)DAT_00aa6134 * 0x148u)) * 0x28u,
                       24);
    }
    tmp = 1;
    __builtin_memcpy(DAT_00b6a2f6, &tmp, 4);
    __builtin_memcpy(&tmp, DAT_00bb00ea, 4);
    tmp = (int)((unsigned int)tmp + 1u);
    __builtin_memcpy(DAT_00bb00ea, &tmp, 4);
    FUN_004f1700();
    return 1;
  }
  tmp = 5;
  __builtin_memcpy(DAT_00b6a5aa, &tmp, 4);
  FUN_004e5000(5);
  if (DAT_00aa6114 != 0) {
    DAT_00aa6134 = 0;
    if (DAT_00aa6138 != 0) {
      piVar5 = (int *)(DAT_00aa6114 + 0x140);
      do {
        __builtin_memcpy(&sel, DAT_00b6a5aa, 4);
        __builtin_memcpy(&cur, DAT_00b6a5ae + (unsigned int)sel * 4u, 4);
        if (*piVar5 == cur) break;
        piVar5 = piVar5 + 0x52;
        DAT_00aa6134 = (int)((unsigned int)DAT_00aa6134 + 1u);
      } while (DAT_00aa6134 < DAT_00aa6138);
    }
  }
  if (DAT_00bb73a6 != '\0') {
    iVar1 = (int)((unsigned int)*(int *)(DAT_00aa6114 + 0x140 +
                                         (int)((unsigned int)DAT_00aa6134 * 0x148u)) * 0x28u);
    __builtin_memcpy(&tmp, DAT_00bb2046 + iVar1, 4);
    if (tmp != 0) {
      __builtin_memcpy(DAT_00bb00e6, DAT_00bb204e + iVar1, 24);
      tmp = 1;
      __builtin_memcpy(DAT_00b6a2f6, &tmp, 4);
      __builtin_memcpy(&tmp, DAT_00bb00ea, 4);
      tmp = (int)((unsigned int)tmp + 1u);
      __builtin_memcpy(DAT_00bb00ea, &tmp, 4);
      FUN_004f1700();
      return 1;
    }
  }
  tmp = 0;
  __builtin_memcpy(DAT_00b6a5aa, &tmp, 4);
  iVar1 = *(int *)(DAT_00aa6114 + 0x140 + (int)((unsigned int)DAT_00aa6134 * 0x148u));
  __builtin_memcpy(DAT_00bb2046 + (unsigned int)iVar1 * 0x28u, &tmp, 4);
  return 0;
}
