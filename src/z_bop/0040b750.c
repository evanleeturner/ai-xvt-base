/* SPDX-License-Identifier: MIT */
/* z_bop 0040b750 */
extern unsigned char DAT_00a00530;
extern unsigned char DAT_005233c8;
extern void **DAT_00803f80;

void __stdcall FUN_004b32b0(void);

void __stdcall FUN_0040b750(void)
{
  unsigned int *puVar1;
  int iVar2;
  unsigned int local_64[20];
  unsigned int local_14;

  puVar1 = local_64;
  for (iVar2 = 0x19; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar1 = 0;
    puVar1 = puVar1 + 1;
  }
  local_64[0] = 100;
  local_64[2] = 0xcc0020;
  local_14 = (unsigned int)*(unsigned short *)(&DAT_00a00530 + (unsigned int)DAT_005233c8 * 2);
  (void)local_14;
  ((void (__stdcall *)(void *, int, int, int, int, unsigned int *))((unsigned char *)*DAT_00803f80 + 0x14))(DAT_00803f80, 0, 0, 0, 0x1000400, local_64);
  FUN_004b32b0();
  return;
}
