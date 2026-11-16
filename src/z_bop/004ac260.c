/* SPDX-License-Identifier: MIT */
/* z_bop 004ac260 */
extern int DAT_00527eac;
extern int DAT_0066e710;
extern int DAT_00803f90;
extern int DAT_0066ddc4;
extern int DAT_005233e0;
extern int DAT_0066dde0;
extern int DAT_005233e8;
extern void **DAT_00803f80;
extern int DAT_0066ddcc;

int __stdcall FUN_004aba10(void);
int __stdcall FUN_004ac460(void);
void __stdcall FUN_004aafe0(void);

void __stdcall FUN_004ac260(void)
{
  int bVar1;
  int iVar2;
  int *puVar3;
  int *puVar4;
  unsigned int local_84;
  unsigned int local_80;
  int local_7c;
  int local_78;
  int local_74;
  int local_70;
  int local_6c;
  int local_68;
  int local_64[25];

  if (DAT_00527eac == 0) {
    iVar2 = FUN_004aba10();
    puVar3 = &DAT_0066e710;
    puVar4 = &DAT_00803f90;
    for (iVar2 = (iVar2 * 0xf & 0x7FFFFFFFU) << 3; iVar2 != 0; iVar2 = iVar2 - 1) {
      *puVar4 = *puVar3;
      puVar3 = puVar3 + 1;
      puVar4 = puVar4 + 1;
    }
  }
  else {
    puVar3 = local_64;
    for (iVar2 = 0x19; iVar2 != 0; iVar2 = iVar2 - 1) {
      *puVar3 = 0;
      puVar3 = puVar3 + 1;
    }
    local_64[0] = 100;
    local_64[2] = 0xcc0020;
    while (1) {
      local_84 = (unsigned int)(DAT_0066ddc4 - DAT_005233e0) >> 1;
      local_74 = 0;
      local_70 = 0;
      local_6c = DAT_005233e0;
      local_80 = (unsigned int)(DAT_0066dde0 - DAT_005233e8) >> 1;
      local_7c = local_84 + DAT_005233e0;
      local_78 = local_80 + DAT_005233e8;
      local_68 = DAT_005233e8;
      (void)local_7c;
      (void)local_78;
      (void)local_70;
      (void)local_6c;
      (void)local_68;
      iVar2 = ((int (__stdcall *)(void *, void *, int, void *, unsigned int, void *))((char *)*DAT_00803f80 + 0x14))
                        (DAT_00803f80, &local_84, DAT_0066ddcc, &local_74, 0x20000, local_64);
      if (iVar2 == 0) break;
      if (iVar2 == -0x7789fe3e) {
        bVar1 = FUN_004ac460();
        if (bVar1 == 0) {
          return;
        }
        FUN_004aafe0();
      }
      if (iVar2 != -0x7789fde4) {
        return;
      }
    }
  }
  return;
}
