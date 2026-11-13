/* SPDX-License-Identifier: MIT */
/* z_bop 0043f010 */
extern int DAT_009ecc34;
extern int DAT_009e9670;
extern char DAT_009e9683;
extern int DAT_009a1fe8;
extern char DAT_009e9a77;
extern int DAT_00a08374;

void __cdecl FUN_00440760(int, unsigned int);

void __stdcall FUN_0043f010(void)
{
  char cVar1;
  char bVar2;
  int iVar3;
  unsigned int uVar4;
  int local_int;

  if (*(char *)((int)&DAT_009e9683 + DAT_009ecc34 * 0x5bd) == '\0') {
    __builtin_memcpy(&local_int, (void *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd), 4);
    iVar3 = local_int;
    if (iVar3 == -1) {
      bVar2 = 0;
    }
    else {
      cVar1 = *(char *)(iVar3 * 0x23 + 3 + DAT_009a1fe8);
      if (cVar1 == '\x01' || cVar1 == '\x02' || cVar1 == '\x03' || cVar1 == '\x0e' || cVar1 == '\x04') {
        bVar2 = 1;
      }
      else {
        bVar2 = 0;
      }
    }
    if (!bVar2 && (iVar3 = iVar3 * 0x23 + DAT_009a1fe8, *(char *)(iVar3 + 3) != '\x05') &&
        (*(char *)((int)&DAT_009e9a77 + DAT_009ecc34 * 0x5bd) == '\0' ||
         *(char *)((int)&DAT_009e9a77 + DAT_009ecc34 * 0x5bd) == '\x13')) {
      uVar4 = (unsigned int)DAT_00a08374;
      __builtin_memcpy(&local_int, (void *)(iVar3 + 0x1f), 4);
      __builtin_memcpy(&local_int, (void *)(local_int + 0xa9), 4);
      if ((*(unsigned char *)(local_int + 0x1e4) & 1) == 0) {
        FUN_00440760(uVar4 + 0x6d, 0);
        FUN_00440760(uVar4 + 0x6e, 0);
      }
      __builtin_memcpy(&local_int, (void *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd), 4);
      __builtin_memcpy(&local_int, (void *)(local_int * 0x23 + 0x1f + DAT_009a1fe8), 4);
      __builtin_memcpy(&local_int, (void *)(local_int + 0xa9), 4);
      if ((*(unsigned char *)(local_int + 0x1e3) & 1) == 0) {
        FUN_00440760(uVar4 + 0x6c, 0);
      }
    }
  }
  return;
}
