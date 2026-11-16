/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5283 */
extern unsigned int DAT_006572c8;
extern unsigned int DAT_0052f1d8;
extern unsigned int DAT_0052f1d4;
extern unsigned int DAT_00521b60;
extern unsigned int DAT_0052f1f4;
extern unsigned int DAT_0052f1c0;
extern unsigned char DAT_0052f1cc;

extern void __stdcall FUN_0045bbe0(void);
extern void __cdecl FUN_004b51f0(unsigned int, unsigned int, unsigned int);
extern void __stdcall FUN_004b7958(void);

unsigned int __stdcall FUN_004b5283(void)
{
  unsigned int uVar1;
  int iVar2;

  if (DAT_006572c8 == 0) {
    FUN_0045bbe0();
    uVar1 = 0;
  }
  else {
    DAT_0052f1d8 = 0x3363;
    DAT_0052f1d4 = 1;
    FUN_0045bbe0();
    iVar2 = ((int (__stdcall *)(void *, void *, void *))(*(void **)DAT_006572c8))((void *)DAT_006572c8, &DAT_00521b60, &DAT_0052f1f4);
    if (iVar2 == 0) {
      FUN_0045bbe0();
      DAT_0052f1c0 = 0;
      iVar2 = ((int (__stdcall *)(void *, void *, int))(*(void **)(*(void **)DAT_0052f1f4 + 0x10)))((void *)DAT_0052f1f4, FUN_004b7958, 0);
      if (iVar2 == 0) {
        if (DAT_0052f1c0 == 0) {
          uVar1 = 0;
        }
        else {
          FUN_004b51f0(0, 0x52f210, 0x79);
          FUN_0045bbe0();
          DAT_0052f1cc = 1;
          FUN_0045bbe0();
          uVar1 = 1;
        }
      }
      else {
        FUN_004b51f0(iVar2, 0x52f210, 0x79);
        FUN_0045bbe0();
        uVar1 = 0;
      }
    }
    else {
      FUN_004b51f0(iVar2, 0x52f210, 0x79);
      FUN_0045bbe0();
      uVar1 = 0;
    }
  }
  return uVar1;
}
