/* SPDX-License-Identifier: MIT */
/* z_bop 00436bc0 */
extern int DAT_009a1fe8;
extern int DAT_009d6959;
extern int DAT_009e967c;
extern int DAT_009ecc34;
extern int DAT_009d8c88;
extern int DAT_009fd458;
extern int DAT_009e968a;

unsigned int __cdecl FUN_00484f80(unsigned short param_1);
void __cdecl FUN_00482eb0(unsigned short param_1, unsigned short param_2, unsigned char param_3);

void __cdecl FUN_00436bc0(unsigned int param_1)
{
  unsigned int iVar1;
  unsigned int uVar2;
  unsigned char bVar3;
  _Bool bVar4;
  
  iVar1 = *(unsigned int *)((param_1 * 0x23 + 0x1f) + DAT_009a1fe8);
  switch (*(unsigned char *)(iVar1 + 0x88)) {
  case 0:
    bVar3 = 0x3f;
    break;
  case 1:
  case 4:
    bVar3 = 0x37;
    break;
  case 2:
    bVar3 = 0x33;
    break;
  default:
    bVar3 = 0x3b;
  }
  
  if ((DAT_009d6959 == 0) &&
      (uVar2 = *(unsigned short *)((unsigned int)&DAT_009e967c + DAT_009ecc34 * 0x5bd),
       *(unsigned char *)(*(unsigned int *)(iVar1 + 0xa9) + 0x1f6 + uVar2) == 0)) {
    if ((unsigned char)(&DAT_009d8c88)[(unsigned int)*(unsigned char *)((param_1 & 0xffff) * 0x23 + 0x16 + DAT_009a1fe8) * 0x566] == uVar2) {
      bVar4 = 0;
    }
    else {
      bVar4 = (&DAT_009fd458)[(unsigned int)(unsigned char)(&DAT_009d8c88)[(unsigned int)*(unsigned char *)((param_1 & 0xffff) * 0x23 + 0x16 + DAT_009a1fe8) * 0x566] + uVar2 * 0x1e5] == 0;
    }
    if (bVar4) {
      return;
    }
  }
  
  uVar2 = FUN_00484f80((unsigned short)param_1);
  if (((unsigned char)uVar2 == 0) && (*(unsigned short *)((unsigned int)&DAT_009e968a + DAT_009ecc34 * 0x5bd) != param_1)) {
    FUN_00482eb0((unsigned short)param_1, 0xffff, bVar3);
  }
  return;
}
