/* SPDX-License-Identifier: MIT */
/* z_xvt 0040a3c0 */
extern unsigned int DAT_0071ecf4;
extern unsigned char DAT_0070a189;
extern unsigned char DAT_0085e791;
extern unsigned char DAT_0085e79a;
extern unsigned int DAT_006d4988;

extern unsigned int FUN_0040a1a0(unsigned int, unsigned int, int);

unsigned int __cdecl FUN_0040a3c0(unsigned int param_1, int param_2)
{
  unsigned int uVar1;
  
  if (param_2 != (int)DAT_0071ecf4) {
    return 0;
  }
  if (DAT_0070a189 == '\0') {
    return 0;
  }
  if (DAT_0085e791 == '\0') {
    return 0;
  }
  if (DAT_0085e79a != '\0') {
    uVar1 = (unsigned int)*(unsigned char *)(param_1 * 0x23 + 3 + DAT_006d4988);
    switch(uVar1) {
    case 0x89:
    case 0x8a:
    case 0x8b:
    case 0x8c:
    case 0x8d:
    case 0x8e:
    case 0x8f:
    case 0x90:
    case 0x91:
    case 0x92:
    case 0x93:
      uVar1 = FUN_0040a1a0(uVar1 - 0x85, param_1, param_2);
      return uVar1;
    case 0x94:
    case 0x95:
      uVar1 = FUN_0040a1a0(uVar1 - 0x8a, param_1, param_2);
      return uVar1;
    case 0x96:
    case 0x97:
      uVar1 = FUN_0040a1a0(uVar1 - 0x87, param_1, param_2);
      return uVar1;
    case 0x98:
    case 0x99:
    case 0x9a:
    case 0x9b:
      uVar1 = FUN_0040a1a0(0x11, param_1, param_2);
    }
    return uVar1;
  }
  return 0;
}
