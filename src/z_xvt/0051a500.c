/* SPDX-License-Identifier: MIT */
/* z_xvt 0051a500 */
extern void * __cdecl _fopen(const char *filename, const char *mode);
extern unsigned long __cdecl __beginthreadex(void *security, unsigned stack_size, unsigned (__stdcall *start_address)(void *), void *arglist, unsigned create_flags, unsigned *thread_id);
extern int __stdcall GetThreadPriority(HANDLE hThread);
extern void __stdcall InitializeCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
extern int __cdecl FUN_0051a1e0(uint param_1, char *param_2, uint param_3);
extern void __cdecl FUN_0051c100(char param_1, char param_2);
extern void __cdecl FUN_0051cdc0(char param_1, const char *param_2, int param_3);
extern uint __cdecl FUN_0051c160(uint param_1, uint param_2);
extern uint __cdecl FUN_0048f3c0(uint param_1, uint param_2);
extern int __cdecl FUN_0051aff0(uint *param_1);
extern void __cdecl FUN_0051c600(uint *param_1);
extern void __cdecl FUN_0051c6f0(uint param_1);
extern unsigned long __stdcall timeGetTime(void);
extern unsigned long __stdcall ResumeThread(HANDLE hThread);
extern int __stdcall FUN_0051a700(void);

extern void *LAB_0051a910;

extern int DAT_0074278c;
extern void *DAT_00742780;
extern const char DAT_0052a330[];
extern HANDLE DAT_00742784;
extern CRITICAL_SECTION DAT_007427a0;
extern char DAT_00742728;
extern const char s__amovie_a_wrk_00531a40[];
extern uint DAT_006550d4;
extern uint DAT_007426c4;
extern uint DAT_00742790;
extern uint DAT_00742788;
extern uint DAT_00742724;
extern uint _DAT_00742720;
extern uint DAT_00742814;
extern uint DAT_007426d0;
extern int DAT_00742660;
extern unsigned long _DAT_005366d8;
extern int DAT_00742730;
extern int DAT_00742668;
extern uint *DAT_00757c30;
extern short DAT_005366f0;

uint __cdecl FUN_0051a500(uint *param_1)
{
  int iVar1;
  uint uVar2;
  unsigned short uVar3;
  int iVar4;
  unsigned char local_8[4];
  uint local_4;

  iVar4 = 0;
  DAT_0074278c = 0;
  DAT_00742780 = _fopen((char *)*param_1, DAT_0052a330);
  iVar1 = 0;
  if (DAT_00742780 != (void *)0x0) {
    DAT_00742784 = (HANDLE)__beginthreadex((void *)0x0, 0, (unsigned (__stdcall *)(void *))&LAB_0051a910, local_8, 4, &local_4);
    GetThreadPriority(DAT_00742784);
    InitializeCriticalSection((LPCRITICAL_SECTION)&DAT_007427a0);
    iVar1 = FUN_0051a1e0(0x531a40, (char *)0x0, param_1[3]);
    if (iVar1 != 0) {
      FUN_0051c100((char)param_1[3], (char)DAT_00742728);
      FUN_0051cdc0((char)DAT_00742728, s__amovie_a_wrk_00531a40, (int)param_1);
      uVar2 = FUN_0051c160(param_1[1], param_1[2]);
      if ((short)uVar2 == 0) {
        return (uint)((uint)(short)(uVar2 >> 0x10) << 16 | 0xffff);
      }
      DAT_006550d4 = FUN_0048f3c0(0x20000, 2);
      if (DAT_006550d4 != 0) {
        DAT_007426c4 = FUN_0048f3c0(0x300, 2);
        if (DAT_007426c4 != 0) {
          DAT_00742790 = param_1[7];
          DAT_00742788 = param_1[8];
          DAT_00742724 = param_1[9];
          _DAT_00742720 = param_1[10];
          DAT_00742814 = param_1[0xb];
          iVar1 = FUN_0051aff0(param_1);
          DAT_0074278c = iVar1;
          if (0 < iVar1) {
            do {
              FUN_0051c600((uint *)(&DAT_007426d0)[iVar4]);
              if (iVar4 == 0) {
                FUN_0051c6f0(DAT_007426d0);
              }
              iVar4 = iVar4 + 1;
            } while (iVar4 < DAT_0074278c);
            DAT_00742660 = 0;
            _DAT_005366d8 = timeGetTime();
            ResumeThread(DAT_00742784);
            iVar1 = (&DAT_00742730)[DAT_00742660];
            DAT_00742668 = iVar1;
          }
          goto LAB_0051a6c0;
        }
      }
      FUN_0051a700();
      return 0xffffffff;
    }
    iVar1 = FUN_0051a700();
  }
LAB_0051a6c0:
  uVar3 = (unsigned short)((uint)iVar1 >> 0x10);
  if (DAT_0074278c < 0) {
    return (uint)((uint)uVar3 << 16 | (uint)(unsigned short)DAT_0074278c);
  }
  if (DAT_0074278c == 0) {
    param_1[0xd] = 1;
    return (uint)((uint)uVar3 << 16 | (uint)(unsigned short)DAT_0074278c);
  }
  DAT_00757c30 = param_1;
  return (uint)((uint)uVar3 << 16 | (uint)(unsigned short)DAT_005366f0);
}
