/* SPDX-License-Identifier: MIT */
/* z_bop 00472360 */
extern float _DAT_005181d0;

void __cdecl FUN_00421300(float *param_1, float *param_2);
void __cdecl FUN_004210a0(float *param_1, float *param_2);
void __cdecl FUN_00420fe0(float *param_1, float *param_2);
void __cdecl FUN_004211d0(float *param_1, float *param_2);

void __cdecl FUN_00472360(int param_1, int param_2, int param_3, int param_4)
{
  float local_50[4];
  float local_40[16];

  local_50[0] = 0.0f;
  local_50[2] = 0.0f;
  local_50[3] = (float)*(unsigned char *)(*(int *)(*(int *)(param_2 + 0x1f) + 0xa9) + 0x30a + param_4) * _DAT_005181d0;
  local_50[1] = -1.0f;
  FUN_00421300(local_40, local_50);
  FUN_004210a0((float *)(param_3 + 0x44), local_40);
  FUN_00420fe0((float *)(param_3 + 0x38), local_40);
  FUN_004211d0((float *)(param_3 + 0x14), local_40);
  return;
}
