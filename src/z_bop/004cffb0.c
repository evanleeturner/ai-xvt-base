/* SPDX-License-Identifier: MIT */
/* z_bop 004cffb0 */
extern int DAT_00aa6cfc;
extern char DAT_00aa7b0d;
extern int DAT_00acf571[];
extern int DAT_00acf581[];
extern int DAT_00acebc5;
extern int DAT_00b53575;
extern int DAT_00b53579;

extern void __stdcall FUN_004d4970(void);
extern void __stdcall FUN_004ce130(void);
extern int __stdcall FUN_004d48e0(void);

int __stdcall FUN_004cffb0(void)
{
  int iVar1;
  char cVar2;
  int iVar3;
  
  cVar2 = DAT_00aa7b0d;
  if (DAT_00acebc5 == 0) {
    return 0;
  }
  FUN_004d4970();
  FUN_004ce130();
  iVar3 = DAT_00b53575;
  iVar1 = DAT_00b53579;
  if (DAT_00b53579 < 0x201) {
    while (1) {
      if (iVar1 < 1) {
        if (cVar2 != '\0') {
          DAT_00aa6cfc = FUN_004d48e0();
        }
        return 0;
      }
      if (DAT_00acf571[iVar3 * 0x84] == 0 && DAT_00acf581[iVar3 * 0x84] == 3) break;
      iVar3 = iVar3 + 1;
      if (0x3ff < iVar3) {
        iVar3 = 0;
      }
      iVar1 = iVar1 - 1;
    }
    if (cVar2 != '\0') {
      DAT_00aa6cfc = FUN_004d48e0();
    }
    return 1;
  }
  if (cVar2 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return 1;
}
