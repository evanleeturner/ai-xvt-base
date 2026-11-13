/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f5c0 */
unsigned char in(unsigned short port);
void out(unsigned short port, unsigned char value);

void __cdecl FUN_0048f5c0(unsigned char *param_1, unsigned char param_2, short param_3)
{
  unsigned char bVar1;
  int iVar2;
  
  iVar2 = (int)param_3;
  if (iVar2 != 0) {
    do {
      bVar1 = in(0x3da);
    } while ((bVar1 & 8) != 0);
    do {
      bVar1 = in(0x3da);
    } while ((bVar1 & 8) == 0);
    out(0x3c8, param_2);
    do {
      out(0x3c9, *param_1);
      out(0x3c9, param_1[1]);
      out(0x3c9, param_1[2]);
      param_1 += 3;
      iVar2 -= 1;
    } while (iVar2 != 0);
  }
  return;
}
