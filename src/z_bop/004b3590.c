/* SPDX-License-Identifier: MIT */
/* z_bop 004b3590 */
extern unsigned int DAT_00a90b1c;
extern unsigned int _DAT_00a90b10;
extern unsigned int _DAT_00a90b14;
extern unsigned int _DAT_00a90b18;
extern unsigned int DAT_00528c5c;
extern unsigned int _DAT_00a90b70;
extern unsigned int _DAT_00a90b90;
extern unsigned int _DAT_00a90bb0;
extern unsigned int _DAT_00a90bd0;
extern unsigned int DAT_00a90b60;
extern unsigned int DAT_00a90b30;
extern unsigned int DAT_00528c94;
extern unsigned int DAT_00528c6c;
extern unsigned int _DAT_00a90b40;
extern unsigned int _DAT_00a90b54;
extern unsigned int DAT_00a90a70;
extern unsigned int DAT_00a90a74;
extern unsigned int _DAT_00a90ae8;

long long __cdecl __ftol(void);
void __stdcall FUN_004b1c10(void);
void __stdcall FUN_004b1c90(void);
void __cdecl FUN_004b1d10(void *param_1, unsigned int param_2);
void __stdcall FUN_004b1d70(void);
void __cdecl FUN_004b1de0(void *param_1, unsigned int param_2);
void __stdcall FUN_004b2020(void);
void __stdcall FUN_004b1c50(void);
void __cdecl FUN_004b1a90(void *param_1, unsigned int param_2);
void __cdecl FUN_004b2680(void *param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4);
void __cdecl FUN_004b3160(unsigned int param_1);

void __stdcall FUN_004b3590(void)
{
  unsigned char bVar1;
  long long lVar2;
  long long lVar3;
  long long lVar4;
  long long lVar5;

  if ((DAT_00a90b1c != 0) &&
     ((((_DAT_00a90b10 & 0x7fffffff) != 0 || ((_DAT_00a90b14 & 0x7fffffff) != 0)) ||
      ((_DAT_00a90b18 & 0x7fffffff) != 0)))) {
    lVar2 = __ftol();
    lVar3 = __ftol();
    lVar4 = __ftol();
    lVar5 = __ftol();
    bVar1 = (unsigned char)lVar5;
    if (bVar1 != 0) {
      if (*(int *)(DAT_00528c5c + 0x18) != 0) {
        _DAT_00a90b70 =
             ((unsigned int)bVar1 << 0x10 | (unsigned int)(unsigned char)lVar3) << 8 | (unsigned int)(unsigned char)lVar2 << 0x10 |
             (unsigned int)(unsigned char)lVar4;
        _DAT_00a90b90 = _DAT_00a90b70;
        _DAT_00a90bb0 = _DAT_00a90b70;
        _DAT_00a90bd0 = _DAT_00a90b70;
        FUN_004b1c10();
        FUN_004b1c90();
        FUN_004b1d10(&DAT_00a90b60, 4);
        FUN_004b1d70();
        FUN_004b1de0(&DAT_00a90b30, 2);
        FUN_004b2020();
        FUN_004b1c50();
        return;
      }
      FUN_004b1a90((void *)DAT_00528c94,
                   (unsigned int)(unsigned short)((unsigned short)(unsigned char)((unsigned char)lVar3 >>
                                                (*(unsigned char *)(DAT_00528c6c + 0x24) & 0x1f)) <<
                                  (*(unsigned char *)(DAT_00528c6c + 0x18) & 0x1f) |
                                  (unsigned short)(unsigned char)((unsigned char)lVar2 >>
                                                (*(unsigned char *)(DAT_00528c6c + 0x20) & 0x1f)) <<
                                  (*(unsigned char *)(DAT_00528c6c + 0x14) & 0x1f) |
                                  (unsigned short)(bVar1 >> (*(unsigned char *)(DAT_00528c6c + 0x34) & 0x1f)) <<
                                  (*(unsigned char *)(DAT_00528c6c + 0x30) & 0x1f) |
                                 (unsigned short)(unsigned char)((unsigned char)lVar4 >>
                                               (*(unsigned char *)(DAT_00528c6c + 0x28) & 0x1f)) <<
                                 (*(unsigned char *)(DAT_00528c6c + 0x1c) & 0x1f)));
      FUN_004b1c10();
      FUN_004b1c90();
      FUN_004b2680((void *)DAT_00528c94, 0xa90a70, 0, 1);
      _DAT_00a90b70 = 0xffffffff;
      _DAT_00a90b90 = 0xffffffff;
      _DAT_00a90bb0 = 0xffffffff;
      _DAT_00a90bd0 = 0xffffffff;
      _DAT_00a90b40 = (unsigned int)&DAT_00a90a70;
      _DAT_00a90b54 = (unsigned int)&DAT_00a90a70;
      FUN_004b1d10(&DAT_00a90b60, 4);
      FUN_004b1d70();
      FUN_004b1de0(&DAT_00a90b30, 2);
      FUN_004b2020();
      FUN_004b1c50();
      ((void (__stdcall *)(void *))(*(void **)DAT_00a90a74 + 8))((void *)DAT_00a90a74);
      ((void (__stdcall *)(void *))(*(void **)DAT_00a90a70 + 8))((void *)DAT_00a90a70);
      _DAT_00a90ae8 = 0;
      FUN_004b3160(0xa90a70);
    }
  }
  return;
}
