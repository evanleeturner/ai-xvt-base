/* SPDX-License-Identifier: MIT */
/* z_bop 00442250 */
extern unsigned short DAT_0052747c;
extern unsigned int DAT_009a8074;
extern unsigned int DAT_00527e90;
void __cdecl FUN_0040b520(void);

void __cdecl FUN_00442250(unsigned short param_1, unsigned int param_2)
{
  unsigned short uVar1;
  unsigned short uVar2;
  unsigned char *puVar3;
  unsigned char *puVar4;
  
  uVar1 = 0;
  puVar3 = (unsigned char *)((unsigned int)DAT_0052747c + DAT_009a8074);
  if (param_2 != 0) {
    do {
      *puVar3 = 1;
      puVar4 = puVar3 + 1;
      uVar2 = param_1;
      if (0xff < param_1) {
        *puVar4 = 0;
        puVar4 = puVar3 + 2;
        uVar2 = param_1 - 0xff;
        if (0xff < uVar2) {
          uVar2 = param_1 - 0x1ff;
          *puVar4 = 0;
          puVar4 = puVar3 + 3;
        }
      }
      uVar1 = uVar1 + 1;
      *puVar4 = (unsigned char)uVar2;
      puVar3 = puVar4 + 1;
    } while (uVar1 < param_2);
  }
  if (DAT_00527e90 != 0) {
    FUN_0040b520();
  }
  return;
}
