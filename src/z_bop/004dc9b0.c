/* SPDX-License-Identifier: MIT */
/* z_bop 004dc9b0 */
extern unsigned int *DAT_00aa6cfc;
extern char DAT_00aa7b0d;
extern unsigned int DAT_00aa7b1f;
extern int DAT_00aa7b1b;
extern int DAT_00aa7b17;

unsigned int * __stdcall FUN_004d48e0(void);
void __stdcall FUN_004d4e20(void);
void __cdecl FUN_004d4ec0(int);
void __stdcall FUN_004d4970(void);

unsigned int __stdcall FUN_004dc9b0(void)
{
  char cVar1;
  unsigned int *puVar2;
  unsigned int *puVar3;
  int iVar4;
  int iVar5;
  unsigned int *puVar6;
  unsigned int *puVar7;
  
  cVar1 = DAT_00aa7b0d;
  puVar2 = FUN_004d48e0();
  FUN_004d4e20();
  iVar5 = 0x1e0;
  puVar3 = DAT_00aa6cfc;
  do {
    puVar6 = puVar2;
    puVar7 = puVar3;
    for (iVar4 = ((DAT_00aa7b1f & 0xfffffff8) * 5 & 0xfffffff) << 2; iVar4 != 0; iVar4 = iVar4 + -1)
    {
      *puVar7 = *puVar6;
      puVar6 = puVar6 + 1;
      puVar7 = puVar7 + 1;
    }
    puVar3 = (unsigned int *)((int)puVar3 + DAT_00aa7b1b);
    puVar2 = (unsigned int *)((int)puVar2 + DAT_00aa7b17);
    iVar5 = iVar5 + -1;
  } while (iVar5 != 0);
  FUN_004d4ec0(0);
  if (cVar1 == '\0') {
    FUN_004d4970();
  }
  return 1;
}
