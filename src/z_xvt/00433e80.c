/* SPDX-License-Identifier: MIT */
/* z_xvt 00433e80 */
extern int DAT_006da4f8;
extern int DAT_006db59c;
extern unsigned int DAT_0071e53c;
extern void *DAT_006d4988;
extern void *DAT_0052686c;
extern int DAT_0071ecf4;
extern int DAT_006db7b4;
extern void *DAT_0071e6b8;
extern unsigned int DAT_00732a38;
extern unsigned int DAT_00703c88;

extern int __cdecl FUN_004340e0(int param_1, unsigned int param_2, int param_3);
extern int __cdecl FUN_0045b880(int param_1, unsigned int param_2);

void __stdcall FUN_00433e80(void)
{
  unsigned int uVar5 = 0;
  int iVar6;
  
  DAT_006da4f8 = 0;
  DAT_006db59c = 0;
  
  if (DAT_0071e53c != 0) {
    iVar6 = 0;
    do {
      unsigned int uVar3 = (unsigned int)((unsigned char *)DAT_006d4988)[iVar6 + 3];
      if (uVar3 != 0) {
        unsigned char switchVal = ((unsigned char *)DAT_006d4988)[iVar6 + 2];
        int callResult = 0;
        
        switch (switchVal) {
          case 0: case 1: case 2: case 3: case 4: case 5:
            callResult = FUN_004340e0(uVar5, (unsigned int)((unsigned short *)DAT_0052686c)[uVar3], DAT_0071ecf4);
            break;
          case 6: case 7: case 0xb: case 0xd:
            callResult = FUN_0045b880(uVar5, (unsigned int)((unsigned short *)DAT_0052686c)[uVar3]);
            break;
          default:
            break;
        }
        
        if (callResult != 0 && DAT_006da4f8 < 0x128) {
          *(unsigned int *)(DAT_0071e6b8 + DAT_006da4f8 * 0xc) = (unsigned int)DAT_006db7b4;
          *(unsigned int *)(DAT_0071e6b8 + 4 + DAT_006da4f8 * 0xc) = uVar5;
          *(int *)(DAT_0071e6b8 + 8 + DAT_006da4f8 * 0xc) = DAT_006db59c;
          DAT_006db59c = (int)(DAT_0071e6b8 + (DAT_006da4f8 + -2 + (DAT_006da4f8 + 1) * 2) * 4);
          DAT_006da4f8 = DAT_006da4f8 + 1;
        }
      }
      iVar6 = iVar6 + 0x23;
      uVar5 = uVar5 + 1;
    } while (uVar5 < DAT_0071e53c);
  }
  
  if (DAT_00732a38 < DAT_00703c88 + DAT_00732a38) {
    iVar6 = DAT_00732a38 * 0x23;
    uVar5 = DAT_00732a38;
    do {
      unsigned int uVar3 = (unsigned int)((unsigned char *)DAT_006d4988)[iVar6 + 3];
      unsigned char bVar1 = ((unsigned char *)DAT_006d4988)[iVar6 + 2];
      
      if (uVar3 != 0 && bVar1 > 7 && bVar1 < 10) {
        int callResult = FUN_004340e0(uVar5, (unsigned int)((unsigned short *)DAT_0052686c)[uVar3], DAT_0071ecf4);
        if (callResult != 0 && DAT_006da4f8 < 0x128) {
          *(unsigned int *)(DAT_0071e6b8 + DAT_006da4f8 * 0xc) = (unsigned int)DAT_006db7b4;
          *(unsigned int *)(DAT_0071e6b8 + 4 + DAT_006da4f8 * 0xc) = uVar5;
          *(int *)(DAT_0071e6b8 + 8 + DAT_006da4f8 * 0xc) = DAT_006db59c;
          DAT_006db59c = (int)(DAT_0071e6b8 + (DAT_006da4f8 + -2 + (DAT_006da4f8 + 1) * 2) * 4);
          DAT_006da4f8 = DAT_006da4f8 + 1;
        }
      }
      iVar6 = iVar6 + 0x23;
      uVar5 = uVar5 + 1;
    } while (uVar5 < DAT_00703c88 + DAT_00732a38);
  }
}
