/* SPDX-License-Identifier: MIT */
/* z_bop 004c83a0 */
/* Prototypes */
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004ddc10(int param_1);
int _sprintf(char *buf, const char *fmt, ...);
void FUN_004db640(int param_1, char *param_2, int *param_3, int param_4);
void _free(void *p);
void FUN_004e5000(int param_1);
int FUN_004d9d70(int *param_1, int param_2, int param_3, int param_4, int param_5,
                 unsigned int param_6, int param_7);
void FUN_004b4cc0(char *param_1, int *param_2);
void FUN_004b5570(char *param_1, int param_2, int param_3);

/* Globals (placeholders) */
extern char DAT_00528118[];
extern char s__s___c_s__c_s_0052b48c[];
extern char DAT_00b69d20[];
extern char DAT_00b6a2e0[];
extern char DAT_00b6bc9e[];
extern int *DAT_00b69cd8;
extern unsigned int DAT_00b69cc8;
extern int DAT_00664f2c;
extern void *DAT_00b6a258;
extern void *DAT_00b6a2d4;
extern void *DAT_00b6a2cc;
extern void *DAT_00aa6114;
extern unsigned int DAT_00aa6138;
extern unsigned int DAT_00b6a25c;
extern unsigned int DAT_00b69d18;
extern int DAT_00b69e30;
extern unsigned int DAT_00bb2814;
extern unsigned int DAT_00bb2810;
extern unsigned int DAT_00664ee8[16];
extern unsigned int DAT_00664f60[16];
extern int DAT_00664ed0;
extern int DAT_00664fbc;
extern int DAT_00664ec8;
extern int DAT_00664f28;
extern int DAT_00b6a2c0;
/* packed (unaligned) globals */
extern unsigned char DAT_00b6d962[4];
extern char DAT_00b7c342;
extern char DAT_00b7c6c6;
extern char DAT_00b7ca46;
extern char DAT_00b7ca4a;
extern char DAT_00b7ca4e;
extern char DAT_00b7d6c6;
extern char DAT_00b7d6ca;
extern char DAT_00b7d6ce;

unsigned int __stdcall FUN_004c83a0(void)
{
  char cVar1;
  char *pcVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int iVar6;
  int *piVar7;
  unsigned int *puVar8;
  int iVar9;
  unsigned int uVar10;
  int *piVar11;
  int bVar12;
  unsigned int uVar13;
  unsigned int uVar14;
  char *puVar15;
  unsigned int local_24;
  int local_20[4];
  int local_10[4]; /* local_10, local_c, local_8, local_4 (contiguous) */
  int t0, t1, t2;
  int v;

  FUN_004d6460(local_20, 0x54, 0x5a, 0x194, 0x6a);
  if (DAT_00b6a2e0[0] == '\0') {
    pcVar2 = FUN_004ddc10(0x33a);
    _sprintf(DAT_00b69d20, DAT_00528118, pcVar2);
  }
  else {
    puVar15 = DAT_00b6a2e0;
    uVar14 = 4;
    puVar8 = (unsigned int *)DAT_00b6bc9e;
    uVar13 = 6;
    pcVar2 = FUN_004ddc10(0x33a);
    _sprintf(DAT_00b69d20, s__s___c_s__c_s_0052b48c, pcVar2, uVar13, (char *)puVar8, uVar14, puVar15);
  }
  FUN_004db640(0xf, DAT_00b69d20, local_20, 0xffff);
  if (DAT_00b6a2e0[0] == '\0') {
    return 0;
  }
  if (DAT_00b69cd8 == (int *)0) {
    return 0;
  }
  if (DAT_00664f2c != 0) {
    if (DAT_00b6a258 != (void *)0) {
      _free(DAT_00b6a258);
      DAT_00b6a258 = (void *)0;
    }
    if (DAT_00b6a2d4 != (void *)0) {
      _free(DAT_00b6a2d4);
      DAT_00b6a2d4 = (void *)0;
    }
    if (DAT_00b6a2cc != (void *)0) {
      _free(DAT_00b6a2cc);
      DAT_00b6a2cc = (void *)0;
    }
    FUN_004e5000(5);
    uVar10 = 0;
    DAT_00b6a258 = DAT_00aa6114;
    DAT_00b6a25c = DAT_00aa6138;
    DAT_00aa6114 = (void *)0;
    FUN_004e5000(0);
    DAT_00b6a2d4 = DAT_00aa6114;
    DAT_00b69d18 = DAT_00aa6138;
    DAT_00b69e30 = 4;
    DAT_00aa6114 = (void *)0;
    FUN_004e5000(0);
    DAT_00b69e30 = 0;
    DAT_00b6a2cc = DAT_00aa6114;
    DAT_00bb2814 = DAT_00aa6138;
    DAT_00aa6114 = (void *)0;
    puVar8 = DAT_00664ee8;
    for (iVar4 = 0x10; iVar4 != 0; iVar4 = iVar4 + -1) {
      *puVar8 = 0;
      puVar8 = puVar8 + 1;
    }
    puVar8 = DAT_00664f60;
    for (iVar4 = 0x10; iVar4 != 0; iVar4 = iVar4 + -1) {
      *puVar8 = 0;
      puVar8 = puVar8 + 1;
    }
    if (DAT_00b6a25c != 0) {
      iVar4 = 0;
      do {
        uVar5 = 0xffffffff;
        pcVar2 = (char *)((int)DAT_00b6a258 + iVar4 + 0x40);
        do {
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar2 + 1;
        } while (cVar1 != '\0');
        for (iVar6 = (int)(~uVar5 - 2); iVar6 != 0; iVar6 = iVar6 + -1) {
          if (*(char *)((int)DAT_00b6a258 + iVar6 + iVar4 + 0x40) == '(') {
            *(char *)((int)DAT_00b6a258 + iVar6 + iVar4 + 0x40) = 0;
            break;
          }
        }
        iVar4 = (int)((unsigned int)iVar4 + 0x148u);
        uVar10 = uVar10 + 1;
      } while (uVar10 < DAT_00b6a25c);
    }
    DAT_00664ed0 = 0;
    DAT_00664fbc = 0;
    if (DAT_00b6a25c != 0) {
      piVar7 = (int *)((int)DAT_00b6a258 + 0x140);
      local_24 = DAT_00b6a25c;
      __builtin_memcpy(&v, DAT_00b6d962, 4);
      iVar4 = (int)((unsigned int)v * 0x109e0u);
      do {
        iVar6 = (int)((unsigned int)iVar4 + (unsigned int)*piVar7 * 0x24u);
        __builtin_memcpy(&t0, &DAT_00b7c342 + iVar6, 4);
        __builtin_memcpy(&t1, &DAT_00b7c6c6 + iVar6, 4);
        if ((t0 != 0) || (t1 != 0)) {
          uVar10 = 0;
          piVar11 = DAT_00b69cd8;
          if (DAT_00b69cc8 != 0) {
            do {
              if (*piVar11 == *piVar7) break;
              uVar10 = uVar10 + 1;
              piVar11 = piVar11 + 0x109;
            } while (uVar10 < DAT_00b69cc8);
          }
          if (uVar10 != DAT_00b69cc8) {
            uVar10 = 0;
            DAT_00664ec8 = 0;
            if (DAT_00b69d18 != 0) {
              puVar8 = DAT_00664ee8;
              piVar11 = (int *)((int)DAT_00b6a2d4 + 0x140);
              __builtin_memcpy(&v, DAT_00b6d962, 4);
              iVar6 = (int)((unsigned int)v * 0x84fu);
              do {
                iVar3 = (int)(((unsigned int)*piVar11 + (unsigned int)iVar6) * 0x20u);
                __builtin_memcpy(&t0, &DAT_00b7ca46 + iVar3, 4);
                __builtin_memcpy(&t1, &DAT_00b7ca4a + iVar3, 4);
                if ((t0 == *piVar7) && (t1 != 0)) {
                  __builtin_memcpy(&t2, &DAT_00b7ca4e + iVar3, 4);
                  if (t2 != 0) {
                    *puVar8 = 1;
                    DAT_00664ec8 = DAT_00664ec8 + 1;
                  }
                  puVar8 = puVar8 + 1;
                  if ((unsigned int *)0x664f27 < puVar8) break;
                }
                piVar11 = piVar11 + 0x52;
                uVar10 = uVar10 + 1;
              } while (uVar10 < DAT_00b69d18);
            }
            uVar10 = 0;
            DAT_00664f28 = 0;
            if (DAT_00bb2814 != 0) {
              puVar8 = DAT_00664f60;
              piVar11 = (int *)((int)DAT_00b6a2cc + 0x140);
              __builtin_memcpy(&v, DAT_00b6d962, 4);
              iVar6 = (int)((unsigned int)v * 0x84fu);
              do {
                iVar3 = (int)(((unsigned int)*piVar11 + (unsigned int)iVar6) * 0x20u);
                __builtin_memcpy(&t0, &DAT_00b7d6c6 + iVar3, 4);
                __builtin_memcpy(&t1, &DAT_00b7d6ca + iVar3, 4);
                if ((t0 == *piVar7) && (t1 != 0)) {
                  __builtin_memcpy(&t2, &DAT_00b7d6ce + iVar3, 4);
                  if (t2 != 0) {
                    *puVar8 = 1;
                    DAT_00664f28 = DAT_00664f28 + 1;
                  }
                  puVar8 = puVar8 + 1;
                  if ((unsigned int *)0x664f9f < puVar8) break;
                }
                piVar11 = piVar11 + 0x52;
                uVar10 = uVar10 + 1;
              } while (uVar10 < DAT_00bb2814);
            }
            DAT_00664fbc = DAT_00664fbc + 1;
          }
        }
        piVar7 = piVar7 + 0x52;
        local_24 = local_24 - 1;
      } while (local_24 != 0);
    }
    DAT_00664f2c = 0;
  }
  FUN_004d6460(local_20, 0x1a9, 0x6b, 0x1b2, 0x1b1);
  if (1 < DAT_00664fbc) {
    DAT_00664ed0 = FUN_004d9d70(local_20, DAT_00664ed0, DAT_00664fbc, 0, 5, DAT_00bb2810, 0xc);
  }
  iVar4 = 0;
  uVar10 = 0;
  if (DAT_00b6a25c != 0) {
    piVar7 = (int *)((int)DAT_00b6a258 + 0x140);
    do {
      __builtin_memcpy(&v, DAT_00b6d962, 4);
      iVar6 = (int)((unsigned int)*piVar7 * 0x24u + (unsigned int)v * 0x109e0u);
      __builtin_memcpy(&t0, &DAT_00b7c342 + iVar6, 4);
      __builtin_memcpy(&t1, &DAT_00b7c6c6 + iVar6, 4);
      if ((t0 != 0) || (t1 != 0)) {
        uVar5 = 0;
        bVar12 = DAT_00b69cc8 == 0;
        piVar11 = DAT_00b69cd8;
        if (DAT_00b69cc8 != 0) {
          do {
            if (*piVar11 == *piVar7) break;
            uVar5 = uVar5 + 1;
            piVar11 = piVar11 + 0x109;
          } while (uVar5 < DAT_00b69cc8);
          bVar12 = uVar5 == DAT_00b69cc8;
        }
        if (!bVar12) {
          if (DAT_00664ed0 == iVar4) {
            FUN_004d6460(local_20, 0x58, 0x6f, 0x1a8, 0x7d);
            FUN_004db640(0xc, (char *)((int)DAT_00b6a258 + uVar10 * 0x148 + 0x40), local_20,
                         DAT_00b6a2c0);
            iVar4 = (int)(uVar5 * 0x424u);
            FUN_004b4cc0((char *)(DAT_00b69cd8 + uVar5 * 0x109 + 1), local_10);
            iVar3 = (int)(0x100u - (unsigned int)(((int)((unsigned int)(local_10[2] - local_10[0]) + 1u)) >> 1));
            iVar6 = (int)(0x117u - (unsigned int)(((int)((unsigned int)(local_10[3] - local_10[1]) + 1u)) >> 1));
            FUN_004b5570((char *)(DAT_00b69cd8 + uVar5 * 0x109 + 1), iVar3, iVar6);
            if (DAT_00664ec8 != 0) {
              piVar7 = (int *)DAT_00664ee8;
              iVar9 = iVar4;
              do {
                if (*piVar7 != 0) {
                  FUN_004b5570((char *)((int)DAT_00b69cd8 + iVar9 + 0x24), iVar3, iVar6);
                }
                iVar9 = iVar9 + 0x20;
                piVar7 = piVar7 + 1;
              } while ((unsigned int *)piVar7 < &DAT_00664ee8[15]);
            }
            if (DAT_00664f28 == 0) {
              return 1;
            }
            piVar7 = (int *)DAT_00664f60;
            do {
              if (*piVar7 != 0) {
                FUN_004b5570((char *)((int)DAT_00b69cd8 + iVar4 + 0x224), iVar3, iVar6);
              }
              iVar4 = iVar4 + 0x20;
              piVar7 = piVar7 + 1;
            } while ((unsigned int *)piVar7 < &DAT_00664f60[15]);
            return 1;
          }
          iVar4 = iVar4 + 1;
        }
      }
      piVar7 = piVar7 + 0x52;
      uVar10 = uVar10 + 1;
    } while (uVar10 < DAT_00b6a25c);
  }
  return 1;
}
