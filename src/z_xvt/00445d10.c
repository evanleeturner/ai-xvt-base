/* SPDX-License-Identifier: MIT */
/* z_xvt 00445d10 */
int __cdecl FUN_0042b150(int param_1);
void __cdecl FUN_00483410(short param_1, short param_2, int param_3);

extern int DAT_00732e54;
extern int DAT_006d4988;
extern int DAT_0070a1d2;

void __cdecl FUN_00445d10(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int local_int_27;
  int local_int_1f;
  unsigned short local_short_d;
  
  iVar1 = FUN_0042b150(DAT_00732e54);
  iVar3 = param_1 * 0x23 + DAT_006d4988;
  __builtin_memcpy(&local_int_27, (void *)(iVar1 + 0x27), 4);
  iVar4 = *(int *)(iVar3 + 0xc) - local_int_27;
  if ((iVar4 < 0) || (iVar4 < 0x101)) {
    local_short_d = 0x4000;
    __builtin_memcpy((void *)(DAT_00732e54 + 0xd), &local_short_d, 2);
    *(unsigned char *)(DAT_00732e54 + 0x9c) = 0;
    *(unsigned char *)(DAT_00732e54 + 0x97) = 2;
  }
  else {
    __builtin_memcpy(&local_int_1f, (void *)(iVar3 + 0x1f), 4);
    if (*(char *)(local_int_1f + 0x89) != '\0') {
      FUN_00483410(*(short *)(iVar3 + 0x12), *(short *)(iVar3 + 0x10), iVar3);
    }
    iVar2 = (int)*(short *)(local_int_1f + 0x8e);
    if (*(char *)(iVar3 + 2) == '\0') {
      iVar3 = iVar2 * (unsigned int)DAT_0070a1d2 * 3;
    }
    else {
      iVar3 = iVar2 * (unsigned int)DAT_0070a1d2 * 2;
    }
    __builtin_memcpy(&local_short_d, (void *)(DAT_00732e54 + 0xd), 2);
    if ((iVar4 <= -iVar3) && (0x4000 < local_short_d)) {
      local_short_d = (unsigned short)((short)((int)(local_short_d - 0x4000) >> 1) + 0x4000);
      __builtin_memcpy((void *)(iVar1 + 0x2e), &local_short_d, 2);
      *(unsigned char *)(DAT_00732e54 + 0x9c) = 1;
      return;
    }
  }
  return;
}
