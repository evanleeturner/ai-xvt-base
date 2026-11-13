/* SPDX-License-Identifier: MIT */
/* z_bop 0041ba60 */
extern int DAT_009a1fe8;
extern int DAT_0051c583;
extern uint __cdecl FUN_004269e0(uint param_1);

int __cdecl FUN_0041ba60(uint param_1)
{
  char *pcVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  unsigned short sVar1;
  unsigned short sVar2;
  int iVar4;
  unsigned short sVar3;
  unsigned short sVar4;
  
  iVar2 = (param_1 & 0xffff) * 0x23;
  iVar3 = DAT_009a1fe8 + iVar2;
  __builtin_memcpy(&pcVar1, (void *)(iVar3 + 0x1f), sizeof(pcVar1));
  if (pcVar1 == (char *)0x0) {
    return 0;
  }
  if (*pcVar1 == '\0') {
    uint arg = ((((uint)iVar3 >> 8) << 8) | *(unsigned char *)(iVar3 + 3)) & 0xffff00ff;
    uVar4 = FUN_004269e0(arg);
    __builtin_memcpy(&sVar1, (void *)((char *)&DAT_0051c583 + (uVar4 & 0xffff) * 0x101), sizeof(sVar1));
    uVar4 = (uint)sVar1;
    __builtin_memcpy(&sVar2, (void *)(pcVar1 + 0x79), sizeof(sVar2));
    uVar5 = (uint)sVar2;
    if (uVar4 < uVar5) {
      return uVar5 << 0xc;
    }
  }
  else {
    if (*pcVar1 != '\x01') {
      return 0;
    }
    __builtin_memcpy(&iVar4, (void *)(pcVar1 + 0xa5), sizeof(iVar4));
    if (iVar4 != 0) {
      __builtin_memcpy(&sVar3, (void *)(iVar4 + 8), sizeof(sVar3));
      return (uint)sVar3 << 0xc;
    }
    __builtin_memcpy(&sVar4, (void *)(pcVar1 + 0x79), sizeof(sVar4));
    uVar4 = (uint)sVar4;
  }
  return uVar4 << 0xc;
}
