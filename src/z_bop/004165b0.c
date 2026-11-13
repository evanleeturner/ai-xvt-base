/* SPDX-License-Identifier: MIT */
/* z_bop 004165b0 */
extern char DAT_00bb7378;
extern char DAT_00bb737e;
extern int DAT_00b6a2fe;
extern unsigned char DAT_009fd434;
extern unsigned char DAT_00a004c8;
extern unsigned char DAT_009e967c;
extern unsigned int DAT_009ecc34;
extern unsigned char DAT_009d738a;
extern unsigned char DAT_009d738b;
extern unsigned char DAT_009a2062;
extern unsigned char DAT_009e967a;
extern int DAT_009ec468;
extern unsigned long DAT_009a7ec4;

void __cdecl FUN_004a50d0(unsigned int param_1, unsigned int param_2, unsigned char param_3);
int __cdecl FUN_004a5340(int param_1);
unsigned long __stdcall timeGetTime(void);

void __stdcall FUN_004165b0(void)
{
  short sVar1;
  unsigned int uVar2;
  unsigned long DVar3;
  unsigned char bVar4;
  
  if (DAT_00bb7378 == '\0') {
    return;
  }
  if (DAT_00bb737e == '\0') {
    return;
  }
  if (DAT_00b6a2fe != 1) {
    return;
  }
  bVar4 = 0;
  if ((DAT_009fd434 == 2) && (DAT_00a004c8 == '\0')) {
    uVar2 = (unsigned int)*(unsigned short *)((int)&DAT_009e967c + DAT_009ecc34 * 0x5bd);
    if (((&DAT_009d738a)[uVar2 * 3] == '\x02') || ((&DAT_009d738b)[uVar2 * 3] == '\x01')) {
      bVar4 = 7;
    }
    else if (((&DAT_009d738a)[uVar2 * 3] == '\x01') && (DAT_009a2062 != '\x02')) {
      sVar1 = *(short *)((int)&DAT_009e967a + DAT_009ecc34 * 0x5bd);
      if (DAT_009a2062 == '\x04') {
        bVar4 = (-(sVar1 == 1) & 2U) + 4;
      }
      else {
        bVar4 = (-(sVar1 == 1) & 2U) + 3;
      }
    }
    if (bVar4 == 0) goto LAB_004166a6;
    FUN_004a50d0((unsigned int)bVar4, 0, 0);
    DAT_009ec468 = FUN_004a5340((unsigned int)bVar4);
    DAT_00a004c8 = '\x01';
    DAT_009fd434 = bVar4;
  }
  if (bVar4 != 0) {
    return;
  }
LAB_004166a6:
  DVar3 = timeGetTime();
  DAT_009ec468 = DAT_009ec468 - (DVar3 - DAT_009a7ec4);
  DAT_009a7ec4 = DVar3;
  if (DAT_009ec468 < 1) {
    FUN_004a50d0(2, 0, 0);
    DAT_009ec468 = FUN_004a5340(2);
    DAT_009fd434 = 2;
  }
  return;
}
