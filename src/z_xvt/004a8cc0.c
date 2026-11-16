/* SPDX-License-Identifier: MIT */
/* z_xvt 004a8cc0 */
/* Unresolved: calling conventions of the callees other than this function are not in the evidence; plain (cdecl) declarations are assumed.
   FUN_004aeef0: the original pushes one 4-byte argument and cleans it after the call (stack-check finding), so it is cdecl with one
   4-byte parameter. The pushed value is not recoverable from the evidence, and the callee's decompile never reads it; 0 is a placeholder.
   DAT_0070a184 is a function pointer whose convention is also unresolved; it is called with 5 arguments.
   Struct layouts are not supplied, so unaligned fields are read via __builtin_memcpy. */

extern void FUN_0048ee20(char a);
extern void FUN_004a98a0(void);
extern void FUN_004ac9c0(void);
extern void FUN_004a6f90(void);
extern void FUN_0040a840(unsigned int a);
extern void FUN_004a6a80(void);
extern void FUN_004aa0d0(void);
extern void FUN_004ad120(void);
extern void FUN_004ad970(void);
extern void FUN_004ae130(void);
extern void FUN_004ae560(void);
extern void FUN_0048eda0(unsigned int a);
extern unsigned int FUN_00478320(unsigned int a, unsigned int b);
extern void FUN_004b15a0(int a, unsigned short b, int c);
extern void FUN_004ae700(void);
extern void FUN_004aeef0(unsigned int a);
extern void FUN_004af7c0(void);
extern void FUN_004b2880(void);
extern void FUN_004a2e70(void);

extern int DAT_0071ecf4;
extern unsigned char DAT_0071b758[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_0071b756[];
extern unsigned char DAT_0071b730[];
extern char DAT_00656c80;
extern short DAT_00655184;
extern unsigned short DAT_00656c90[];
extern unsigned char DAT_00655ab0[];
extern unsigned char DAT_00655ab2[];
extern unsigned char DAT_00655ab4[];
extern unsigned char DAT_00655ab6[];
extern unsigned int DAT_0071f2f0[];
extern void (*DAT_0070a184)(unsigned int, unsigned short, unsigned short, unsigned short, int);
extern int DAT_006d4988;
extern unsigned short DAT_00655c56;
extern unsigned short DAT_00655c54;
extern unsigned short DAT_00655c58;
extern unsigned short DAT_00655c5a;
extern unsigned short DAT_00656cd6;
extern unsigned short DAT_00655c0c;
extern unsigned short DAT_00655c0e;
extern unsigned short DAT_00655c10;
extern unsigned short DAT_00655c12;
extern unsigned short DAT_00656cca;

void __stdcall FUN_004a8cc0(void)
{
  unsigned char bVar1;
  int iVar2;
  unsigned int uVar3;
  unsigned short uVar4;
  int bVar5;
  char cVar6;
  short sVar7;
  int iVarA;
  int iVarB;

  FUN_0048ee20('\x02');
  FUN_004a98a0();
  FUN_004ac9c0();
  iVar2 = (int)((unsigned int)DAT_0071ecf4 * 0x5bdU);
  if (DAT_0071b758[iVar2] == '\0') {
    uVar4 = (unsigned short)(((DAT_00656c80 == '\0') - 1) & 4);
  }
  else {
    __builtin_memcpy(&sVar7, &DAT_0071b74a[iVar2], 2);
    if (sVar7 == -1) {
      uVar4 = 1;
    }
    else {
      uVar4 = (unsigned short)((unsigned int)DAT_0071b756[iVar2] + 1);
    }
    DAT_00656c80 = '\x01';
    if (DAT_0071b756[iVar2] != '\x02') {
      DAT_00656c80 = '\0';
    }
  }
  iVar2 = DAT_00655184 + 0x24;
  if (DAT_00656c90[iVar2] != uVar4) {
    DAT_00656c90[iVar2] = uVar4;
    iVar2 = (int)((unsigned int)iVar2 * 0xcU);
    (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)*(unsigned short *)(DAT_00655ab4 + iVar2) + (unsigned int)uVar4],
                    *(unsigned short *)(DAT_00655ab0 + iVar2),
                    *(unsigned short *)(DAT_00655ab2 + iVar2),
                    *(unsigned short *)(DAT_00655ab6 + iVar2), 0);
  }
  FUN_004a6f90();
  FUN_0040a840((unsigned int)uVar4);
  FUN_004a6a80();
  FUN_004aa0d0();
  FUN_004ad120();
  FUN_004ad970();
  FUN_004ae130();
  FUN_004ae560();

  __builtin_memcpy(&iVar2, &DAT_0071b730[(int)((unsigned int)DAT_0071ecf4 * 0x5bdU)], 4);
  if ((iVar2 == -1) ||
      ((((cVar6 = *(char *)(int)((unsigned int)iVar2 * 0x23U + 3U + (unsigned int)DAT_006d4988), cVar6 != '\x01') && (cVar6 != '\x02'))
        && (cVar6 != '\x03')) && ((cVar6 != '\x0e' && (cVar6 != '\x04'))))) {
    bVar5 = 0;
  }
  else {
    bVar5 = 1;
  }
  if (bVar5) {
    __builtin_memcpy(&iVarA, (void *)(int)((unsigned int)iVar2 * 0x23U + 0x1fU + (unsigned int)DAT_006d4988), 4);
    __builtin_memcpy(&iVar2, (void *)(iVarA + 0xa7), 4);
    if ((*(unsigned char *)(iVar2 + 0x1db) & 0x20) != 0) {
      if ((unsigned int)DAT_00655c56 + (unsigned int)DAT_00655c54 != 0) {
        bVar1 = *(unsigned char *)(iVar2 + 0x210);
        if ((unsigned int)DAT_00656cd6 != (unsigned int)bVar1) {
          DAT_00656cd6 = (unsigned short)bVar1;
          (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)DAT_00655c58 + (unsigned int)bVar1],
                          DAT_00655c54, DAT_00655c56, DAT_00655c5a, 0);
        }
      }
    }
  }

  __builtin_memcpy(&iVarA, &DAT_0071b730[(int)((unsigned int)DAT_0071ecf4 * 0x5bdU)], 4);
  __builtin_memcpy(&iVarB, (void *)(int)((unsigned int)iVarA * 0x23U + 0x1fU + (unsigned int)DAT_006d4988), 4);
  __builtin_memcpy(&iVar2, (void *)(iVarB + 0xa7), 4);
  if ((*(unsigned char *)(iVar2 + 0x1db) & 0x40) != 0) {
    unsigned int uIdx;
    unsigned short sv79;
    FUN_0048eda0(0x40);
    __builtin_memcpy(&iVarA, &DAT_0071b730[(int)((unsigned int)DAT_0071ecf4 * 0x5bdU)], 4);
    uIdx = (unsigned int)iVarA * 0x23U;
    __builtin_memcpy(&iVarB, (void *)(int)(uIdx + 0x1fU + (unsigned int)DAT_006d4988), 4);
    __builtin_memcpy(&sv79, (void *)(iVarB + 0x79), 2);
    uVar3 = FUN_00478320(((uIdx >> 0x10) << 0x10) | (unsigned int)sv79, 0x71c7);
    if (DAT_00655184 == 0) {
      cVar6 = '\x02';
    }
    else {
      cVar6 = '\0';
    }
    FUN_0048ee20(cVar6);
    FUN_004b15a0(DAT_00655184 + 0x18, (unsigned short)uVar3, 1);
  }

  __builtin_memcpy(&iVarA, &DAT_0071b730[(int)((unsigned int)DAT_0071ecf4 * 0x5bdU)], 4);
  __builtin_memcpy(&iVarB, (void *)(int)((unsigned int)iVarA * 0x23U + 0x1fU + (unsigned int)DAT_006d4988), 4);
  __builtin_memcpy(&iVar2, (void *)(iVarB + 0xa7), 4);
  if ((*(unsigned char *)(iVar2 + 0x1db) & 0x40) != 0) {
    unsigned short s_bd;
    unsigned short s_bf;
    FUN_0048eda0(0x40);
    __builtin_memcpy(&iVarA, &DAT_0071b730[(int)((unsigned int)DAT_0071ecf4 * 0x5bdU)], 4);
    __builtin_memcpy(&iVarB, (void *)(int)((unsigned int)iVarA * 0x23U + 0x1fU + (unsigned int)DAT_006d4988), 4);
    __builtin_memcpy(&iVar2, (void *)(iVarB + 0xa7), 4);
    __builtin_memcpy(&s_bd, (void *)(iVar2 + 0xbd), 2);
    __builtin_memcpy(&s_bf, (void *)(iVar2 + 0xbf), 2);
    uVar4 = (unsigned short)(s_bd / 0x28f);
    if (s_bf == 0) {
      uVar4 = (unsigned short)(uVar4 * 2);
    }
    if (DAT_00655184 == 0) {
      cVar6 = '\x02';
    }
    else {
      cVar6 = '\0';
    }
    FUN_0048ee20(cVar6);
    FUN_004b15a0(DAT_00655184 + 0x19, uVar4, 1);
  }

  FUN_004ae700();
  FUN_004aeef0(0);
  FUN_004af7c0();

  __builtin_memcpy(&iVar2, &DAT_0071b730[(int)((unsigned int)DAT_0071ecf4 * 0x5bdU)], 4);
  if ((iVar2 == -1) ||
      ((((cVar6 = *(char *)(int)((unsigned int)iVar2 * 0x23U + 3U + (unsigned int)DAT_006d4988), cVar6 != '\x01') && (cVar6 != '\x02'))
        && (cVar6 != '\x03')) && ((cVar6 != '\x0e' && (cVar6 != '\x04'))))) {
    bVar5 = 0;
  }
  else {
    bVar5 = 1;
  }
  if ((bVar5) && (DAT_00655c0c != 0)) {
    __builtin_memcpy(&iVarA, (void *)(int)((unsigned int)iVar2 * 0x23U + 0x1fU + (unsigned int)DAT_006d4988), 4);
    __builtin_memcpy(&iVarB, (void *)(iVarA + 0xa7), 4);
    bVar5 = (*(unsigned char *)(iVarB + 0x29) & 2) == 0;
    uVar4 = (unsigned short)bVar5;
    if (DAT_00656cca != uVar4) {
      DAT_00656cca = uVar4;
      (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)DAT_00655c10 + (unsigned int)bVar5],
                      DAT_00655c0c, DAT_00655c0e, DAT_00655c12, 0);
    }
  }
  FUN_004b2880();
  FUN_004a2e70();
  return;
}
