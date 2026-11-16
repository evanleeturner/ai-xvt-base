/* SPDX-License-Identifier: MIT */
/* z_xvt 0051a870 */
extern unsigned short DAT_005366d4;
extern unsigned short DAT_006550d0;
extern CRITICAL_SECTION DAT_007427a0;
extern int *DAT_00742668;
extern void (__cdecl *DAT_00742724)(int *, int, int, int, unsigned int, unsigned int);

void __stdcall EnterCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
void __stdcall LeaveCriticalSection(LPCRITICAL_SECTION lpCriticalSection);

void __stdcall FUN_0051a870(void)
{
  int iVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  int *piVar4;
  
  uVar2 = (unsigned int)DAT_005366d4;
  uVar3 = (unsigned int)DAT_006550d0;
  EnterCriticalSection(&DAT_007427a0);
  if ((DAT_00742668 != (int *)0x0) && ((unsigned int)*DAT_00742668 == uVar3 - uVar2)) {
    DAT_00742668 = DAT_00742668 + 1;
    uVar2 = 0xffffffff;
    piVar4 = DAT_00742668;
    do {
      if (uVar2 == 0) break;
      uVar2 = uVar2 - 1;
      iVar1 = *piVar4;
      piVar4 = (int *)((int)piVar4 + 1);
    } while ((char)iVar1 != '\0');
    if (DAT_00742724 != (void *)0x0) {
      (*DAT_00742724)(DAT_00742668, 0, 0, 0x7f,
                      *(unsigned int *)((int)DAT_00742668 + ~uVar2),
                      *(unsigned int *)((char *)((int)DAT_00742668 + ~uVar2) + 4));
      uVar2 = 0xffffffff;
      piVar4 = DAT_00742668;
      do {
        if (uVar2 == 0) break;
        uVar2 = uVar2 - 1;
        iVar1 = *piVar4;
        piVar4 = (int *)((int)piVar4 + 1);
      } while ((char)iVar1 != '\0');
      DAT_00742668 = (int *)((int)DAT_00742668 + ~uVar2 + 8);
    }
  }
  LeaveCriticalSection(&DAT_007427a0);
  return;
}
