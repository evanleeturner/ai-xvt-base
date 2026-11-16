/* SPDX-License-Identifier: MIT */
/* z_xvt 004e4820 */
typedef struct _iobuf FILE;

extern uint DAT_00757ccc;
extern uint DAT_00757cc0;
extern byte DAT_0081bfba[4];
extern byte DAT_0081bfbe[];
extern char DAT_0081b740[];
extern char *PTR_s_train_005330e0[];
extern char s__s__s_005331e4[];
extern char DAT_00523570[];
extern uint DAT_00534aac;
extern byte *PTR_DAT_005348a0;

int __cdecl _sprintf(char *, const char *, ...);
FILE * __cdecl FUN_004be160(char *, char *);
int __cdecl FUN_004be220(FILE *, int, int);
int __cdecl FUN_004be320(FILE *, byte *, int);
int __cdecl FUN_004be1f0(FILE *);
char * __cdecl _fgets(char *, int, FILE *);
int __cdecl _atoi(const char *);
int __cdecl __isctype(uint, int);

void __cdecl FUN_004e4820(byte *param_1)
{
  char cVar1;
  byte bVar2;
  FILE *_File;
  char *pcVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  int *piVar8;
  uint _C;
  byte *pbVar9;
  uint uVar10;
  int iTrain;
  int iVal;

  if (param_1 != (byte *)0) {
    pbVar9 = param_1;
    for (iVar5 = 0x100; iVar5 != 0; iVar5 = iVar5 + -1) {
      pbVar9[0] = 0;
      pbVar9[1] = 0;
      pbVar9[2] = 0;
      pbVar9[3] = 0;
      pbVar9 = pbVar9 + 4;
    }
    uVar6 = 0;
    if (DAT_00757ccc != 0) {
      piVar8 = (int *)(DAT_00757cc0 + 0x140);
      do {
        __builtin_memcpy(&iTrain, DAT_0081bfba, 4);
        __builtin_memcpy(&iVal, DAT_0081bfbe + (uint)iTrain * 4u, 4);
        if (*piVar8 == iVal) break;
        piVar8 = piVar8 + 0x51;
        uVar6 = uVar6 + 1;
      } while (uVar6 < DAT_00757ccc);
      if (uVar6 < DAT_00757ccc) {
        __builtin_memcpy(&iTrain, DAT_0081bfba, 4);
        _sprintf(DAT_0081b740, s__s__s_005331e4, PTR_s_train_005330e0[iTrain],
                 (char *)(uVar6 * 0x144 + DAT_00757cc0));
        _File = FUN_004be160(DAT_0081b740, DAT_00523570);
        if (_File != (FILE *)0) {
          __builtin_memcpy(&iTrain, DAT_0081bfba, 4);
          if ((iTrain == 2) || (iTrain == 4)) {
            _fgets(DAT_0081b740, 0x100, _File);
            for (iVar5 = _atoi(DAT_0081b740); iVar5 != 0; iVar5 = (int)((uint)iVar5 - 1u)) {
              _fgets(DAT_0081b740, 0x100, _File);
            }
            uVar6 = 0;
            pbVar9 = param_1;
            while (pcVar3 = _fgets(DAT_0081b740, 0x100, _File), pcVar3 != (char *)0) {
              uVar7 = 0xffffffff;
              pcVar3 = DAT_0081b740;
              do {
                if (uVar7 == 0) break;
                uVar7 = uVar7 - 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar3 + 1;
              } while (cVar1 != '\0');
              uVar10 = 0;
              if (~uVar7 != 1) {
                do {
                  bVar2 = *(byte *)(DAT_0081b740 + uVar10);
                  _C = (uint)bVar2;
                  if (DAT_00534aac < 2) {
                    uVar4 = *(ushort *)(PTR_DAT_005348a0 + _C * 2) & 0x157;
                  }
                  else {
                    uVar4 = (uint)__isctype(_C, 0x157);
                  }
                  if ((uVar4 != 0) || (_C == 10)) {
                    *pbVar9 = bVar2;
                    pbVar9 = pbVar9 + 1;
                    uVar6 = uVar6 + 1;
                    if (0x3fe < uVar6) goto LAB_004e49fd;
                  }
                  uVar10 = uVar10 + 1;
                } while (uVar10 < ~uVar7 - 1);
              }
              if (0x3fe < uVar6) break;
            }
LAB_004e49fd:
            param_1[uVar6] = 0;
          }
          else {
            FUN_004be220(_File, -0x400, 2);
            FUN_004be320(_File, param_1, 0x400);
            param_1[0x3ff] = 0;
          }
          FUN_004be1f0(_File);
        }
      }
    }
  }
  return;
}
