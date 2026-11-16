/* SPDX-License-Identifier: MIT */
/* z_xvt 004f2410 */
extern short DAT_00650406;
extern short _DAT_00650408;
extern short DAT_00650424;
extern short DAT_00650426;
extern short DAT_00650428;
extern short DAT_0065042a;
extern int DAT_00650434;
extern short DAT_0065042c;
extern int DAT_00650438;
extern short DAT_0065042e;
extern short DAT_00650430;
extern short DAT_00650432;
extern int DAT_0065043c;
extern int DAT_00743880;
extern int DAT_0075766c;
extern char DAT_007562e1;
extern char DAT_007562de;
extern void *DAT_006507a2;
extern void *DAT_00650822;
extern void *DAT_006508a2;
extern int DAT_006508fa;
extern int DAT_00650904;
extern int DAT_00650936;
extern int DAT_00650440;

void __stdcall FUN_004f2bd0(void);
void __stdcall FUN_004f2c10(void);
void *__cdecl _malloc(unsigned int);
void __cdecl FUN_004dcfd0(int *, int, int, int, int);

void __stdcall FUN_004f2410(void)
{
  void *pvVar1;
  int iVar2;
  short sVar3;
  int *puVar4;

  DAT_00650406 = 1;
  sVar3 = 0;
  _DAT_00650408 = 0;
  DAT_00650424 = 0;
  DAT_00650426 = 0;
  DAT_00650428 = 0;
  DAT_0065042a = 0;
  DAT_00650434 = 0;
  DAT_0065042c = 0x20;
  DAT_00650438 = 0;
  DAT_0065042e = 0x20;
  DAT_00650430 = 0x20;
  DAT_00650432 = 0x20;
  DAT_0065043c = 0;
  puVar4 = &DAT_00743880;
  for (iVar2 = 0x4f7d; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar4 = 0;
    puVar4 = puVar4 + 1;
  }
  DAT_0075766c = 1;
  DAT_007562e1 = 0;
  DAT_007562de = 1;
  FUN_004f2bd0();
  FUN_004f2c10();
  do {
    pvVar1 = _malloc(0x28);
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_006507a2)[iVar2] = pvVar1;
  } while (sVar3 < 0x20);
  sVar3 = 0;
  do {
    pvVar1 = _malloc(0x140);
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_00650822)[iVar2] = pvVar1;
  } while (sVar3 < 0x20);
  sVar3 = 0;
  do {
    pvVar1 = _malloc(0x400);
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_006508a2)[iVar2] = pvVar1;
  } while (sVar3 < 0x14);
  sVar3 = 0;
  do {
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_006508fa)[iVar2] = 0;
  } while (sVar3 < 2);
  sVar3 = 0;
  do {
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_00650904)[iVar2] = 0;
  } while (sVar3 < 8);
  sVar3 = 0;
  do {
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_00650936)[iVar2] = 0;
  } while (sVar3 < 8);
  FUN_004dcfd0(&DAT_00650440, 0, 0, 0x168, 0xec);
  return;
}
