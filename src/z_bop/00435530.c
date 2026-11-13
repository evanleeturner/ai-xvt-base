/* SPDX-License-Identifier: MIT */
/* z_bop 00435530 */
extern unsigned int __cdecl FUN_00425c60(unsigned int, unsigned int);
extern int DAT_009a1fe8;
extern int DAT_009d9192;
extern int DAT_009e99e9;
extern int DAT_009e9678;
extern int DAT_009e9a1b;
extern int DAT_009d8c89;

void __cdecl FUN_00435530(int param_1, int param_2, int param_3)
{
  unsigned int uVar1;
  int iVar2;
  int iVar3;

  if (param_3 == 2 || param_3 == 3) {
    iVar2 = param_2 * 0x23 + DAT_009a1fe8;
    iVar3 = *(int *)(iVar2 + 0x1b);
    if (iVar3 != -1 || (iVar3 = *(int *)((int)&DAT_009d9192 + (unsigned int)*(unsigned char *)(iVar2 + 0x16) * 0x566), iVar3 != -1)) {
      iVar2 = *(int *)(*(int *)(iVar2 + 0x1f) + 0xa9);
      uVar1 = FUN_00425c60(*(unsigned int *)(iVar2 + 0xd7), *(unsigned int *)(iVar2 + 0xd3));
      if (0x7fff < (unsigned short)uVar1) {
        iVar2 = *(int *)((int)&DAT_009d9192 + param_1 * 0x566);
        if (iVar2 != -1) {
          if (param_3 == 3) {
            *(short *)((void *)&DAT_009e99e9 + (unsigned int)*(unsigned short *)((int)&DAT_009e9678 + iVar2 * 0x5bd) * 2 + iVar3 * 0x5bd) =
               *(short *)((void *)&DAT_009e99e9 + (unsigned int)*(unsigned short *)((int)&DAT_009e9678 + iVar2 * 0x5bd) * 2 + iVar3 * 0x5bd) + 1;
          }
          return;
        }
        if (param_3 == 3) {
          *(short *)((void *)&DAT_009e9a1b + (unsigned int)*(unsigned char *)((int)&DAT_009d8c89 + param_1 * 0x566) * 2 + iVar3 * 0x5bd) =
             *(short *)((void *)&DAT_009e9a1b + (unsigned int)*(unsigned char *)((int)&DAT_009d8c89 + param_1 * 0x566) * 2 + iVar3 * 0x5bd) + 1;
        }
      }
    }
  }
  return;
}
