/* SPDX-License-Identifier: MIT */
/* z_bop 00502a50 */
extern unsigned int DAT_00b6a2c0;
extern unsigned char DAT_00b6a5a6;
extern unsigned char DAT_00a91d18[];
extern unsigned char DAT_00b6d5b6[];
extern unsigned char DAT_00b6d876;
extern char DAT_00b69d20[];
extern char DAT_0052b460[];
extern char s__d__d__0052b450[];

void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(int param_1);
void __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
unsigned int __cdecl FUN_004db4e0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
int __cdecl _sprintf(char *param_1, const char *param_2, ...);

unsigned int __stdcall FUN_00502a50(void)
{
  char *pcVar1;
  int iVar3;
  int iVar4;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10[4];
  unsigned char *p;
  int f_m4;
  int f_m3;
  int f_m1;
  int f0;
  int f1;
  int f2;
  int f3;
  int f4;

  FUN_004d6460(local_10, 0x54, 0x5a, 0x1b2, 0x6a);
  pcVar1 = FUN_004ddc10(0x139);
  FUN_004db640(0xf, pcVar1, local_10, 0xffff);
  pcVar1 = FUN_004ddc10(0xba);
  FUN_004db4e0(0xc, pcVar1, 0x58, 0x72, DAT_00b6a2c0);
  pcVar1 = FUN_004ddc10(0x14a);
  FUN_004db4e0(0xc, pcVar1, 0xf8, 0x72, DAT_00b6a2c0);
  pcVar1 = FUN_004ddc10(0x150);
  FUN_004db4e0(0xc, pcVar1, 0x134, 0x72, DAT_00b6a2c0);
  iVar3 = 0;
  pcVar1 = FUN_004ddc10(0x151);
  FUN_004db4e0(0xc, pcVar1, 0x170, 0x72, DAT_00b6a2c0);
  local_20 = 0;
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  iVar4 = 0x86;
  p = (unsigned char *)&DAT_00b6d5b6;
  do {
    __builtin_memcpy(&f_m3, p - 12, 4);
    if (f_m3 != 0) {
      __builtin_memcpy(&f_m4, p - 16, 4);
      if (DAT_00a91d18[(int)((unsigned int)f_m4 * 0x562u)] == DAT_00b6a5a6) {
        __builtin_memcpy(&f0, p, 4);
        __builtin_memcpy(&f1, p + 4, 4);
        __builtin_memcpy(&f2, p + 8, 4);
        __builtin_memcpy(&f3, p + 12, 4);
        __builtin_memcpy(&f4, p + 16, 4);
        iVar3 = (int)((unsigned int)iVar3 + (unsigned int)f0);
        local_20 = (int)((unsigned int)local_20 + (unsigned int)f1);
        local_1c = (int)((unsigned int)local_1c + (unsigned int)f3);
        local_18 = (int)((unsigned int)local_18 + (unsigned int)f2);
        local_14 = (int)((unsigned int)local_14 + (unsigned int)f4);
        FUN_004db4e0(0xc, (char *)(p - 0x1e), 0x58, iVar4, 0xffff);
        __builtin_memcpy(&f_m1, p - 4, 4);
        _sprintf(DAT_00b69d20, DAT_0052b460, f_m1);
        FUN_004db4e0(0xc, DAT_00b69d20, 0xf8, iVar4, 0xffff);
        _sprintf(DAT_00b69d20, s__d__d__0052b450, f0, f1);
        FUN_004db4e0(0xc, DAT_00b69d20, 0x134, iVar4, 0xffff);
        _sprintf(DAT_00b69d20, DAT_0052b460, f2);
        FUN_004db4e0(0xc, DAT_00b69d20, 0x170, iVar4, 0xffff);
      }
      iVar4 = (int)((unsigned int)iVar4 + 0x14u);
    }
    p = p + 0x58;
  } while (p < (unsigned char *)&DAT_00b6d876);
  pcVar1 = FUN_004ddc10(0x129);
  FUN_004db4e0(0xc, pcVar1, 0x58, 0x132, DAT_00b6a2c0);
  _sprintf(DAT_00b69d20, DAT_0052b460, iVar3);
  FUN_004db4e0(0xc, DAT_00b69d20, 0x120, 0x132, 0xffff);
  pcVar1 = FUN_004ddc10(0x14e);
  FUN_004db4e0(0xc, pcVar1, 0x58, 0x146, DAT_00b6a2c0);
  _sprintf(DAT_00b69d20, DAT_0052b460, local_20);
  FUN_004db4e0(0xc, DAT_00b69d20, 0x120, 0x146, 0xffff);
  pcVar1 = FUN_004ddc10(299);
  FUN_004db4e0(0xc, pcVar1, 0x58, 0x15a, DAT_00b6a2c0);
  _sprintf(DAT_00b69d20, DAT_0052b460, local_1c);
  FUN_004db4e0(0xc, DAT_00b69d20, 0x120, 0x15a, 0xffff);
  pcVar1 = FUN_004ddc10(300);
  FUN_004db4e0(0xc, pcVar1, 0x58, 0x16e, DAT_00b6a2c0);
  _sprintf(DAT_00b69d20, DAT_0052b460, local_18);
  FUN_004db4e0(0xc, DAT_00b69d20, 0x120, 0x16e, 0xffff);
  pcVar1 = FUN_004ddc10(0x12f);
  FUN_004db4e0(0xc, pcVar1, 0x58, 0x182, DAT_00b6a2c0);
  _sprintf(DAT_00b69d20, DAT_0052b460, local_14);
  FUN_004db4e0(0xc, DAT_00b69d20, 0x120, 0x182, 0xffff);
  return 1;
}
