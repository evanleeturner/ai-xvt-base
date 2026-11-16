/* SPDX-License-Identifier: MIT */
/* z_xvt 00402b60 */
typedef struct _iobuf FILE;

int _fscanf(FILE *, const char *, ...);
int __strnicmp(const char *, const char *, uint);
int _printf(const char *, ...);
long _ftell(FILE *);
int _fseek(FILE *, long, int);
int _fgetc(FILE *);
int __strcmpi(const char *, const char *);
int _atoi(const char *);
uint FUN_00402b60(FILE *param_1, int *param_2, undefined4 *param_3);

extern char DAT_00536820[];
extern undefined DAT_00536720[];
extern undefined *PTR_PTR_005234d8[26];
extern char s_binary_00523540[];
extern char s__256s_00523550[];
extern char DAT_00523574[];
extern char s_Node_Type_Not_Supported__ignored_00523578[];
extern char s_READ_NODE_ERROR__005235ec[];
extern char s_READ_NODE_WARNING__005235b0[];
extern char s__e__e__e_005235a4[];
extern char s__e__e_0052359c[];
extern char DAT_005235e8[];
extern char DAT_005235c8[];
extern char DAT_005235c4[];
extern char DAT_005235e0[];
extern char DAT_005235dc[];
extern char s_false_005235d4[];
extern char DAT_005235d0[];
extern char s_DEFAULT_005225a0[];

uint FUN_00402b60(FILE *param_1, int *param_2, undefined4 *param_3)
{
  undefined *puVar1;
  int bVar2;
  char cVar3;
  undefined1 uVar4;
  int iVar5;
  long lVar6;
  int iVar7;
  long _Offset;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  int *piVar11;
  int *piVar12;
  undefined **ppuVar13;
  int *piVar14;
  char *pcVar15;
  char *pcVar16;
  int *piVar17;
  char local_71, local_70, local_6f, local_6e, local_6d, local_6c, local_6b, local_6a;
  char local_69, local_68, local_67, local_66, local_65, local_64, local_63, local_62;
  char local_61, local_60, local_5f, local_5e, local_5d, local_5c, local_5b, local_5a;
  char local_59, local_58, local_57, local_56, local_55, local_54, local_53, local_52;
  char local_51, local_50, local_4f, local_4e, local_4d, local_4c, local_4b, local_4a;
  char local_49, local_48, local_47, local_46, local_45, local_44, local_43, local_42;
  char local_41, local_40, local_3f, local_3e, local_3d, local_3c, local_3b, local_3a;
  char local_39;
  uint local_38;
  int *local_34;
  int local_30;
  undefined **local_2c;
  int *local_28;
  uint local_24;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10;
  int local_c;
  int *local_8;
  int local_4;

  if (param_2 != (int *)0x0) {
    *param_3 = (undefined4)param_2;
  }
  local_38 = 0;
  iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
  if (iVar5 != 1) {
    return 0xffffffff;
  }
  local_24 = 0;
  ppuVar13 = PTR_PTR_005234d8;
  do {
    uVar8 = 0xffffffff;
    pcVar16 = DAT_00536820;
    do {
      if (uVar8 == 0) break;
      uVar8 = uVar8 - 1;
      cVar3 = *pcVar16;
      pcVar16 = pcVar16 + 1;
    } while (cVar3 != '\0');
    iVar5 = __strnicmp(DAT_00536820, *(char **)*ppuVar13, ~uVar8 - 1);
    if (iVar5 == 0) break;
    ppuVar13 = ppuVar13 + 1;
    local_24 = local_24 + 1;
  } while (ppuVar13 < (undefined **)s_binary_00523540);
  local_28 = (int *)0x0;
  if (local_24 == 8) {
    iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
    if (iVar5 != 1) {
      _printf(s_READ_NODE_ERROR__005235ec);
      return 0;
    }
    local_38 = 0xffffffff;
    pcVar16 = DAT_00536820;
    do {
      if (local_38 == 0) break;
      local_38 = local_38 - 1;
      cVar3 = *pcVar16;
      pcVar16 = pcVar16 + 1;
    } while (cVar3 != '\0');
    local_38 = ~local_38;
    if (param_2 != (int *)0x0) {
      uVar8 = 0xffffffff;
      pcVar16 = DAT_00536820;
      do {
        pcVar15 = pcVar16;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar15 = pcVar16 + 1;
        cVar3 = *pcVar16;
        pcVar16 = pcVar15;
      } while (cVar3 != '\0');
      uVar8 = ~uVar8;
      piVar11 = (int *)(pcVar15 + -uVar8);
      piVar12 = param_2;
      for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        *piVar12 = *piVar11;
        piVar11 = piVar11 + 1;
        piVar12 = piVar12 + 1;
      }
      for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *(char *)piVar12 = (char)*piVar11;
        piVar11 = (int *)((uint)piVar11 + 1);
        piVar12 = (int *)((uint)piVar12 + 1);
      }
      local_28 = param_2;
      param_2 = (int *)((uint)param_2 + local_38);
      *param_3 = (undefined4)param_2;
    }
    local_24 = local_38;
    iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
    if (iVar5 != 1) {
      _printf(s_READ_NODE_ERROR__005235ec);
      return local_24;
    }
    local_24 = 0;
    ppuVar13 = PTR_PTR_005234d8;
    do {
      uVar8 = 0xffffffff;
      pcVar16 = DAT_00536820;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar3 = *pcVar16;
        pcVar16 = pcVar16 + 1;
      } while (cVar3 != '\0');
      iVar5 = __strnicmp(DAT_00536820, *(char **)*ppuVar13, ~uVar8 - 1);
      if (iVar5 == 0) break;
      ppuVar13 = ppuVar13 + 1;
      local_24 = local_24 + 1;
    } while (ppuVar13 < (undefined **)s_binary_00523540);
  }
  if (0x19 < (int)local_24) {
    _printf(s_Node_Type_Not_Supported__ignored_00523578);
    lVar6 = _ftell(param_1);
    iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
    if ((iVar5 == 1) && ((char)(uint)local_28 == '{')) {
      _fseek(param_1, lVar6, 0);
      bVar2 = 1;
    }
    else {
      _fseek(param_1, lVar6, 0);
      bVar2 = 0;
    }
    if (bVar2) {
      do {
        iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
        if (iVar5 != 1) break;
      } while ((char)(uint)local_28 != '{');
      do {
        iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
        if (iVar5 != 1) {
          return local_38;
        }
      } while ((char)(uint)local_28 != '}');
    }
    return local_38;
  }
  piVar11 = param_2;
  if (param_2 != (int *)0x0) {
    *param_2 = (int)local_28;
    param_2[1] = local_24;
    piVar11 = param_2 + 6;
    param_2[4] = *(int *)(PTR_PTR_005234d8[local_24] + 4);
    local_8 = param_2;
  }
  piVar12 = piVar11;
  if (piVar11 != (int *)0x0) {
    local_8[5] = (int)piVar11;
    piVar12 = piVar11 + (int)((uint)*(int *)(PTR_PTR_005234d8[local_24] + 4) * 3);
    local_34 = piVar11;
  }
  local_2c = &PTR_PTR_005234d8[local_24];
  local_38 = local_38 + 0x18 + (uint)*(int *)(*local_2c + 4) * 0xc;
  if (local_24 != 7) {
    do {
      iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
      if (iVar5 != 1) break;
    } while ((char)(uint)local_28 != '{');
    iVar5 = 0;
    piVar11 = (int *)(*local_2c + 4);
    if (0 < *(int *)(*local_2c + 4)) {
      do {
        DAT_00536720[iVar5] = 0;
        iVar5 = (int)((uint)iVar5 + 1);
      } while (iVar5 < *piVar11);
    }
    local_28 = local_34;
    local_c = 0;
    if (0 < *piVar11) {
      do {
        lVar6 = _ftell(param_1);
        iVar5 = _fscanf(param_1, DAT_00523574, &local_71);
        if ((iVar5 == 1) && (local_71 == '}')) {
          _fseek(param_1, lVar6, 0);
          bVar2 = 1;
        }
        else {
          _fseek(param_1, lVar6, 0);
          bVar2 = 0;
        }
        if (bVar2) break;
        local_18 = _ftell(param_1);
        iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
        if (iVar5 != 1) {
          _printf(s_READ_NODE_ERROR__005235ec);
          return local_38;
        }
        iVar5 = 0;
        if (0 < *(int *)(*local_2c + 4)) {
          local_24 = 0;
          do {
            uVar8 = 0xffffffff;
            pcVar16 = DAT_00536820;
            do {
              if (uVar8 == 0) break;
              uVar8 = uVar8 - 1;
              cVar3 = *pcVar16;
              pcVar16 = pcVar16 + 1;
            } while (cVar3 != '\0');
            iVar7 = __strnicmp(DAT_00536820,
                               (char *)**(undefined4 **)((uint)*(int *)(*local_2c + 8) + local_24),
                               ~uVar8 - 1);
            if (iVar7 == 0) break;
            iVar5 = (int)((uint)iVar5 + 1);
            local_24 = local_24 + 4;
          } while (iVar5 < *(int *)(*local_2c + 4));
        }
        puVar1 = *local_2c;
        if (*(int *)(puVar1 + 4) == iVar5) {
          _fseek(param_1, local_18, 0);
          break;
        }
        DAT_00536720[iVar5] = 1;
        local_20 = (int)((uint)iVar5 * 4);
        local_34 = local_28 + (int)((uint)iVar5 * 3);
        switch (*(undefined4 *)((uint)*(int *)((uint)*(int *)(puVar1 + 8) + (uint)local_20) + 4)) {
        case 1:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_70);
          if ((iVar5 == 1) && (local_70 == '\"')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_6f);
              if (iVar5 != 1) break;
            } while (local_6f != '\"');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
              local_34[1] = 1;
              *local_34 = 1;
            }
            while (1) {
              iVar5 = _fgetc(param_1);
              cVar3 = (char)iVar5;
              if (cVar3 == -1) {
                _printf(s_READ_NODE_ERROR__005235ec);
                return local_38;
              }
              local_38 = local_38 + 1;
              if (cVar3 == '\"') break;
              if (piVar12 != (int *)0x0) {
                *(char *)piVar12 = cVar3;
                piVar12 = (int *)((uint)piVar12 + 1);
              }
            }
            if (piVar12 != (int *)0x0) {
              *(char *)piVar12 = '\0';
              piVar12 = (int *)((uint)piVar12 + 1);
            }
          }
          else if (piVar12 == (int *)0x0) {
            iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
            if (iVar5 != 1) {
              _printf(s_READ_NODE_ERROR__005235ec);
              return local_38;
            }
            uVar8 = 0xffffffff;
            pcVar16 = DAT_00536820;
            do {
              if (uVar8 == 0) break;
              uVar8 = uVar8 - 1;
              cVar3 = *pcVar16;
              pcVar16 = pcVar16 + 1;
            } while (cVar3 != '\0');
            local_38 = local_38 + ~uVar8;
          }
          else {
            local_34[2] = (int)piVar12;
            local_34[1] = 1;
            *local_34 = 1;
            iVar5 = _fscanf(param_1, DAT_005235e8, piVar12);
            if (iVar5 != 1) {
              _printf(s_READ_NODE_ERROR__005235ec);
              return local_38;
            }
            uVar8 = 0xffffffff;
            piVar11 = piVar12;
            do {
              if (uVar8 == 0) break;
              uVar8 = uVar8 - 1;
              iVar5 = *piVar11;
              piVar11 = (int *)((uint)piVar11 + 1);
            } while ((char)iVar5 != '\0');
            piVar12 = (int *)((uint)piVar12 + ~uVar8);
            local_38 = local_38 + ~uVar8;
          }
          break;
        case 3:
          if (piVar12 == (int *)0x0) {
            uVar8 = FUN_00402b60(param_1, (int *)0x0, (undefined4 *)0x0);
            local_38 = local_38 + uVar8;
          }
          else {
            uVar8 = FUN_00402b60(param_1, piVar12, (undefined4 *)(local_34 + 2));
            local_38 = local_38 + uVar8;
            local_34[1] = 1;
            piVar12 = (int *)((uint)piVar12 + uVar8);
            *local_34 = 3;
          }
          break;
        case 4:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_6e);
          if ((iVar5 == 1) && (local_6e == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_6d);
              if (iVar5 != 1) break;
            } while (local_6d != '[');
            iVar5 = 0;
            local_18 = _ftell(param_1);
LAB_00403310:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_6c);
            if ((iVar7 == 1) && (local_6c == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              uVar8 = FUN_00402b60(param_1, (int *)0x0, (undefined4 *)0x0);
              if (uVar8 != 0) {
                iVar5 = (int)((uint)iVar5 + 1);
                lVar6 = _ftell(param_1);
                iVar7 = _fscanf(param_1, DAT_00523574, &local_6b);
                if ((iVar7 == 1) && (local_6b == ']')) {
                  _fseek(param_1, lVar6, 0);
                  bVar2 = 1;
                }
                else {
                  _fseek(param_1, lVar6, 0);
                  bVar2 = 0;
                }
                if (!bVar2) {
                  do {
                    iVar7 = _fscanf(param_1, DAT_00523574, &local_6a);
                    if (iVar7 != 1) break;
                  } while (local_6a != ',');
                }
              }
              goto LAB_00403310;
            }
            _fseek(param_1, local_18, 0);
            local_38 = local_38 + (uint)iVar5 * 4;
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
              piVar12 = piVar12 + iVar5;
              local_34[1] = iVar5;
              *local_34 = 4;
            }
            iVar5 = 0;
LAB_00403425:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_69);
            if ((iVar7 == 1) && (local_69 == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              if (piVar12 == (int *)0x0) {
                uVar8 = FUN_00402b60(param_1, (int *)0x0, (undefined4 *)0x0);
                if (uVar8 != 0) {
                  local_38 = local_38 + uVar8;
                  lVar6 = _ftell(param_1);
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_66);
                  if ((iVar7 == 1) && (local_66 == ']')) {
                    _fseek(param_1, lVar6, 0);
                    bVar2 = 1;
                  }
                  else {
                    _fseek(param_1, lVar6, 0);
                    bVar2 = 0;
                  }
                  if (!bVar2) {
                    do {
                      iVar7 = _fscanf(param_1, DAT_00523574, &local_65);
                      if (iVar7 != 1) break;
                    } while (local_65 != ',');
                  }
                }
              }
              else {
                uVar8 = FUN_00402b60(param_1, piVar12,
                                     (undefined4 *)((uint)local_34[2] + (uint)iVar5 * 4));
                if (uVar8 != 0) {
                  piVar12 = (int *)((uint)piVar12 + uVar8);
                  local_38 = local_38 + uVar8;
                  iVar5 = (int)((uint)iVar5 + 1);
                  lVar6 = _ftell(param_1);
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_68);
                  if ((iVar7 == 1) && (local_68 == ']')) {
                    _fseek(param_1, lVar6, 0);
                    bVar2 = 1;
                  }
                  else {
                    _fseek(param_1, lVar6, 0);
                    bVar2 = 0;
                  }
                  if (!bVar2) {
                    do {
                      iVar7 = _fscanf(param_1, DAT_00523574, &local_67);
                      if (iVar7 != 1) break;
                    } while (local_67 != ',');
                  }
                }
              }
              goto LAB_00403425;
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_64);
              if (iVar5 != 1) break;
            } while (local_64 != ']');
          }
          else {
            local_38 = local_38 + 4;
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
              piVar12 = piVar12 + 1;
              local_34[1] = 1;
              *local_34 = 4;
              if (piVar12 != (int *)0x0) {
                uVar8 = FUN_00402b60(param_1, piVar12, (undefined4 *)((uint)local_34[2] + 4));
                local_38 = local_38 + uVar8;
                piVar12 = (int *)((uint)piVar12 + uVar8);
                break;
              }
            }
            uVar8 = FUN_00402b60(param_1, (int *)0x0, (undefined4 *)0x0);
            local_38 = local_38 + uVar8;
          }
          break;
        case 5:
          iVar5 = _fscanf(param_1, DAT_005235c8, &local_30);
          if (iVar5 != 1) {
            _printf(s_READ_NODE_ERROR__005235ec);
            return local_38;
          }
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            local_34[1] = 1;
            *local_34 = 5;
            *piVar12 = local_30;
            piVar12 = piVar12 + 1;
          }
          local_38 = local_38 + 4;
          break;
        case 6:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_63);
          if ((iVar5 == 1) && (local_63 == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_62);
              if (iVar5 != 1) break;
            } while (local_62 != '[');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = 0;
LAB_00403711:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_61);
            if ((iVar7 == 1) && (local_61 == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              iVar7 = _fscanf(param_1, DAT_005235c8, &local_30);
              if (iVar7 != 1) {
                _printf(s_READ_NODE_ERROR__005235ec);
                return local_38;
              }
              if (piVar12 != (int *)0x0) {
                *piVar12 = local_30;
                piVar12 = piVar12 + 1;
              }
              iVar5 = (int)((uint)iVar5 + 1);
              local_38 = local_38 + 4;
              lVar6 = _ftell(param_1);
              iVar7 = _fscanf(param_1, DAT_00523574, &local_60);
              if ((iVar7 == 1) && (local_60 == ']')) {
                _fseek(param_1, lVar6, 0);
                bVar2 = 1;
              }
              else {
                _fseek(param_1, lVar6, 0);
                bVar2 = 0;
              }
              if (!bVar2) {
                do {
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_5f);
                  if (iVar7 != 1) break;
                } while (local_5f != ',');
              }
              goto LAB_00403711;
            }
            if (piVar12 != (int *)0x0) {
              local_34[1] = iVar5;
              *local_34 = 6;
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_5e);
              if (iVar5 != 1) break;
            } while (local_5e != ']');
          }
          else {
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = _fscanf(param_1, DAT_005235c8, &local_30);
            if (iVar5 != 1) {
              _printf(s_READ_NODE_ERROR__005235ec);
              return local_38;
            }
            if (piVar12 != (int *)0x0) {
              *piVar12 = local_30;
              piVar12 = piVar12 + 1;
            }
            local_38 = local_38 + 4;
            if (piVar12 != (int *)0x0) {
              local_34[1] = 1;
              *local_34 = 6;
            }
          }
          break;
        case 7:
          iVar5 = _fscanf(param_1, DAT_005235c4, &local_1c);
          if (iVar5 != 1) {
            _printf(s_READ_NODE_WARNING__005235b0);
          }
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            local_34[1] = 0;
            *local_34 = 5;
            *piVar12 = local_1c;
            piVar12 = piVar12 + 1;
          }
          local_38 = local_38 + 4;
          break;
        case 8:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_5d);
          if ((iVar5 == 1) && (local_5d == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_5c);
              if (iVar5 != 1) break;
            } while (local_5c != '[');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = 0;
LAB_0040398b:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_5b);
            if ((iVar7 == 1) && (local_5b == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              iVar7 = _fscanf(param_1, DAT_005235c4, &local_1c);
              if (iVar7 != 1) {
                _printf(s_READ_NODE_WARNING__005235b0);
              }
              if (piVar12 != (int *)0x0) {
                *piVar12 = local_1c;
                piVar12 = piVar12 + 1;
              }
              iVar5 = (int)((uint)iVar5 + 1);
              local_38 = local_38 + 4;
              lVar6 = _ftell(param_1);
              iVar7 = _fscanf(param_1, DAT_00523574, &local_5a);
              if ((iVar7 == 1) && (local_5a == ']')) {
                _fseek(param_1, lVar6, 0);
                bVar2 = 1;
              }
              else {
                _fseek(param_1, lVar6, 0);
                bVar2 = 0;
              }
              if (!bVar2) {
                do {
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_59);
                  if (iVar7 != 1) break;
                } while (local_59 != ',');
              }
              goto LAB_0040398b;
            }
            if (piVar12 != (int *)0x0) {
              local_34[1] = iVar5;
              *local_34 = 8;
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_58);
              if (iVar5 != 1) break;
            } while (local_58 != ']');
          }
          else {
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = _fscanf(param_1, DAT_005235c4, &local_1c);
            if (iVar5 != 1) {
              _printf(s_READ_NODE_WARNING__005235b0);
            }
            if (piVar12 != (int *)0x0) {
              *piVar12 = local_1c;
              piVar12 = piVar12 + 1;
            }
            local_38 = local_38 + 4;
            if (piVar12 != (int *)0x0) {
              local_34[1] = 1;
              *local_34 = 8;
            }
          }
          break;
        case 9:
          iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
          if (iVar5 != 1) {
            _printf(s_READ_NODE_ERROR__005235ec);
            return local_38;
          }
          iVar5 = __strcmpi(DAT_00536820, DAT_005235e0);
          if ((iVar5 == 0) || (iVar5 = __strcmpi(DAT_00536820, DAT_005235dc), iVar5 == 0)) {
            uVar4 = 1;
          }
          else {
            iVar5 = __strcmpi(DAT_00536820, s_false_005235d4);
            if ((iVar5 != 0) && (iVar5 = __strcmpi(DAT_00536820, DAT_005235d0), iVar5 != 0)) {
              _printf(s_READ_NODE_ERROR__005235ec);
              return local_38;
            }
            uVar4 = 0;
          }
          local_38 = local_38 + 1;
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            local_34[1] = 1;
            *local_34 = 9;
            *(undefined1 *)piVar12 = uVar4;
            piVar12 = (int *)((uint)piVar12 + 1);
          }
          break;
        case 0xb:
        case 0xd:
          iVar5 = _fscanf(param_1, s__e__e__e_005235a4, &local_14, &local_10, &local_4);
          if (iVar5 != 3) {
            _printf(s_READ_NODE_WARNING__005235b0);
          }
          piVar11 = piVar12;
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            local_34[1] = 1;
            *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            piVar11 = piVar12 + 3;
            *piVar12 = local_14;
            piVar12[1] = local_10;
            piVar12[2] = local_4;
          }
          local_38 = local_38 + 0xc;
          piVar12 = piVar11;
          break;
        case 0xc:
        case 0xe:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_57);
          if ((iVar5 == 1) && (local_57 == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (!bVar2) {
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = _fscanf(param_1, s__e__e__e_005235a4, &local_14, &local_10, &local_4);
            if (iVar5 != 3) {
              _printf(s_READ_NODE_WARNING__005235b0);
            }
            piVar11 = piVar12;
            if (piVar12 != (int *)0x0) {
              *piVar12 = local_14;
              piVar11 = piVar12 + 3;
              piVar12[1] = local_10;
              piVar12[2] = local_4;
            }
            local_38 = local_38 + 0xc;
            piVar12 = piVar11;
            if (piVar11 != (int *)0x0) {
              local_34[1] = 1;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
          }
          else {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_56);
              if (iVar5 != 1) break;
            } while (local_56 != '[');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = 0;
LAB_00403d30:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_55);
            if ((iVar7 == 1) && (local_55 == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              iVar7 = _fscanf(param_1, s__e__e__e_005235a4, &local_14, &local_10, &local_4);
              if (iVar7 != 3) {
                _printf(s_READ_NODE_WARNING__005235b0);
              }
              piVar11 = piVar12;
              if (piVar12 != (int *)0x0) {
                *piVar12 = local_14;
                piVar11 = piVar12 + 3;
                piVar12[1] = local_10;
                piVar12[2] = local_4;
              }
              iVar5 = (int)((uint)iVar5 + 1);
              local_38 = local_38 + 0xc;
              lVar6 = _ftell(param_1);
              iVar7 = _fscanf(param_1, DAT_00523574, &local_54);
              if ((iVar7 == 1) && (local_54 == ']')) {
                _fseek(param_1, lVar6, 0);
                bVar2 = 1;
              }
              else {
                _fseek(param_1, lVar6, 0);
                bVar2 = 0;
              }
              piVar12 = piVar11;
              if (!bVar2) {
                do {
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_53);
                  if (iVar7 != 1) break;
                } while (local_53 != ',');
              }
              goto LAB_00403d30;
            }
            if (piVar12 != (int *)0x0) {
              local_34[1] = iVar5;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_52);
              if (iVar5 != 1) break;
            } while (local_52 != ']');
          }
          break;
        case 0xf:
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            local_34[1] = 1;
            *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
          }
          iVar5 = 0x10;
          local_38 = local_38 + 0x40;
          do {
            iVar7 = _fscanf(param_1, DAT_005235c4, &local_1c);
            if (iVar7 != 1) {
              _printf(s_READ_NODE_WARNING__005235b0);
            }
            if (piVar12 != (int *)0x0) {
              *piVar12 = local_1c;
              piVar12 = piVar12 + 1;
            }
            iVar5 = (int)((uint)iVar5 - 1);
          } while (iVar5 != 0);
          break;
        case 0x10:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_51);
          if ((iVar5 == 1) && (local_51 == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_50);
              if (iVar5 != 1) break;
            } while (local_50 != '[');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = 0;
LAB_0040403a:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_4f);
            if ((iVar7 == 1) && (local_4f == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              local_30 = 0;
              do {
                iVar7 = _fscanf(param_1, DAT_005235c4, &local_1c);
                if (iVar7 != 1) {
                  _printf(s_READ_NODE_WARNING__005235b0);
                }
                if (piVar12 != (int *)0x0) {
                  *piVar12 = local_1c;
                  piVar12 = piVar12 + 1;
                }
                local_38 = local_38 + 4;
                local_30 = (int)((uint)local_30 + 1);
              } while (local_30 < 0x10);
              iVar5 = (int)((uint)iVar5 + 1);
              lVar6 = _ftell(param_1);
              iVar7 = _fscanf(param_1, DAT_00523574, &local_4e);
              if ((iVar7 == 1) && (local_4e == ']')) {
                _fseek(param_1, lVar6, 0);
                bVar2 = 1;
              }
              else {
                _fseek(param_1, lVar6, 0);
                bVar2 = 0;
              }
              if (!bVar2) {
                do {
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_4d);
                  if (iVar7 != 1) break;
                } while (local_4d != ',');
              }
              goto LAB_0040403a;
            }
            if (piVar12 != (int *)0x0) {
              local_34[1] = iVar5;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_4c);
              if (iVar5 != 1) break;
            } while (local_4c != ']');
          }
          else {
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            local_30 = 0;
            do {
              iVar5 = _fscanf(param_1, DAT_005235c4, &local_1c);
              if (iVar5 != 1) {
                _printf(s_READ_NODE_WARNING__005235b0);
              }
              if (piVar12 != (int *)0x0) {
                *piVar12 = local_1c;
                piVar12 = piVar12 + 1;
              }
              local_38 = local_38 + 4;
              local_30 = (int)((uint)local_30 + 1);
            } while (local_30 < 0x10);
            if (piVar12 != (int *)0x0) {
              local_34[1] = 1;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
          }
          break;
        case 0x11:
          iVar5 = _fscanf(param_1, s__e__e_0052359c, &local_14, &local_10, &local_4);
          if (iVar5 != 2) {
            _printf(s_READ_NODE_WARNING__005235b0);
          }
          piVar11 = piVar12;
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            local_34[1] = 1;
            piVar11 = piVar12 + 2;
            *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            *piVar12 = local_14;
            piVar12[1] = local_10;
          }
          local_38 = local_38 + 8;
          piVar12 = piVar11;
          break;
        case 0x12:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_4b);
          if ((iVar5 == 1) && (local_4b == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (!bVar2) {
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = _fscanf(param_1, s__e__e_0052359c, &local_14, &local_10);
            if (iVar5 != 2) {
              _printf(s_READ_NODE_WARNING__005235b0);
            }
            piVar11 = piVar12;
            if (piVar12 != (int *)0x0) {
              *piVar12 = local_14;
              piVar11 = piVar12 + 2;
              piVar12[1] = local_10;
            }
            local_38 = local_38 + 8;
            piVar12 = piVar11;
            if (piVar11 != (int *)0x0) {
              local_34[1] = 1;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
          }
          else {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_4a);
              if (iVar5 != 1) break;
            } while (local_4a != '[');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = 0;
LAB_0040433b:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_49);
            if ((iVar7 == 1) && (local_49 == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              iVar7 = _fscanf(param_1, s__e__e_0052359c, &local_14, &local_10);
              if (iVar7 != 2) {
                _printf(s_READ_NODE_WARNING__005235b0);
              }
              piVar11 = piVar12;
              if (piVar12 != (int *)0x0) {
                *piVar12 = local_14;
                piVar11 = piVar12 + 2;
                piVar12[1] = local_10;
              }
              iVar5 = (int)((uint)iVar5 + 1);
              local_38 = local_38 + 8;
              lVar6 = _ftell(param_1);
              iVar7 = _fscanf(param_1, DAT_00523574, &local_48);
              if ((iVar7 == 1) && (local_48 == ']')) {
                _fseek(param_1, lVar6, 0);
                bVar2 = 1;
              }
              else {
                _fseek(param_1, lVar6, 0);
                bVar2 = 0;
              }
              piVar12 = piVar11;
              if (!bVar2) {
                do {
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_47);
                  if (iVar7 != 1) break;
                } while (local_47 != ',');
              }
              goto LAB_0040433b;
            }
            if (piVar12 != (int *)0x0) {
              local_34[1] = iVar5;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_46);
              if (iVar5 != 1) break;
            } while (local_46 != ']');
          }
          break;
        case 0x13:
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            local_34[1] = 1;
            *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
          }
          iVar5 = 4;
          local_38 = local_38 + 0x10;
          do {
            iVar7 = _fscanf(param_1, DAT_005235c4, &local_1c);
            if (iVar7 != 1) {
              _printf(s_READ_NODE_WARNING__005235b0);
            }
            if (piVar12 != (int *)0x0) {
              *piVar12 = local_1c;
              piVar12 = piVar12 + 1;
            }
            iVar5 = (int)((uint)iVar5 - 1);
          } while (iVar5 != 0);
          break;
        case 0x14:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_45);
          if ((iVar5 == 1) && (local_45 == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_44);
              if (iVar5 != 1) break;
            } while (local_44 != '[');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = 0;
LAB_0040462e:
            lVar6 = _ftell(param_1);
            iVar7 = _fscanf(param_1, DAT_00523574, &local_43);
            if ((iVar7 == 1) && (local_43 == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              local_30 = 0;
              do {
                iVar7 = _fscanf(param_1, DAT_005235c4, &local_1c);
                if (iVar7 != 1) {
                  _printf(s_READ_NODE_WARNING__005235b0);
                }
                if (piVar12 != (int *)0x0) {
                  *piVar12 = local_1c;
                  piVar12 = piVar12 + 1;
                }
                local_38 = local_38 + 4;
                local_30 = (int)((uint)local_30 + 1);
              } while (local_30 < 4);
              iVar5 = (int)((uint)iVar5 + 1);
              lVar6 = _ftell(param_1);
              iVar7 = _fscanf(param_1, DAT_00523574, &local_42);
              if ((iVar7 == 1) && (local_42 == ']')) {
                _fseek(param_1, lVar6, 0);
                bVar2 = 1;
              }
              else {
                _fseek(param_1, lVar6, 0);
                bVar2 = 0;
              }
              if (!bVar2) {
                do {
                  iVar7 = _fscanf(param_1, DAT_00523574, &local_41);
                  if (iVar7 != 1) break;
                } while (local_41 != ',');
              }
              goto LAB_0040462e;
            }
            if (piVar12 != (int *)0x0) {
              local_34[1] = iVar5;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_40);
              if (iVar5 != 1) break;
            } while (local_40 != ']');
          }
          else {
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            local_30 = 0;
            do {
              iVar5 = _fscanf(param_1, DAT_005235c4, &local_1c);
              if (iVar5 != 1) {
                _printf(s_READ_NODE_WARNING__005235b0);
              }
              if (piVar12 != (int *)0x0) {
                *piVar12 = local_1c;
                piVar12 = piVar12 + 1;
              }
              local_38 = local_38 + 4;
              local_30 = (int)((uint)local_30 + 1);
            } while (local_30 < 4);
            if (piVar12 != (int *)0x0) {
              local_34[1] = 1;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
          }
          break;
        case 0x15:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_3f);
          if ((iVar5 == 1) && (local_3f == '}')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            DAT_00536820[0] = s_DEFAULT_005225a0[0];
            DAT_00536820[1] = s_DEFAULT_005225a0[1];
            DAT_00536820[2] = s_DEFAULT_005225a0[2];
            DAT_00536820[3] = s_DEFAULT_005225a0[3];
            DAT_00536820[4] = s_DEFAULT_005225a0[4];
            DAT_00536820[5] = s_DEFAULT_005225a0[5];
            DAT_00536820[6] = s_DEFAULT_005225a0[6];
            DAT_00536820[7] = s_DEFAULT_005225a0[7];
          }
          else {
            iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
            if (iVar5 != 1) {
              _printf(s_READ_NODE_ERROR__005235ec);
              return local_38;
            }
          }
          piVar11 = *(int **)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 8);
          iVar5 = piVar11[1];
          local_24 = piVar11[2];
          local_30 = 0;
          if (0 < *piVar11) {
            do {
              iVar7 = __strcmpi(DAT_00536820, *(char **)((uint)iVar5 + (uint)local_30 * 4));
              if (iVar7 == 0) break;
              local_30 = (int)((uint)local_30 + 1);
            } while (local_30 < *piVar11);
          }
          if (*piVar11 == local_30) {
            local_30 = _atoi(DAT_00536820);
          }
          else {
            local_30 = *(int *)(local_24 + (uint)local_30 * 4);
          }
          piVar11 = piVar12;
          if (piVar12 != (int *)0x0) {
            local_34[2] = (int)piVar12;
            piVar11 = piVar12 + 1;
            local_34[1] = 1;
            *local_34 = 0x15;
            *piVar12 = local_30;
          }
          local_38 = local_38 + 4;
          piVar12 = piVar11;
          break;
        case 0x16:
          lVar6 = _ftell(param_1);
          iVar5 = _fscanf(param_1, DAT_00523574, &local_3e);
          if ((iVar5 == 1) && (local_3e == '[')) {
            _fseek(param_1, lVar6, 0);
            bVar2 = 1;
          }
          else {
            _fseek(param_1, lVar6, 0);
            bVar2 = 0;
          }
          if (bVar2) {
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_3d);
              if (iVar5 != 1) break;
            } while (local_3d != '[');
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            local_18 = 0;
LAB_00404a4f:
            lVar6 = _ftell(param_1);
            iVar5 = _fscanf(param_1, DAT_00523574, &local_3c);
            if ((iVar5 == 1) && (local_3c == ']')) {
              _fseek(param_1, lVar6, 0);
              bVar2 = 1;
            }
            else {
              _fseek(param_1, lVar6, 0);
              bVar2 = 0;
            }
            if (!bVar2) {
              iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
              if (iVar5 != 1) {
                _printf(s_READ_NODE_ERROR__005235ec);
                return local_38;
              }
              piVar11 = *(int **)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 8);
              iVar5 = piVar11[1];
              local_24 = piVar11[2];
              local_30 = 0;
              if (0 < *piVar11) {
                do {
                  iVar7 = __strcmpi(DAT_00536820, *(char **)((uint)iVar5 + (uint)local_30 * 4));
                  if (iVar7 == 0) break;
                  local_30 = (int)((uint)local_30 + 1);
                } while (local_30 < *piVar11);
              }
              if (*piVar11 == local_30) {
                local_30 = _atoi(DAT_00536820);
              }
              else {
                local_30 = *(int *)(local_24 + (uint)local_30 * 4);
              }
              if (piVar12 != (int *)0x0) {
                *piVar12 = local_30;
                piVar12 = piVar12 + 1;
              }
              local_38 = local_38 + 4;
              local_18 = (int)((uint)local_18 + 1);
              lVar6 = _ftell(param_1);
              iVar5 = _fscanf(param_1, DAT_00523574, &local_3b);
              if ((iVar5 == 1) && (local_3b == ']')) {
                _fseek(param_1, lVar6, 0);
                bVar2 = 1;
              }
              else {
                _fseek(param_1, lVar6, 0);
                bVar2 = 0;
              }
              if (!bVar2) {
                do {
                  iVar5 = _fscanf(param_1, DAT_00523574, &local_3a);
                  if (iVar5 != 1) break;
                } while (local_3a != ',');
              }
              goto LAB_00404a4f;
            }
            if (piVar12 != (int *)0x0) {
              local_34[1] = local_18;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
            do {
              iVar5 = _fscanf(param_1, DAT_00523574, &local_39);
              if (iVar5 != 1) break;
            } while (local_39 != ']');
          }
          else {
            if (piVar12 != (int *)0x0) {
              local_34[2] = (int)piVar12;
            }
            iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
            if (iVar5 != 1) {
              _printf(s_READ_NODE_ERROR__005235ec);
              return local_38;
            }
            piVar11 = *(int **)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 8);
            iVar5 = piVar11[1];
            local_24 = piVar11[2];
            local_30 = 0;
            if (0 < *piVar11) {
              do {
                iVar7 = __strcmpi(DAT_00536820, *(char **)((uint)iVar5 + (uint)local_30 * 4));
                if (iVar7 == 0) break;
                local_30 = (int)((uint)local_30 + 1);
              } while (local_30 < *piVar11);
            }
            if (*piVar11 == local_30) {
              local_30 = _atoi(DAT_00536820);
            }
            else {
              local_30 = *(int *)(local_24 + (uint)local_30 * 4);
            }
            if (piVar12 != (int *)0x0) {
              *piVar12 = local_30;
              piVar12 = piVar12 + 1;
            }
            local_38 = local_38 + 4;
            if (piVar12 != (int *)0x0) {
              local_34[1] = 1;
              *local_34 = *(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)local_20) + 4);
            }
          }
        }
        local_c = (int)((uint)local_c + 1);
      } while (local_c < *(int *)(*local_2c + 4));
    }
    local_c = 0;
    if (0 < *(int *)(*local_2c + 4)) {
      iVar5 = 0;
      piVar11 = local_28;
      do {
        if (DAT_00536720[local_c] == '\0') {
          if (piVar12 != (int *)0x0) {
            piVar14 = *(int **)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)iVar5) + 0xc);
            *piVar11 = *piVar14;
            piVar11[1] = piVar14[1];
            piVar11[2] = (int)piVar12;
            local_28 = *(int **)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)iVar5) + 0x14);
            piVar14 = *(int **)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)iVar5) + 0x10);
            piVar17 = piVar12;
            for (uVar8 = (uint)local_28 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
              *piVar17 = *piVar14;
              piVar14 = piVar14 + 1;
              piVar17 = piVar17 + 1;
            }
            for (uVar8 = (uint)local_28 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
              *(char *)piVar17 = (char)*piVar14;
              piVar14 = (int *)((uint)piVar14 + 1);
              piVar17 = (int *)((uint)piVar17 + 1);
            }
            piVar12 = (int *)((uint)piVar12 +
                              (uint)*(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)iVar5) + 0x14));
          }
          local_38 = local_38 +
                     (uint)*(int *)((uint)*(int *)((uint)*(int *)(*local_2c + 8) + (uint)iVar5) + 0x14);
        }
        local_c = (int)((uint)local_c + 1);
        piVar11 = piVar11 + 3;
        iVar5 = (int)((uint)iVar5 + 4);
      } while (local_c < *(int *)(*local_2c + 4));
    }
    lVar6 = _ftell(param_1);
    local_18 = 0;
    while (1) {
      _Offset = _ftell(param_1);
      iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
      if ((iVar5 == 1) && ((char)(uint)local_28 == '}')) {
        _fseek(param_1, _Offset, 0);
        bVar2 = 1;
      }
      else {
        _fseek(param_1, _Offset, 0);
        bVar2 = 0;
      }
      if (bVar2) break;
      uVar8 = FUN_00402b60(param_1, (int *)0x0, (undefined4 *)0x0);
      if (uVar8 != 0) {
        local_18 = (int)((uint)local_18 + 1);
      }
    }
    _fseek(param_1, lVar6, 0);
    if (local_18 == 0) {
      if (piVar12 != (int *)0x0) {
        local_8[3] = 0;
        local_8[2] = 0;
      }
    }
    else {
      local_38 = local_38 + (uint)local_18 * 4;
      if (piVar12 == (int *)0x0) {
        piVar11 = (int *)0x0;
        piVar12 = local_28;
      }
      else {
        local_8[3] = (int)piVar12;
        piVar11 = piVar12 + local_18;
        local_8[2] = local_18;
      }
      while (1) {
        lVar6 = _ftell(param_1);
        iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
        if ((iVar5 == 1) && ((char)(uint)local_28 == '}')) {
          _fseek(param_1, lVar6, 0);
          bVar2 = 1;
        }
        else {
          _fseek(param_1, lVar6, 0);
          bVar2 = 0;
        }
        if (bVar2) break;
        if (piVar11 == (int *)0x0) {
          uVar8 = FUN_00402b60(param_1, (int *)0x0, (undefined4 *)0x0);
          if (uVar8 != 0) {
            local_38 = local_38 + uVar8;
          }
        }
        else {
          uVar8 = FUN_00402b60(param_1, piVar11, (undefined4 *)piVar12);
          if (uVar8 != 0) {
            piVar11 = (int *)((uint)piVar11 + uVar8);
            local_38 = local_38 + uVar8;
            piVar12 = piVar12 + 1;
          }
        }
      }
    }
    do {
      iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
      if (iVar5 != 1) {
        return local_38;
      }
    } while ((char)(uint)local_28 != '}');
    return local_38;
  }
  lVar6 = _ftell(param_1);
  iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
  if ((iVar5 == 1) && ((char)(uint)local_28 == '\"')) {
    _fseek(param_1, lVar6, 0);
    bVar2 = 1;
  }
  else {
    _fseek(param_1, lVar6, 0);
    bVar2 = 0;
  }
  if (bVar2) {
    do {
      iVar5 = _fscanf(param_1, DAT_00523574, &local_28);
      if (iVar5 != 1) break;
    } while ((char)(uint)local_28 != '\"');
    if (piVar12 != (int *)0x0) {
      local_34[2] = (int)piVar12;
      local_34[1] = 1;
      *local_34 = 1;
    }
    while (1) {
      iVar5 = _fgetc(param_1);
      cVar3 = (char)iVar5;
      if (cVar3 == -1) {
        _printf(s_READ_NODE_ERROR__005235ec);
        return local_38;
      }
      local_38 = local_38 + 1;
      if (cVar3 == '\"') break;
      if (piVar12 != (int *)0x0) {
        *(char *)piVar12 = cVar3;
        piVar12 = (int *)((uint)piVar12 + 1);
      }
    }
    if (piVar12 != (int *)0x0) {
      *(char *)piVar12 = '\0';
      piVar12 = (int *)((uint)piVar12 + 1);
    }
  }
  else {
    iVar5 = _fscanf(param_1, s__256s_00523550, DAT_00536820);
    if (iVar5 != 1) {
      _printf(s_READ_NODE_ERROR__005235ec);
      return local_38;
    }
    uVar8 = 0xffffffff;
    pcVar16 = DAT_00536820;
    do {
      if (uVar8 == 0) break;
      uVar8 = uVar8 - 1;
      cVar3 = *pcVar16;
      pcVar16 = pcVar16 + 1;
    } while (cVar3 != '\0');
    if (piVar12 != (int *)0x0) {
      uVar9 = 0xffffffff;
      pcVar16 = DAT_00536820;
      do {
        pcVar15 = pcVar16;
        if (uVar9 == 0) break;
        uVar9 = uVar9 - 1;
        pcVar15 = pcVar16 + 1;
        cVar3 = *pcVar16;
        pcVar16 = pcVar15;
      } while (cVar3 != '\0');
      uVar9 = ~uVar9;
      piVar11 = (int *)(pcVar15 + -uVar9);
      piVar14 = piVar12;
      for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
        *piVar14 = *piVar11;
        piVar11 = piVar11 + 1;
        piVar14 = piVar14 + 1;
      }
      for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
        *(char *)piVar14 = (char)*piVar11;
        piVar11 = (int *)((uint)piVar11 + 1);
        piVar14 = (int *)((uint)piVar14 + 1);
      }
      local_34[2] = (int)piVar12;
      piVar12 = (int *)((uint)piVar12 + ~uVar8);
      local_34[1] = 1;
      *local_34 = 1;
    }
    local_38 = local_38 + ~uVar8;
  }
  if (piVar12 != (int *)0x0) {
    local_8[3] = 0;
    local_8[2] = 0;
  }
  return local_38;
}
