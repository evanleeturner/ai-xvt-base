/* SPDX-License-Identifier: MIT */
/* z_xvt 00498e50 */
extern unsigned char DAT_0085e798;
extern unsigned char DAT_0085e79e;
extern int DAT_0081bd0e;
extern unsigned char DAT_0072f4e4;
extern unsigned char DAT_0073257c;
extern int DAT_0071ecf4;
extern unsigned char DAT_00709d0a;
extern unsigned char DAT_00709d0b;
extern unsigned char DAT_006d4a02;
extern int DAT_0071e528;
extern unsigned long DAT_006da844;
extern unsigned char DAT_0071b73c;
extern unsigned char DAT_0071b73a;

extern unsigned int __cdecl FUN_004392c0(unsigned int param_1, unsigned int param_2, unsigned char param_3);
extern int __cdecl FUN_00439530(int param_1);
extern unsigned long __stdcall timeGetTime(void);

void __stdcall FUN_00498e50(void)
{
  short sVar1;
  unsigned int uVar2;
  unsigned long DVar3;
  unsigned char bVar4;
  
  if (DAT_0085e798 == 0) {
    return;
  }
  if (DAT_0085e79e == 0) {
    return;
  }
  if (DAT_0081bd0e != 1) {
    return;
  }
  bVar4 = 0;
  if ((DAT_0072f4e4 == 2) && (DAT_0073257c == 0)) {
    uVar2 = (unsigned int)*(unsigned short *)((int)&DAT_0071b73c + DAT_0071ecf4 * 0x5bd);
    if (((&DAT_00709d0a)[uVar2 * 3] == 2) || ((&DAT_00709d0b)[uVar2 * 3] == 1)) {
      bVar4 = 7;
    }
    else if (((&DAT_00709d0a)[uVar2 * 3] == 1) && (DAT_006d4a02 != 2)) {
      sVar1 = *(short *)((int)&DAT_0071b73a + DAT_0071ecf4 * 0x5bd);
      if (DAT_006d4a02 == 4) {
        bVar4 = (-(sVar1 == 1) & 2U) + 4;
      }
      else {
        bVar4 = (-(sVar1 == 1) & 2U) + 3;
      }
    }
    if (bVar4 == 0) goto LAB_00498f46;
    FUN_004392c0((unsigned int)bVar4, 0, 0);
    DAT_0071e528 = FUN_00439530((unsigned int)bVar4);
    DAT_0073257c = 1;
    DAT_0072f4e4 = bVar4;
  }
  if (bVar4 != 0) {
    return;
  }
LAB_00498f46:
  DVar3 = timeGetTime();
  DAT_0071e528 = DAT_0071e528 - (DVar3 - DAT_006da844);
  DAT_006da844 = DVar3;
  if (DAT_0071e528 < 1) {
    FUN_004392c0(2, 0, 0);
    DAT_0071e528 = FUN_00439530(2);
    DAT_0072f4e4 = 2;
  }
  return;
}
