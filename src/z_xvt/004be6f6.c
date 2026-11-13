/* SPDX-License-Identifier: MIT */
/* z_xvt 004be6f6 */
unsigned long __stdcall GetLogicalDriveStringsA(unsigned long, char *);
unsigned int __stdcall GetDriveTypeA(const char *);
void *__cdecl _fopen(const char *, const char *);
int __cdecl _fclose(void *);
extern int in_stack_00000160;
extern char DAT_0052a330[];

char __stdcall FUN_004be6f6(void)
{
  char cVar1;
  unsigned long nBufferLength;
  unsigned int UVar2;
  void *_File;
  unsigned int uVar3;
  unsigned int uVar4;
  int iVar5;
  int iVar6;
  char *lpRootPathName;
  char *pcVar7;
  const char *pCVar8;
  char *pcVar9;
  char *pcVar10;
  int iStack00000004;
  char stack0x00000058[0x200];
  char stack0x00000008[0x200];
  
  iVar6 = 0;
  iStack00000004 = 0;
  nBufferLength = GetLogicalDriveStringsA(0, (char *)0x0);
  GetLogicalDriveStringsA(nBufferLength, stack0x00000058);
  lpRootPathName = stack0x00000058;
  if (0 < (int)nBufferLength) {
    do {
      UVar2 = GetDriveTypeA(lpRootPathName);
      if (UVar2 == 5) {
        uVar3 = 0xffffffff;
        pCVar8 = lpRootPathName;
        do {
          pcVar7 = (char *)pCVar8;
          if (uVar3 == 0) break;
          uVar3 = uVar3 - 1;
          pcVar7 = (char *)pCVar8 + 1;
          cVar1 = *pCVar8;
          pCVar8 = pcVar7;
        } while (cVar1 != '\0');
        uVar3 = ~uVar3;
        pcVar7 = pcVar7 - uVar3;
        pcVar10 = stack0x00000008;
        for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
          *(unsigned int *)pcVar10 = *(unsigned int *)pcVar7;
          pcVar7 = pcVar7 + 4;
          pcVar10 = pcVar10 + 4;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *pcVar10 = *pcVar7;
          pcVar7 = pcVar7 + 1;
          pcVar10 = pcVar10 + 1;
        }
        uVar3 = 0xffffffff;
        pcVar7 = (char *)(in_stack_00000160 + 1);
        do {
          pcVar10 = pcVar7;
          if (uVar3 == 0) break;
          uVar3 = uVar3 - 1;
          pcVar10 = pcVar7 + 1;
          cVar1 = *pcVar7;
          pcVar7 = pcVar10;
        } while (cVar1 != '\0');
        uVar3 = ~uVar3;
        iVar5 = -1;
        pcVar7 = stack0x00000008;
        do {
          pcVar9 = pcVar7;
          if (iVar5 == 0) break;
          iVar5 = iVar5 + -1;
          pcVar9 = pcVar7 + 1;
          cVar1 = *pcVar7;
          pcVar7 = pcVar9;
        } while (cVar1 != '\0');
        pcVar7 = pcVar10 - uVar3;
        pcVar10 = pcVar9 + -1;
        for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
          *(unsigned int *)pcVar10 = *(unsigned int *)pcVar7;
          pcVar7 = pcVar7 + 4;
          pcVar10 = pcVar10 + 4;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *pcVar10 = *pcVar7;
          pcVar7 = pcVar7 + 1;
          pcVar10 = pcVar10 + 1;
        }
        _File = _fopen(stack0x00000008, DAT_0052a330);
        if (_File != (void *)0x0) {
          _fclose(_File);
          iStack00000004 = 1;
          break;
        }
      }
      lpRootPathName = lpRootPathName + 4;
      iVar6 = iVar6 + 4;
    } while (iVar6 < (int)nBufferLength);
  }
  if (iStack00000004 == 0) {
    return '\0';
  }
  return *lpRootPathName;
}
