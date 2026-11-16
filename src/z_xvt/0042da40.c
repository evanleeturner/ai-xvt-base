/* SPDX-License-Identifier: MIT */
/* z_xvt 0042da40 */
typedef struct _iobuf FILE;

extern int DAT_005250c0;
extern int DAT_00732e48;
extern unsigned int DAT_005250e0;
extern unsigned int DAT_00525558;
extern int DAT_005373c0;
extern unsigned int DAT_00732a18;
extern unsigned int _DAT_005373d0;
extern unsigned int _DAT_005373d4;
extern char DAT_005373d8;
extern unsigned char DAT_005373d9;
extern unsigned int DAT_0070a150[8];
extern int DAT_006db730[];
extern int DAT_0071f720[];
extern char DAT_0071b740[];
extern char DAT_0071e528;
extern int DAT_0072f444;
extern int DAT_005250c8;
extern FILE *DAT_005250d0;
extern char DAT_00525164[];
extern char DAT_00525188[];
extern char s_serverlog_txt_00525190[];
extern char s__2x_00525180[];
extern char s__8x__2x__2x__2x__2x_00525168[];

void FUN_00431110(int param_1, unsigned int *param_2, unsigned int param_3);
FILE *_fopen(const char *, const char *);
int _fprintf(FILE *, const char *, ...);
int _fflush(FILE *);

void __stdcall FUN_0042da40(void)
{
  unsigned char *piVar1;
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
  int *local_10;
  char *local_c;
  int *local_8;
  int iEntry0;
  int iEntry8;
  int iZero;
  short sDelta;
  unsigned int uLog;

  DAT_005250c0 = (int)((unsigned int)DAT_005250c0 + 1u);
  if (1 < DAT_00732e48) {
    uVar2 = DAT_005250e0 + DAT_00525558;
    DAT_005373c0 = (int)((unsigned int)DAT_005373c0 + (uVar2 - DAT_00732a18));
    _DAT_005373d0 = 2;
    DAT_005250e0 = uVar2;
    _DAT_005373d4 = uVar2;
    if (0x1d8 < DAT_005373c0) {
      _DAT_005373d4 = uVar2 | 0x80000000u;
      DAT_005373c0 = 0;
      for (iVar3 = 0; iVar3 < 8; iVar3 = (int)((unsigned int)iVar3 + 1u)) {
        DAT_0070a150[iVar3] = 0;
      }
    }
    DAT_005373d8 = '\0';
    uVar10 = 9;
    local_10 = &DAT_006db730[0];
    local_8 = &DAT_0071f720[0];
    local_c = &DAT_0071b740[0];
    pbVar12 = &DAT_005373d9;
    do {
      pbVar4 = pbVar12;
      if (*local_c != '\0') {
        DAT_005373d8 = (char)(DAT_005373d8 + '\x01');
        pbVar4 = pbVar12 + 1;
        uVar10 = uVar10 + 1;
        iVar3 = 0;
        *pbVar12 = 0;
        piVar1 = (unsigned char *)local_8;
        if (0 < *local_10) {
          do {
            __builtin_memcpy(&iEntry0, piVar1, 4);
            __builtin_memcpy(&iEntry8, piVar1 + 8, 4);
            if ((iEntry0 != 0) && (iEntry8 <= (int)uVar2)) {
              if ((int)(0x1f8u - (unsigned int)DAT_0072f444) < (int)((unsigned int)pbVar4 - 0x5373c8u)) break;
              *pbVar12 = (unsigned char)(*pbVar12 + 1);
              uVar7 = uVar2 - (unsigned int)iEntry8;
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
              if (piVar1[0xe] != '\0') {
                uVar7 = uVar7 | 0x80;
              }
              *pbVar4 = (unsigned char)uVar7;
              pbVar5 = pbVar4 + 1;
              iVar11 = (int)(uVar10 + 1u);
              uVar8 = uVar7 & 0x7f;
              if (uVar8 == 0x7f) {
                pbVar5 = pbVar4 + 5;
                iVar11 = (int)(uVar10 + 5u);
                __builtin_memcpy(pbVar4 + 1, &iEntry8, 4);
              }
              else if (uVar8 == 0x7e) {
                pbVar5 = pbVar4 + 3;
                iVar11 = (int)(uVar10 + 3u);
                sDelta = (short)((unsigned short)uVar2 - (unsigned short)iEntry8);
                __builtin_memcpy(pbVar4 + 1, &sDelta, 2);
              }
              else if (uVar8 == 0x7d) {
                pbVar5 = pbVar4 + 2;
                iVar11 = (int)(uVar10 + 2u);
                pbVar4[1] = (unsigned char)((unsigned char)((unsigned char)uVar2 - (unsigned char)iEntry8) + 0x83);
              }
              pbVar6 = pbVar5;
              if ((uVar7 & 0x80) != 0) {
                pbVar6 = pbVar5 + 1;
                iVar11 = (int)((unsigned int)iVar11 + 1u);
                *pbVar5 = piVar1[0xe];
              }
              pbVar4 = pbVar6 + 2;
              uVar10 = (unsigned int)iVar11 + 2u;
              *pbVar6 = piVar1[0xf];
              pbVar6[1] = piVar1[0x10];
              *pbVar6 = (unsigned char)(*pbVar6 & 0xfe);
              pbVar6[1] = (unsigned char)(pbVar6[1] & 0xfe);
              *pbVar6 = (unsigned char)(*pbVar6 | (piVar1[0x11] & 1));
              pbVar6[1] = (unsigned char)(pbVar6[1] | ((piVar1[0x11] & 2) >> 1));
              iZero = 0;
              __builtin_memcpy(piVar1, &iZero, 4);
            }
            iVar3 = (int)((unsigned int)iVar3 + 1u);
            piVar1 = piVar1 + 0x12;
          } while (iVar3 < *local_10);
        }
      }
      local_10 = local_10 + 1;
      local_8 = local_8 + 0x7e9;
      local_c = local_c + 0x5bd;
      pbVar12 = pbVar4;
    } while (local_c < &DAT_0071e528);
    FUN_00431110(0, (unsigned int *)&_DAT_005373d0, uVar10);
    if ((DAT_005250c8 == 1) &&
        ((DAT_005250d0 != (FILE *)0 ||
          (DAT_005250d0 = _fopen(s_serverlog_txt_00525190, DAT_00525164), DAT_005250d0 != (FILE *)0)))) {
      pbVar12 = &DAT_005373d9;
      pcVar9 = &DAT_0071b740[0];
      _fprintf(DAT_005250d0, DAT_00525188, _DAT_005373d4);
      do {
        if (*pcVar9 != '\0') {
          uVar2 = (unsigned int)*pbVar12;
          _fprintf(DAT_005250d0, s__2x_00525180, uVar2);
          pbVar12 = pbVar12 + 1;
          for (; uVar2 != 0; uVar2 = uVar2 - 1) {
            __builtin_memcpy(&uLog, pbVar12, 4);
            _fprintf(DAT_005250d0, s__8x__2x__2x__2x__2x_00525168, uLog,
                     (unsigned int)pbVar12[6], (unsigned int)pbVar12[7],
                     (unsigned int)pbVar12[8], (unsigned int)pbVar12[9]);
            pbVar12 = pbVar12 + 10;
          }
        }
        pcVar9 = pcVar9 + 0x5bd;
      } while (pcVar9 < &DAT_0071e528);
      _fflush(DAT_005250d0);
    }
  }
  return;
}
