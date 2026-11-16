/* SPDX-License-Identifier: MIT */
/* z_bop 0042ce60 */
extern undefined4 *DAT_00a10514;
extern undefined4 DAT_00a10518;
extern undefined4 DAT_00a1065c;
extern undefined4 DAT_00a5fbe4;
extern undefined4 DAT_00a604cc;
extern undefined4 DAT_00a6052c;
extern undefined4 DAT_00a606dc;

void __stdcall FUN_0042d020(void);

undefined4 __stdcall FUN_0042ce60(void)
{
  undefined4 *puVar1;
  undefined4 *puVar2;
  
  if (DAT_00a10514 == (undefined4 *)0x0) {
    return 1;
  }
  FUN_0042d020();
  (*(void (__stdcall **)(undefined4 *))(*DAT_00a10514 + 8))(DAT_00a10514);
  DAT_00a10514 = (undefined4 *)0x0;
  puVar2 = &DAT_00a1065c;
  do {
    *puVar2 = 0;
    puVar1 = (undefined4 *)((int)puVar2 + 0x145);
    *(undefined1 *)((int)puVar2 - 0x50) = 0;
    puVar2 = puVar1;
  } while (puVar1 < &DAT_00a5fbe4);
  puVar2 = &DAT_00a604cc;
  do {
    *puVar2 = 0xffffffff;
    puVar1 = puVar2 + 3;
    puVar2[1] = 0;
    puVar2[2] = 0;
    puVar2 = puVar1;
  } while (puVar1 < &DAT_00a6052c);
  DAT_00a10518 = 0;
  DAT_00a606dc = 0;
  return 1;
}
