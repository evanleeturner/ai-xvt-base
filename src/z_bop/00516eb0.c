/* SPDX-License-Identifier: MIT */
/* z_bop 00516eb0 */
extern void *PTR_DirectDrawCreate_00bb93f8;

HRESULT __stdcall DirectDrawCreate(GUID *lpGUID, LPDIRECTDRAW *lplpDD, LPUNKNOWN pUnkOuter)
{
    return ((HRESULT (__stdcall *)(GUID *, LPDIRECTDRAW *, LPUNKNOWN))PTR_DirectDrawCreate_00bb93f8)(lpGUID, lplpDD, pUnkOuter);
}
