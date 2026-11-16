/* SPDX-License-Identifier: MIT */
/* z_xvt 004e5320 */
extern unsigned int DAT_00758620[];          /* packet buffer base, indexed in dwords */
extern unsigned char DAT_0081bfd2[];         /* unaligned source, copied with memcpy */
extern unsigned int DAT_0081bfba;            /* packed: 0x81bfba is not 4-aligned */
extern unsigned int DAT_0081bfbe[];          /* packed: 4-byte elements at unaligned base */
extern int DAT_0081bcb8;
extern unsigned char DAT_00757cee[];         /* packed 0x2a-byte records, accessed unaligned */
extern unsigned char DAT_00757e3e[];
extern unsigned char DAT_00757cf6[];
extern unsigned char DAT_00757e44[];
extern unsigned char DAT_0085e7b4;
extern unsigned char DAT_0085e7b5;
extern unsigned char DAT_0085e7b6;
extern unsigned char DAT_0085e7b7;
extern unsigned char DAT_0085e7b8;
extern unsigned char DAT_0085e7b9;
extern unsigned char DAT_0085e7ba;
extern unsigned char DAT_0085e7bb;
extern unsigned char DAT_0085e7bc;
extern unsigned char DAT_0085e7bd;
extern unsigned char DAT_0085e7be;
extern unsigned char DAT_0085e7bf;
extern unsigned char DAT_0085e78f;
extern unsigned char DAT_0085e7c4;
extern unsigned char DAT_0085e790;

unsigned int * __cdecl FUN_004cc7b0(int *param_1);
unsigned int __cdecl FUN_004ced20(int param_1);
void __cdecl FUN_004cb830(int param_1, unsigned int *param_2, unsigned int param_3);
unsigned int __cdecl FUN_004cde00(void);
int __cdecl _rand(void);

void __cdecl FUN_004e5320(int param_1)
{
  int iVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  unsigned int *puVar4;
  unsigned int uVar5;
  int iVar6;
  int *piVar7;
  int *piVar8;
  int iVar10;
  int bVar11;
  int aiStack_8[2];
  unsigned char *pbVar7;
  unsigned char *pbVar9;
  int iTmp0;
  int iTmp1;
  unsigned int uTmp;
  unsigned int uIdx;
  unsigned int uElem;

  DAT_00758620[0] = 0x41;
  __builtin_memcpy(&DAT_00758620[1], &DAT_0081bfd2[0], 32);
  __builtin_memcpy(&uIdx, &DAT_0081bfba, 4);
  DAT_00758620[10] = uIdx;
  __builtin_memcpy(&uElem, (unsigned char *)&DAT_0081bfbe[0] + uIdx * 4u, 4);
  DAT_00758620[11] = uElem;
  if (DAT_0081bcb8 == 0) {
    DAT_00758620[12] = 8;
    iVar10 = 0xe;
    DAT_00758620[13] = FUN_004cde00();
    iVar6 = 0;
    puVar4 = FUN_004cc7b0(aiStack_8);
    if (0 < aiStack_8[0]) {
      piVar7 = (int *)(puVar4 + 8);
      do {
        if (piVar7[1] == 1) {
          DAT_00758620[iVar10] = (unsigned int)*piVar7;
          iVar10 = (int)((unsigned int)iVar10 + 3u);
          DAT_00758620[iVar10 - 2] = (unsigned int)(int)(*(unsigned char *)(piVar7 - 8)) - 1u;
          uVar5 = FUN_004ced20(*piVar7);
          DAT_00758620[iVar10 - 1] = uVar5;
        }
        piVar7 = piVar7 + 10;
        iVar6 = iVar6 + 1;
      } while (iVar6 < aiStack_8[0]);
    }
    uVar5 = (unsigned int)iVar10 << 2;
  }
  else {
    DAT_00758620[12] = 8;
    DAT_00758620[13] = 8;
    puVar4 = FUN_004cc7b0(aiStack_8);
    aiStack_8[1] = 0xe;
    pbVar7 = &DAT_00757cee[0];
    do {
      iVar6 = 0;
      bVar11 = (aiStack_8[0] == 0);
      if (0 < aiStack_8[0]) {
        piVar8 = (int *)(puVar4 + 8);
        do {
          __builtin_memcpy(&iTmp0, pbVar7, 4);
          if (((*piVar8 != 0) && (piVar8[1] != 0)) && (iTmp0 == *piVar8)) {
            iTmp1 = (int)((unsigned int)(int)(*(unsigned char *)(puVar4 + iVar6 * 10)) - 1u);
            __builtin_memcpy(pbVar7 + 4, &iTmp1, 4);
            break;
          }
          piVar8 = piVar8 + 10;
          iVar6 = iVar6 + 1;
        } while (iVar6 < aiStack_8[0]);
        bVar11 = (iVar6 == aiStack_8[0]);
      }
      if (bVar11) {
        iTmp0 = 0;
        __builtin_memcpy(pbVar7, &iTmp0, 4);
      }
      pbVar7 = pbVar7 + 0x2a;
    } while (pbVar7 < &DAT_00757e3e[0]);
    iVar6 = 0xe;
    pbVar7 = &DAT_00757cee[0];
    do {
      __builtin_memcpy(&iVar10, pbVar7, 4);
      __builtin_memcpy(&iVar1, pbVar7 + 4, 4);
      DAT_00758620[iVar6] = (unsigned int)iVar10;
      DAT_00758620[iVar6 + 1] = (unsigned int)iVar1;
      iVar6 = (int)((unsigned int)iVar6 + 3u);
      pbVar7 = pbVar7 + 0x2a;
      uVar5 = FUN_004ced20(iVar10);
      DAT_00758620[iVar6 - 1] = uVar5;
    } while (pbVar7 < &DAT_00757e3e[0]);
    FUN_004cb830(param_1, &DAT_00758620[0], (unsigned int)iVar6 * 4u);
    iVar6 = 1;
    DAT_00758620[0] = 0x71;
    pbVar7 = &DAT_00757cf6[0];
    do {
      __builtin_memcpy(&uTmp, pbVar7 + 4, 4);
      uVar2 = uTmp;
      __builtin_memcpy(&uTmp, pbVar7, 4);
      DAT_00758620[iVar6] = uTmp;
      __builtin_memcpy(&uTmp, pbVar7 + 8, 4);
      uVar3 = uTmp;
      pbVar9 = pbVar7 + 0x2a;
      DAT_00758620[iVar6 + 1] = uVar2;
      iVar6 = (int)((unsigned int)iVar6 + 5u);
      __builtin_memcpy(&uTmp, pbVar7 + 12, 4);
      uVar2 = uTmp;
      DAT_00758620[iVar6 - 3] = uVar3;
      __builtin_memcpy(&uTmp, pbVar7 + 16, 4);
      uVar3 = uTmp;
      DAT_00758620[iVar6 - 2] = uVar2;
      DAT_00758620[iVar6 - 1] = uVar3;
      pbVar7 = pbVar9;
    } while (pbVar9 < &DAT_00757e44[0] + 2);
    uVar5 = (unsigned int)iVar6 * 4u;
  }
  FUN_004cb830(param_1, &DAT_00758620[0], uVar5);
  DAT_00758620[0] = 0x59;
  DAT_00758620[1] = (unsigned int)DAT_0085e7b4;
  DAT_00758620[2] = (unsigned int)DAT_0085e7b5;
  DAT_00758620[3] = (unsigned int)DAT_0085e7b6;
  DAT_00758620[4] = (unsigned int)DAT_0085e7b7;
  DAT_00758620[5] = (unsigned int)DAT_0085e7b8;
  DAT_00758620[6] = (unsigned int)DAT_0085e7b9;
  DAT_00758620[7] = (unsigned int)DAT_0085e7ba;
  DAT_00758620[8] = (unsigned int)DAT_0085e7bb;
  DAT_00758620[9] = (unsigned int)DAT_0085e7bc;
  DAT_00758620[10] = (unsigned int)DAT_0085e7bd;
  DAT_00758620[11] = (unsigned int)DAT_0085e7be;
  DAT_00758620[12] = (unsigned int)DAT_0085e7bf;
  DAT_00758620[13] = (unsigned int)_rand();
  DAT_00758620[14] = (unsigned int)DAT_0085e78f;
  DAT_00758620[15] = (unsigned int)DAT_0085e7c4;
  DAT_00758620[16] = (unsigned int)DAT_0085e790;
  FUN_004cb830(param_1, &DAT_00758620[0], 0x44);
  return;
}
