/* SPDX-License-Identifier: MIT */
/* z_bop 004037b0 */
void __cdecl FUN_00427bd0(short param_1, short param_2, int param_3);
void __cdecl FUN_00427d30(short param_1, short param_2, int param_3);

extern int DAT_009d8a54;
extern int DAT_009d8a58;
extern int DAT_009d8b60;

void __cdecl FUN_004037b0(int param_1, int param_2, int param_3, int param_4)
{
  __int64 lVar1;
  __int64 lVar2;
  __int64 lVar3;
  int local_base;
  __builtin_memcpy(&local_base, (void *)(param_1 + 0x1f), 4);

  char c_92;
  __builtin_memcpy(&c_92, (void *)(local_base + 0x92), 1);
  if (c_92 != '\0') {
    FUN_00427bd0(*(short *)(param_1 + 0x12), *(short *)(param_1 + 0x10), param_1);
    FUN_00427d30(*(short *)(param_1 + 0x14), 0, param_1);
  }

  short s_99; __builtin_memcpy(&s_99, (void *)(local_base + 0x99), 2);
  short s_9f; __builtin_memcpy(&s_9f, (void *)(local_base + 0x9f), 2);
  short s_93; __builtin_memcpy(&s_93, (void *)(local_base + 0x93), 2);
  lVar1 = (__int64)param_2 * (__int64)(int)s_99;
  lVar2 = (__int64)param_3 * (__int64)(int)s_9f;
  lVar3 = (__int64)param_4 * (__int64)(int)s_93;
  DAT_009d8a58 = ((unsigned int)lVar2 >> 0xf | (int)((unsigned __int64)lVar2 >> 0x20) << 0x11) +
                 ((unsigned int)lVar1 >> 0xf | (int)((unsigned __int64)lVar1 >> 0x20) << 0x11) +
                 ((unsigned int)lVar3 >> 0xf | (int)((unsigned __int64)lVar3 >> 0x20) << 0x11);

  short s_9b; __builtin_memcpy(&s_9b, (void *)(local_base + 0x9b), 2);
  short s_a1; __builtin_memcpy(&s_a1, (void *)(local_base + 0xa1), 2);
  short s_95; __builtin_memcpy(&s_95, (void *)(local_base + 0x95), 2);
  lVar1 = (__int64)param_2 * (__int64)(int)s_9b;
  lVar2 = (__int64)param_3 * (__int64)(int)s_a1;
  lVar3 = (__int64)param_4 * (__int64)(int)s_95;
  DAT_009d8a54 = ((unsigned int)lVar2 >> 0xf | (int)((unsigned __int64)lVar2 >> 0x20) << 0x11) +
                 ((unsigned int)lVar1 >> 0xf | (int)((unsigned __int64)lVar1 >> 0x20) << 0x11) +
                 ((unsigned int)lVar3 >> 0xf | (int)((unsigned __int64)lVar3 >> 0x20) << 0x11);

  short s_9d; __builtin_memcpy(&s_9d, (void *)(local_base + 0x9d), 2);
  short s_a3; __builtin_memcpy(&s_a3, (void *)(local_base + 0xa3), 2);
  short s_97; __builtin_memcpy(&s_97, (void *)(local_base + 0x97), 2);
  lVar1 = (__int64)param_2 * (__int64)(int)s_9d;
  lVar2 = (__int64)param_3 * (__int64)(int)s_a3;
  lVar3 = (__int64)param_4 * (__int64)(int)s_97;
  DAT_009d8b60 = ((unsigned int)lVar2 >> 0xf | (int)((unsigned __int64)lVar2 >> 0x20) << 0x11) +
                 ((unsigned int)lVar1 >> 0xf | (int)((unsigned __int64)lVar1 >> 0x20) << 0x11) +
                 ((unsigned int)lVar3 >> 0xf | (int)((unsigned __int64)lVar3 >> 0x20) << 0x11);
  return;
}
