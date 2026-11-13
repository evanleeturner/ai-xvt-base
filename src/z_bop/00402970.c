/* SPDX-License-Identifier: MIT */
/* z_bop 00402970 */
extern int DAT_00a08104;
extern int DAT_00a07cf0;
extern int DAT_009d9132;
extern int DAT_009a73b0;
extern int DAT_009a1fe8;
extern int DAT_009d9134;
extern int DAT_009d911a;

void __stdcall FUN_00403400(void);
void __stdcall FUN_0049f360(void);

void __cdecl FUN_00402970(unsigned int param_1)
{
  unsigned char cVar1;
  char *pcVar2;
  unsigned int iVar3;

  iVar3 = DAT_00a08104;
  pcVar2 = (char *)(&DAT_00a07cf0)[*(unsigned char *)(iVar3 + 0x34)];
  cVar1 = *pcVar2;
  if (cVar1 == 0xFF) goto LAB_00402b12;
  if (cVar1 == 0xFD) {
    if (*(unsigned short *)(&DAT_009d9132 + (unsigned int)*(unsigned char *)((unsigned int)DAT_009a73b0 * 0x23 + 0x16 + DAT_009a1fe8) * 0x566) == 0) {
LAB_00402af8:
      *(unsigned short *)(iVar3 + 0x42) = 0x8000;
    }
    else {
      *(unsigned short *)(iVar3 + 0x42) = 0x800C;
    }
  }
  else if (cVar1 == 0xFE) {
    if (*(char *)(iVar3 + 0x1EB) == '\0' ||
        *(unsigned short *)(&DAT_009d9132 + (unsigned int)*(unsigned char *)((unsigned int)DAT_009a73b0 * 0x23 + 0x16 + DAT_009a1fe8) * 0x566) == 0) {
      if (*(unsigned short *)(&DAT_009d9134 + (unsigned int)*(unsigned char *)((unsigned int)DAT_009a73b0 * 0x23 + 0x16 + DAT_009a1fe8) * 0x566) == 0)
        goto LAB_00402af8;
      *(unsigned short *)(iVar3 + 0x42) = 0x800D;
    }
    else {
      *(unsigned short *)(iVar3 + 0x42) = 0x800C;
    }
  }
  else if (cVar1 == 0xF9) {
    *(unsigned short *)(iVar3 + 0x42) = 0x800C;
  }
  else {
    if (*(unsigned short *)(&DAT_009d911a + (unsigned int)*(unsigned char *)((unsigned int)DAT_009a73b0 * 0x23 + 0x16 + DAT_009a1fe8) * 0x566 + (unsigned int)*(unsigned char *)(iVar3 + 0x36) * 2) == 0)
      goto LAB_00402af8;
    *(unsigned short *)(iVar3 + 0x42) = *(unsigned char *)(iVar3 + 0x36) + 0x8000;
  }
  *(unsigned short *)(iVar3 + 0x44) = 0;
  *(unsigned char *)(iVar3 + 0x48) = 0;
  if (*(unsigned short *)(iVar3 + 0x42) != 0xFFFF) {
    FUN_00403400();
  }
LAB_00402b12:
  *(unsigned short *)(iVar3 + 0x64) = 0;
  cVar1 = pcVar2[1];
  if (cVar1 != 0xFF) {
    *(char *)(iVar3 + 0x5E) = (char)cVar1;
    FUN_0049f360();
  }
  *(unsigned short *)(iVar3 + 0x6A) = 0xFFFF;
  *(unsigned short *)(iVar3 + 0x6C) = 0;
  *(unsigned short *)(iVar3 + 0x6E) = 0xFFFF;
  *(unsigned int *)(iVar3 + 0x3C) = ((param_1 & 7) * *(int *)(iVar3 + 0x38)) >> 3;
  return;
}
