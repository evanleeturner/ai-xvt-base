/* SPDX-License-Identifier: MIT */
/* z_xvt 004dbd20 */
extern char DAT_00759645;
extern unsigned int *DAT_00758830;
extern int DAT_00759657;
extern int DAT_00759653;
extern int DAT_0075964f;

extern unsigned int __stdcall FUN_004d4d00(void);
extern void __stdcall FUN_004d52b0(void);
extern void __cdecl FUN_004d5350(int);
extern void __stdcall FUN_004d4d90(void);

unsigned int __stdcall FUN_004dbd20(void)
{
  char cVar1;
  unsigned int *puVar2;
  int iVar3;
  int iVar4;
  unsigned int *puVar5;
  unsigned int *puVar6;
  unsigned int *puVar7;
  
  cVar1 = DAT_00759645;
  puVar2 = (unsigned int *)FUN_004d4d00();
  FUN_004d52b0();
  iVar3 = 0x1e0;
  puVar5 = DAT_00758830;
  do {
    puVar6 = puVar2;
    puVar7 = puVar5;
    for (iVar4 = ((DAT_00759657 & 0xfffffff8) * 5 & 0xfffffff) << 2; iVar4 != 0; iVar4 = iVar4 + -1)
    {
      *puVar7 = *puVar6;
      puVar6 = puVar6 + 1;
      puVar7 = puVar7 + 1;
    }
    puVar5 = (unsigned int *)((int)puVar5 + DAT_00759653);
    puVar2 = (unsigned int *)((int)puVar2 + DAT_0075964f);
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_004d5350(0);
  if (cVar1 == '\0') {
    FUN_004d4d90();
  }
  return 1;
}
