/* SPDX-License-Identifier: MIT */
/* z_bop 00464d70 */
extern int DAT_005242c8;
extern int DAT_00a080f8;
extern unsigned int DAT_005242e8;
extern unsigned int DAT_00523418;
extern int DAT_00557350;
extern unsigned int DAT_00a07cc8;
extern unsigned int DAT_00557360;
extern unsigned int DAT_00557364;
extern unsigned char DAT_00557368;
extern unsigned int DAT_009d77d0[8];
extern int DAT_009a8db0[];
extern int DAT_009ed670[];
extern char DAT_009e9680[];
extern char DAT_009ec468[];
extern int DAT_009fd394;
extern int DAT_005242d0;
extern void *DAT_005242d8;
extern char DAT_0052436c[];
extern char DAT_00524390[];
extern char s_serverlog_txt_00524398[];
extern char s__2x_00524388[];
extern char s__8x__2x__2x__2x__2x_00524370[];

/* NOTE: calling convention and return type of FUN_0046d350 are unverified. */
void FUN_0046d350(int, unsigned int *, unsigned int);
void *FID_conflict___wfopen(char *, char *);
int _fprintf(void *, char *, ...);
int _fflush(void *);

void __stdcall FUN_00464d70(void)
{
  unsigned int uVar2;
  int iVar3;
  unsigned char *pbVar4;
  unsigned char *pbVar5;
  unsigned char *pbVar6;
  unsigned int uVar7;
  unsigned int uVar8;
  char *pcVar9;
  unsigned int uVar10;
  int iVar11;
  unsigned char *pbVar12;
  unsigned int *puVar13;
  int *local_10;
  char *local_c;
  int *local_8;
  unsigned char *piVar1;
  int tmpA;
  int tmpB;
  short tmpS;

  DAT_005242c8 = DAT_005242c8 + 1;
  if (1 < DAT_00a080f8) {
    uVar2 = DAT_005242e8 + DAT_00523418;
    DAT_00557350 = (int)((unsigned int)DAT_00557350 + (uVar2 - DAT_00a07cc8));
    DAT_00557360 = 2;
    DAT_005242e8 = uVar2;
    DAT_00557364 = uVar2;
    if (0x1d8 < DAT_00557350) {
      DAT_00557364 = uVar2 | 0x80000000;
      DAT_00557350 = 0;
      puVar13 = DAT_009d77d0;
      for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar13 = 0;
        puVar13 = puVar13 + 1;
      }
    }
    DAT_00557368 = 0;
    uVar10 = 9;
    local_10 = DAT_009a8db0;
    local_8 = DAT_009ed670;
    local_c = DAT_009e9680;
    pbVar12 = (unsigned char *)((int)&DAT_00557368 + 1);
    do {
      pbVar4 = pbVar12;
      if (*local_c != '\0') {
        DAT_00557368 = (unsigned char)(DAT_00557368 + 1);
        pbVar4 = pbVar12 + 1;
        uVar10 = uVar10 + 1;
        iVar3 = 0;
        *pbVar12 = 0;
        piVar1 = (unsigned char *)local_8;
        if (0 < *local_10) {
          do {
            __builtin_memcpy(&tmpA, piVar1, 4);
            __builtin_memcpy(&tmpB, piVar1 + 8, 4);
            if ((tmpA != 0) && (tmpB <= (int)uVar2)) {
              if (0x1f8 - DAT_009fd394 < (int)pbVar4 - 0x557358) break;
              *pbVar12 = (unsigned char)(*pbVar12 + 1);
              uVar7 = uVar2 - (unsigned int)tmpB;
              if ((int)uVar7 < 0x10000) {
                if ((int)uVar7 < 0x170) {
                  if (0x7c < (int)uVar7) {
                    uVar7 = 0x7d;
                  }
                }
                else {
                  uVar7 = 0x7e;
                }
              }
              else {
                uVar7 = 0x7f;
              }
              if (piVar1[0xe] != 0) {
                uVar7 = uVar7 | 0x80;
              }
              *pbVar4 = (unsigned char)uVar7;
              pbVar5 = pbVar4 + 1;
              iVar11 = (int)(uVar10 + 1);
              uVar8 = uVar7 & 0x7f;
              if (uVar8 == 0x7f) {
                pbVar5 = pbVar4 + 5;
                iVar11 = (int)(uVar10 + 5);
                __builtin_memcpy(pbVar4 + 1, &tmpB, 4);
              }
              else if (uVar8 == 0x7e) {
                pbVar5 = pbVar4 + 3;
                iVar11 = (int)(uVar10 + 3);
                tmpS = (short)((short)uVar2 - (short)tmpB);
                __builtin_memcpy(pbVar4 + 1, &tmpS, 2);
              }
              else if (uVar8 == 0x7d) {
                pbVar5 = pbVar4 + 2;
                iVar11 = (int)(uVar10 + 2);
                pbVar4[1] = (unsigned char)(((char)uVar2 - (char)tmpB) + 0x83);
              }
              pbVar6 = pbVar5;
              if ((uVar7 & 0x80) != 0) {
                pbVar6 = pbVar5 + 1;
                iVar11 = iVar11 + 1;
                *pbVar5 = piVar1[0xe];
              }
              pbVar4 = pbVar6 + 2;
              uVar10 = (unsigned int)(iVar11 + 2);
              *pbVar6 = piVar1[0xf];
              pbVar6[1] = piVar1[0x10];
              *pbVar6 = (unsigned char)(*pbVar6 & 0xfe);
              pbVar6[1] = (unsigned char)(pbVar6[1] & 0xfe);
              *pbVar6 = (unsigned char)(*pbVar6 | (piVar1[0x11] & 1));
              pbVar6[1] = (unsigned char)(pbVar6[1] | ((piVar1[0x11] & 2) >> 1));
              tmpA = 0;
              __builtin_memcpy(piVar1, &tmpA, 4);
            }
            iVar3 = iVar3 + 1;
            piVar1 = piVar1 + 0x12;
          } while (iVar3 < *local_10);
        }
      }
      local_10 = local_10 + 1;
      local_8 = local_8 + 0x7e9;
      local_c = local_c + 0x5bd;
      pbVar12 = pbVar4;
    } while ((unsigned int)local_c < (unsigned int)DAT_009ec468);
    FUN_0046d350(0, &DAT_00557360, uVar10);
    if ((DAT_005242d0 == 1) &&
        ((DAT_005242d8 != (void *)0 ||
          (DAT_005242d8 = FID_conflict___wfopen(s_serverlog_txt_00524398, DAT_0052436c),
           DAT_005242d8 != (void *)0)))) {
      pbVar12 = (unsigned char *)((int)&DAT_00557368 + 1);
      pcVar9 = DAT_009e9680;
      _fprintf(DAT_005242d8, DAT_00524390, DAT_00557364);
      do {
        if (*pcVar9 != '\0') {
          uVar2 = (unsigned int)*pbVar12;
          _fprintf(DAT_005242d8, s__2x_00524388, uVar2);
          pbVar12 = pbVar12 + 1;
          for (; uVar2 != 0; uVar2 = uVar2 - 1) {
            __builtin_memcpy(&tmpA, pbVar12, 4);
            _fprintf(DAT_005242d8, s__8x__2x__2x__2x__2x_00524370, tmpA,
                     (unsigned int)pbVar12[6], (unsigned int)pbVar12[7],
                     (unsigned int)pbVar12[8], (unsigned int)pbVar12[9]);
            pbVar12 = pbVar12 + 10;
          }
        }
        pcVar9 = pcVar9 + 0x5bd;
      } while ((unsigned int)pcVar9 < (unsigned int)DAT_009ec468);
      _fflush(DAT_005242d8);
    }
  }
  return;
}
