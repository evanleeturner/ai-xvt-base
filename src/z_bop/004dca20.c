/* SPDX-License-Identifier: MIT */
/* z_bop 004dca20 */
void * __stdcall FUN_004d48e0(void);
void __stdcall FUN_004d4e20(void);
void __cdecl FUN_004d4ec0(int param_1);
void __stdcall FUN_004d4970(void);

extern char DAT_00aa7b0d;
extern void * DAT_00aa6cfc;
extern int DAT_00aa7b1f;
extern int DAT_00aa7b1b;
extern int DAT_00aa7b17;

undefined4 __stdcall FUN_004dca20(void)
{
  char cVar1;
  undefined4 *puVar2;
  int iVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  
  cVar1 = DAT_00aa7b0d;
  puVar2 = (undefined4 *)FUN_004d48e0();
  FUN_004d4e20();
  iVar3 = 0x1e0;
  puVar5 = (undefined4 *)DAT_00aa6cfc;
  do {
    puVar6 = puVar5;
    puVar7 = puVar2;
    for (iVar4 = ((DAT_00aa7b1f & 0xfffffff8) * 5 & 0xfffffff) << 2; iVar4 != 0; iVar4 = iVar4 + -1)
    {
      *puVar7 = *puVar6;
      puVar6 = puVar6 + 1;
      puVar7 = puVar7 + 1;
    }
    puVar5 = (undefined4 *)((int)puVar5 + DAT_00aa7b1b);
    puVar2 = (undefined4 *)((int)puVar2 + DAT_00aa7b17);
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_004d4ec0(0);
  if (cVar1 == '\0') {
    FUN_004d4970();
  }
  return 1;
}
