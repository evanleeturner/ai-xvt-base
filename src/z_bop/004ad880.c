/* SPDX-License-Identifier: MIT */
/* z_bop 004ad880 */
extern unsigned int FUN_00425b70(unsigned int param_1, unsigned short param_2);
extern void FUN_004ad9e0(int param_1, unsigned int param_2);
extern void FUN_004ada90(int param_1, unsigned int param_2);
extern unsigned int DAT_009a1fe8;
extern unsigned short DAT_009a7394;
extern unsigned char DAT_0051c585;
extern unsigned char DAT_0051c587;

void __cdecl FUN_004ad880(int param_1, int param_2, int param_3, unsigned short param_4)
{
  unsigned int uVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  unsigned short tmp_us;
  short tmp_s;

  __builtin_memcpy(&tmp_us, (void *)(*(int *)(DAT_009a1fe8 + 0x1f + param_1 * 0x23) + 0x79), 2);
  uVar3 = (unsigned int)param_2 - (unsigned int)tmp_us;
  if (uVar3 != 0) {
    if (uVar3 < 0x8000) {
      __builtin_memcpy(&tmp_us, &DAT_0051c585 + (unsigned int)DAT_009a7394 * 0x101, 2);
      uVar1 = FUN_00425b70(((unsigned int)(DAT_009a7394 >> 8) << 16) | tmp_us, 0x4000);
      uVar1 = uVar1 & 0xffff;
      if (uVar1 == 0) {
        uVar1 = 1;
      }
      __builtin_memcpy(&tmp_s, &DAT_0051c585 + (unsigned int)DAT_009a7394 * 0x101, 2);
      uVar2 = FUN_00425b70((unsigned int)(unsigned short)(tmp_s - (short)uVar1), param_4);
      uVar1 = uVar1 + (uVar2 & 0xffff);
      __builtin_memcpy(&tmp_s, (void *)(*(int *)(*(int *)(DAT_009a1fe8 + 0x1f + param_1 * 0x23) + 0xa9) + 0xbf), 2);
      if (tmp_s == 0) {
        uVar1 = uVar1 * 3;
      }
      if (uVar1 <= uVar3) {
        uVar3 = uVar1;
      }
      FUN_004ad9e0(param_1, uVar3);
      return;
    }
    if (param_3 == 1) {
      __builtin_memcpy(&tmp_us, &DAT_0051c587 + (unsigned int)DAT_009a7394 * 0x101, 2);
      uVar1 = FUN_00425b70(tmp_us, 0x4000);
      uVar1 = uVar1 & 0xffff;
      if (uVar1 == 0) {
        uVar1 = 1;
      }
      __builtin_memcpy(&tmp_s, &DAT_0051c587 + (unsigned int)DAT_009a7394 * 0x101, 2);
      uVar2 = FUN_00425b70(((unsigned int)(DAT_009a7394 >> 8) << 16) | (unsigned short)(tmp_s - (short)uVar1), (unsigned short)(-param_4 - 1));
      uVar1 = uVar1 + (uVar2 & 0xffff);
      uVar2 = -uVar3;
      if (uVar1 <= -uVar3) {
        uVar2 = uVar1;
      }
      FUN_004ada90(param_1, uVar2);
    }
  }
  return;
}
