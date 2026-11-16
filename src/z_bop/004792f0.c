/* SPDX-License-Identifier: MIT */
/* z_bop 004792f0 */
int FUN_004792f0(int param_1, int param_2, int param_3)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;

  iVar3 = -1;
  iVar4 = 0;
  piVar5 = (int *)(*(int *)(param_1 + 0x14) + 4);
  if (0 < *(int *)(param_1 + 0x10)) {
    do {
      iVar1 = *piVar5;
      if ((param_2 == iVar1) && (piVar5[1] == param_3)) {
        iVar3 = piVar5[4];
        break;
      }
      iVar2 = piVar5[1];
      if (param_2 == iVar2) {
        if (param_3 == iVar1) {
          iVar3 = piVar5[4];
          break;
        }
        if ((param_2 == iVar2) && (piVar5[2] == param_3)) {
          iVar3 = piVar5[5];
          break;
        }
      }
      if ((param_2 == piVar5[2]) && (param_3 == iVar2)) {
        iVar3 = piVar5[5];
        break;
      }
      iVar2 = piVar5[3];
      if (iVar2 == -1) {
        if ((param_2 == iVar1) && (param_3 == piVar5[2])) {
          iVar3 = piVar5[6];
          break;
        }
        if ((param_2 == piVar5[2]) && (param_3 == iVar1)) {
          iVar3 = piVar5[6];
          break;
        }
      }
      else {
        if ((param_2 == piVar5[2]) && (param_3 == iVar2)) {
          iVar3 = piVar5[6];
          break;
        }
        if ((param_2 == iVar2) && (param_3 == piVar5[2])) {
          iVar3 = piVar5[6];
          break;
        }
        if ((param_2 == iVar1) && (param_3 == iVar2)) {
          iVar3 = piVar5[7];
          break;
        }
        if ((param_2 == iVar2) && (param_3 == iVar1)) {
          iVar3 = piVar5[7];
          break;
        }
      }
      piVar5 = piVar5 + 0x10;
      iVar4 = iVar4 + 1;
    } while (iVar4 < *(int *)(param_1 + 0x10));
  }
  if ((iVar3 == -1) || (iVar4 = iVar4 + 1, *(int *)(param_1 + 0x10) <= iVar4)) {
    return iVar3;
  }
  do {
    iVar1 = piVar5[0x10];
    if ((param_2 == iVar1) && (piVar5[0x11] == param_3)) {
      return -1;
    }
    iVar2 = piVar5[0x11];
    if (param_2 == iVar2) {
      if (param_3 == iVar1) {
        return -1;
      }
      if ((param_2 == iVar2) && (piVar5[0x12] == param_3)) {
        return -1;
      }
    }
    if ((param_2 == piVar5[0x12]) && (param_3 == iVar2)) {
      return -1;
    }
    iVar2 = piVar5[0x13];
    if (iVar2 == -1) {
      if ((param_2 == iVar1) && (param_3 == piVar5[0x12])) {
        return -1;
      }
      if ((param_2 == piVar5[0x12]) && (param_3 == iVar1)) {
        return -1;
      }
    }
    else {
      if ((param_2 == piVar5[0x12]) && (param_3 == iVar2)) {
        return -1;
      }
      if ((param_2 == iVar2) && (param_3 == piVar5[0x12])) {
        return -1;
      }
      if ((param_2 == iVar1) && (param_3 == iVar2)) {
        return -1;
      }
      if ((param_2 == iVar2) && (param_3 == iVar1)) {
        return -1;
      }
    }
    iVar4 = iVar4 + 1;
    piVar5 = piVar5 + 0x10;
    if (*(int *)(param_1 + 0x10) <= iVar4) {
      return iVar3;
    }
  } while (1);
}
