/* SPDX-License-Identifier: MIT */
/* z_xvt 004058c0 */
extern unsigned int __cdecl FUN_00405cf0(int);
extern unsigned short DAT_00732f94;
extern unsigned short DAT_0070acc6;
extern unsigned short DAT_007092b0;
extern unsigned short DAT_006db592;
extern unsigned short DAT_0071ecf0;
extern unsigned short DAT_0071e534;
extern unsigned short DAT_0071b6b0;
extern unsigned short DAT_006d9d88;
extern unsigned short DAT_006d9d92;
extern unsigned short DAT_00732f92;
extern unsigned short DAT_0073254c;
extern unsigned short DAT_00732554;
extern unsigned short DAT_0071eae4;

void __stdcall FUN_004058c0(void)
{
  unsigned int uVar1;
  unsigned short uVar2;
  unsigned short uVar3;
  
  do {
    uVar1 = FUN_00405cf0(-2);
    if ((short)uVar1 != 0) break;
    if ((DAT_00732f94 & 0xf) != 0) goto LAB_004058f8;
  } while (DAT_0070acc6 == 0);
  if ((DAT_00732f94 & 0xf) != 0) goto LAB_00405906;
  if (DAT_0070acc6 != 0) {
LAB_004058f8:
    while (((DAT_00732f94 & 0xf) != 0 || (DAT_0070acc6 != 0))) {
LAB_00405906:
      uVar3 = 0;
      FUN_00405cf0(-2);
      DAT_007092b0 = DAT_006db592;
      DAT_0071ecf0 = 0;
      DAT_0071e534 = 0;
      if (DAT_0071b6b0 != 0) {
        DAT_0071ecf0 = (unsigned short)(DAT_006d9d88 << 7);
        uVar3 = (unsigned short)(DAT_006d9d92 << 6);
        DAT_0071e534 = DAT_0070acc6;
      }
      if ((unsigned char)((unsigned char)DAT_00732f92 | 1) != 0) {
        if (DAT_0071ecf0 == 0) {
          DAT_0071ecf0 = (unsigned short)(DAT_0073254c * 0x78);
        }
        if (uVar3 == 0) {
          uVar3 = (unsigned short)(DAT_00732554 * 0x32);
        }
        DAT_0071e534 = DAT_0071e534 | DAT_00732f94;
      }
      uVar2 = DAT_0071ecf0;
      if (0x7fff < DAT_0071ecf0) {
        uVar2 = (unsigned short)(-DAT_0071ecf0);
      }
      if ((short)uVar2 < 0x801) {
        DAT_0071ecf0 = 0;
      }
      uVar2 = uVar3;
      if (0x7fff < uVar3) {
        uVar2 = (unsigned short)(-uVar3);
      }
      DAT_0071eae4 = uVar3;
      if ((short)uVar2 < 0x601) {
        DAT_0071eae4 = 0;
      }
    }
  }
  return;
}
