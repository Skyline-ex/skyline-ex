#include "lib.hpp"
#include "nn/fs.hpp"
/* Define hook StubCopyright. Trampoline indicates the original function should be kept. */
/* HOOK_DEFINE_REPLACE can be used if the original function does not need to be kept. */
// HOOK_DEFINE_TRAMPOLINE(StubCopyright) {

//     /* Define the callback for when the function is called. Don't forget to make it static and name it Callback. */
//     static void Callback(bool enabled) {

//         /* Call the original function, with the argument always being false. */
//         Orig(false);
//     }

// };

static int COUNT = 0;
void MountRomCallback(char const*, void*, unsigned long) {
    EXL_ASSERT(COUNT == 0);
    COUNT++;
}

void MountRomExInline(exl::hook::nx64::InlineCtx* ctx) {
    EXL_ASSERT(COUNT == 1);
    COUNT++;
}

void MountRomInline(exl::hook::nx64::InlineCtx* ctx) {
    EXL_ASSERT(COUNT == 2);
    COUNT++;
}

static Result (*MountRomOrig)(char const*, void*, unsigned long);

Result MountRomHook(char const* name, void* arg2, unsigned long arg3) {
    EXL_ASSERT(COUNT == 3);
    COUNT++;
    return MountRomOrig(name, arg2, arg3);
}

void PostMountHook(exl::hook::nx64::InlineCtx* ctx) {
    EXL_ASSERT(COUNT == 4);
}

/* Declare function to dynamic link with. */

extern "C" void exl_main(void* x0, void* x1) {
    /* Setup hooking enviroment. */
    envSetOwnProcessHandle(exl::util::proc_handle::Get());
    exl::hook::Initialize();
    /* Install the hook at the provided function pointer. Function type is checked against the callback function. */
    // StubCopyright::InstallAtFuncPtr(nn::oe::SetCopyrightVisibility);

    /* Alternative install funcs: */
    /* InstallAtPtr takes an absolute address as a uintptr_t. */
    /* InstallAtOffset takes an offset into the main module. */

    /*
    For sysmodules/applets, you have to call the entrypoint when ready
    exl::hook::CallTargetEntrypoint(x0, x1);
    */
}

extern "C" NORETURN void exl_exception_entry() {
    /* TODO: exception handling */
    EXL_ABORT(0x420);
}