/* SPDX-License-Identifier: MIT */
/* z_xvt 004b821c */
extern unsigned int DAT_0052f1dc;
extern unsigned int *DAT_0064ee60;
extern unsigned int DAT_0064ef48;
extern unsigned int DAT_0064ef4c;
extern unsigned int _DAT_0064ef44;
extern unsigned int DAT_0064ee24;
extern unsigned int DAT_0064ee20;

void __cdecl FUN_004b821c(unsigned int param_1)
{
  if (DAT_0052f1dc != param_1) {
    if (((DAT_0052f1dc ^ (param_1 & 0xc00)) & 0xc00) != 0) {
      if ((param_1 & 0xc00) == 0) {
        *(unsigned char *)DAT_0064ee60 = 8;
        *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
        *(unsigned short *)((int)DAT_0064ee60 + 2) = 4;
        DAT_0064ee60 = DAT_0064ee60 + 1;
        *DAT_0064ee60 = 0x13;
        DAT_0064ee60[1] = 2;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x14;
        DAT_0064ee60[1] = 1;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        if ((param_1 & 0x800) == 0) {
          *DAT_0064ee60 = 0x15;
          DAT_0064ee60[1] = 2;
        }
        else {
          *DAT_0064ee60 = 0x15;
          DAT_0064ee60[1] = 4;
        }
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x1b;
        DAT_0064ee60[1] = 0;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
      else {
        *(unsigned char *)DAT_0064ee60 = 8;
        *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
        *(unsigned short *)((int)DAT_0064ee60 + 2) = 4;
        DAT_0064ee60 = DAT_0064ee60 + 1;
        *DAT_0064ee60 = 0x13;
        DAT_0064ee60[1] = 5;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x14;
        DAT_0064ee60[1] = 6;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        if ((param_1 & 0x400) == 0) {
          *DAT_0064ee60 = 0x15;
          DAT_0064ee60[1] = 4;
        }
        else {
          *DAT_0064ee60 = 0x15;
          DAT_0064ee60[1] = 2;
        }
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x1b;
        DAT_0064ee60[1] = 1;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
    }
    if (((DAT_0052f1dc ^ (param_1 & 0x3000)) & 0x3000) != 0) {
      *(unsigned char *)DAT_0064ee60 = 8;
      *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
      *(unsigned short *)((int)DAT_0064ee60 + 2) = 2;
      DAT_0064ee60 = DAT_0064ee60 + 1;
      if ((param_1 & 0x1000) == 0) {
        *DAT_0064ee60 = 0x17;
        DAT_0064ee60[1] = 8;
      }
      else {
        *DAT_0064ee60 = 0x17;
        DAT_0064ee60[1] = 2;
      }
      DAT_0064ee60 = DAT_0064ee60 + 2;
      if ((param_1 & 0x2000) == 0) {
        *DAT_0064ee60 = 0xe;
        DAT_0064ee60[1] = 0;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
      else {
        *DAT_0064ee60 = 0xe;
        DAT_0064ee60[1] = 1;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
    }
    if (((DAT_0052f1dc ^ (param_1 & 0x300)) & 0x300) != 0) {
      *(unsigned char *)DAT_0064ee60 = 8;
      *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
      *(unsigned short *)((int)DAT_0064ee60 + 2) = 2;
      DAT_0064ee60 = DAT_0064ee60 + 1;
      *DAT_0064ee60 = 0x11;
      if ((param_1 & 0x100) == 0) {
        DAT_0064ee60[1] = 1;
      }
      else {
        DAT_0064ee60[1] = 2;
      }
      DAT_0064ee60 = DAT_0064ee60 + 2;
      *DAT_0064ee60 = 0x12;
      if ((param_1 & 0x200) == 0) {
        DAT_0064ee60[1] = 1;
      }
      else {
        DAT_0064ee60[1] = 2;
      }
      DAT_0064ee60 = DAT_0064ee60 + 2;
    }
    if (((DAT_0052f1dc ^ (param_1 & 0xc000)) & 0xc000) != 0) {
      *(unsigned char *)DAT_0064ee60 = 8;
      *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
      *(unsigned short *)((int)DAT_0064ee60 + 2) = 1;
      DAT_0064ee60 = DAT_0064ee60 + 1;
      if ((param_1 & 0x4000) == 0) {
        *DAT_0064ee60 = 3;
        DAT_0064ee60[1] = 1;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
      else {
        *DAT_0064ee60 = 3;
        DAT_0064ee60[1] = 3;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
    }
    if (((DAT_0052f1dc ^ (param_1 & 0x80)) & 0x80) != 0) {
      if ((param_1 & 0x80) == 0) {
        *(unsigned char *)DAT_0064ee60 = 8;
        *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
        *(unsigned short *)((int)DAT_0064ee60 + 2) = 1;
        DAT_0064ee60 = DAT_0064ee60 + 1;
        *DAT_0064ee60 = 0x1c;
        DAT_0064ee60[1] = 0;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
      else {
        *(unsigned char *)DAT_0064ee60 = 8;
        *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
        *(unsigned short *)((int)DAT_0064ee60 + 2) = 5;
        DAT_0064ee60 = DAT_0064ee60 + 1;
        *DAT_0064ee60 = 0x1c;
        DAT_0064ee60[1] = 1;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x22;
        DAT_0064ee60[1] = DAT_0064ef48 << 8 | DAT_0064ef4c << 0x10 | _DAT_0064ef44;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x23;
        DAT_0064ee60[1] = 3;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x24;
        DAT_0064ee60[1] = DAT_0064ee24;
        DAT_0064ee60 = DAT_0064ee60 + 2;
        *DAT_0064ee60 = 0x25;
        DAT_0064ee60[1] = DAT_0064ee20;
        DAT_0064ee60 = DAT_0064ee60 + 2;
      }
    }
    DAT_0052f1dc = param_1;
  }
  return;
}
