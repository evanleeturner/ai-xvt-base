/* SPDX-License-Identifier: MIT */
/* z_bop 0041c570 */
extern int DAT_009a1fd4;
extern int DAT_009ec604;
extern int DAT_009a1fd8;
extern int DAT_00a081e8;
extern int DAT_009a1fd0;
extern int DAT_00a08298;
extern int DAT_00a07bdc;
extern int DAT_009d80c0;
extern int DAT_00a07c54;
extern int DAT_009d8c18;
extern int DAT_00a07c58;
extern int DAT_009d1150;
extern uint DAT_009d77fc;
extern uint DAT_009d6828;
extern uint DAT_009cd260;

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

uint __cdecl FUN_0041c570(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  ushort uVar9;
  uint uVar8;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int local_1c;
  int local_18;
  int local_14;
  int local_10;
  uint local_c;
  uint local_8;
  int iVarD;
  long long prod;

  iVar1 = (int)((uint)DAT_009a1fd4 - (uint)DAT_009ec604);
  iVar2 = (int)((uint)DAT_009a1fd8 - (uint)DAT_00a081e8);
  iVar3 = (int)((uint)DAT_009a1fd0 - (uint)DAT_00a08298);
  iVar4 = (int)((uint)DAT_00a07bdc - (uint)DAT_009d80c0);
  iVar5 = (int)((uint)DAT_00a07c54 - (uint)DAT_009d8c18);
  iVar12 = (int)((uint)DAT_009d80c0 - (uint)DAT_009ec604);
  iVar6 = (int)((uint)DAT_00a07c58 - (uint)DAT_009d1150);
  if (param_1 < iVar12) {
    iVar7 = (int)((uint)param_1 - (uint)iVar12);
    local_c = (uint)((uint)iVar4 - (uint)iVar1);
    uVar9 = (ushort)((uint)iVar7 >> 0x10);
    if (-1 < (int)local_c) {
      return (uint)uVar9 << 0x10;
    }
    if (iVar7 < (int)local_c) {
      return (uint)uVar9 << 0x10;
    }
    local_8 = 0u - ((uint)iVar12 + (uint)param_1);
  }
  else if ((int)(0u - (uint)iVar12) == param_1 || (int)(0u - (uint)param_1) < iVar12) {
    local_8 = 0u - ((uint)iVar12 + (uint)param_1);
    iVar7 = (int)((uint)param_1 - (uint)iVar12);
    local_c = 0;
  }
  else {
    local_8 = 0u - ((uint)iVar12 + (uint)param_1);
    local_c = (uint)iVar4 - (uint)iVar1;
    if ((int)local_c < 0) {
      return local_c & 0xffff0000;
    }
    if ((int)local_c <= (int)local_8) {
      return local_c & 0xffff0000;
    }
    iVar7 = (int)((uint)param_1 - (uint)iVar12);
  }
  iVar12 = (int)((uint)DAT_009d8c18 - (uint)DAT_00a081e8);
  uVar9 = (ushort)((uint)iVar7 >> 0x10);
  if (param_1 < iVar12) {
    local_1c = (int)((uint)param_1 - (uint)iVar12);
    iVar13 = (int)((uint)iVar5 - (uint)iVar2);
    if (-1 < iVar13) {
      return (uint)uVar9 << 0x10;
    }
    if (local_1c < iVar13) {
      return (uint)uVar9 << 0x10;
    }
    local_14 = (int)(0u - ((uint)iVar12 + (uint)param_1));
  }
  else if ((int)(0u - (uint)iVar12) == param_1 || (int)(0u - (uint)param_1) < iVar12) {
    iVar13 = 0;
    local_1c = (int)((uint)param_1 - (uint)iVar12);
    local_14 = (int)(0u - ((uint)iVar12 + (uint)param_1));
  }
  else {
    local_14 = (int)(0u - ((uint)iVar12 + (uint)param_1));
    iVar13 = (int)((uint)iVar5 - (uint)iVar2);
    if (iVar13 < 0) {
      return (uint)uVar9 << 0x10;
    }
    if (iVar13 <= local_14) {
      return (uint)uVar9 << 0x10;
    }
    local_1c = (int)((uint)param_1 - (uint)iVar12);
  }
  iVar12 = (int)((uint)DAT_009d1150 - (uint)DAT_00a08298);
  if (param_1 < iVar12) {
    local_18 = (int)((uint)param_1 - (uint)iVar12);
    iVar11 = (int)((uint)iVar6 - (uint)iVar3);
    if (-1 < iVar11) {
      return (uint)uVar9 << 0x10;
    }
    if (local_18 < iVar11) {
      return (uint)uVar9 << 0x10;
    }
    local_10 = (int)(0u - ((uint)iVar12 + (uint)param_1));
  }
  else if ((int)(0u - (uint)iVar12) == param_1 || (int)(0u - (uint)param_1) < iVar12) {
    iVar11 = 0;
    local_18 = (int)((uint)param_1 - (uint)iVar12);
    local_10 = (int)(0u - ((uint)iVar12 + (uint)param_1));
  }
  else {
    local_10 = (int)(0u - ((uint)iVar12 + (uint)param_1));
    iVar11 = (int)((uint)iVar6 - (uint)iVar3);
    if (iVar11 < 0) {
      return (uint)uVar9 << 0x10;
    }
    if (iVar11 <= local_10) {
      return (uint)uVar9 << 0x10;
    }
    local_18 = (int)((uint)param_1 - (uint)iVar12);
  }
  local_14 = (int)((uint)local_14 << 8);
  local_10 = (int)((uint)local_10 << 8);
  if (local_c == 0) {
    iVar12 = (int)((uint)DAT_00a07bdc - (uint)DAT_009a1fd4);
    iVarD = (int)((uint)iVar4 - (uint)iVar1);
    if (param_1 < iVar12) {
      uVar10 = 0;
      if (iVarD == 0 || (iVarD == -1 && (int)((uint)iVar7 << 8) == (int)0x80000000)) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      local_8 = (uint)((int)((uint)iVar7 << 8) / iVarD);
    }
    else if ((int)(0u - (uint)iVar12) == param_1 || (int)(0u - (uint)param_1) < iVar12) {
      local_8 = 0xff;
      uVar10 = 0;
    }
    else {
      uVar10 = 0;
      if (iVarD == 0 || (iVarD == -1 && (local_8 << 8) == 0x80000000u)) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      local_8 = (uint)((int)(local_8 << 8) / iVarD);
    }
  }
  else {
    uVar8 = (uint)((int)((uint)iVar7 << 8) / (int)local_c);
    local_8 = (uint)((int)(local_8 << 8) / (int)local_c);
    uVar10 = uVar8;
    if (-1 < (int)local_c) {
      uVar10 = local_8;
      local_8 = uVar8;
    }
  }
  if (iVar13 == 0) {
    iVar4 = (int)((uint)DAT_00a07c54 - (uint)DAT_009a1fd8);
    iVarD = (int)((uint)iVar5 - (uint)iVar2);
    if (param_1 < iVar4) {
      if (iVarD == 0 || (iVarD == -1 && (int)((uint)local_1c << 8) == (int)0x80000000)) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      uVar8 = (uint)((int)((uint)local_1c << 8) / iVarD);
    }
    else if ((int)(0u - (uint)iVar4) == param_1 || (int)(0u - (uint)param_1) < iVar4) {
      uVar8 = 0xff;
    }
    else {
      if (iVarD == 0 || (iVarD == -1 && local_14 == (int)0x80000000)) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      uVar8 = (uint)(local_14 / iVarD);
    }
    uVar9 = (ushort)(uVar8 >> 0x10);
    if ((int)local_8 < 0) {
      return (uint)uVar9 << 0x10;
    }
    if ((int)uVar10 < 0) {
      uVar10 = 0;
    }
    if ((int)uVar8 < (int)uVar10) {
      return (uint)uVar9 << 0x10;
    }
  }
  else {
    uVar8 = (uint)((int)((uint)local_1c << 8) / iVar13);
    uVar9 = (ushort)(uVar8 >> 0x10);
    if (-1 < iVar13) {
      if ((int)uVar8 < (int)uVar10) {
        return (uint)uVar9 << 0x10;
      }
      if ((int)uVar8 < (int)local_8) {
        local_8 = uVar8;
      }
      uVar8 = (uint)(local_14 / iVar13);
      if ((int)local_8 < (int)uVar8) {
        return uVar8 & 0xffff0000;
      }
      if ((int)uVar10 < (int)uVar8) {
        uVar10 = uVar8;
      }
      goto LAB_0041c907;
    }
    if ((int)local_8 < (int)uVar8) {
      return (uint)uVar9 << 0x10;
    }
    if ((int)uVar10 < (int)uVar8) {
      uVar10 = uVar8;
    }
    uVar8 = (uint)(local_14 / iVar13);
    if ((int)uVar8 < (int)uVar10) {
      return uVar8 & 0xffff0000;
    }
  }
  if ((int)uVar8 < (int)local_8) {
    local_8 = uVar8;
  }
LAB_0041c907:
  if (iVar11 == 0) {
    iVar4 = (int)((uint)DAT_00a07c58 - (uint)DAT_009a1fd0);
    iVarD = (int)((uint)iVar6 - (uint)iVar3);
    if (param_1 < iVar4) {
      if (iVarD == 0 || (iVarD == -1 && (int)((uint)local_18 << 8) == (int)0x80000000)) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      uVar8 = (uint)((int)((uint)local_18 << 8) / iVarD);
    }
    else if ((int)(0u - (uint)iVar4) == param_1 || (int)(0u - (uint)param_1) < iVar4) {
      uVar8 = 0xff;
    }
    else {
      if (iVarD == 0 || (iVarD == -1 && local_10 == (int)0x80000000)) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
      }
      uVar8 = (uint)(local_10 / iVarD);
    }
    uVar9 = (ushort)(uVar8 >> 0x10);
    if ((int)local_8 < 0) {
      return (uint)uVar9 << 0x10;
    }
    if ((int)uVar10 < 0) {
      uVar10 = 0;
    }
    if ((int)uVar8 < (int)uVar10) {
      return (uint)uVar9 << 0x10;
    }
  }
  else {
    uVar8 = (uint)((int)((uint)local_18 << 8) / iVar11);
    uVar9 = (ushort)(uVar8 >> 0x10);
    if (iVar11 < 0) {
      if ((int)local_8 < (int)uVar8) {
        return (uint)uVar9 << 0x10;
      }
      if ((int)uVar10 < (int)uVar8) {
        uVar10 = uVar8;
      }
      uVar8 = (uint)(local_10 / iVar11);
      if ((int)uVar8 < (int)uVar10) {
        return uVar8 & 0xffff0000;
      }
    }
    else {
      if ((int)uVar8 < (int)uVar10) {
        return (uint)uVar9 << 0x10;
      }
      if ((int)uVar8 < (int)local_8) {
        local_8 = uVar8;
      }
      uVar8 = (uint)(local_10 / iVar11);
      if ((int)local_8 < (int)uVar8) {
        return uVar8 & 0xffff0000;
      }
      if ((int)uVar10 < (int)uVar8) {
        uVar10 = uVar8;
      }
    }
  }
  if ((int)uVar10 < 0x100) {
    uVar10 = (uint)(ushort)(uVar10 << 7);
    prod = (long long)(int)uVar10 * (long long)iVar1;
    DAT_009d77fc = (uint)prod >> 0xf | (uint)(int)((unsigned long long)prod >> 0x20) << 0x11;
    prod = (long long)(int)uVar10 * (long long)iVar2;
    DAT_009d6828 = (uint)prod >> 0xf | (uint)(int)((unsigned long long)prod >> 0x20) << 0x11;
    prod = (long long)(int)uVar10 * (long long)iVar3;
    DAT_009cd260 = (uint)prod >> 0xf | (uint)(int)((unsigned long long)prod >> 0x20) << 0x11;
    return ((DAT_009cd260 >> 0x10) & 0xffff) << 0x10 | 0xffff;
  }
  return uVar8 & 0xffff0000;
}
