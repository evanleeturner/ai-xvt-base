/* SPDX-License-Identifier: MIT */
/* z_bop 00483030 */
extern int DAT_009a1fe8;
extern char DAT_00521f1c[];
extern char DAT_0051c659[];
extern char DAT_0051c65b[];
extern char DAT_0051c65d[];
extern char DAT_0051c65f[];

int __cdecl FUN_00483030(int param_1)
{
  int iVar4;
  int local_1f;
  int local_a9;
  int local_521f1c;
  unsigned char uVar1;
  unsigned short uVar3;
  short local_51c65b;
  short local_51c65d;
  short local_51c65f;
  unsigned char local_51c659;

  iVar4 = param_1 * 0x23 + DAT_009a1fe8;
  __builtin_memcpy(&local_1f, (char *)iVar4 + 0x1f, 4);
  if (local_1f == 0) {
    __builtin_memcpy(&local_521f1c, (char *)&DAT_00521f1c + (unsigned int)*(unsigned char *)(iVar4 + 3) * 0x1a, 4);
    return local_521f1c;
  }
  __builtin_memcpy(&local_a9, (char *)local_1f + 0xa9, 4);
  if (local_a9 != 0) {
    uVar1 = *(unsigned char *)(local_a9 + 4);
    uVar3 = uVar1 | (uVar1 << 8);
    __builtin_memcpy(&local_51c65b, (char *)&DAT_0051c65b + uVar3, 2);
    __builtin_memcpy(&local_51c65d, (char *)&DAT_0051c65d + uVar3, 2);
    __builtin_memcpy(&local_51c65f, (char *)&DAT_0051c65f + uVar3, 2);
    __builtin_memcpy(&local_51c659, (char *)&DAT_0051c659 + uVar3, 1);
    return ((int)local_51c65b + (int)local_51c65d + (int)local_51c65f) / 3 << (local_51c659 & 0x1f);
  }
  __builtin_memcpy(&local_521f1c, (char *)&DAT_00521f1c + (unsigned int)*(unsigned char *)(iVar4 + 3) * 0x1a, 4);
  return local_521f1c;
}
