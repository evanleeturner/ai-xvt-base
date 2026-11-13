/* SPDX-License-Identifier: MIT */
/* z_bop 004f69b0 */
extern short DAT_006691e6;
extern short _DAT_006691e8;
extern short DAT_00669204;
extern short DAT_00669206;
extern short DAT_00669208;
extern short DAT_0066920a;
extern undefined4 _DAT_00669214;
extern short DAT_0066920c;
extern undefined4 DAT_00669218;
extern short DAT_0066920e;
extern short DAT_00669210;
extern short DAT_00669212;
extern undefined4 DAT_0066921c;
extern undefined4 DAT_00a91cc0;
extern char DAT_00aa5aac;
extern char DAT_00aa4721;
extern char DAT_00aa471e;
extern void *DAT_00669582;
extern void *DAT_00669602;
extern void *DAT_00669682;
extern undefined4 DAT_006696da;
extern undefined4 DAT_006696e4;
extern undefined4 DAT_00669716;
extern undefined4 DAT_00669220;

void __stdcall FUN_004f7340(void);
void __stdcall FUN_004f7380(void);
void *__cdecl _malloc(unsigned int);
void __cdecl FUN_004d6460(undefined4 *, undefined4, undefined4, undefined4, undefined4);

void __stdcall FUN_004f69b0(void)
{
  void *pvVar1;
  int iVar2;
  short sVar3;
  undefined4 *puVar4;
  
  DAT_006691e6 = 1;
  sVar3 = 0;
  _DAT_006691e8 = 0;
  DAT_00669204 = 0;
  DAT_00669206 = 0;
  DAT_00669208 = 0;
  DAT_0066920a = 0;
  _DAT_00669214 = 0;
  DAT_0066920c = 0x20;
  DAT_00669218 = 0;
  DAT_0066920e = 0x20;
  DAT_00669210 = 0x20;
  DAT_00669212 = 0x20;
  DAT_0066921c = 0;
  puVar4 = &DAT_00a91cc0;
  for (iVar2 = 0x4f7d; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar4 = 0;
    puVar4 = puVar4 + 1;
  }
  DAT_00aa5aac = 1;
  DAT_00aa4721 = 0;
  DAT_00aa471e = 1;
  FUN_004f7340();
  FUN_004f7380();
  do {
    pvVar1 = _malloc(0x28);
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_00669582)[iVar2] = pvVar1;
  } while (sVar3 < 0x20);
  sVar3 = 0;
  do {
    pvVar1 = _malloc(0x140);
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_00669602)[iVar2] = pvVar1;
  } while (sVar3 < 0x20);
  sVar3 = 0;
  do {
    pvVar1 = _malloc(0x400);
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_00669682)[iVar2] = pvVar1;
  } while (sVar3 < 0x14);
  sVar3 = 0;
  do {
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_006696da)[iVar2] = 0;
  } while (sVar3 < 2);
  sVar3 = 0;
  do {
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_006696e4)[iVar2] = 0;
  } while (sVar3 < 8);
  sVar3 = 0;
  do {
    iVar2 = (int)sVar3;
    sVar3 = sVar3 + 1;
    (&DAT_00669716)[iVar2] = 0;
  } while (sVar3 < 8);
  FUN_004d6460((undefined4 *)&DAT_00669220,0,0,0x168,0xec);
  return;
}
