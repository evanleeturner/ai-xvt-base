/* SPDX-License-Identifier: MIT */
/* z_xvt 004dbd90 */
extern char DAT_00759645;
extern void * DAT_00758830;
extern int DAT_00759657;
extern int DAT_00759653;
extern int DAT_0075964f;

void * __stdcall FUN_004d4d00(void);
void __stdcall FUN_004d52b0(void);
void __cdecl FUN_004d5350(int);
void __stdcall FUN_004d4d90(void);

int __stdcall FUN_004dbd90(void)
{
  char cVar1;
  int *puVar2;
  int *puVar3;
  int iVar4;
  int iVar5;
  int *puVar6;
  int *puVar7;
  
  cVar1 = DAT_00759645;
  puVar2 = (int *)FUN_004d4d00();
  FUN_004d52b0();
  iVar5 = 0x1e0;
  puVar3 = (int *)DAT_00758830;
  do {
    puVar6 = puVar3;
    puVar7 = puVar2;
    for (iVar4 = ((DAT_00759657 & 0xfffffff8) * 5 & 0xfffffff) << 2; iVar4 != 0; iVar4 = iVar4 - 1)
    {
      *puVar7 = *puVar6;
      puVar6 = puVar6 + 1;
      puVar7 = puVar7 + 1;
    }
    puVar3 = (int *)((int)puVar3 + DAT_00759653);
    puVar2 = (int *)((int)puVar2 + DAT_0075964f);
    iVar5 = iVar5 - 1;
  } while (iVar5 != 0);
  FUN_004d5350(0);
  if (cVar1 == '\0') {
    FUN_004d4d90();
  }
  return 1;
}
