/* SPDX-License-Identifier: MIT */
/* z_xvt 004d5830 */
typedef unsigned int undefined4;

extern undefined4 DAT_00758830;
extern char DAT_0075964f[];
extern char DAT_00759657[];
extern char DAT_0075965b[];
extern const char DAT_00523570[];
extern const char s_frontscreen_d_bmp_005329fc[];

int __cdecl _sprintf(char *dst, const char *fmt, ...);
void * __cdecl _fopen(const char *filename, const char *mode);
int __cdecl _fclose(void *stream);
undefined4 __stdcall FUN_004d4d00(void);
undefined4 FUN_004d3610(char *param_1, undefined4 param_2, int param_3, int param_4, undefined4 param_5, undefined4 param_6, unsigned int param_7, int param_8);
void __stdcall FUN_004d4d90(void);

undefined4 __stdcall FUN_004d5830(void)
{
  void *_File;
  undefined4 uVar1;
  int iVar2;
  char local_40[64];
  undefined4 local_75964f;
  undefined4 local_759657;
  char local_75965b;
  
  iVar2 = 0;
  while (1) {
    _sprintf(local_40, s_frontscreen_d_bmp_005329fc, iVar2);
    _File = _fopen(local_40, DAT_00523570);
    if (_File == 0) {
      break;
    }
    iVar2 = iVar2 + 1;
    _fclose(_File);
  }
  DAT_00758830 = FUN_004d4d00();
  __builtin_memcpy(&local_75964f, DAT_0075964f, 4);
  __builtin_memcpy(&local_759657, DAT_00759657, 4);
  __builtin_memcpy(&local_75965b, DAT_0075965b, 1);
  uVar1 = FUN_004d3610(local_40, DAT_00758830, 0x280, 0x1e0, local_75964f, local_759657, (unsigned int)local_75965b, 0x7596d3);
  FUN_004d4d90();
  return uVar1;
}
