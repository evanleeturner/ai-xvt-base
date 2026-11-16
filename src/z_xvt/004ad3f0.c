/* SPDX-License-Identifier: MIT */
/* z_xvt 004ad3f0 */
/* Unresolved: calling conventions of FUN_* callees and the DAT_00703c80 / DAT_0070a184
   function pointers are not in the evidence; plain __cdecl is assumed. Several
   globals are treated as byte arrays and accessed at the offsets the decompile uses. */

extern ushort DAT_00655184;
extern uint   DAT_006da9ec;
extern uint   DAT_006da538;
extern uint   DAT_006db594;
extern uint   DAT_00525518;
extern int    DAT_0071ecf4;
extern uint   DAT_006d4988;
extern unsigned char DAT_006d4a4e;
extern unsigned char DAT_0071f2e2;
extern unsigned char DAT_0071e524;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b757[];
extern unsigned char DAT_0071b758[];
extern ushort DAT_00656cae[];
extern ushort DAT_00656ca6[];
extern unsigned char DAT_00655b68[];
extern unsigned char DAT_00655b64[];
extern unsigned char DAT_00655b66[];
extern unsigned char DAT_00655b34[];
extern unsigned char DAT_00655b36[];
extern unsigned char DAT_00655b38[];
extern unsigned char DAT_00655b3a[];
extern uint   DAT_0071f2f0[];
extern void (*DAT_00703c80)(void);
extern void (*DAT_0070a184)(uint, short, short, short, int);

void  __cdecl FUN_00461f20(uint a, uint b, uint c, uint d);
void  __cdecl FUN_0048ee20(char a);
void  __cdecl FUN_0048ed00(short a, short b, short c, short d);
void  __cdecl FUN_0048ece0(short a, short b);
void  __cdecl FUN_0048eda0(int a);
void  __cdecl FUN_0048ed70(uint a);
uint  __cdecl FUN_004a7aa0(uint a);
void  __cdecl FUN_0040ccc0(uint a, uint b, uint c);

void __cdecl FUN_004ad3f0(uint param_1, ushort param_2, uint param_3)
{
  unsigned char *piVar1;
  unsigned char bVar2;
  uint iVar3;
  uint iVar4;
  int bVar5;
  uint uVar6;
  uint uVar7;
  char cVar8;
  short sVar9;
  ushort uVar10;
  uint uVar11;
  uint t0;
  uint t1;

  if (DAT_00655184 != 0) {
    FUN_00461f20(DAT_006da9ec, DAT_006da538, DAT_006db594, DAT_00525518 * DAT_006da538);
    iVar4 = (uint)DAT_0071ecf4 * 0x5bd;
    __builtin_memcpy(&t0, &DAT_0071b730[iVar4], 4);
    iVar3 = t0 * 0x23;
    piVar1 = (unsigned char *)(iVar3 + 0x1f + DAT_006d4988);
    __builtin_memcpy(&t1, piVar1, 4);
    __builtin_memcpy(&iVar3, (unsigned char *)(t1 + 0xa7), 4);
    if (*(char *)(iVar3 + 0x228) == '\0') {
      uVar10 = 0;
    }
    else {
      uVar10 = (ushort)*(unsigned char *)(iVar3 + 0x346 + (param_1 & 0xffff) * 4);
    }
    DAT_00656cae[(uint)DAT_00655184 + (uint)param_2] = uVar10;
    if (uVar10 != 0) {
      __builtin_memcpy(&t1, piVar1, 4);
      __builtin_memcpy(&iVar3, (unsigned char *)(t1 + 0xa7), 4);
    }
    if ((uVar10 == 0) || ((*(unsigned char *)(iVar3 + 0x1df) & 8) == 0)) {
      cVar8 = '\0';
    }
    else if (DAT_0071b758[iVar4] == '\0') {
      cVar8 = '\x01';
    }
    else if ((param_3 & 0xffff) == (uint)DAT_0071b757[iVar4]) {
      bVar2 = *(unsigned char *)((uint)DAT_0071b757[iVar4] + 0x22b + iVar3);
      if ((bVar2 & 0x7f) == 3) {
        cVar8 = '\x02';
      }
      else {
        cVar8 = (char)(((param_2 & 1) == (ushort)(bVar2 >> 7)) + 1);
      }
    }
    else {
      cVar8 = '\x01';
    }
    if (uVar10 == 0) {
      FUN_0048ee20('\x02');
      sVar9 = (short)((uint)((uint)DAT_0071f2e2 + 1) * (uint)param_2);
      FUN_0048ed00((short)(sVar9 + 2), 2,
                   (short)(sVar9 + (ushort)DAT_0071f2e2 + 3),
                   (short)((uint)DAT_006d4a4e + 3));
      FUN_0048eda0(0);
      (*DAT_00703c80)();
    }
    else {
      FUN_0048ee20('\x02');
      FUN_0048ed00(0, 0, (short)DAT_006da538, (short)DAT_006db594);
      FUN_0048ece0((short)((uint)((uint)DAT_0071f2e2 + 1) * (uint)param_2 + 2), 2);
      FUN_0048eda0(0);
      if (cVar8 == '\x02') {
        uVar11 = 0x52;
      }
      else {
        uVar11 = 0x4a;
      }
      FUN_0048ed70(uVar11);
      DAT_0071e524 = 0;
      if (*(short *)(&DAT_00655b68[((uint)DAT_00655184 + (uint)param_2) * 0xc]) != 0) {
        FUN_0048ee20('\0');
        __builtin_memcpy(&t0, &DAT_0071b730[(uint)DAT_0071ecf4 * 0x5bd], 4);
        uVar11 = FUN_004a7aa0(((DAT_006d4988 & 0xffffff00u) |
                               (uint)*(unsigned char *)(t0 * 0x23 + 3 + DAT_006d4988))
                              & 0xffff00ffu);
        uVar6 = FUN_004a7aa0(0xc);
        if ((short)uVar11 == (short)uVar6) {
          FUN_0040ccc0(uVar10, 2, 1);
        }
        else {
          FUN_0040ccc0(uVar10, 1, 1);
        }
      }
    }
    FUN_00461f20(0, 0x140, 200, 0);
    return;
  }
  __builtin_memcpy(&t0, &DAT_0071b730[(uint)DAT_0071ecf4 * 0x5bd], 4);
  __builtin_memcpy(&t1, (unsigned char *)(t0 * 0x23 + 0x1f + DAT_006d4988), 4);
  __builtin_memcpy(&iVar3, (unsigned char *)(t1 + 0xa7), 4);
  if (*(char *)(iVar3 + 0x228) == '\0') {
    uVar10 = 0;
  }
  else {
    uVar10 = (ushort)*(unsigned char *)(iVar3 + 0x346 + (param_1 & 0xffff) * 4);
  }
  uVar11 = (uint)param_2;
  if (DAT_00656cae[uVar11] != uVar10) {
    DAT_00656cae[uVar11] = uVar10;
    FUN_0048ee20('\x02');
    FUN_0048ed00(0, 0, (short)DAT_006da538, (short)DAT_006db594);
    FUN_0048ece0(*(short *)(&DAT_00655b64[((uint)DAT_00655184 + uVar11) * 0xc]),
                 *(short *)(&DAT_00655b66[((uint)DAT_00655184 + uVar11) * 0xc]));
    FUN_0048eda0(0);
    FUN_0048ed70(0x4a);
    DAT_0071e524 = 0;
    __builtin_memcpy(&t0, &DAT_0071b730[(uint)DAT_0071ecf4 * 0x5bd], 4);
    uVar6 = FUN_004a7aa0(((DAT_006d4988 & 0xffffff00u) |
                          (uint)*(unsigned char *)(t0 * 0x23 + 3 + DAT_006d4988))
                         & 0xffff00ffu);
    uVar7 = FUN_004a7aa0(0xc);
    if ((short)uVar6 == (short)uVar7) {
      uVar6 = 2;
    }
    else {
      uVar6 = 1;
    }
    FUN_0040ccc0(uVar10, uVar6, 1);
  }
  if (uVar10 != 0) {
    __builtin_memcpy(&t0, &DAT_0071b730[(uint)DAT_0071ecf4 * 0x5bd], 4);
    __builtin_memcpy(&t1, (unsigned char *)(t0 * 0x23 + 0x1f + DAT_006d4988), 4);
    __builtin_memcpy(&iVar3, (unsigned char *)(t1 + 0xa7), 4);
    if ((*(unsigned char *)(iVar3 + 0x1df) & 8) != 0) {
      if (DAT_0071b758[(uint)DAT_0071ecf4 * 0x5bd] == '\0') {
        uVar10 = 1;
      }
      else if ((param_3 & 0xffff) == (uint)DAT_0071b757[(uint)DAT_0071ecf4 * 0x5bd]) {
        bVar2 = *(unsigned char *)((uint)DAT_0071b757[(uint)DAT_0071ecf4 * 0x5bd] + 0x22b + iVar3);
        if ((bVar2 & 0x7f) == 3) {
          uVar10 = 2;
        }
        else {
          uVar10 = (ushort)(((param_2 & 1) == (ushort)(bVar2 >> 7)) + 1);
        }
      }
      else {
        uVar10 = 1;
      }
      goto LAB_004ad8a5;
    }
  }
  uVar10 = 0;
LAB_004ad8a5:
  __builtin_memcpy(&iVar3, &DAT_0071b730[(uint)DAT_0071ecf4 * 0x5bd], 4);
  if (((int)iVar3 == -1) ||
      (((cVar8 = *(char *)(iVar3 * 0x23 + 3 + DAT_006d4988), cVar8 != '\x01') && (cVar8 != '\x02'))
       && (cVar8 != '\x03') && (cVar8 != '\x0e') && (cVar8 != '\x04'))) {
    bVar5 = 0;
  }
  else {
    bVar5 = 1;
  }
  if ((bVar5) && (uVar10 == 2)) {
    uVar10 = 4;
  }
  if (DAT_00656ca6[uVar11] != uVar10) {
    DAT_00656ca6[uVar11] = uVar10;
    iVar3 = uVar11 * 0xc;
    (*DAT_0070a184)(DAT_0071f2f0[(uint)*(ushort *)(&DAT_00655b38[iVar3]) + (uint)uVar10],
                    *(short *)(&DAT_00655b34[iVar3]),
                    *(short *)(&DAT_00655b36[iVar3]),
                    *(short *)(&DAT_00655b3a[iVar3]), 0);
  }
  return;
}
