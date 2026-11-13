/* SPDX-License-Identifier: MIT */
/* z_xvt 004b4060 */
extern unsigned char DAT_0070a188;
extern unsigned char DAT_006d4998;
extern unsigned short DAT_0070acc2;
extern unsigned int DAT_00709ff4;

unsigned __int64 __fastcall FUN_004b4060(unsigned int param_1, unsigned int param_2, unsigned char *param_3)
{
  unsigned int uVar1;
  unsigned char bVar2;
  unsigned char *pbVar3;
  
  uVar1 = 0;
  bVar2 = *param_3;
  if (bVar2 != 0) {
    param_2 = 0;
    do {
      pbVar3 = param_3 + 1;
      if (bVar2 == 10) break;
      if (0x1f < bVar2) {
        if (bVar2 == 0xfe) {
          pbVar3 = param_3 + 2;
        }
        else {
          if ((((DAT_0070a188 != 0) && (DAT_006d4998 == 0)) && (0x60 < bVar2)) &&
             (bVar2 < 0x7b)) {
            bVar2 = bVar2 - 0x20;
          }
          uVar1 = (unsigned int)(unsigned short)((short)uVar1 +
                                (unsigned short)*(unsigned char *)((unsigned int)(unsigned char)(bVar2 - 0x20) * (unsigned int)DAT_0070acc2 +
                                                 DAT_00709ff4));
        }
      }
      bVar2 = *pbVar3;
      param_3 = pbVar3;
    } while (bVar2 != 0);
  }
  return ((unsigned __int64)param_2 << 32) | uVar1;
}
