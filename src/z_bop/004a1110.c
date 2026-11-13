/* SPDX-License-Identifier: MIT */
/* z_bop 004a1110 */
extern int DAT_00a08104;
extern int DAT_009a1fe8;
extern int DAT_009a73b6;

unsigned short FUN_00425cb0(void);
unsigned int FUN_00404620(int param_1);
void FUN_004a4cb0(unsigned short param_1);

void __cdecl FUN_004a1110(int param_1)
{
  unsigned short uVar1;
  unsigned short uVar2;
  short sVar3;
  unsigned int uVar4;
  int local_b9;
  unsigned int local_store;
  short local_read;

  uVar1 = FUN_00425cb0();
  uVar1 = (uVar1 & 0x1f) + 0x32;
  uVar2 = FUN_00425cb0();
  sVar3 = (uVar2 & 0xff) + 0x180;
  
  __builtin_memcpy(&local_b9, (void *)(DAT_00a08104 + 0xb9), sizeof(int));
  if (local_b9 >= 0) {
    uVar1 = (unsigned short)(0 - uVar1);
    sVar3 = (short)(0 - sVar3);
  }
  
  local_store = (unsigned int)uVar1;
  __builtin_memcpy((void *)(DAT_00a08104 + 0xb9), &local_store, sizeof(unsigned int));
  
  __builtin_memcpy(&local_read, (void *)(param_1 * 0x23 + 0x10 + DAT_009a1fe8), sizeof(short));
  *(short *)(DAT_009a73b6 + 0x32) = (short)(local_read + sVar3);
  
  uVar4 = FUN_00404620(DAT_00a08104);
  FUN_004a4cb0(((unsigned short)(uVar4 >> 1) & 0x7fff) + 0x8000);
  
  *(unsigned short *)(DAT_009a73b6 + 0x3a) = 0x76;
  return;
}
