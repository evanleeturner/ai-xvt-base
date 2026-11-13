/* SPDX-License-Identifier: MIT */
/* z_bop 00467c50 */
extern uint DAT_009a73b6;
extern uint DAT_00a08104;
extern uint DAT_009a1fe0;
extern uint DAT_009a1fe8;
extern ushort DAT_009a73b0;
extern char DAT_009d1320[];
extern uint DAT_009a73bc;
extern char s_enterhangarpln_0051bfd4[];
extern ushort DAT_009a73c0;
extern ushort DAT_00a07cec;

void __stdcall FUN_00403400(void);

uint __stdcall FUN_00467c50(void)
{
  byte bVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  ushort uVar7;
  char *pcVar8;
  char *pcVar9;
  int bVar10;
  int local_1;
  int tmp_ptr;
  int tmp_val;

  uVar2 = DAT_009a73b6;
  bVar1 = *(byte *)(DAT_00a08104 + 5);
  if (bVar1 == 0xff) {
    return DAT_00a08104 & 0xffff0000;
  }
  if ((int)DAT_009a1fe0 <= (int)(uint)bVar1) {
    return DAT_009a1fe0 & 0xffff0000;
  }
  iVar5 = (uint)bVar1 * 0x23 + DAT_009a1fe8;
  
  __builtin_memcpy(&tmp_ptr, (void *)(iVar5 + 0x1f), 4);
  __builtin_memcpy(&iVar4, (void *)(tmp_ptr + 0xa9), 4);
  
  local_1 = *(char *)((uint)DAT_009a73b0 * 0x23 + 0x16 + DAT_009a1fe8) != *(char *)(iVar5 + 0x16) ||
            *(char *)(iVar5 + 3) == '\0';
  if ((*(char *)(iVar4 + 7) == '\x03') || (*(char *)(iVar4 + 7) == '\x04')) {
    local_1 = 1;
  }
  if (*(char *)(iVar4 + 0x73) != '\0') {
    local_1 = 1;
  }
  
  __builtin_memcpy(&tmp_val, (void *)(iVar5 + 0x1b), 4);
  if (tmp_val != -1) {
    local_1 = 1;
  }
  
  if (local_1 == 0) {
    iVar4 = 0xf;
    bVar10 = 1;
    pcVar8 = (char *)((int)&DAT_009d1320 + (uint)*(byte *)(DAT_009a73bc + 0x34) * 0x55);
    pcVar9 = s_enterhangarpln_0051bfd4;
    do {
      if (iVar4 == 0) break;
      iVar4 = iVar4 + -1;
      bVar10 = *pcVar8 == *pcVar9;
      pcVar8 = pcVar8 + 1;
      pcVar9 = pcVar9 + 1;
    } while (bVar10);
    uVar3 = (uint)*(byte *)(DAT_009a73bc + 0x34);
    if (bVar10) {
      *(uint *)(DAT_009a73b6 + 0xe) = 0x3b;
      uVar3 = uVar2;
    }
  }
  else {
    uVar3 = (uint)DAT_00a07cec;
    uVar7 = DAT_00a07cec;
    if ((int)(uint)DAT_00a07cec < (int)DAT_009a1fe0) {
      do {
        iVar6 = (uint)uVar7 * 0x23 + DAT_009a1fe8;
        
        __builtin_memcpy(&tmp_ptr, (void *)(iVar6 + 0x1f), 4);
        __builtin_memcpy(&iVar5, (void *)(tmp_ptr + 0xa9), 4);
        
        if ((*(char *)(iVar6 + 3) != '\0') && (*(byte *)(iVar6 + 0x16) == DAT_009a73c0)) {
          if (DAT_009a73b0 == uVar7) {
            *(char *)(iVar5 + 5) = 0xff;
            *(char *)(iVar5 + 0xaf) = *(char *)(iVar4 + 0xaf);
            *(char *)(iVar5 + 0x36) = *(char *)(iVar4 + 0x36);
            *(ushort *)(iVar5 + 0x42) = *(ushort *)(iVar4 + 0x42);
            *(ushort *)(iVar5 + 0x44) = *(ushort *)(iVar4 + 0x44);
            *(char *)(iVar5 + 0x48) = *(char *)(iVar4 + 0x48);
            FUN_00403400();
          }
          else {
            *(char *)(iVar5 + 5) = (char)DAT_009a73b0;
          }
        }
        uVar7 = uVar7 + 1;
        uVar3 = (uint)uVar7;
      } while ((int)uVar3 < (int)DAT_009a1fe0);
    }
  }
  return (uVar3 & 0xffff0000) | (local_1 & 0xff);
}
