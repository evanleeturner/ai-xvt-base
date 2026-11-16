/* SPDX-License-Identifier: MIT */
/* z_bop 004d6ca0 */
extern unsigned char in(unsigned short port);
extern void out(unsigned short port, unsigned char value);
extern int DAT_00b69cb0;
extern void *DAT_00aa7b9a;
extern unsigned char DAT_00aa7b9e;

void __fastcall FUN_004d6ca0(int param_1)
{
  unsigned char bVar1;
  int iVar2;
  unsigned char *pbVar3;

  if (DAT_00b69cb0 != 0) {
    if (param_1 != 0) {
      pbVar3 = &DAT_00aa7b9e;
      iVar2 = 0xff;
      do {
        bVar1 = in(0x3da);
      } while ((bVar1 & 8) != 0);
      do {
        bVar1 = in(0x3da);
      } while ((bVar1 & 8) == 0);
      out(0x3c8, 0);
      do {
        out(0x3c9, *pbVar3 >> 2);
        out(0x3c9, pbVar3[1] >> 2);
        out(0x3c9, pbVar3[2] >> 2);
        pbVar3 += 4;
        iVar2 -= 1;
      } while (iVar2 != 0);
    }
    return;
  }
  (*(void (**)(void *, int, int, int, void *))(*(void **)DAT_00aa7b9a + 0x18))(DAT_00aa7b9a, 0, 0, 0x100, &DAT_00aa7b9e);
  return;
}
