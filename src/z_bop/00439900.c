/* SPDX-License-Identifier: MIT */
/* z_bop 00439900 */
extern unsigned short DAT_00a08374;
extern int DAT_009ecc34;
extern unsigned char DAT_00a0a2d0[];   /* short array, indexed by DAT_00a08374 */
extern unsigned char DAT_009e9a67[];   /* odd address: packed, stride 0x5bd */
extern unsigned char DAT_009e9a69[];   /* odd address: packed, stride 0x5bd */
extern unsigned char DAT_00a09240[];   /* 0xc-byte records */
extern unsigned char DAT_00a0924c[];   /* 0xc-byte records */
extern unsigned char DAT_005233c8;
extern void (*DAT_009d1300)(void);
extern char *DAT_00a08c70;
extern char DAT_009a1ed0[];
extern int DAT_00a0a7d8;
extern int DAT_00a0a7dc;

void __cdecl FUN_0043a5e0(void);
void __cdecl FUN_004419d0(void);
void __cdecl FUN_004aba50(void);
void __cdecl FUN_0040e0c0(int, int, int, int);
void __cdecl FUN_004a9640(char);
void __cdecl FUN_004a95c0(unsigned int);
void __cdecl FUN_004a9590(int);
void __cdecl FUN_004a9520(short, short, short, short);
void __cdecl FUN_004a9500(short, short);
void __cdecl FUN_0043c290(unsigned short, int);
void __cdecl FUN_004a97f0(int);
void __cdecl FUN_004a9860(int);
void __cdecl FUN_004a9820(char *);
/* callee ends with RET 0x0: cdecl, one stack arg; Ghidra's extraout_ECX/EDX are incidental */
void __cdecl FUN_004a9a50(char *);
void __cdecl FUN_004abe50(void);

void __stdcall FUN_00439900(void)
{
  char cVar1;
  unsigned short uVar2;
  int iVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  char *pcVar6;
  char *pcVar7;
  char acStack_50[80];
  short sA;
  short sB;

  FUN_0043a5e0();
  FUN_004419d0();
  uVar4 = (unsigned int)DAT_00a08374;
  iVar3 = DAT_009ecc34;
  __builtin_memcpy(&sA, DAT_009e9a67 + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
  if (*(short *)(DAT_00a0a2d0 + uVar4 * 2) != sA) {
    FUN_004aba50();
    FUN_0040e0c0(0, 0x140, 200, 0);
    FUN_004a9640('\x02');
    FUN_004a95c0((unsigned int)DAT_005233c8);
    FUN_004a9590(0x4e);
    iVar3 = (int)((unsigned int)DAT_00a08374 * 0xcu);
    FUN_004a9520(*(short *)(DAT_00a09240 + iVar3),
                 *(short *)(DAT_00a09240 + iVar3 + 2),
                 (short)(*(short *)(DAT_00a09240 + iVar3 + 8) + *(short *)(DAT_00a09240 + iVar3)),
                 (short)(*(short *)(DAT_00a09240 + iVar3 + 10) + *(short *)(DAT_00a09240 + iVar3 + 2)));
    FUN_004a9500(*(short *)(DAT_00a09240 + (unsigned int)DAT_00a08374 * 0xcu),
                 *(short *)(DAT_00a09240 + (unsigned int)DAT_00a08374 * 0xcu + 2));
    (*DAT_009d1300)();
    __builtin_memcpy(&uVar2, DAT_009e9a67 + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
    pcVar6 = DAT_00a08c70;
    if (uVar2 != 0xffff) {
      FUN_0043c290(uVar2, 3);
      pcVar6 = DAT_009a1ed0;
    }
    uVar4 = 0xffffffff;
    do {
      pcVar7 = pcVar6;
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      pcVar7 = pcVar6 + 1;
      cVar1 = *pcVar6;
      pcVar6 = pcVar7;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar6 = pcVar7 + -uVar4;
    pcVar7 = acStack_50;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *(unsigned int *)pcVar7 = *(unsigned int *)pcVar6;
      pcVar6 = pcVar6 + 4;
      pcVar7 = pcVar7 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *pcVar7 = *pcVar6;
      pcVar6 = pcVar6 + 1;
      pcVar7 = pcVar7 + 1;
    }
    FUN_004a97f0(DAT_00a0a7d8);
    FUN_004a9860(0x20);
    FUN_004a9820(acStack_50);
    FUN_004a9a50(DAT_009a1ed0);
    FUN_004abe50();
    iVar3 = DAT_009ecc34;
    uVar4 = (unsigned int)DAT_00a08374;
    __builtin_memcpy(&sA, DAT_009e9a67 + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
    *(short *)(DAT_00a0a2d0 + uVar4 * 2) = sA;
  }
  __builtin_memcpy(&sB, DAT_009e9a69 + (unsigned int)iVar3 * 0x5bdu, 2);
  if (*(short *)(DAT_00a0a2d0 + uVar4 * 2 + 2) != sB) {
    FUN_004aba50();
    FUN_0040e0c0(0, 0x140, 200, 0);
    FUN_004a9640('\x02');
    FUN_004a95c0((unsigned int)DAT_005233c8);
    FUN_004a9590(0x52);
    iVar3 = (int)((unsigned int)DAT_00a08374 * 0xcu);
    FUN_004a9520(*(short *)(DAT_00a0924c + iVar3),
                 *(short *)(DAT_00a0924c + iVar3 + 2),
                 (short)(*(short *)(DAT_00a0924c + iVar3 + 8) + *(short *)(DAT_00a0924c + iVar3)),
                 (short)(*(short *)(DAT_00a0924c + iVar3 + 10) + *(short *)(DAT_00a0924c + iVar3 + 2)));
    FUN_004a9500(*(short *)(DAT_00a0924c + (unsigned int)DAT_00a08374 * 0xcu),
                 *(short *)(DAT_00a0924c + (unsigned int)DAT_00a08374 * 0xcu + 2));
    (*DAT_009d1300)();
    pcVar6 = DAT_00a08c70;
    __builtin_memcpy(&uVar2, DAT_009e9a69 + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
    if (uVar2 != 0xffff) {
      __builtin_memcpy(&uVar2, DAT_009e9a69 + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
      FUN_0043c290(uVar2, 3);
      pcVar6 = DAT_009a1ed0;
    }
    uVar4 = 0xffffffff;
    do {
      pcVar7 = pcVar6;
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      pcVar7 = pcVar6 + 1;
      cVar1 = *pcVar6;
      pcVar6 = pcVar7;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar6 = pcVar7 + -uVar4;
    pcVar7 = acStack_50;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *(unsigned int *)pcVar7 = *(unsigned int *)pcVar6;
      pcVar6 = pcVar6 + 4;
      pcVar7 = pcVar7 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *pcVar7 = *pcVar6;
      pcVar6 = pcVar6 + 1;
      pcVar7 = pcVar7 + 1;
    }
    FUN_004a97f0(DAT_00a0a7dc);
    FUN_004a9860(0x20);
    FUN_004a9820(acStack_50);
    FUN_004a9a50(DAT_009a1ed0);
    FUN_004abe50();
    __builtin_memcpy(&sB, DAT_009e9a69 + (unsigned int)DAT_009ecc34 * 0x5bdu, 2);
    *(short *)(DAT_00a0a2d0 + (unsigned int)DAT_00a08374 * 2 + 2) = sB;
  }
  return;
}
