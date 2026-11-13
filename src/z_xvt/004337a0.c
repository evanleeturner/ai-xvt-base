/* SPDX-License-Identifier: MIT */
/* z_xvt 004337a0 */
void __cdecl FUN_0048ed00(short, short, short, short);
void __cdecl FUN_00433e80(void);
void __cdecl FUN_004341f0(void);
void __cdecl FUN_004343f0(int);
void __cdecl FUN_00435c70(void);

extern short DAT_006d4984;
extern short DAT_006da4fc;
extern short DAT_006db7bc;
extern short DAT_007091ae;
extern int DAT_0071ecf4;
extern char DAT_0071bb23[];

void __stdcall FUN_004337a0(void)
{
  FUN_0048ed00(DAT_006db7bc, DAT_006da4fc, (short)(DAT_007091ae + DAT_006db7bc), (short)(DAT_006d4984 + DAT_006da4fc));
  FUN_00433e80();
  FUN_004341f0();
  int local_val;
  __builtin_memcpy(&local_val, (char *)&DAT_0071bb23 + DAT_0071ecf4 * 0x5bd, 4);
  if (local_val < -0x10000) {
    FUN_004343f0(1);
    FUN_00435c70();
    FUN_004343f0(0);
    return;
  }
  FUN_004343f0(0);
  FUN_00435c70();
  FUN_004343f0(1);
  return;
}
