/* SPDX-License-Identifier: MIT */
/* z_bop 004c7cc0 */
extern unsigned char DAT_00b6a2e0[];
extern unsigned char DAT_00b6bc9e[];
extern char DAT_00b69d20[];
extern char DAT_00528118[];
extern char s__s___c_s__c_s_0052b48c[];
extern char s_jewelsound_0052ad74[];
extern unsigned char DAT_00525034[];

extern unsigned char DAT_00b6d962[4];   /* int at unaligned address */
extern unsigned char DAT_00b7c342[];    /* int fields, unaligned */
extern unsigned char DAT_00b7c6c6[];
extern unsigned char DAT_00b7ca4a[];
extern unsigned char DAT_00b7d6ca[];
extern unsigned char DAT_00b7ca5e[];
extern unsigned char DAT_00b7d6de[];

extern int DAT_00b6a248;
extern void *DAT_00b6a258;
extern unsigned int DAT_00b6a25c;
extern void *DAT_00aa6114;
extern unsigned int DAT_00aa6138;
extern int DAT_00664f2c;
extern int DAT_00664eac;
extern int DAT_00664fb8;
extern unsigned int DAT_00b69e34;
extern int DAT_00b6a2c0;
extern int DAT_00b6a2a0;
extern int DAT_00bb2810;

void FUN_004de090(int *a, int *b);
void FUN_004d6460(int *r, int a, int b, int c, int d);
char *FUN_004ddc10(int a);
int _sprintf(char *buf, const char *fmt, ...);
void FUN_004db640(int a, char *s, int *r, int c);
void _free(void *p);
void FUN_004e5000(int a);
void FUN_004b4cc0(char *s, int *r);
int FUN_004d9d70(int *r, int a, int b, int c, int d, int e, int f);
void FUN_004d4cf0(int *r);
void FUN_004d4d20(int *r);
void FUN_004d64a0(int *r, int a, int b);
void FUN_004b5de0(char *s, int a, int b);
int FUN_004d6b90(int *r, int a, int b);
void FUN_004d6950(int *r, int a, int b, short c);
int FUN_004da780(int *r, unsigned char *a, unsigned char *b, char *c, int d, int e, int f, char *g);
void FUN_004d3350(void);
void FUN_004d4f70(void);
void FUN_004d4970(void);
void FUN_004d4c10(void);
void FUN_004d49a0(void);
void FUN_004efce0(char *s, int a);
void FUN_004d4f60(void);
void FUN_004caf60(void);
void FUN_004d3400(void);

unsigned int __stdcall FUN_004c7cc0(void)
{
  char cVar1;
  int bVar2;
  char *pcVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;
  int iVar7;
  unsigned int uVar8;
  int iVar9;
  unsigned int uVar10;
  unsigned char *puVar11;
  unsigned int uVar12;
  unsigned char *puVar13;
  unsigned int local_48;
  int local_44;
  unsigned int local_40;
  int local_3c;
  int local_38[4];
  int local_28[4];
  int local_18;
  int local_14;
  int local_10[4];
  int iBase;
  int iTmp;
  int iSkip;

  FUN_004de090(&local_14, &local_18);
  FUN_004d6460(local_28, 0x54, 0x5a, 0x194, 0x6a);
  if ((char)DAT_00b6a2e0[0] == '\0') {
    pcVar3 = FUN_004ddc10(0x31c);
    _sprintf(DAT_00b69d20, DAT_00528118, pcVar3);
  }
  else {
    puVar13 = DAT_00b6a2e0;
    uVar12 = 4;
    puVar11 = DAT_00b6bc9e;
    uVar10 = 6;
    pcVar3 = FUN_004ddc10(0x31c);
    _sprintf(DAT_00b69d20, s__s___c_s__c_s_0052b48c, pcVar3, uVar10, puVar11, uVar12, puVar13);
  }
  FUN_004db640(0xf, DAT_00b69d20, local_28, 0xffff);
  if ((char)DAT_00b6a2e0[0] == '\0') {
    return 0;
  }
  if (DAT_00b6a248 == 0) {
    return 0;
  }
  if (DAT_00664f2c != 0) {
    if (DAT_00b6a258 != (void *)0) {
      _free(DAT_00b6a258);
      DAT_00b6a258 = (void *)0;
    }
    FUN_004e5000(5);
    uVar8 = 0;
    DAT_00b6a258 = DAT_00aa6114;
    DAT_00b6a25c = DAT_00aa6138;
    DAT_00aa6114 = (void *)0;
    if (DAT_00aa6138 != 0) {
      iVar9 = 0;
      do {
        uVar6 = 0xffffffff;
        pcVar3 = (char *)((int)DAT_00b6a258 + iVar9 + 0x40);
        do {
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          cVar1 = *pcVar3;
          pcVar3 = pcVar3 + 1;
        } while (cVar1 != '\0');
        for (iVar7 = (int)(~uVar6 - 2); iVar7 != 0; iVar7 = iVar7 + -1) {
          if (*(char *)((int)DAT_00b6a258 + iVar7 + iVar9 + 0x40) == '(') {
            *(char *)((int)DAT_00b6a258 + iVar7 + iVar9 + 0x40) = 0;
            break;
          }
        }
        iVar9 = iVar9 + 0x148;
        uVar8 = uVar8 + 1;
      } while (uVar8 < DAT_00b6a25c);
    }
    iVar9 = 0;
    DAT_00664eac = 0;
    DAT_00664fb8 = 0;
    local_48 = 0;
    if (DAT_00b6a25c != 0) {
      do {
        __builtin_memcpy(&iBase, DAT_00b6d962, 4);
        iVar7 = (int)((unsigned int)*(int *)((int)DAT_00b6a258 + iVar9 + 0x140) * 0x24u
                      + (unsigned int)iBase * 0x109e0u);
        __builtin_memcpy(&iTmp, &DAT_00b7c342[iVar7], 4);
        iSkip = (iTmp != 0);
        if (!iSkip) {
          __builtin_memcpy(&iTmp, &DAT_00b7c6c6[iVar7], 4);
          iSkip = (iTmp != 0);
        }
        if (iSkip) {
          uVar8 = 0;
          bVar2 = 0;
          if (DAT_00b69e34 != 0) {
            iVar7 = 0;
            do {
              if (*(int *)((int)DAT_00b6a258 + iVar9 + 0x140) ==
                  *(int *)(DAT_00b6a248 + 0x120 + iVar7)) {
                iVar5 = iVar7 + DAT_00b6a248;
                if (*(int *)(iVar7 + 0x124 + DAT_00b6a248) == 0) {
                  __builtin_memcpy(&iBase, DAT_00b6d962, 4);
                  iVar4 = (int)(((unsigned int)iBase * 0x84fu + (unsigned int)*(int *)(iVar5 + 0x128)) * 0x20u);
                  __builtin_memcpy(&iTmp, &DAT_00b7ca4a[iVar4], 4);
                  if (iTmp == 0) {
                    __builtin_memcpy(&iVar4, &DAT_00b7d6ca[iVar4], 4);
                    if (iVar4 == 0) goto LAB_004c7f69;
                  }
                }
                else {
                  __builtin_memcpy(&iBase, DAT_00b6d962, 4);
                  iVar4 = (int)(((unsigned int)iBase * 0x84fu + (unsigned int)*(int *)(iVar5 + 0x128)) * 0x20u);
                  __builtin_memcpy(&iTmp, &DAT_00b7ca5e[iVar4], 4);
                  if (iTmp == 0) {
                    __builtin_memcpy(&iVar4, &DAT_00b7d6de[iVar4], 4);
                    if (iVar4 == 0) goto LAB_004c7f69;
                  }
                }
                if (!bVar2) {
                  bVar2 = 1;
                  DAT_00664fb8 = DAT_00664fb8 + 2;
                }
                DAT_00664fb8 = DAT_00664fb8 + 1;
                FUN_004b4cc0((char *)(iVar5 + 0x80), local_38);
                DAT_00664fb8 = DAT_00664fb8 + (int)((unsigned int)((local_38[3] - local_38[1]) + 0x12) / 0xfu);
              }
LAB_004c7f69:
              iVar7 = iVar7 + 300;
              uVar8 = uVar8 + 1;
            } while (uVar8 < DAT_00b69e34);
          }
        }
        iVar9 = iVar9 + 0x148;
        local_48 = local_48 + 1;
      } while (local_48 < DAT_00b6a25c);
    }
    DAT_00664f2c = 0;
  }
  FUN_004d6460(local_28, 0x1a9, 0x6b, 0x1b2, 0x1b1);
  if (0x15 < DAT_00664fb8) {
    DAT_00664eac = FUN_004d9d70(local_28, DAT_00664eac, DAT_00664fb8, 0, 5, DAT_00bb2810, 0xb);
  }
  FUN_004d4cf0(local_10);
  FUN_004d6460(local_28, 0x58, 0x6f, 0x1a8, 0x1b1);
  FUN_004d4d20(local_28);
  local_3c = 0;
  iVar9 = (int)((unsigned int)DAT_00664eac * 0xfffffff1u + 0x6fu);
  FUN_004d6460(local_28, 0x58, iVar9, 0x1a8, (int)((unsigned int)DAT_00664eac * 0xfffffff1u + 0x7du));
  local_48 = 0;
  if (DAT_00b6a25c != 0) {
    local_44 = 0;
    do {
      __builtin_memcpy(&iBase, DAT_00b6d962, 4);
      iVar7 = (int)((unsigned int)*(int *)((int)DAT_00b6a258 + local_44 + 0x140) * 0x24u
                    + (unsigned int)iBase * 0x109e0u);
      __builtin_memcpy(&iTmp, &DAT_00b7c342[iVar7], 4);
      iSkip = (iTmp != 0);
      if (!iSkip) {
        __builtin_memcpy(&iTmp, &DAT_00b7c6c6[iVar7], 4);
        iSkip = (iTmp != 0);
      }
      if (iSkip) {
        bVar2 = 0;
        local_40 = 0;
        if (DAT_00b69e34 != 0) {
          iVar7 = 0;
          do {
            if (*(int *)((int)DAT_00b6a258 + local_44 + 0x140) ==
                *(int *)(DAT_00b6a248 + 0x120 + iVar7)) {
              iVar5 = iVar7 + DAT_00b6a248;
              if (*(int *)(iVar5 + 0x124) == 0) {
                __builtin_memcpy(&iBase, DAT_00b6d962, 4);
                iVar5 = (int)(((unsigned int)iBase * 0x84fu + (unsigned int)*(int *)(iVar5 + 0x128)) * 0x20u);
                __builtin_memcpy(&iTmp, &DAT_00b7ca4a[iVar5], 4);
                if (iTmp == 0) {
                  __builtin_memcpy(&iVar5, &DAT_00b7d6ca[iVar5], 4);
                  if (iVar5 == 0) goto LAB_004c82da;
                }
              }
              else {
                __builtin_memcpy(&iBase, DAT_00b6d962, 4);
                iVar5 = (int)(((unsigned int)iBase * 0x84fu + (unsigned int)*(int *)(iVar5 + 0x128)) * 0x20u);
                __builtin_memcpy(&iTmp, &DAT_00b7ca5e[iVar5], 4);
                if (iTmp == 0) {
                  __builtin_memcpy(&iVar5, &DAT_00b7d6de[iVar5], 4);
                  if (iVar5 == 0) goto LAB_004c82da;
                }
              }
              if (!bVar2) {
                iVar9 = iVar9 + 0x1e;
                bVar2 = 1;
                FUN_004db640(0xc, (char *)((int)DAT_00b6a258 + local_44 + 0x40), local_28, DAT_00b6a2c0);
                FUN_004d64a0(local_28, 0, 0x1e);
              }
              FUN_004db640(0xc, (char *)(DAT_00b6a248 + iVar7 + 0xa0), local_28, 0xffff);
              FUN_004d64a0(local_28, 0, 0xf);
              FUN_004b4cc0((char *)(DAT_00b6a248 + iVar7 + 0x80), local_38);
              iVar4 = (int)(0x100u - ((unsigned int)((local_38[2] - local_38[0]) + 1) >> 1));
              iVar5 = (int)(((unsigned int)((local_38[3] - local_38[1]) + 0x12) / 0xfu) * 0xfu);
              FUN_004b5de0((char *)(DAT_00b6a248 + iVar7 + 0x80), iVar4, iVar9 + 0x12);
              FUN_004d64a0(local_38, iVar4, iVar9 + 0x12);
              iVar4 = FUN_004d6b90(local_38, local_14, local_18);
              if (iVar4 != 0) {
                FUN_004d6950(local_38, 0, 0, (short)DAT_00b6a2a0);
                iVar4 = FUN_004da780(local_38, DAT_00525034, DAT_00525034, (char *)0, 0xf, 0xffff,
                                     0x24, s_jewelsound_0052ad74);
                if (iVar4 != 0) {
                  local_3c = (int)(local_40 + 1);
                }
              }
              iVar9 = (int)((unsigned int)iVar9 + 0xfu + (unsigned int)iVar5);
              FUN_004d64a0(local_28, 0, iVar5);
              if (0x1b1 < iVar9) goto LAB_004c8318;
            }
LAB_004c82da:
            iVar7 = iVar7 + 300;
            local_40 = local_40 + 1;
          } while (local_40 < DAT_00b69e34);
        }
        if (0x1b1 < iVar9) break;
      }
      local_44 = local_44 + 0x148;
      local_48 = local_48 + 1;
    } while (local_48 < DAT_00b6a25c);
  }
LAB_004c8318:
  FUN_004d4d20(local_10);
  if (local_3c != 0) {
    FUN_004d3350();
    FUN_004d4f70();
    FUN_004d4970();
    FUN_004d4c10();
    FUN_004d49a0();
    FUN_004d4c10();
    FUN_004efce0((char *)((unsigned int)local_3c * 300u + (unsigned int)DAT_00b6a248 - 300u), 0);
    FUN_004d4c10();
    FUN_004d49a0();
    FUN_004d4c10();
    FUN_004d4f60();
    FUN_004caf60();
    FUN_004d3400();
  }
  return 1;
}
