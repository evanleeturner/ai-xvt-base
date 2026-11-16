/* SPDX-License-Identifier: MIT */
/* z_bop 004f6b80 */
typedef struct _iobuf FILE;

extern int DAT_00aa6114;
extern int DAT_00aa6134;
extern int DAT_00aa6138;
extern char *PTR_s_train_0052c1e8[];
extern char s__s__s_0052c300[];
extern char DAT_0051a854[];

extern unsigned char DAT_00b6a5aa[];
extern unsigned char DAT_00b6a5ae[];
extern unsigned char DAT_00b6a5a6[];

extern unsigned int DAT_00a91cc0[];
extern short DAT_00aa5ab2;
extern short DAT_00aa5aac;
extern short DAT_00aa5aae;
extern unsigned char DAT_00aa471c[];
extern unsigned char DAT_00aa1f20[];
extern unsigned char DAT_00aa3ba0[];
extern unsigned char DAT_00aa47ba[];
extern unsigned int DAT_00669258[];
extern int DAT_00669214;
extern unsigned char DAT_00669582[];
extern unsigned char DAT_00669602[];
extern unsigned char DAT_00669682[];

void FUN_004e5000(int);
int _sprintf(char *, const char *, ...);
FILE *FUN_004cc4a0(char *, const char *);
void FUN_004cc660(FILE *, void *);
void FUN_004cc590(FILE *);
void FUN_004cc6c0(FILE *, void *, int);
void FUN_004cc630(FILE *, void *);
void FUN_004cc5c0(FILE *, int, int);

void __stdcall FUN_004f6b80(void)
{
  FILE *pFVar1;
  int iVar2;
  int *piVar3;
  unsigned int uVar4;
  unsigned char *puVar5;
  unsigned short *puVar6;
  unsigned int *puVar7;
  unsigned char *puVar8;
  unsigned int *puVar9;
  unsigned int *puVar10;
  unsigned char *pbVar11;
  char local_439;
  short local_438;
  short local_436;
  int local_434;
  int local_430;
  char local_42c[256];
  unsigned int local_32c[203];
  int iTrain;
  int iTmp;
  unsigned int uTmp;

  __builtin_memcpy(&iTrain, DAT_00b6a5aa, 4);
  FUN_004e5000(iTrain);
  if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
    piVar3 = (int *)(DAT_00aa6114 + 0x140);
    do {
      __builtin_memcpy(&iTrain, DAT_00b6a5aa, 4);
      __builtin_memcpy(&iTmp, DAT_00b6a5ae + (unsigned int)iTrain * 4u, 4);
      if (*piVar3 == iTmp) break;
      piVar3 = piVar3 + 0x52;
      DAT_00aa6134 = DAT_00aa6134 + 1;
    } while ((unsigned int)DAT_00aa6134 < (unsigned int)DAT_00aa6138);
  }
  __builtin_memcpy(&iTrain, DAT_00b6a5aa, 4);
  _sprintf(local_42c, s__s__s_0052c300, PTR_s_train_0052c1e8[iTrain],
           (char *)((unsigned int)DAT_00aa6134 * 0x148u + (unsigned int)DAT_00aa6114));
  pFVar1 = FUN_004cc4a0(local_42c, DAT_0051a854);
  if (pFVar1 != (FILE *)0) {
    puVar7 = DAT_00a91cc0;
    for (iVar2 = 0x4f7d; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar7 = 0;
      puVar7 = puVar7 + 1;
    }
    FUN_004cc660(pFVar1, &DAT_00aa5ab2);
    if (((DAT_00aa5ab2 != 0xe) && (DAT_00aa5ab2 != 0xd)) && (DAT_00aa5ab2 != 0xc)) {
      FUN_004cc590(pFVar1);
      return;
    }
    FUN_004cc660(pFVar1, &DAT_00aa5aac);
    FUN_004cc660(pFVar1, &DAT_00aa5aae);
    iVar2 = 0;
    FUN_004cc6c0(pFVar1, DAT_00aa471c, 0x9e);
    if (0 < DAT_00aa5aac) {
      puVar7 = DAT_00a91cc0;
      do {
        iVar2 = iVar2 + 1;
        FUN_004cc6c0(pFVar1, puVar7, 0x562);
        puVar7 = (unsigned int *)((int)puVar7 + 0x562);
      } while (iVar2 < DAT_00aa5aac);
    }
    iVar2 = 0;
    if (0 < DAT_00aa5aae) {
      do {
        iVar2 = iVar2 + 1;
        FUN_004cc660(pFVar1, &local_438);
        FUN_004cc6c0(pFVar1, DAT_00aa1f20 + local_438 * 0x72, 0x72);
      } while (iVar2 < DAT_00aa5aae);
    }
    puVar8 = DAT_00aa3ba0;
    do {
      iVar2 = 0;
      FUN_004cc660(pFVar1, &local_438);
      puVar5 = puVar8;
      if (0 < local_438) {
        do {
          iVar2 = iVar2 + 1;
          FUN_004cc6c0(pFVar1, puVar5, 0x2a);
          puVar5 = puVar5 + 0x2a;
        } while (iVar2 < local_438);
      }
      puVar8 = puVar8 + 0x126;
    } while (puVar8 < DAT_00aa471c);
    puVar6 = (unsigned short *)DAT_00aa47ba;
    do {
      FUN_004cc660(pFVar1, &local_438);
      if (local_438 != 0) {
        FUN_004cc6c0(pFVar1, puVar6, 0x1e5);
      }
      puVar6 = (unsigned short *)((int)puVar6 + 0x1e5);
    } while ((unsigned char *)puVar6 < (unsigned char *)&DAT_00aa5aac);
    local_430 = 0;
    do {
      uVar4 = 0;
      local_434 = 0;
      FUN_004cc6c0(pFVar1, local_32c, 0x32a);
      do {
        FUN_004cc630(pFVar1, &local_439);
        __builtin_memcpy(&uTmp, DAT_00b6a5a6, 4);
        if ((uTmp == uVar4) && (local_439 != '\0')) {
          puVar7 = local_32c;
          puVar9 = DAT_00669258;
          for (iVar2 = 0xca; iVar2 != 0; iVar2 = iVar2 + -1) {
            *puVar9 = *puVar7;
            puVar7 = puVar7 + 1;
            puVar9 = puVar9 + 1;
          }
          *(unsigned short *)puVar9 = *(unsigned short *)puVar7;
          local_434 = 1;
          DAT_00669214 = local_430;
        }
        uVar4 = uVar4 + 1;
      } while (uVar4 < 10);
      pbVar11 = DAT_00669582;
      do {
        __builtin_memcpy(&puVar9, pbVar11, 4);
        if (local_434 != 0) {
          puVar10 = puVar9;
          for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 + -1) {
            *puVar10 = 0;
            puVar10 = puVar10 + 1;
          }
        }
        FUN_004cc660(pFVar1, &local_436);
        if (local_436 != 0) {
          if (local_434 == 0) {
            FUN_004cc5c0(pFVar1, (int)local_436, 1);
          }
          else {
            FUN_004cc6c0(pFVar1, puVar9, (int)local_436);
          }
        }
        if (local_434 != 0) {
          *(unsigned char *)((int)local_436 + (int)puVar9) = 0;
        }
        pbVar11 = pbVar11 + 4;
      } while (pbVar11 < DAT_00669602);
      pbVar11 = DAT_00669602;
      do {
        __builtin_memcpy(&puVar9, pbVar11, 4);
        if (local_434 != 0) {
          puVar10 = puVar9;
          for (iVar2 = 0x50; iVar2 != 0; iVar2 = iVar2 + -1) {
            *puVar10 = 0;
            puVar10 = puVar10 + 1;
          }
        }
        FUN_004cc660(pFVar1, &local_436);
        if (local_436 != 0) {
          if (local_434 == 0) {
            FUN_004cc5c0(pFVar1, (int)local_436, 1);
          }
          else {
            FUN_004cc6c0(pFVar1, puVar9, (int)local_436);
          }
        }
        if (local_434 != 0) {
          *(unsigned char *)((int)local_436 + (int)puVar9) = 0;
        }
        pbVar11 = pbVar11 + 4;
      } while (pbVar11 < DAT_00669682);
      local_430 = local_430 + 1;
    } while (local_430 < 8);
    FUN_004cc590(pFVar1);
  }
  return;
}
