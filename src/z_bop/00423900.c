/* SPDX-License-Identifier: MIT */
/* z_bop 00423900 */
extern short DAT_009a8d48;
extern short DAT_009a8d42;
extern short DAT_00a07cd0;
extern unsigned char *DAT_009fe7d4;
extern short DAT_009ed230;
extern short DAT_009e9652;
extern short DAT_009e9654;
extern short DAT_009a7bb0;
extern short DAT_009a7bb2;
extern short DAT_009ec458;
extern short DAT_009d8c2a;
extern short DAT_009d8c14;
extern short DAT_009d8c2c;
extern short DAT_009d114c;
extern short DAT_009ec462;

unsigned int __stdcall FUN_00423900(void)
{
  short sVar1;
  short sVar2;
  short *psVar3;
  short sVar4;
  /* original reads uninitialized stack here; zero-initialized to satisfy the build */
  short local_4 = 0;

  sVar1 = 0;
  if (DAT_009a8d48 < 0) {
    do {
      DAT_009a8d42 = (short)(DAT_009a8d42 - (*(short *)(DAT_009fe7d4 + 0x2c) + 1));
      sVar1 = (short)(sVar1 + DAT_00a07cd0);
      DAT_009a8d48 = (short)(DAT_009a8d48 + *(short *)(DAT_009fe7d4 + 0x2e) + 1);
    } while (DAT_009a8d48 < 0);
  }
  if (DAT_00a07cd0 <= DAT_009a8d48) {
    do {
      sVar1 = (short)(sVar1 - DAT_00a07cd0);
      DAT_009a8d42 = (short)(DAT_009a8d42 + *(short *)(DAT_009fe7d4 + 0x2c) + 1);
      DAT_009a8d48 = (short)(DAT_009a8d48 - (*(short *)(DAT_009fe7d4 + 0x2e) + 1));
    } while (DAT_00a07cd0 <= DAT_009a8d48);
  }
  sVar2 = (short)(DAT_009ed230 - DAT_009a8d48);
  psVar3 = (short *)(DAT_009fe7d4 + 0x2e);
  DAT_009a8d42 = (short)(DAT_009a8d42 - *(short *)(DAT_009fe7d4 + 0x30 + sVar2 * 4));
  DAT_009e9652 = DAT_009ed230;
  DAT_009e9654 = DAT_009a8d42;
  DAT_009a7bb0 = (short)(DAT_009ed230 - *psVar3);
  DAT_009ec458 = 0;
  DAT_009d8c2a = -1;
  DAT_009a7bb2 = (short)(*(short *)(DAT_009fe7d4 + 0x2c) + DAT_009a8d42);
  DAT_009d8c14 = DAT_009d8c2c;
  if (DAT_009d8c2c <= DAT_009a8d42) {
    return 0;
  }
  if (DAT_009a8d42 < 0) {
    if (DAT_009a7bb2 < 0) {
      DAT_009ec462 = (short)(sVar1 + sVar2);
      DAT_009d114c = (short)0xffff;
      return 1;
    }
    DAT_009d8c2a = (short)(-DAT_009a8d42);
    if (*(short *)(DAT_009fe7d4 + 0x2c) < DAT_009d8c2a) {
      DAT_009ec458 = -1;
    }
    else {
      sVar4 = 0;
      DAT_009ec458 = local_4;
      if (-1 < *psVar3) {
        do {
          local_4 = DAT_009d8c2a;
          if ((int)*(unsigned short *)(DAT_009fe7d4 + 0x30 + sVar4 * 4) == (int)DAT_009d8c2a) {
            DAT_009ec458 = *(short *)(DAT_009fe7d4 + 0x32 + sVar4 * 4);
            break;
          }
          sVar4 = (short)(sVar4 + 1);
          DAT_009ec458 = DAT_009d8c2a;
        } while (sVar4 <= *psVar3);
      }
    }
  }
  if (DAT_009a7bb2 < DAT_009d8c2c) {
    if (-1 < DAT_009a7bb2) {
      local_4 = (short)(*(short *)(DAT_009fe7d4 + 0xe) + -1);
      goto LAB_00423b64;
    }
    if (DAT_009d8c2c <= DAT_009a7bb2) goto LAB_00423afc;
  }
  else {
LAB_00423afc:
    if (DAT_009a8d42 < DAT_009d8c2c) {
      sVar4 = 0;
      DAT_009d8c14 = (short)(DAT_009d8c2c - DAT_009a8d42);
      if (-1 < *psVar3) {
        do {
          if ((int)*(unsigned short *)(DAT_009fe7d4 + 0x30 + sVar4 * 4) == (int)DAT_009d8c14) {
            local_4 = (short)(*(short *)(DAT_009fe7d4 + 0x32 + sVar4 * 4) + -1);
            break;
          }
          sVar4 = (short)(sVar4 + 1);
          local_4 = DAT_009d8c14;
        } while (sVar4 <= *psVar3);
      }
      goto LAB_00423b64;
    }
  }
  local_4 = -1;
LAB_00423b64:
  DAT_009d114c = local_4;
  DAT_009ec462 = (short)(sVar1 + sVar2);
  return 1;
}
