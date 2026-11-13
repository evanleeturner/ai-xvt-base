/* SPDX-License-Identifier: MIT */
/* z_xvt 00414d00 */
void __cdecl FUN_00414d00(int param_1, undefined4 *param_2, int param_3)
{
  float fVar1;
  float fVar2;
  float fVar3;
  
  *(undefined4 *)(param_1 + 0x10) = *param_2;
  fVar1 = *(float *)(param_1 + 0x10);
  *(undefined4 *)(param_1 + 0x14) = param_2[1];
  fVar2 = *(float *)(param_1 + 0x14);
  *(undefined4 *)(param_1 + 0x18) = param_2[2];
  fVar3 = *(float *)(param_1 + 0x18);
  *(float *)(param_1 + 0x10) =
       *(float *)(param_3 + 0xc) * fVar1 +
       *(float *)(param_3 + 0x18) * fVar2 + *(float *)(param_3 + 0x24) * fVar3;
  *(float *)(param_1 + 0x14) =
       *(float *)(param_3 + 0x10) * fVar1 +
       *(float *)(param_3 + 0x1c) * fVar2 + *(float *)(param_3 + 0x28) * fVar3;
  *(float *)(param_1 + 0x18) =
       fVar3 * *(float *)(param_3 + 0x2c) + *(float *)(param_3 + 0x20) * fVar2 +
       *(float *)(param_3 + 0x14) * fVar1;
  *(undefined4 *)(param_1 + 0x1c) = param_2[3];
  fVar1 = *(float *)(param_1 + 0x1c);
  *(undefined4 *)(param_1 + 0x20) = param_2[4];
  fVar2 = *(float *)(param_1 + 0x20);
  *(undefined4 *)(param_1 + 0x24) = param_2[5];
  fVar3 = *(float *)(param_1 + 0x24);
  *(float *)(param_1 + 0x1c) =
       *(float *)(param_3 + 0xc) * fVar1 +
       *(float *)(param_3 + 0x18) * fVar2 + *(float *)(param_3 + 0x24) * fVar3;
  *(float *)(param_1 + 0x20) =
       *(float *)(param_3 + 0x10) * fVar1 +
       *(float *)(param_3 + 0x28) * fVar3 + *(float *)(param_3 + 0x1c) * fVar2;
  *(float *)(param_1 + 0x24) =
       fVar3 * *(float *)(param_3 + 0x2c) + *(float *)(param_3 + 0x20) * fVar2 +
       *(float *)(param_3 + 0x14) * fVar1;
  return;
}
