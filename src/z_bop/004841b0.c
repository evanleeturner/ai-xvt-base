/* SPDX-License-Identifier: MIT */
/* z_bop 004841b0 */
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_00a08104;
extern byte DAT_009e968a;
extern byte DAT_009e967e;
extern byte DAT_009e967c;
extern byte DAT_009d8c8b;
extern byte DAT_009d8c88;
extern byte DAT_009d9156;
extern byte DAT_009d8c94;

uint __cdecl FUN_004841b0(uint param_1)
{
  byte bVar1;
  int iVar2;
  ushort uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  ushort tmp_us;
  int tmp_i;
  uint tmp_u;
  short tmp_s;
  
  iVar2 = param_1 * 0x5bd;
  __builtin_memcpy(&uVar3, (void*)((int)&DAT_009e968a + iVar2), 2);
  if (uVar3 == 0xffff) {
    return param_1 & 0xffff0000;
  }
  if (DAT_009a1fe0 <= (int)uVar3) {
    return 0;
  }
  iVar4 = (uint)uVar3 * 0x23 + DAT_009a1fe8;
  __builtin_memcpy(&tmp_i, (void*)(iVar4 + 0x1b), 4);
  if (tmp_i != -1) {
    return DAT_009a1fe8 & 0xffff0000;
  }
  __builtin_memcpy(&tmp_u, (void*)(iVar4 + 0x1f), 4);
  __builtin_memcpy(&tmp_i, (void*)(tmp_u + 0xa9), 4);
  DAT_00a08104 = tmp_i;
  if (*(char *)(DAT_00a08104 + 7) != '\0') {
    return tmp_u & 0xffff0000;
  }
  uVar3 = (ushort)((uint)DAT_00a08104 >> 0x10);
  __builtin_memcpy(&tmp_s, (void*)(DAT_00a08104 + 0x1e5), 2);
  if (tmp_s == 0) {
    return (uint)uVar3 << 0x10;
  }
  uVar6 = (uint)*(byte *)(iVar4 + 0x16);
  __builtin_memcpy(&tmp_us, (void*)((int)&DAT_009e967e + iVar2), 2);
  uVar5 = (uint)tmp_us;
  if (uVar6 == uVar5) {
    return (uVar3 << 0x10) | 1;
  }
  bVar1 = (&DAT_009d8c8b)[uVar6 * 0x566];
  uVar3 = *(byte *)(iVar4 + 0x16) / 0x5f;
  if (bVar1 == 0) {
    return (uint)uVar3 << 0x10;
  }
  __builtin_memcpy(&tmp_us, (void*)((int)&DAT_009e967c + iVar2), 2);
  if ((uint)(byte)(&DAT_009d8c88)[uVar6 * 0x566] == (uint)tmp_us) {
    return (uVar3 << 0x10) | 1;
  }
  if (((&DAT_009d9156)[uVar6 * 0x566] != '\0') &&
     ((&DAT_009d9156)[uVar5 * 0x566] == (&DAT_009d9156)[uVar6 * 0x566])) {
    return (uVar3 << 0x10) | 1;
  }
  if ((uint)bVar1 - (uint)(byte)(&DAT_009d8c94)[uVar5 * 0x566] == 8) {
    return 1;
  }
  return (uint)((uint)bVar1 - (uint)tmp_us == 1);
}
